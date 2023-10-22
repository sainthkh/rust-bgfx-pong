extern crate libc;

use libc::c_char;
use libc::c_int;
use std::ffi::CString;

#[link(name = "bgfxRelease", kind = "static")]
extern "C" {
    fn sa_init(width: i32, height: i32) -> ();
    fn sa_set_main_callback(cb: extern fn()) -> ();
    fn sa_run(argc: c_int, argv: *const *const c_char) -> ();
    fn bgfx_shutdown() -> ();
}

pub fn init(width: i32, height: i32) -> () {
    unsafe {
        sa_init(width, height)
    }
}

pub fn set_main_callback(cb: extern fn()) -> () {
    unsafe {
        sa_set_main_callback(cb)
    }
}

pub fn run() -> () {
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

    unsafe {
        sa_run(c_args.len() as c_int, c_args.as_ptr())
    }
}

pub fn shutdown() {
    unsafe {
        bgfx_shutdown()
    }
}
