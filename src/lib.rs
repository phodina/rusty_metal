#![feature(lang_items, const_fn, ptr_internals)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;

use multiboot2::{BootInformation, MemoryMapTag, ElfSectionsTag};

#[macro_use]
mod vga;
mod memory;
use memory::area_frame_allocator::AreaFrameAllocator;
use memory::FrameAllocator;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {

    vga::clear_screen();

    let boot_info = get_boot_info(multiboot_information_address);
    let elf_sections_tag = get_elf_sections(&boot_info);
    
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    let memory_map_tag = get_memory_map_tag(&boot_info);
    
    let mut frame_allocator = AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());

    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
    }
    
    println!("Kernel start: 0x{:x} end: 0x{:x}", kernel_start, kernel_end);
    println!("Multiboot start: 0x{:x} end: 0x{:x}", multiboot_start, multiboot_end);

    loop{}
}

fn get_boot_info(multiboot_information_address: usize) -> &'static BootInformation {
    unsafe{ multiboot2::load(multiboot_information_address) }
}

fn get_memory_map_tag(boot_info: &'static BootInformation) -> &'static MemoryMapTag {

    boot_info.memory_map_tag()
        .expect("Memory map tag required")
}

fn print_memory_sections(memory_map_tag: &MemoryMapTag) {

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }
}

fn get_elf_sections(boot_info: &'static BootInformation) -> &'static ElfSectionsTag {
    boot_info.elf_sections_tag()
        .expect("Elf-sections tag required")    
}

fn print_elf_sections(elf_sections_tag: &'static ElfSectionsTag) {
    
    println!("kernel sections:");

    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                 section.addr, section.size, section.flags);
    }
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
