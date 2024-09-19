use core::alloc::Layout;
use core::ffi::{c_char, c_void, CStr};
use core::iter::zip;
use core::mem;
use core::ops::{BitOrAssign, Deref, DerefMut, Shl};
use core::ptr::{copy_nonoverlapping, null, NonNull};
use core::slice;

use std::alloc::{alloc, alloc_zeroed};
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};

use anyhow::{bail, ensure, Context as _};
use tracing::{instrument, trace, trace_span};
use tracing_subscriber::EnvFilter;
use wasmtime::component::{types, Resource, ResourceAny, ResourceType, Type, Val};
use wasmtime::Store;
use wasmtime_wasi::WasiView;

mod ffi;

static ENGINE: LazyLock<wasmtime::Engine> = LazyLock::new(wasmtime::Engine::default);

#[repr(C)]
#[derive(Debug)]
pub struct List<T> {
    pub ptr: *const T,
    pub len: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct Config {
    pub wasm: List<u8>,
}

pub struct Instance {
    instance: west::Instance,
    subscriber: Arc<dyn tracing::Subscriber + Send + Sync + 'static>,
}

impl Deref for Instance {
    type Target = west::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}

#[instrument(level = "trace")]
fn instantiate(config: Config) -> anyhow::Result<Instance> {
    let Config { wasm } = config;
    ensure!(!wasm.ptr.is_null(), "`wasm_ptr` must not be null");
    let wasm = unsafe { slice::from_raw_parts(wasm.ptr, wasm.len) };
    let instance = west::instantiate(west::Config {
        engine: ENGINE.clone(),
        wasm,
    })
    .context("failed to instantiate component")?;
    let subscriber = tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::from_env("WEST_LOG"))
        .finish();
    Ok(Instance {
        instance,
        subscriber: Arc::new(subscriber),
    })
}

#[instrument(level = "trace")]
fn align_of_record(ty: &types::Record) -> usize {
    ty.fields().map(|ty| align_of(&ty.ty)).max().unwrap_or(1)
}

#[instrument(level = "trace")]
fn align_of_tuple(ty: &types::Tuple) -> usize {
    ty.types().map(|ty| align_of(&ty)).max().unwrap_or(1)
}

#[instrument(level = "trace", skip_all)]
fn max_case_alignment<'a>(cases: impl IntoIterator<Item = types::Case<'a>>) -> usize {
    cases
        .into_iter()
        .filter_map(|types::Case { ty, .. }| ty.as_ref().map(align_of))
        .max()
        .unwrap_or(1)
}

#[instrument(level = "trace")]
fn align_of_variant(ty: &types::Variant) -> usize {
    let cases = ty.cases();
    let disc = match cases.len() {
        ..=0x0000_00ff => 1,
        0x0000_0100..=0x0000_ffff => 2,
        0x0001_0000.. => 4,
    };
    max_case_alignment(cases).max(disc)
}

#[instrument(level = "trace")]
fn align_of_option(ty: &types::OptionType) -> usize {
    align_of(&ty.ty())
}

#[instrument(level = "trace")]
fn align_of_result(ty: &types::ResultType) -> usize {
    let ok = ty.ok().as_ref().map_or(1, align_of);
    let err = ty.err().as_ref().map_or(1, align_of);
    ok.max(err)
}

#[instrument(level = "trace")]
fn align_of(ty: &Type) -> usize {
    match ty {
        Type::Bool | Type::S8 | Type::U8 => 1,
        Type::S16 | Type::U16 => 2,
        Type::S32 | Type::U32 | Type::Float32 | Type::Char | Type::Own(_) | Type::Borrow(_) => 4,
        Type::S64 | Type::U64 | Type::Float64 => 8,
        Type::String | Type::List(_) => mem::align_of::<(*const (), usize)>(),
        Type::Record(ty) => align_of_record(ty),
        Type::Tuple(ty) => align_of_tuple(ty),
        Type::Variant(ty) => align_of_variant(ty),
        Type::Enum(ty) => match ty.names().len() {
            ..=0x0000_00ff => 1,
            0x0000_0100..=0x0000_ffff => 2,
            0x0001_0000.. => 4,
        },
        Type::Option(ty) => align_of_option(ty),
        Type::Result(ty) => align_of_result(ty),
        Type::Flags(ty) => match ty.names().len() {
            ..=8 => 1,
            9..=16 => 2,
            _ => 4,
        },
    }
}

#[instrument(level = "trace")]
fn align_to(addr: usize, align: usize) -> usize {
    addr.div_ceil(align).saturating_mul(align)
}

#[instrument(level = "trace")]
fn size_of_record(ty: &types::Record) -> usize {
    let mut size = 0usize;
    for types::Field { ty, .. } in ty.fields() {
        size = align_to(size, align_of(&ty)).saturating_add(size_of(&ty));
    }
    align_to(size, align_of_record(ty))
}

#[instrument(level = "trace")]
fn size_of_tuple(ty: &types::Tuple) -> usize {
    let mut size = 0usize;
    for ty in ty.types() {
        size = align_to(size, align_of(&ty)).saturating_add(size_of(&ty));
    }
    align_to(size, align_of_tuple(ty))
}

#[instrument(level = "trace")]
fn size_of_variant(ty: &types::Variant) -> usize {
    let cases = ty.cases();
    let size: usize = match cases.len() {
        ..=0x0000_00ff => 1,
        0x0000_0100..=0x0000_ffff => 2,
        0x0001_0000.. => 4,
    };
    let size = align_to(size, max_case_alignment(ty.cases()));
    let size = size.saturating_add(
        cases
            .map(|types::Case { ty, .. }| ty.as_ref().map(size_of).unwrap_or_default())
            .max()
            .unwrap_or_default(),
    );
    align_to(size, align_of_variant(ty))
}

