#![no_std]
#![no_main]

extern crate panic_halt;
extern crate riscv;
extern crate riscv_rt;

use riscv::register::{mie, mip, mhartid};
use riscv::asm::wfi;
use riscv_rt::entry;

#[export_name = "_mp_hook"]
pub extern "Rust" fn user_mp_hook() -> bool {
    let hartid = mhartid::read();
    if hartid == 0 {
        true
    } else {
        let addr = 0x02000000 + hartid * 4;
        unsafe {
            // Clear IPI
            (addr as *mut u32).write_volatile(0);

            // Start listening for software interrupts
            mie::set_msoft();

            loop {
                wfi();
                if mip::read().msoft() {
                    break;
                }
            }

            // Start listening for software interrupts
            mie::clear_msoft();

            // Clear IPI
            (addr as *mut u32).write_volatile(0);
        }
        false
    }
}

#[entry]
fn main() -> ! {
    let hartid = mhartid::read();

    if hartid == 0 {
        // Waking hart 1...
        let addr = 0x02000004;
        unsafe {
            (addr as *mut u32).write_volatile(1);
        }
    }

    loop { }
}
