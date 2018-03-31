#![feature(lang_items, const_fn, ptr_internals)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

use core::fmt::Write;

#[macro_use]
mod vga;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {

    // ATTENTION: we have a very small stack and no guard page
    vga::clear_screen();

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }
    
    loop{}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> !
{
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