#[instrument(level = "trace")]
fn size_of_option(ty: &types::OptionType) -> usize {
    let size = size_of(&ty.ty()).saturating_add(1);
    align_to(size, align_of_option(ty))
}

#[instrument(level = "trace")]
fn size_of_result(ty: &types::ResultType) -> usize {
    let ok = ty.ok().as_ref().map(size_of).unwrap_or_default();
    let err = ty.err().as_ref().map(size_of).unwrap_or_default();
    let size = ok.max(err).saturating_add(1);
    align_to(size, align_of_result(ty))
}

#[instrument(level = "trace")]
fn size_of(ty: &Type) -> usize {
    match ty {
        Type::Bool | Type::S8 | Type::U8 => 1,
        Type::S16 | Type::U16 => 2,
        Type::S32 | Type::U32 | Type::Float32 | Type::Char | Type::Own(_) | Type::Borrow(_) => 4,
        Type::S64 | Type::U64 | Type::Float64 => 8,
        Type::String | Type::List(_) => mem::size_of::<(*const (), usize)>(),
        Type::Record(ty) => size_of_record(ty),
        Type::Tuple(ty) => size_of_tuple(ty),
        Type::Variant(ty) => size_of_variant(ty),
        Type::Enum(ty) => match ty.names().len() {
            ..=0x0000_00ff => 1,
            0x0000_0100..=0x0000_ffff => 2,
            0x0001_0000.. => 4,
        },
        Type::Option(ty) => size_of_option(ty),
        Type::Result(ty) => size_of_result(ty),
        Type::Flags(ty) => match ty.names().len() {
            ..=8 => 1,
            9..=16 => 2,
            _ => 4,
        },
    }
}

#[instrument(level = "trace")]
fn args_of_variant(ty: &types::Variant) -> usize {
    ty.cases()
        .map(|ty| ty.ty.map(|ty| args_of(&ty)).unwrap_or_default())
        .max()
        .unwrap_or_default()
        .saturating_add(1)
}

#[instrument(level = "trace")]
fn args_of_result(ty: &types::ResultType) -> usize {
    let ok = ty.ok().as_ref().map(args_of).unwrap_or_default();
    let err = ty.err().as_ref().map(args_of).unwrap_or_default();
    ok.max(err).saturating_add(1)
}

#[instrument(level = "trace")]
fn args_of(ty: &Type) -> usize {
    match ty {
        Type::Bool
        | Type::S8
        | Type::U8
        | Type::S16
        | Type::U16
        | Type::S32
        | Type::U32
        | Type::Float32
        | Type::Char
        | Type::Own(_)
        | Type::Borrow(_)
        | Type::S64
        | Type::U64
        | Type::Float64
        | Type::Enum(_)
        | Type::Flags(_) => 1,
        Type::String | Type::List(_) => 2,
        Type::Record(ty) => ty.fields().map(|ty| args_of(&ty.ty)).sum(),
        Type::Tuple(ty) => ty.types().map(|ty| args_of(&ty)).sum(),
        Type::Variant(ty) => args_of_variant(ty),
        Type::Option(ty) => args_of(&ty.ty()).saturating_add(1),
        Type::Result(ty) => args_of_result(ty),
    }
}

#[inline]
fn flag_bits<'a, T: BitOrAssign + Shl<u8, Output = T> + From<u8>>(
    names: impl IntoIterator<Item = &'a str>,
    flags: impl IntoIterator<Item = &'a str>,
) -> T {
    let mut v = T::from(0);
    let flags: HashSet<&str> = flags.into_iter().collect();
    for (i, name) in zip(0u8.., names) {
        if flags.contains(name) {
            v |= T::from(1) << i;
        }
    }
    v
}

fn find_enum_discriminant<'a, T>(
    iter: impl IntoIterator<Item = T>,
    names: impl IntoIterator<Item = &'a str>,
    disc: &str,
) -> anyhow::Result<T> {
    zip(iter, names)
        .find_map(|(i, name)| (name == disc).then_some(i))
        .context("unknown enum discriminant")
}

fn find_variant_discriminant<'a, T>(
    iter: impl IntoIterator<Item = T>,
    cases: impl IntoIterator<Item = types::Case<'a>>,
    disc: &str,
) -> anyhow::Result<(T, Option<Type>)> {
    zip(iter, cases)
        .find_map(|(i, types::Case { name, ty })| (name == disc).then_some((i, ty)))
        .context("unknown variant discriminant")
}

fn deref_arg<T>(args: *const *mut c_void) -> anyhow::Result<(NonNull<T>, *const *mut c_void)> {
    let args: NonNull<*mut c_void> =
        NonNull::new(args.cast_mut()).context("argument cannot be null")?;
    let data = unsafe { args.read() };
    let data = NonNull::new(data.cast()).context("value cannot be null")?;
    Ok((data, args.as_ptr().wrapping_add(1)))
}

