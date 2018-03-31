#![feature(lang_items, const_fn, ptr_internals)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;

use core::fmt::Write;

#[macro_use]
mod vga;

#[no_mangle]
pub extern "C" fn rust_main() {

    // ATTENTION: we have a very small stack and no guard page
    vga::clear_screen();
    println!("Hello World{}", "!");
    println!("This message is written in Rust!");
    
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
