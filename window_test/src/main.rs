mod win32_window;
mod window;

#[cfg(target_os ="windows")]
use win32_window::create_window;
use window::Window;

fn main() {
    let window = create_window("Test Window", 800, 600);
    while window.is_open() {
        window.poll_events();
    }
}