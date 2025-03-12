#![no_std]
#![no_main]
#![feature(
    lang_items,
    abi_x86_interrupt,
    naked_functions,
    ptr_internals,
    slice_pattern,
    iter_advance_by,
    alloc_error_handler,
)]
#![allow(internal_features)]
#![allow(clippy::missing_safety_doc)]
#![deny(unsafe_op_in_unsafe_fn)]

#![macro_use]
pub mod vga_graphics;

pub mod arch;

//use syn_syscall;

use core::panic::PanicInfo;

use spin::Once;

pub static PHYSICAL_OFFSET: Once<usize> = Once::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//use mem::addr::VirtAddr;

//pub fn phys_offset() -> VirtAddr {
//    return;
//}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_start() {

    arch::arch_main();

    loop {
        hcf();
    }
}
pub fn hcf() {
    arch::x86_64::cpu_local::hcf();
}