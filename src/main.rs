#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m::asm;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//Registers:
//System tick - control and status register:
const SYST_CSR: *mut u32 = 0xE000_E010 as *mut u32;
//System tick - reload value register:
const SYST_RVR: *mut u32 = 0xE000_E014 as *mut u32;
//System tick - current value register:
const SYST_CVR: *mut u32 = 0xE000_E018 as *mut u32;

// Configuration of Timers:
// CPU frequency is 12.5MHz by default
const CPU_FREQ: u32 = 12_500_000;

// Run-Mode clock Configuration (RCC):
const RCC: u32 = 0x400FE060;
// Prescaler value:
const SYSCTL_SYSDIV_12: u32 = 0xB; // Check Datasheet for other prescaler values
const SYSCTL_SYSDIV_16: u32 = 0xF; //Different Prescaler value just change "SYSCTL_SYSDIV_12" to "SYSCTL_SYSDIV_16"

#[entry]
fn main() -> ! {
    hprintln!("Starting program!");

    //Set Prescaler value for RCC
    unsafe {
        let sysdiv = SYSCTL_SYSDIV_12 << 23; // change here for running faster or slower interrupts. at the
                                             // moment the interrupt is triggering slightly faster then a second with SYSCTL_SYSDIV_12
        let orig = *(RCC as *const u32);
        let mask: u32 = !0b1111 << 23;
        let rcc = (orig & mask) | sysdiv;
        *(RCC as *mut u32) = rcc;
    }

    let sleep_dur: u32 = CPU_FREQ;

    unsafe {
        // Set the timer duration ( set delaytime to 1sec ):
        *SYST_RVR = sleep_dur;
        // Clear the current value by writing to the register:
        *SYST_CVR = 0;
        // Enable the timer:
        *SYST_CSR = 0b111;
    }

    loop {
        asm::wfi();
    }
}
#[exception]
fn SysTick() {
    hprintln!("ugh, woke up :(")
}
