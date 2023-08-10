#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blogos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blogos::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{structures::paging::Translate, VirtAddr};
    use blogos::memory;

    println!("Hello World{}", "!");
    blogos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blogos::hlt_loop();
}

/// This function is called on panic.
// #[cfg(not(test))]
#[cfg(any(not(test), rust_analyzer))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blogos::hlt_loop();
}

//#[cfg(test)]
#[cfg(all(test, not(rust_analyzer)))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blogos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
