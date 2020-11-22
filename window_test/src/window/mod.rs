
pub struct BitmapBuffer<'a> {
    pub width: i32,
    pub height: i32,
    pub bitmap: &'a mut [u32]
}

pub trait Window {
    fn is_open(&self) -> bool;
    fn poll_events(&self);
    fn update_buffer(&mut self, buffer: &mut [u32], width: i32, height: i32);
    fn get_bitmap(&self) -> BitmapBuffer;
    
    fn create_window(name: &str, width: i32, height: i32) -> Self where Self: Sized;
}
