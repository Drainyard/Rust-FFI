#[derive(Debug)]
pub struct WindowPtrData {
    pub running: bool
}

pub trait Window {
    fn is_running(&self) -> bool;
    fn poll_events(&self);
    fn create_window(name: &str) -> Self where Self: Sized;
}
