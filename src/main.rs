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
    println!("Hello World{}", "!");

    blogos::init();

    #[allow(unconditional_recursion)]
    #[allow(dead_code)]
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    //stack_overflow();

    //let ptr = 0xdeadbeaf as *mut u8;
    //unsafe { *ptr = 42; }

    //let ptr = 0x204bda as *mut u8;

    // read from a code page
    //unsafe { let x = *ptr; }
    //println!("read worked");

    // write to a code page
    //unsafe { *ptr = 42; }
    //println!("write worked");

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

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
