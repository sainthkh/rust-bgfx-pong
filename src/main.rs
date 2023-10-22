pub mod bgfx;

extern fn callback() {
    bgfx::init(800, 600);

    bgfx::shutdown();
}

fn main() {
    bgfx::set_main_callback(callback);

    bgfx::run();
}
