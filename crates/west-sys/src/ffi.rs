use core::ffi::{c_char, c_void};
use core::ptr::{self, null_mut};

use std::ffi::CString;
use std::sync::{LazyLock, Mutex};

use crate::{call, instantiate, Config, List, PASSTHROUGH_LEN, PASSTHROUGH_PTR};

static ERROR: LazyLock<Mutex<Option<CString>>> = LazyLock::new(Mutex::default);

fn store_error(err: anyhow::Error) {
    let _ = ERROR
        .lock()
        .unwrap()
        .insert(CString::new(format!("{err:?}")).expect("failed to construct error string"));
}

#[no_mangle]
pub extern "C" fn default_config() -> Config {
    Config {
        wasm: List {
            ptr: PASSTHROUGH_PTR,
            len: PASSTHROUGH_LEN,
        },
    }
}

#[no_mangle]
pub extern "C" fn error_take(buf: *mut c_char, len: usize) -> usize {
    if let Some(err) = ERROR.lock().unwrap().take() {
        let len = err.count_bytes().saturating_add(1).min(len);
        unsafe { ptr::copy_nonoverlapping(err.as_ptr(), buf, len) };
        len
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn error_len() -> usize {
    if let Some(err) = ERROR.lock().unwrap().as_ref() {
        err.count_bytes().saturating_add(1)
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn instance_new(config: Config) -> *mut c_void {
    match instantiate(config) {
        Ok(instance) => Box::into_raw(Box::new(instance)).cast(),
        Err(err) => {
            store_error(err);
            null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn instance_free(instance: *mut c_void) {
    unsafe { drop(Box::from_raw(instance)) }
}

#[no_mangle]
pub extern "C" fn instance_call(
    instance_ptr: *mut c_void,
    instance: *const c_char,
    name: *const c_char,
    args: *const *mut c_void,
) -> bool {
    match call(instance_ptr, instance, name, args) {
        Ok(()) => true,
        Err(err) => {
            store_error(err);
            false
        }
    }
}
