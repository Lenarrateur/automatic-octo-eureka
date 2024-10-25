
#![no_std]
#![no_main]

use core::ptr;

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;

#[derive(Clone, Copy)]
pub enum PinMode {
    Input,
    Output,
}

#[derive(Clone, Copy)]
pub enum PinState {
    High,
    Low,
}

fn modify_reg(reg: *mut u8, pin: u8, set: bool) {
    let value = unsafe { ptr::read_volatile(reg) };
    unsafe { ptr::write_volatile(reg, if set { value | (1 << pin) } else { value & !(1 << pin) }) };
}

pub fn set_pin_mode(pin: u8, mode: PinMode) {
    modify_reg(DDRB, pin, matches!(mode, PinMode::Output));
}

pub fn write_pin(pin: u8, state: PinState) {
    modify_reg(PORTB, pin, matches!(state, PinState::High));
}

pub fn read_pin(pin: u8) -> PinState {
    if unsafe { ptr::read_volatile(PINB) } & (1 << pin) != 0 {
        PinState::High
    } else {
        PinState::Low
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() {}