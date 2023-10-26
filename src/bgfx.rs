extern crate libc;

use libc::c_char;
use libc::c_int;
use libc::c_void;
use std::ffi::CString;

pub const BGFX_DEBUG_TEXT: u32 = 0x08;

pub const BGFX_RESET_VSYNC: u32 = 0x80;

pub const BGFX_STATE_WRITE_R: u64 =  0x0000000000000001;
pub const BGFX_STATE_WRITE_G: u64 =  0x0000000000000002;
pub const BGFX_STATE_WRITE_B: u64 =  0x0000000000000004;
pub const BGFX_STATE_WRITE_A: u64 =  0x0000000000000008;
pub const BGFX_STATE_WRITE_Z: u64 = 0x0000004000000000;
pub const BGFX_STATE_DEPTH_TEST_LESS: u64 = 0x0000000000000010;
pub const BGFX_STATE_CULL_CW: u64 = 0x0000001000000000;
pub const BGFX_STATE_MSAA: u64 = 0x0100000000000000;

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

pub enum RendererType {
    Noop,         
    Agc,         
    Direct3D9,    
    Direct3D11,  
    Direct3D12,  
    Gnm,         
    Metal,       
    Nvn,         
    OpenGLES,    
    OpenGL,     
    Vulkan,     
    WebGPU,     

    Count
}

#[derive(Copy, Clone)]
#[repr(u16)]
pub enum Attrib {
    Position, 
    Normal,   
    Tangent,   
    Bitangent, 
    Color0,    
    Color1,    
    Color2,    
    Color3,    
    Indices,   
    Weight,    
    TexCoord0, 
    TexCoord1, 
    TexCoord2, 
    TexCoord3, 
    TexCoord4, 
    TexCoord5, 
    TexCoord6, 
    TexCoord7,

    Count,
}

#[derive(Copy, Clone)]
pub enum AttribType {
    Uint8,  
    Uint10, 
    Int16,  
    Half,   
    Float,  

    Count
}

#[repr(C)]
pub struct VertexLayout {
    hash: u32,
    stride: u16,
    offset: [Attrib; 18],
    attributes: [Attrib; 18],
}

impl VertexLayout {
    pub fn new() -> VertexLayout {
        VertexLayout {
            hash: 0,
            stride: 0,
            offset: [Attrib::Position; 18],
            attributes: [Attrib::Position; 18],
        }
    }

    pub fn begin(&mut self) -> &mut Self {
        unsafe {
            bgfx_vertex_layout_begin(self as *mut _, 0);
        }

        self
    }

    pub fn add(&mut self, attrib: Attrib, num: u8, attrib_type: AttribType, normalized: bool) -> &mut Self {
        unsafe {
            bgfx_vertex_layout_add(self as *mut _, attrib, num, attrib_type as i32, normalized, false);
        }

        self
    }

    pub fn end(&mut self) -> &mut Self {
        unsafe {
            bgfx_vertex_layout_end(self as *mut _);
        }

        self
    }
}

pub struct Timer {
    offset: i64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            offset: unsafe { sa_hp_counter() },
        }
    }

    pub fn elapsed(&self) -> f32 {
        let now = unsafe { sa_hp_counter() };
        let freq = unsafe { sa_hp_frequency() };

        ((now - self.offset) as f64 / freq as f64) as f32
    }
}