#[instrument(level = "debug", skip(store, ty, src), ret(level = "debug"))]
fn lower(
    store: &mut Store<impl WasiView>,
    ty: &Type,
    dst: NonNull<c_void>,
    src: Val,
) -> anyhow::Result<*mut c_void> {
    match (src, ty) {
        (Val::Bool(val), Type::Bool) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::S8(val), Type::S8) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::U8(val), Type::U8) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::S16(val), Type::S16) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::U16(val), Type::U16) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::S32(val), Type::S32) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::U32(val), Type::U32) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::S64(val), Type::S64) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::U64(val), Type::U64) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::Float32(val), Type::Float32) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::Float64(val), Type::Float64) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::Char(val), Type::Char) => {
            let dst = dst.cast();
            unsafe { dst.write(val) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::String(val), Type::String) => {
            let dst = dst.cast::<(*const u8, usize)>();
            if val.is_empty() {
                unsafe { dst.write((null(), 0)) }
            } else {
                let len = val.len();
                let layout = Layout::from_size_align(len, 1)
                    .context("failed to construct string memory layout")?;
                trace!(?layout, "allocating string");
                let data = unsafe { alloc(layout) };
                ensure!(!data.is_null(), "failed to allocate list");
                unsafe { copy_nonoverlapping(val.as_ptr(), data, len) }
                trace!(?data, len, "writing string");
                unsafe { dst.write((data, len)) }
            }
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::List(vals), Type::List(ty)) => {
            let dst = dst.cast::<(*const c_void, usize)>();
            if vals.is_empty() {
                unsafe { dst.write((null(), 0)) }
            } else {
                let ty = ty.ty();
                let len = vals.len();
                let size = size_of(&ty).saturating_mul(len);
                let align = align_of(&ty);
                let layout = Layout::from_size_align(size, align)
                    .context("failed to construct list memory layout")?;
                trace!(?layout, "allocating list");
                let start = unsafe { alloc_zeroed(layout) }.cast::<c_void>();
                ensure!(!start.is_null(), "failed to allocate list");
                let mut data = start;
                for (i, val) in vals.into_iter().enumerate() {
                    let dst = NonNull::new(data)
                        .with_context(|| format!("list element `{i}` cannot be null"))?;
                    data = lower(store, &ty, dst, val)
                        .with_context(|| format!("failed to lower list element `{i}`"))?;
                }
                trace!(?start, len, "writing list");
                unsafe { dst.write((start, len)) }
            }
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        (Val::Record(vals), Type::Record(ty)) => {
            let mut data = dst.as_ptr();
            for (i, (ty, (_, val))) in zip(ty.fields(), vals).enumerate() {
                let offset = data.cast::<u8>().align_offset(align_of(&ty.ty));
                if offset > 0 {
                    data = data.wrapping_add(offset).cast();
                }
                let dst = NonNull::new(data)
                    .with_context(|| format!("record field `{i}` cannot be null"))?;
                data = lower(store, &ty.ty, dst, val)
                    .with_context(|| format!("failed to lower record field `{i}`"))?;
            }
            Ok(data)
        }
        (Val::Tuple(vals), Type::Tuple(ty)) => {
            let mut data = dst.as_ptr();
            for (i, (ty, val)) in zip(ty.types(), vals).enumerate() {
                let offset = data.cast::<u8>().align_offset(align_of(&ty));
                if offset > 0 {
                    data = data.wrapping_add(offset).cast();
                }
                let dst = NonNull::new(data)
                    .with_context(|| format!("tuple element `{i}` cannot be null"))?;
                data = lower(store, &ty, dst, val)
                    .with_context(|| format!("failed to lower tuple element `{i}`"))?;
            }
            Ok(data)
        }
        (Val::Variant(disc, val), Type::Variant(ty)) => {
            let cases = ty.cases();
            let data = dst.cast::<u8>().as_ptr().wrapping_add(size_of_variant(ty));
            let align = max_case_alignment(ty.cases());
            let (mut dst, ty) = match cases.len() {
                ..=0x0000_00ff => {
                    let (disc, ty) = find_variant_discriminant(0u8.., cases, &disc)?;
                    let dst = dst.cast();
                    unsafe { dst.write(disc) };
                    (dst.as_ptr().wrapping_add(1).cast::<c_void>(), ty)
                }
                0x0000_0100..=0x0000_ffff => {
                    let (disc, ty) = find_variant_discriminant(0u16.., cases, &disc)?;
                    let dst = dst.cast();
                    unsafe { dst.write(disc) };
                    (dst.as_ptr().wrapping_add(1).cast::<c_void>(), ty)
                }
                0x0001_0000..=0xffff_ffff => {
                    let (disc, ty) = find_variant_discriminant(0u32.., cases, &disc)?;
                    let dst = dst.cast();
                    unsafe { dst.write(disc) };
                    (dst.as_ptr().wrapping_add(1).cast::<c_void>(), ty)
                }
                0x1_0000_0000.. => bail!("variant case count does not fit in u32"),
            };
            if let Some((ty, val)) = ty.zip(val) {
                let offset = dst.cast::<u8>().align_offset(align);
                if offset > 0 {
                    dst = dst.wrapping_add(offset).cast();
                }
                let dst = NonNull::new(dst).with_context(|| {
                    format!("variant value for discriminant `{disc}` cannot be null")
                })?;
                lower(store, &ty, dst, *val).with_context(|| {
                    format!("failed to lower variant value for discriminant `{disc}`")
                })?;
            }
            Ok(data.cast())
        }
        (Val::Enum(disc), Type::Enum(ty)) => {
            let names = ty.names();
            match names.len() {
                ..=0x0000_00ff => {
                    let disc = find_enum_discriminant(0u8.., names, &disc)?;
                    let dst = dst.cast();
                    unsafe { dst.write(disc) };
                    Ok(dst.as_ptr().wrapping_add(1).cast())
                }
                0x0000_0100..=0x0000_ffff => {
                    let disc = find_enum_discriminant(0u16.., names, &disc)?;
                    let dst = dst.cast();
                    unsafe { dst.write(disc) };
                    Ok(dst.as_ptr().wrapping_add(1).cast())
                }
                0x0001_0000..=0xffff_ffff => {
                    let disc = find_enum_discriminant(0u32.., names, &disc)?;
                    let dst = dst.cast();
                    unsafe { dst.write(disc) };
                    Ok(dst.as_ptr().wrapping_add(1).cast())
                }
                0x1_0000_0000.. => bail!("enum name count does not fit in u32"),
            }
        }
        (Val::Option(val), Type::Option(ty)) => {
            let dst = dst.cast::<u8>();
            let data = dst.as_ptr().wrapping_add(size_of_option(ty));
            match val {
                None => {
                    unsafe { dst.write(0) };
                }
                Some(val) => {
                    unsafe { dst.write(1) };
                    let mut dst = dst.as_ptr().wrapping_add(1);
                    let offset = dst.align_offset(align_of(&ty.ty()));
                    if offset > 0 {
                        dst = dst.wrapping_add(offset);
                    }
                    let dst = NonNull::new(dst.cast())
                        .context("`option::some` payload cannot be null")?;
                    lower(store, &ty.ty(), dst, *val).context("failed to lower `option::some`")?;
                }
            }
            Ok(data.cast())
        }
        (Val::Result(val), Type::Result(ty)) => {
            let dst = dst.cast::<u8>();
            let data = dst.as_ptr().wrapping_add(size_of_result(ty));
            let align = align_of_result(ty);
            match val {
                Ok(val) => {
                    unsafe { dst.write(0) };
                    if let Some((ty, val)) = ty.ok().zip(val) {
                        let mut dst = dst.as_ptr().wrapping_add(1);
                        let offset = dst.align_offset(align);
                        if offset > 0 {
                            dst = dst.wrapping_add(offset);
                        }
                        let dst = NonNull::new(dst.cast())
                            .context("typed `result::ok` payload cannot be null")?;
                        lower(store, &ty, dst, *val).context("failed to lower `result::ok`")?;
                    }
                }
                Err(val) => {
                    unsafe { dst.write(1) };
                    if let Some((ty, val)) = ty.err().zip(val) {
                        let mut dst = dst.as_ptr().wrapping_add(1);
                        let offset = dst.align_offset(align);
                        if offset > 0 {
                            dst = dst.wrapping_add(offset);
                        }
                        let dst = NonNull::new(dst.cast())
                            .context("typed `result::err` payload cannot be null")?;
                        lower(store, &ty, dst, *val).context("failed to lower `result::error`")?;
                    }
                }
            }
            Ok(data.cast())
        }
        (Val::Flags(val), Type::Flags(ty)) => {
            let names = ty.names();
            let vs = val.iter().map(String::as_str);
            match names.len() {
                ..=8 => {
                    let v: u8 = flag_bits(names, vs);
                    let dst = dst.cast();
                    unsafe { dst.write(v) };
                    Ok(dst.as_ptr().wrapping_add(1).cast())
                }
                9..=16 => {
                    let v: u16 = flag_bits(names, vs);
                    let dst = dst.cast();
                    unsafe { dst.write(v) };
                    Ok(dst.as_ptr().wrapping_add(1).cast())
                }
                17..=32 => {
                    let v: u32 = flag_bits(names, vs);
                    let dst = dst.cast();
                    unsafe { dst.write(v) };
                    Ok(dst.as_ptr().wrapping_add(1).cast())
                }
                _ => bail!("flags with over 32 cases are not currently supported"),
            }
        }
        (Val::Resource(val), Type::Own(..) | Type::Borrow(..)) => {
            let dst = dst.cast::<u32>();
            let res = store.data_mut().table().push(val)?;
            unsafe { dst.write(res.rep()) };
            Ok(dst.as_ptr().wrapping_add(1).cast())
        }
        _ => bail!("type mismatch"),
    }
}

