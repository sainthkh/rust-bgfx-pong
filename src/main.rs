pub mod bgfx;

extern fn callback() {
    bgfx::init(800, 600);

    let mut mouse_state = bgfx::MouseState::new();

    while !bgfx::process_events(800, 600, bgfx::BGFX_DEBUG_TEXT, bgfx::BGFX_RESET_VSYNC, &mut mouse_state) {
        bgfx::set_view_rect(0, 0, 0, 800, 600);

        bgfx::touch(0);

        bgfx::dbg_text_clear(0, false);

        bgfx::dbg_text_printf(0, 1, 0x0f, "Hello World!");

        bgfx::frame(false);
    }

    bgfx::shutdown();
}

fn main() {
    bgfx::set_main_callback(callback);

    bgfx::run();
}
