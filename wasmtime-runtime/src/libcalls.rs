//! Runtime library calls. Note that wasm compilers may sometimes perform these
//! inline rather than calling them, particularly when CPUs have special
//! instructions which compute them directly.

use crate::vmcontext::VMContext;
use cranelift_wasm::{DefinedMemoryIndex, MemoryIndex};
use std::time::{Duration, SystemTime};

/// Implementation of f32.ceil
pub extern "C" fn wasmtime_f32_ceil(x: f32) -> f32 {
    x.ceil()
}

/// Implementation of f32.floor
pub extern "C" fn wasmtime_f32_floor(x: f32) -> f32 {
    x.floor()
}

/// Implementation of f32.trunc
pub extern "C" fn wasmtime_f32_trunc(x: f32) -> f32 {
    x.trunc()
}

/// Implementation of f32.nearest
#[allow(clippy::float_arithmetic, clippy::float_cmp)]
pub extern "C" fn wasmtime_f32_nearest(x: f32) -> f32 {
    // Rust doesn't have a nearest function, so do it manually.
    if x == 0.0 {
        // Preserve the sign of zero.
        x
    } else {
        // Nearest is either ceil or floor depending on which is nearest or even.
        let u = x.ceil();
        let d = x.floor();
        let um = (x - u).abs();
        let dm = (x - d).abs();
        if um < dm
            || (um == dm && {
                let h = u / 2.;
                h.floor() == h
            })
        {
            u
        } else {
            d
        }
    }
}

/// Implementation of f64.ceil
pub extern "C" fn wasmtime_f64_ceil(x: f64) -> f64 {
    x.ceil()
}

/// Implementation of f64.floor
pub extern "C" fn wasmtime_f64_floor(x: f64) -> f64 {
    x.floor()
}

/// Implementation of f64.trunc
pub extern "C" fn wasmtime_f64_trunc(x: f64) -> f64 {
    x.trunc()
}

/// Implementation of f64.nearest
#[allow(clippy::float_arithmetic, clippy::float_cmp)]
pub extern "C" fn wasmtime_f64_nearest(x: f64) -> f64 {
    // Rust doesn't have a nearest function, so do it manually.
    if x == 0.0 {
        // Preserve the sign of zero.
        x
    } else {
        // Nearest is either ceil or floor depending on which is nearest or even.
        let u = x.ceil();
        let d = x.floor();
        let um = (x - u).abs();
        let dm = (x - d).abs();
        if um < dm
            || (um == dm && {
                let h = u / 2.;
                h.floor() == h
            })
        {
            u
        } else {
            d
        }
    }
}

/// Implementation of memory.grow for locally-defined 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmtime_memory32_grow(
    vmctx: *mut VMContext,
    delta: u32,
    memory_index: u32,
) -> u32 {
    let instance = (&mut *vmctx).instance();
    let memory_index = DefinedMemoryIndex::from_u32(memory_index);

    instance
        .memory_grow(memory_index, delta)
        .unwrap_or(u32::max_value())
}

/// Implementation of memory.grow for imported 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmtime_imported_memory32_grow(
    vmctx: *mut VMContext,
    delta: u32,
    memory_index: u32,
) -> u32 {
    let instance = (&mut *vmctx).instance();
    let memory_index = MemoryIndex::from_u32(memory_index);

    instance
        .imported_memory_grow(memory_index, delta)
        .unwrap_or(u32::max_value())
}

/// Implementation of memory.size for locally-defined 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmtime_memory32_size(vmctx: *mut VMContext, memory_index: u32) -> u32 {
    let instance = (&mut *vmctx).instance();
    let memory_index = DefinedMemoryIndex::from_u32(memory_index);

    instance.memory_size(memory_index)
}

/// Implementation of memory.size for imported 32-bit memories.
#[no_mangle]
pub unsafe extern "C" fn wasmtime_imported_memory32_size(
    vmctx: *mut VMContext,
    memory_index: u32,
) -> u32 {
    let instance = (&mut *vmctx).instance();
    let memory_index = MemoryIndex::from_u32(memory_index);

    instance.imported_memory_size(memory_index)
}

static mut C: u32 = 0;
/// instruction hook
pub extern "C" fn wasmtime_instruction_hook(namespace: u32, index: u32) {
    unsafe {
        C += 1;
        print!("i f u{}:{} {}\n", namespace, index, C);
    }
}

/// function hook
pub extern "C" fn wasmtime_func_hook_enter(namespace: u32, index: u32) {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("s u{}:{} {}", namespace, index, n.as_millis()),
        Err(_) => panic!("Error getting time"),
    }
}

/// function hook
pub extern "C" fn wasmtime_func_hook_exit(namespace: u32, index: u32) {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => println!("t u{}:{} {}", namespace, index, n.as_millis()),
        Err(_) => panic!("Error getting time"),
    }

}
