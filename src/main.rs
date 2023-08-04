#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blogos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blogos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blogos::init();

    #[allow(unconditional_recursion)]
    #[allow(dead_code)]
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    //stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blogos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blogos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blogos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
