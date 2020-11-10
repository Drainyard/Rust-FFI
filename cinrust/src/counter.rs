// Use u8 slice for opaque structs
#[derive(Copy, Clone)] #[repr(C)] struct Counter { _private : [u8 ; 0] }

extern "C" {
    fn _count_up(c: *mut Counter);
    fn _create_counter(initial: i32) -> *mut Counter;
    fn _free_counter(c: *mut Counter);
    fn _count(c: *mut Counter) -> i32;
}

impl Counter {
    fn count_up(&mut self) {
        unsafe {
            _count_up(self);
        }
    }

    fn count(&mut self) -> i32{
        unsafe {
            _count(self)
        }
    }

    fn free_counter(&mut self) {
        unsafe {
            _free_counter(self);
        }
    }

    fn create_counter(initial: i32) -> &'static mut Counter {
        unsafe {
            &mut (*_create_counter(initial))
        }
    }
}