#[instrument(level = "debug", skip_all, ret(level = "debug"))]
fn lower_results(
    store: &mut Store<impl WasiView>,
    vals: Vec<Val>,
    tys: &[Type],
    args: *const *mut c_void,
) -> anyhow::Result<()> {
    if vals.is_empty() {
        return Ok(());
    }
    let args: NonNull<*mut c_void> =
        NonNull::new(args.cast_mut()).context("argument cannot be null")?;
    let data = unsafe { args.read() };
    zip(vals, tys)
        .enumerate()
        .try_fold(data, |data, (i, (val, ty))| -> anyhow::Result<_> {
            let data =
                NonNull::new(data).with_context(|| format!("result value {i} cannot be null"))?;
            lower(store, ty, data, val).with_context(|| format!("failed to lower result value {i}"))
        })?;
    Ok(())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_bool(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<u8>();
    let data = unsafe { src.read() };
    *dst = Val::Bool(data != 0);
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_s8(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<i8>();
    let data = unsafe { src.read() };
    *dst = Val::S8(data);
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_u8(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<u8>();
    let data = unsafe { src.read() };
    *dst = Val::U8(data);
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_s16(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<i16>();
    let data = unsafe { src.read() };
    *dst = Val::S16(data);
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_u16(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<u16>();
    let data = unsafe { src.read() };
    *dst = Val::U16(data);
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_s32(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<i32>();
    *dst = Val::S32(unsafe { src.read() });
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_u32(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<u32>();
    *dst = Val::U32(unsafe { src.read() });
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_s64(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<i64>();
    *dst = Val::S64(unsafe { src.read() });
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_u64(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<u64>();
    *dst = Val::U64(unsafe { src.read() });
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_f32(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<f32>();
    *dst = Val::Float32(unsafe { src.read() });
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_f64(dst: &mut Val, src: NonNull<c_void>) -> *const c_void {
    let src = src.cast::<f64>();
    *dst = Val::Float64(unsafe { src.read() });
    src.as_ptr().wrapping_add(1).cast()
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_char(dst: &mut Val, src: NonNull<c_void>) -> anyhow::Result<*const c_void> {
    let src = src.cast::<u32>();
    let data = unsafe { src.read() };
    let data = char::from_u32(data).with_context(|| format!("`{data}` is not a valid char"))?;
    *dst = Val::Char(data);
    Ok(src.as_ptr().wrapping_add(1).cast())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_string(dst: &mut Val, src: NonNull<c_void>) -> anyhow::Result<*const c_void> {
    let src = src.cast::<(*mut u8, usize)>();
    let (data, len) = unsafe { src.read() };
    if len > 0 {
        let data = NonNull::new(data).context("string data pointer cannot be null")?;
        let data = NonNull::slice_from_raw_parts(data, len);
        let data = String::from_utf8_lossy(unsafe { data.as_ref() });
        *dst = Val::String(data.into());
    } else {
        *dst = Val::String(String::default());
    }
    Ok(src.as_ptr().wrapping_add(1).cast())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_list(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::List,
) -> anyhow::Result<*const c_void> {
    let src = src.cast::<(*const c_void, usize)>();
    let (mut data, len) = unsafe { src.read() };
    if len > 0 {
        let mut vs = vec![Val::Bool(false); len];
        let ty = ty.ty();
        let align = align_of(&ty);
        for (i, v) in vs.iter_mut().enumerate() {
            let offset = data.cast::<u8>().align_offset(align);
            if offset > 0 {
                data = data.wrapping_add(offset).cast();
            }
            let src = NonNull::new(data.cast_mut())
                .with_context(|| format!("list element `{i}` pointer cannot be null"))?;
            data = lift(store, &ty, v, src)
                .with_context(|| format!("failed to lift list element `{i}`"))?;
        }
        *dst = Val::List(vs);
    } else {
        *dst = Val::List(Vec::default());
    }
    Ok(src.as_ptr().wrapping_add(1).cast())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_record(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::Record,
) -> anyhow::Result<*const c_void> {
    let fields = ty.fields();
    let mut vs = Vec::with_capacity(fields.len());
    let mut data = src.as_ptr().cast_const();
    for (i, ty) in fields.enumerate() {
        let mut v = Val::Bool(false);
        let offset = data.cast::<u8>().align_offset(align_of(&ty.ty));
        if offset > 0 {
            data = data.wrapping_add(offset).cast();
        }
        let src = NonNull::new(data.cast_mut())
            .with_context(|| format!("record field `{i}` pointer cannot be null"))?;
        data = lift(store, &ty.ty, &mut v, src)
            .with_context(|| format!("failed to lift record field `{i}`"))?;
        vs.push((ty.name.to_string(), v));
    }
    *dst = Val::Record(vs);
    Ok(data)
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_tuple(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::Tuple,
) -> anyhow::Result<*const c_void> {
    let types = ty.types();
    let mut vs = vec![Val::Bool(false); types.len()];
    let mut data = src.as_ptr().cast_const();
    for (i, (ty, v)) in zip(types, &mut vs).enumerate() {
        let offset = data.cast::<u8>().align_offset(align_of(&ty));
        if offset > 0 {
            data = data.wrapping_add(offset).cast();
        }
        let src = NonNull::new(data.cast_mut())
            .with_context(|| format!("tuple element `{i}` pointer cannot be null"))?;
        data = lift(store, &ty, v, src)
            .with_context(|| format!("failed to lift tuple element `{i}`"))?;
    }
    *dst = Val::Tuple(vs);
    Ok(data)
}

fn read_discriminant(src: NonNull<c_void>, cases: usize) -> anyhow::Result<(usize, *const c_void)> {
    match cases {
        ..=0x0000_00ff => {
            let data = src.cast::<u8>();
            let disc = unsafe { data.read() };
            Ok((disc.into(), data.as_ptr().wrapping_add(1).cast()))
        }
        0x0000_0100..=0x0000_ffff => {
            let data = src.cast::<u16>();
            let disc = unsafe { data.read() };
            Ok((disc.into(), data.as_ptr().wrapping_add(1).cast()))
        }
        0x0001_0000..=0xffff_ffff => {
            let data = src.cast::<u32>();
            let disc = unsafe { data.read() };
            let disc = disc
                .try_into()
                .with_context(|| format!("discriminant `{disc}` does not fit in usize"))?;
            Ok((disc, data.as_ptr().wrapping_add(1).cast()))
        }
        0x1_0000_0000.. => bail!("case count does not fit in u32"),
    }
}

fn read_variant_case(
    src: NonNull<c_void>,
    ty: &types::Variant,
) -> anyhow::Result<(types::Case, *const c_void)> {
    let mut cases = ty.cases();
    let (disc, src) =
        read_discriminant(src, cases.len()).context("failed to read variant discriminant")?;
    let ty = cases
        .nth(disc)
        .with_context(|| format!("unknown variant discriminant `{disc}`"))?;
    Ok((ty, src))
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_variant(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::Variant,
) -> anyhow::Result<*const c_void> {
    let size = size_of_variant(ty);
    let align = max_case_alignment(ty.cases());
    let (ty, mut data) = read_variant_case(src, ty)?;
    let name = ty.name.to_string();
    if let Some(ty) = ty.ty {
        let mut v = Val::Bool(false);
        let offset = data.cast::<u8>().align_offset(align);
        if offset > 0 {
            data = data.wrapping_add(offset).cast();
        }
        let data = NonNull::new(data.cast_mut())
            .with_context(|| format!("variant value cannot be null for variant case `{name}`"))?;
        lift(store, &ty, &mut v, data)
            .with_context(|| format!("failed to lift variant value for variant case `{name}`"))?;
        *dst = Val::Variant(name, Some(Box::new(v)));
    } else {
        *dst = Val::Variant(name, None);
    }
    Ok(src.as_ptr().cast::<u8>().wrapping_add(size).cast())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_enum(
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::Enum,
) -> anyhow::Result<*const c_void> {
    let mut names = ty.names();
    let (disc, src) =
        read_discriminant(src, names.len()).context("failed to read enum discriminant")?;
    let name = names
        .nth(disc)
        .with_context(|| format!("unknown enum discriminant `{disc}`"))?;
    *dst = Val::Enum(name.to_string());
    Ok(src)
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_flags(
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::Flags,
) -> anyhow::Result<*const c_void> {
    let names = ty.names();
    let (bits, src) = match names.len() {
        ..=8 => {
            let data = src.cast::<u8>();
            let disc = unsafe { data.read() };
            (disc.into(), data.as_ptr().wrapping_add(1).cast())
        }
        9..=16 => {
            let data = src.cast::<u16>();
            let disc = unsafe { data.read() };
            (disc.into(), data.as_ptr().wrapping_add(1).cast())
        }
        17..=32 => {
            let data = src.cast::<u32>();
            let disc = unsafe { data.read() };
            (disc, data.as_ptr().wrapping_add(1).cast())
        }
        _ => bail!("flags with over 32 cases are not currently supported"),
    };
    let mut vs = Vec::with_capacity(bits.count_ones().try_into().unwrap_or(usize::MAX));
    for (i, name) in zip(0.., names) {
        if bits & (1 << i) != 0 {
            vs.push(name.to_string());
        }
    }
    *dst = Val::Flags(vs);
    Ok(src)
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_option(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::OptionType,
) -> anyhow::Result<*const c_void> {
    let src = src.cast::<u8>();
    let data = src.as_ptr().wrapping_add(size_of_option(ty));
    match unsafe { src.read() } {
        0 => {
            *dst = Val::Option(None);
        }
        1 => {
            let mut v = Val::Bool(false);
            let mut src = src.as_ptr().wrapping_add(1);
            let offset = src.align_offset(align_of(&ty.ty()));
            if offset > 0 {
                src = src.wrapping_add(offset);
            }
            let src = NonNull::new(src.cast()).context("`option::some` payload cannot be null")?;
            lift(store, &ty.ty(), &mut v, src)?;
            *dst = Val::Option(Some(Box::new(v)));
        }
        disc => bail!("invalid option discriminant value {disc}"),
    }
    Ok(data.cast())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_result(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &types::ResultType,
) -> anyhow::Result<*const c_void> {
    let src = src.cast::<u8>();
    let data = src.as_ptr().wrapping_add(size_of_result(ty));
    let align = align_of_result(ty);
    match unsafe { src.read() } {
        0 => {
            if let Some(ty) = ty.ok() {
                let mut v = Val::Bool(false);
                let mut src = src.as_ptr().wrapping_add(1);
                let offset = src.align_offset(align);
                if offset > 0 {
                    src = src.wrapping_add(offset);
                }
                let src =
                    NonNull::new(src.cast()).context("`result::ok` payload cannot be null")?;
                lift(store, &ty, &mut v, src)?;
                *dst = Val::Result(Ok(Some(Box::new(v))));
            } else {
                *dst = Val::Result(Ok(None));
            }
        }
        1 => {
            if let Some(ty) = ty.err() {
                let mut v = Val::Bool(false);
                let mut src = src.as_ptr().wrapping_add(1);
                let offset = src.align_offset(align);
                if offset > 0 {
                    src = src.wrapping_add(offset);
                }
                let src =
                    NonNull::new(src.cast()).context("`result::err` payload cannot be null")?;
                lift(store, &ty, &mut v, src)?;
                *dst = Val::Result(Err(Some(Box::new(v))));
            } else {
                *dst = Val::Result(Err(None));
            }
        }
        disc => bail!("invalid result discriminant value {disc}"),
    }
    Ok(data.cast())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_own(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &ResourceType,
) -> anyhow::Result<*const c_void> {
    let src = src.cast::<u32>();
    let rep = unsafe { src.read() };
    let res = store
        .data_mut()
        .table()
        .delete::<ResourceAny>(Resource::new_own(rep))
        .context("failed to delete resource from table")?;
    ensure!(*ty == res.ty());
    *dst = Val::Resource(res);
    Ok(src.as_ptr().wrapping_add(1).cast())
}

#[instrument(level = "trace", skip_all, ret(level = "trace"))]
fn lift_borrow(
    store: &mut Store<impl WasiView>,
    dst: &mut Val,
    src: NonNull<c_void>,
    ty: &ResourceType,
) -> anyhow::Result<*const c_void> {
    let src = src.cast::<u32>();
    let rep = unsafe { src.read() };
    let res = store
        .data_mut()
        .table()
        .get::<ResourceAny>(&Resource::new_borrow(rep))?;
    ensure!(*ty == res.ty());
    *dst = Val::Resource(*res);
    Ok(src.as_ptr().wrapping_add(1).cast())
}

#[instrument(level = "debug", skip_all, ret(level = "debug"))]
fn lift(
    store: &mut Store<impl WasiView>,
    ty: &Type,
    dst: &mut Val,
    src: NonNull<c_void>,
) -> anyhow::Result<*const c_void> {
    match ty {
        Type::Bool => Ok(lift_bool(dst, src)),
        Type::S8 => Ok(lift_s8(dst, src)),
        Type::U8 => Ok(lift_u8(dst, src)),
        Type::S16 => Ok(lift_s16(dst, src)),
        Type::U16 => Ok(lift_u16(dst, src)),
        Type::S32 => Ok(lift_s32(dst, src)),
        Type::U32 => Ok(lift_u32(dst, src)),
        Type::S64 => Ok(lift_s64(dst, src)),
        Type::U64 => Ok(lift_u64(dst, src)),
        Type::Float32 => Ok(lift_f32(dst, src)),
        Type::Float64 => Ok(lift_f64(dst, src)),
        Type::Char => lift_char(dst, src),
        Type::String => lift_string(dst, src),
        Type::List(ty) => lift_list(store, dst, src, ty),
        Type::Record(ty) => lift_record(store, dst, src, ty),
        Type::Tuple(ty) => lift_tuple(store, dst, src, ty),
        Type::Variant(ty) => lift_variant(store, dst, src, ty),
        Type::Enum(ty) => lift_enum(dst, src, ty),
        Type::Option(ty) => lift_option(store, dst, src, ty),
        Type::Result(ty) => lift_result(store, dst, src, ty),
        Type::Flags(ty) => lift_flags(dst, src, ty),
        Type::Own(ty) => lift_own(store, dst, src, ty),
        Type::Borrow(ty) => lift_borrow(store, dst, src, ty),
    }
}

#[instrument(level = "debug", skip_all, ret(level = "debug"))]
fn lift_param(
    store: &mut Store<impl WasiView>,
    ty: &Type,
    val: &mut Val,
    args: *const *mut c_void,
) -> anyhow::Result<*const *mut c_void> {
    match ty {
        Type::Bool => {
            let (data, args) = deref_arg::<i32>(args)?;
            let data = unsafe { data.read() };
            ensure!(data >= 0);
            *val = Val::Bool(data != 0);
            Ok(args)
        }
        Type::S8 => {
            let (data, args) = deref_arg::<i32>(args)?;
            let data = unsafe { data.read() };
            let data = data
                .try_into()
                .with_context(|| format!("s8 value `{data}` does not fit in i8"))?;
            *val = Val::S8(data);
            Ok(args)
        }
        Type::U8 => {
            let (data, args) = deref_arg::<u32>(args)?;
            let data = unsafe { data.read() };
            let data = data
                .try_into()
                .with_context(|| format!("u8 value `{data}` does not fit in u8"))?;
            *val = Val::U8(data);
            Ok(args)
        }
        Type::S16 => {
            let (data, args) = deref_arg::<i32>(args)?;
            let data = unsafe { data.read() };
            let data = data
                .try_into()
                .with_context(|| format!("s16 value `{data}` does not fit in i16"))?;
            *val = Val::S16(data);
            Ok(args)
        }
        Type::U16 => {
            let (data, args) = deref_arg::<u32>(args)?;
            let data = unsafe { data.read() };
            let data = data
                .try_into()
                .with_context(|| format!("u16 value `{data}` does not fit in u16"))?;
            *val = Val::U16(data);
            Ok(args)
        }
        Type::S32 => {
            let (data, args) = deref_arg::<c_void>(args)?;
            lift_s32(val, data);
            Ok(args)
        }
        Type::U32 => {
            let (data, args) = deref_arg::<c_void>(args)?;
            lift_u32(val, data);
            Ok(args)
        }
        Type::S64 => {
            let (data, args) = deref_arg::<c_void>(args)?;
            lift_s64(val, data);
            Ok(args)
        }
        Type::U64 => {
            let (data, args) = deref_arg::<c_void>(args)?;
            lift_u64(val, data);
            Ok(args)
        }
        Type::Float32 => {
            let (data, args) = deref_arg::<c_void>(args)?;
            lift_f32(val, data);
            Ok(args)
        }
        Type::Float64 => {
            let (data, args) = deref_arg::<c_void>(args)?;
            lift_f64(val, data);
            Ok(args)
        }
        Type::Char => {
            let (data, args) = deref_arg::<c_void>(args)?;
            lift_char(val, data).context("failed to lift char")?;
            Ok(args)
        }
        Type::String => {
            let args = NonNull::new(args.cast_mut()).context("argument cannot be null")?;
            let data = unsafe { args.read() };
            let args = args.as_ptr().wrapping_add(1);
            let (len, args) = deref_arg::<u32>(args)?;
            let len = unsafe { len.read() };
            let len = len
                .try_into()
                .with_context(|| format!("string length value `{len}` does not fit in usize"))?;
            if len > 0 {
                let data =
                    NonNull::new(data.cast()).context("string data pointer cannot be null")?;
                let data = NonNull::slice_from_raw_parts(data, len);
                let data = String::from_utf8_lossy(unsafe { data.as_ref() });
                *val = Val::String(data.into());
            } else {
                *val = Val::String(String::default());
            }
            Ok(args)
        }
        Type::List(ty) => {
            let args = NonNull::new(args.cast_mut()).context("argument cannot be null")?;
            let data = unsafe { args.read() };
            let args = args.as_ptr().wrapping_add(1);
            let (len, args) = deref_arg::<u32>(args)?;
            let len = unsafe { len.read() };
            if len > 0 {
                let len = len
                    .try_into()
                    .with_context(|| format!("list length value `{len}` does not fit in usize"))?;
                let mut vs = vec![Val::Bool(false); len];
                let ty = ty.ty();
                let align = align_of(&ty);
                let mut data = data.cast_const();
                for (i, dst) in vs.iter_mut().enumerate() {
                    let offset = data.cast::<u8>().align_offset(align);
                    if offset > 0 {
                        data = data.wrapping_add(offset).cast();
                    }
                    let src = NonNull::new(data.cast_mut())
                        .with_context(|| format!("list element `{i}` cannot be null"))?;
                    data = lift(store, &ty, dst, src)
                        .with_context(|| format!("failed to lift list element `{i}`"))?;
                }
                *val = Val::List(vs);
            } else {
                *val = Val::List(Vec::default());
            }
            Ok(args)
        }
        Type::Record(ty) => {
            let fields = ty.fields();
            let mut vs = Vec::with_capacity(fields.len());
            let mut args = args;
            for (i, ty) in fields.enumerate() {
                let mut v = Val::Bool(false);
                args = lift_param(store, &ty.ty, &mut v, args)
                    .with_context(|| format!("failed to lift record field `{i}`"))?;
                vs.push((ty.name.to_string(), v));
            }
            *val = Val::Record(vs);
            Ok(args)
        }
        Type::Tuple(ty) => {
            let types = ty.types();
            let mut vs = vec![Val::Bool(false); types.len()];
            let mut args = args;
            for (i, (ty, v)) in zip(types, &mut vs).enumerate() {
                args = lift_param(store, &ty, v, args)
                    .with_context(|| format!("failed to lift tuple element `{i}`"))?;
            }
            *val = Val::Tuple(vs);
            Ok(args)
        }
        Type::Variant(ty) => {
            let (disc, data) = deref_arg::<c_void>(args)?;
            let args = args.wrapping_add(args_of_variant(ty));
            let (ty, _) = read_variant_case(disc, ty)?;
            let name = ty.name.to_string();
            if let Some(ty) = ty.ty {
                let mut v = Val::Bool(false);
                lift_param(store, &ty, &mut v, data).with_context(|| {
                    format!("failed to lift variant value for variant case `{name}`")
                })?;
                *val = Val::Variant(name, Some(Box::new(v)));
            } else {
                *val = Val::Variant(name, None);
            }
            Ok(args)
        }
        Type::Enum(ty) => {
            let (disc, args) = deref_arg::<c_void>(args)?;
            lift_enum(val, disc, ty)?;
            Ok(args)
        }
        Type::Option(ty) => {
            let (disc, args) = deref_arg::<u32>(args)?;
            match unsafe { disc.read() } {
                0 => {
                    *val = Val::Option(None);
                    Ok(args.wrapping_add(args_of(&ty.ty())))
                }
                1 => {
                    let mut v = Val::Bool(false);
                    let args = lift_param(store, &ty.ty(), &mut v, args)?;
                    *val = Val::Option(Some(Box::new(v)));
                    Ok(args)
                }
                disc => bail!("invalid option discriminant value {disc}"),
            }
        }
        Type::Result(ty) => {
            let (disc, args) = deref_arg::<u32>(args)?;
            let ok = ty.ok();
            let err = ty.err();
            match unsafe { disc.read() } {
                0 => {
                    if let Some(ty) = ok.as_ref() {
                        let mut v = Val::Bool(false);
                        lift_param(store, ty, &mut v, args)?;
                        *val = Val::Result(Ok(Some(Box::new(v))));
                    } else {
                        *val = Val::Result(Ok(None));
                    }
                }
                1 => {
                    if let Some(ty) = err.as_ref() {
                        let mut v = Val::Bool(false);
                        lift_param(store, ty, &mut v, args)?;
                        *val = Val::Result(Err(Some(Box::new(v))));
                    } else {
                        *val = Val::Result(Err(None));
                    }
                }
                disc => bail!("invalid result discriminant value {disc}"),
            }
            Ok(args.wrapping_add(
                ok.as_ref()
                    .map(args_of)
                    .unwrap_or_default()
                    .max(err.as_ref().map(args_of).unwrap_or_default()),
            ))
        }

        Type::Flags(ty) => {
            let (bits, args) = deref_arg::<c_void>(args)?;
            lift_flags(val, bits, ty)?;
            Ok(args)
        }
        Type::Own(ty) => {
            let (rep, args) = deref_arg::<c_void>(args)?;
            lift_own(store, val, rep, ty)?;
            Ok(args)
        }
        Type::Borrow(ty) => {
            let (rep, args) = deref_arg::<c_void>(args)?;
            lift_borrow(store, val, rep, ty)?;
            Ok(args)
        }
    }
}

#[instrument(level = "debug", skip_all, ret(level = "debug"))]
fn lift_params(
    store: &mut Store<impl WasiView>,
    tys: &[Type],
    args: *const *mut c_void,
) -> anyhow::Result<(Vec<Val>, *const *mut c_void)> {
    if tys.is_empty() {
        return Ok((vec![], args));
    }
    let mut vals = vec![Val::Bool(false); tys.len()];
    let results = zip(&mut vals, tys).enumerate().try_fold(
        args,
        |args, (i, (val, ty))| -> anyhow::Result<_> {
            lift_param(store, ty, val, args)
                .with_context(|| format!("failed to lift parameter {i}"))
        },
    )?;
    Ok((vals, results))
}

#[instrument(level = "debug", ret(level = "debug"))]
fn call(
    instance_ptr: *mut c_void,
    instance: *const c_char,
    name: *const c_char,
    args: *const *mut c_void,
) -> anyhow::Result<()> {
    let mut inst =
        NonNull::new(instance_ptr.cast::<Instance>()).context("`instance_ptr` must not be null")?;
    ensure!(!instance.is_null(), "`instance` must not be null");
    ensure!(!name.is_null(), "`name` must not be null");
    let instance = unsafe { CStr::from_ptr(instance) }
        .to_str()
        .context("`instance` is not valid UTF-8")?;
    let name = unsafe { CStr::from_ptr(name) }
        .to_str()
        .context("`name` is not valid UTF-8")?;
    let inst = unsafe { inst.as_mut() };
    let _log = tracing::subscriber::set_default(Arc::clone(&inst.subscriber));
    let _span = trace_span!("call", "instance" = instance, "name" = name).entered();
    if let Some(ty) = name.strip_prefix("[resource-drop]") {
        let (rep, _) = deref_arg::<u32>(args)?;
        let rep = unsafe { rep.read() };
        let store = inst.store();
        let res = WasiView::table(store.data_mut())
            .delete::<ResourceAny>(Resource::new_own(rep))
            .with_context(|| format!("failed to delete `{ty}` from table"))?;
        res.resource_drop(store)
            .with_context(|| format!("failed to drop `{ty}`"))?;
    } else {
        let mut func = inst
            .func(instance, name)
            .context("failed to lookup function")?;
        let tys = func.params();
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
