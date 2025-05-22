use core::ffi::{c_char, c_void, CStr};
use core::ptr::NonNull;
use core::slice;

use std::sync::{Arc, LazyLock, Mutex};

use anyhow::{bail, ensure, Context as _};
use tracing::{instrument, trace_span};
use tracing_subscriber::EnvFilter;
use wasmtime::component::{Resource, ResourceAny, Val};
use wasmtime_cabish::{deref_arg, lift_params, lower_results, CabishView};

mod ffi;

#[repr(C)]
#[derive(Debug)]
pub struct List<T> {
    pub ptr: *const T,
    pub len: usize,
}

static ENGINE: LazyLock<wasmtime::Engine> = LazyLock::new(wasmtime::Engine::default);

#[repr(C)]
#[derive(Debug)]
pub struct Config {
    pub wasm: List<u8>,
}

pub struct Instance {
    instance: Mutex<wadge::Instance>,
    subscriber: Arc<dyn tracing::Subscriber + Send + Sync + 'static>,
}

#[instrument(level = "trace")]
fn instantiate(config: Config) -> anyhow::Result<Instance> {
    let Config { wasm } = config;
    ensure!(!wasm.ptr.is_null(), "`wasm_ptr` must not be null");
    let wasm = unsafe { slice::from_raw_parts(wasm.ptr, wasm.len) };
    let instance = wadge::instantiate(wadge::Config {
        engine: ENGINE.clone(),
        wasm,
    })
    .context("failed to instantiate component")?;
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::from_env("WADGE_LOG"))
        .finish();
    Ok(Instance {
        instance: instance.into(),
        subscriber: Arc::new(subscriber),
    })
}

#[instrument(level = "debug", ret(level = "debug"))]
fn call(
    instance_ptr: *mut c_void,
    instance: *const c_char,
    name: *const c_char,
    args: *const *mut c_void,
) -> anyhow::Result<()> {
    let inst =
        NonNull::new(instance_ptr.cast::<Instance>()).context("`instance_ptr` must not be null")?;
    ensure!(!instance.is_null(), "`instance` must not be null");
    ensure!(!name.is_null(), "`name` must not be null");
    let instance = unsafe { CStr::from_ptr(instance) }
        .to_str()
        .context("`instance` is not valid UTF-8")?;
    let name = unsafe { CStr::from_ptr(name) }
        .to_str()
        .context("`name` is not valid UTF-8")?;
    let inst = unsafe { inst.as_ref() };
    let _log = tracing::subscriber::set_default(Arc::clone(&inst.subscriber));
    let Ok(mut inst) = inst.instance.lock() else {
        bail!("failed to lock instance mutex")
    };
    let _span = trace_span!("call", "instance" = instance, "name" = name).entered();
    if let Some(ty) = name.strip_prefix("[resource-drop]") {
        let (rep, _) = deref_arg::<u32>(args)?;
        let rep = unsafe { rep.read() };
        let store = inst.store();
        let res = store
            .data_mut()
            .table()
            .delete::<ResourceAny>(Resource::new_own(rep))
            .with_context(|| format!("failed to delete `{ty}` from table"))?;
        res.resource_drop(store)
            .with_context(|| format!("failed to drop `{ty}`"))?;
    } else {
        let mut func = inst
            .func(instance, name)
            .context("failed to lookup function")?;
        let tys = func
            .params()
            .iter()
            .map(|(_, ty)| ty.clone())
            .collect::<Vec<_>>();
        let (params, args) =
            lift_params(func.store(), &tys, args).context("failed to lift parameters")?;
        let results_ty = func.results();
        let mut results = vec![Val::Bool(false); results_ty.len()];
        func.call(&params, &mut results)?;
        lower_results(func.store(), results, &results_ty, args)
            .context("failed to lower results")?;
    }
    Ok(())
}
