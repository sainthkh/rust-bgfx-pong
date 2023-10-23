extern crate libc;

use libc::c_char;
use libc::c_int;
use std::ffi::CString;

pub const BGFX_DEBUG_TEXT: u32 = 0x08;

pub const BGFX_RESET_VSYNC: u32 = 0x80;

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum MouseButton {
    None = 0u8,
    Left = 1u8,
    Middle = 2u8,
    Right = 3u8,

    Count,
}

#[repr(C)]
pub struct MouseState {
    pub mx: i32,
    pub my: i32,
    pub mz: i32,
    pub m_buttons: [MouseButton; 4],
}

impl MouseState {
    pub fn new () -> MouseState {
        MouseState {
            mx: 0,
            my: 0,
            mz: 0,
            m_buttons: [MouseButton::None; 4],
        }
    }
}

#[link(name = "bgfxRelease", kind = "static")]
extern "C" {
    fn sa_init(width: i32, height: i32) -> ();
    fn sa_set_main_callback(cb: extern fn()) -> ();
    fn sa_run(argc: c_int, argv: *const *const c_char) -> ();
    fn sa_process_events(width: u32, height: u32, debug: u32, reset: u32, mouse: &mut MouseState) -> bool;
    fn sa_touch(id: u32) -> ();
    fn bgfx_set_view_rect(id: u8, x: u16, y: u16, width: u16, height: u16) -> ();
    fn bgfx_dbg_text_clear(attr: u8, small: bool) -> ();
    fn bgfx_dbg_text_printf(x: u16, y: u16, attr: u8, format: *const c_char) -> ();
    fn bgfx_frame(capture: bool) -> ();
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

pub fn process_events(width: u32, height: u32, debug: u32, reset: u32, mouse: &mut MouseState) -> bool {
    unsafe {
        sa_process_events(width, height, debug, reset, mouse)
    }
}

pub fn touch(id: u32) -> () {
    unsafe {
        sa_touch(id)
    }
}

pub fn set_view_rect(id: u8, x: u16, y: u16, width: u16, height: u16) -> () {
    unsafe {
        bgfx_set_view_rect(id, x, y, width, height)
    }
}

pub fn dbg_text_clear(attr: u8, small: bool) -> () {
    unsafe {
        bgfx_dbg_text_clear(attr, small)
    }
}

pub fn dbg_text_printf(x: u16, y: u16, attr: u8, str: &str) -> () {
    let c_format = CString::new(str).unwrap();

    unsafe {
        bgfx_dbg_text_printf(x, y, attr, c_format.as_ptr() as *const c_char)
    }
}

pub fn frame(capture: bool) -> () {
    unsafe {
        bgfx_frame(capture)
    }
}

pub fn shutdown() {
    unsafe {
        bgfx_shutdown()
    }
}