#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct VertexBufferHandle {
    idx: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IndexBufferHandle {
    idx: u16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProgramHandle {
    idx: u16,
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

    fn bgfx_vertex_layout_begin(layout: *mut VertexLayout, renderer_type: i32) -> ();
    fn bgfx_vertex_layout_add(layout: *mut VertexLayout, attrib: Attrib, num: u8, attrib_type: i32, normalized: bool, as_int: bool) -> ();
    fn bgfx_vertex_layout_end(layout: *mut VertexLayout) -> ();

    fn sa_create_vertex_buffer(data: *const c_void, size: u32, layout: *const VertexLayout) -> VertexBufferHandle;
    fn bgfx_set_vertex_buffer(stream: u8, handle: VertexBufferHandle) -> ();
    fn sa_create_index_buffer(data: *const c_void, size: u32) -> IndexBufferHandle;
    fn bgfx_set_index_buffer(handle: IndexBufferHandle) -> ();

    #[link_name = "load_program"]
    fn bgfx_load_program(vs_name: *const c_char, fs_name: *const c_char) -> ProgramHandle;

    fn sa_hp_counter() -> i64;
    fn sa_hp_frequency() -> i64;

    fn sa_mtx_lookat(result: *mut f32, eye: *const Vec3, at: *const Vec3) -> ();
    fn sa_mtx_proj(result: *mut f32, fovy: f32, aspect: f32, near: f32, far: f32) -> ();
    fn sa_mtx_rotate_xy(result: *mut f32, x: f32, y: f32) -> ();

    fn bgfx_set_state(state: u64, rgba: u32) -> ();
    fn bgfx_submit(id: u8, program: ProgramHandle, depth: u32, flags: u8) -> ();

    fn bgfx_set_view_transform(id: u8, view: *const f32, proj: *const f32) -> ();
    fn bgfx_set_transform(transform: *const f32, num: u16) -> ();
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

pub fn create_vertex_buffer<T>(data: &Vec<T>, layout: &VertexLayout) -> VertexBufferHandle {
    let size = (data.len() * std::mem::size_of::<T>()) as u32;
    
    unsafe {
        sa_create_vertex_buffer(data.as_ptr() as *const c_void, size, layout as *const VertexLayout)
    }
}

pub fn set_vertex_buffer(stream: u8, handle: &VertexBufferHandle) -> () {
    unsafe {
        bgfx_set_vertex_buffer(stream, handle.clone())
    }
}

pub fn create_index_buffer<T>(data: &Vec<T>) -> IndexBufferHandle {
    let size = (data.len() * std::mem::size_of::<T>()) as u32;
    
    unsafe {
        sa_create_index_buffer(data.as_ptr() as *const c_void, size)
    }
}

pub fn set_index_buffer(handle: &IndexBufferHandle) -> () {
    unsafe {
        bgfx_set_index_buffer(handle.clone())
    }
}

pub fn load_program(vs_name: &str, fs_name: &str) -> ProgramHandle {
    let c_vs_name = CString::new(vs_name).unwrap();
    let c_fs_name = CString::new(fs_name).unwrap();

    unsafe {
        bgfx_load_program(c_vs_name.as_ptr() as *const c_char, c_fs_name.as_ptr() as *const c_char)
    }
}

pub fn mtx_lookat(eye: &Vec3, at: &Vec3) -> [f32; 16] {
    let mut result = [0.0f32; 16];

    unsafe {
        sa_mtx_lookat(result.as_mut_ptr(), eye as *const Vec3, at as *const Vec3)
    }

    result
}

pub fn mtx_proj(fovy: f32, aspect: f32, near: f32, far: f32) -> [f32; 16] {
    let mut result = [0.0f32; 16];

    unsafe {
        sa_mtx_proj(result.as_mut_ptr(), fovy, aspect, near, far)
    }

    result
}

pub fn mtx_rotate_xy(x: f32, y: f32) -> [f32; 16] {
    let mut result = [0.0f32; 16];

    unsafe {
        sa_mtx_rotate_xy(result.as_mut_ptr(), x, y)
    }

    result
}

pub fn set_view_transform(id: u8, view: &[f32; 16], proj: &[f32; 16]) -> () {
    unsafe {
        bgfx_set_view_transform(id, view.as_ptr(), proj.as_ptr())
    }
}

pub fn set_transform(transform: &[f32; 16]) -> () {
    unsafe {
        bgfx_set_transform(transform.as_ptr(), 1)
    }
}

pub fn set_state(state: u64) -> () {
    unsafe {
        bgfx_set_state(state, 0)
    }
}

pub fn submit(id: u8, program: &ProgramHandle) -> () {
    unsafe {
        bgfx_submit(id, program.clone(), 0, 0xff)
    }
}
