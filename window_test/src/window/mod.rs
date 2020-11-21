#[derive(Debug)]
pub struct WindowPtrData {

}

pub trait Window {
    fn is_open(&self) -> bool;
    fn poll_events(&self);
    fn create_window(name: &str, width: i32, height: i32) -> Self where Self: Sized;
}
