mod win32_window;
mod window;

#[cfg(target_os ="windows")]
use win32_window::create_window;
use window::Window;
use std::convert::TryInto;

fn main() {
    let window = create_window("Test Window", 800, 600);
    while window.is_open() {
        let bitmap = window.get_bitmap();

        let width = bitmap.width;
        let height = bitmap.height;
        println!("{}, {}", width, height);

        for x in 0..width {
            for y in 0..height {
                let index: usize = (x + width * y).try_into().unwrap();
                println!("{}", index);
                bitmap.bitmap[index] = 145;
            }

        }
        window.poll_events();
    }
}
