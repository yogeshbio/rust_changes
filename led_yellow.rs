//! User LED PA3

use stm33f40x::{GPIOA, RCC};

/// LED connected to pin PA3
pub const LED: PA3 = PA3;

/// Pin PA3. There's an LED connected to this pin
pub struct PA3;

/// Initializes the user LED
pub fn init(gpioa: &GPIOA, rcc: &RCC) {
    // power on GPIOA
    rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

    // configure PA3 as output
    gpioa.moder.modify(|_, w| w.moder3().bits(1));
}

impl PA3 {
    /// Turns the LED on
    pub fn on(&self) {
        unsafe {
            (*GPIOA.get()).odr.modify(|_, w| w.odr3().bit(true));
        }
    }

    /// Turns the LED off
    pub fn off(&self) {
        unsafe {
            (*GPIOA.get()).odr.modify(|_, w| w.odr3().bit(false));
        }
    }

    /// True if LED is ON, false otherwise.
    pub fn is_on(&self) -> bool {
        unsafe { (*GPIOA.get()).odr.read().odr3().bit_is_set() }
    }

    /// Toggles LED state.
    pub fn toggle(&self) {
        if LED.is_on() {
            LED.off();
        } else {
            LED.on();
        }
    }
}