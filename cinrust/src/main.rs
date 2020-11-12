mod win32;
mod window;

#[cfg(target_os ="windows")]
use win32::create_window;
use window::Window;

fn main() {
    let window = create_window("Test Window");
    while window.is_running() {
        window.poll_events();
    }

}
