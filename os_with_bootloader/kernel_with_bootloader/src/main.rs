#![no_std]
#![no_main]
// #[macro_use]
// #[no_mangle]

mod writer;

use bootloader_api::config::Mapping;
use writer::FrameBufferWriter;
use x86_64::instructions::hlt;

//Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
//optionally pass a custom config
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    let mut frame_buffer_writer=
     FrameBufferWriter::new(buffer, frame_buffer_info);
     
     use core::fmt::Write;//below requires this 
     writeln!(frame_buffer_writer, "Testing testing {} and {}", 1, 4.0/2.0).unwrap();

     macro_rules! print {
        ($($a: expr), *) => {
            write!(frame_buffer_writer, $($a), *).unwrap();
        };
    }
    macro_rules! println {
        ($($a: expr), *) => {
            writeln!(frame_buffer_writer, $($a), *).unwrap();
        };
    }

    print!("Hello from print {}", 25.0/5.0);
    println!("Hello from println!");
    print!("Hello from print\n");

    frame_buffer_writer.change_cursor_position(100, 120);
    println!("My name is Tireni");
    println!("I love Rust!");

    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
    }

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}

// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     //This function is the entry point, since the linker looks for a function
//     //named '_start' by default regardless of the host OS
//     println!("Hello World!");
// }

// #[macro_use]
// framebuffer_print!("Hello, {}!", "world");

