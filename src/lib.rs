#![feature(lang_items, const_fn, ptr_internals)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;

use core::fmt::Write;

mod vga;

#[no_mangle]
pub extern "C" fn rust_main() {

    vga::WRITER.lock().write_str("Hello again\n\n");
    write!(vga::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    
    loop{}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
