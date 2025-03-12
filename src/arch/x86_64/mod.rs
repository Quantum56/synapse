//use alloc::sync::Arc;
use limine::request::{
    DateAtBootRequest, FramebufferRequest, HhdmRequest, ExecutableFileRequest, MemoryMapRequest, StackSizeRequest
};

pub mod cpu_local;

use x86_64::instructions::interrupts;

//use crate::{
//    backtrace
//};

static KERNEL_FILE: ExecutableFileRequest = ExecutableFileRequest::new();

pub fn arch_main() {
    interrupts::disable();

    assert!(!interrupts::are_enabled());
    
    let kernel_file = KERNEL_FILE.get_response().expect("Error getting kernel binary from Limine");
    let kernel_file_base = kernel_file.file();
    let kernel_file_len = kernel_file_base.size() as usize;
    let kernel_file_data = unsafe {
        core::slice::from_raw_parts(kernel_file_base, kernel_file_len)
    };

    
    //backtrace::KERNEL_ELF.call_once()
}