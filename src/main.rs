pub mod bgfx;

#[repr(C)]
struct PosColorVertex {
    x: f32,
    y: f32,
    z: f32,
    abgr: u32,
}

extern fn callback() {
    bgfx::init(800, 600);

    let vertices = vec!(
        PosColorVertex { x: -1.0, y: 1.0, z: 1.0, abgr: 0xff000000 },
        PosColorVertex { x: 1.0, y: 1.0, z: 1.0, abgr: 0xff0000ff },
        PosColorVertex { x: -1.0, y: -1.0, z: 1.0, abgr: 0xff00ff00 },
        PosColorVertex { x: 1.0, y: -1.0, z: 1.0, abgr: 0xff00ffff },
        PosColorVertex { x: -1.0, y: 1.0, z: -1.0, abgr: 0xffff0000 },
        PosColorVertex { x: 1.0, y: 1.0, z: -1.0, abgr: 0xffff00ff },
        PosColorVertex { x: -1.0, y: -1.0, z: -1.0, abgr: 0xffffff00 },
        PosColorVertex { x: 1.0, y: -1.0, z: -1.0, abgr: 0xffffffff },
    );

    let indices = vec!(
        0u16, 1u16, 2u16, // 0
        1u16, 3u16, 2u16, // 1
        4u16, 6u16, 5u16, // 2
        5u16, 6u16, 7u16, // 3
        0u16, 2u16, 4u16, // 4
        4u16, 2u16, 6u16, // 5
        1u16, 5u16, 3u16, // 6
        5u16, 7u16, 3u16, // 7
        0u16, 4u16, 1u16, // 8
        4u16, 5u16, 1u16, // 9
        2u16, 3u16, 6u16, // 10
        6u16, 3u16, 7u16, // 11
    );

    let mut layout = bgfx::VertexLayout::new();
    
    layout
        .begin()
        .add(bgfx::Attrib::Position, 3, bgfx::AttribType::Float, false)
        .add(bgfx::Attrib::Color0, 4, bgfx::AttribType::Uint8, true)
        .end();

    let vbh = bgfx::create_vertex_buffer(&vertices, &layout);
    let ibh = bgfx::create_index_buffer(&indices);

    let program = bgfx::load_program("vs_cubes", "fs_cubes");

    let timer = bgfx::Timer::new();

    let mut mouse_state = bgfx::MouseState::new();

    while !bgfx::process_events(800, 600, bgfx::BGFX_DEBUG_TEXT, bgfx::BGFX_RESET_VSYNC, &mut mouse_state) {
        let at = bgfx::Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let eye = bgfx::Vec3 { x: 0.0, y: 0.0, z: -35.0 };

        let view = bgfx::mtx_lookat(&eye, &at);
        let proj = bgfx::mtx_proj(60.0, 800.0 / 600.0, 0.1, 100.0);

        bgfx::set_view_transform(0, &view, &proj);

        bgfx::set_view_rect(0, 0, 0, 800, 600);

        bgfx::touch(0);

        let state = 
            bgfx::BGFX_STATE_WRITE_R |
            bgfx::BGFX_STATE_WRITE_G |
            bgfx::BGFX_STATE_WRITE_B |
            bgfx::BGFX_STATE_WRITE_A |
            bgfx::BGFX_STATE_WRITE_Z |
            bgfx::BGFX_STATE_DEPTH_TEST_LESS |
            bgfx::BGFX_STATE_CULL_CW |
            bgfx::BGFX_STATE_MSAA;

        let mtx = bgfx::mtx_rotate_xy(0f32, 0f32);

        bgfx::set_transform(&mtx);

        bgfx::set_vertex_buffer(0, &vbh);
        bgfx::set_index_buffer(&ibh);

        bgfx::set_state(state);

        bgfx::submit(0, &program);

        // bgfx::dbg_text_clear(0, false);

        // bgfx::dbg_text_printf(0, 1, 0x0f, "Hello World!");

        bgfx::frame(false);
    }

    bgfx::shutdown();
}

fn main() {
    bgfx::set_main_callback(callback);

    bgfx::run();
}
