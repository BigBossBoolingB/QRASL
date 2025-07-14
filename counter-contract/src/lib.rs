#![no_std]

use core::panic::PanicInfo;

extern "C" {
    fn set_storage(key_ptr: *const u8, value_ptr: *const u8);
}

#[no_mangle]
pub extern "C" fn main() {
    let key = [1; 32];
    let value = [1; 32];
    unsafe {
        set_storage(key.as_ptr(), value.as_ptr());
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
