//! User LED PA2

use stm32f40x::{GPIOA, RCC};

/// LED connected to pin PA2
pub const LED: PA2 = PA2;

/// Pin PA2. There's an LED connected to this pin
pub struct PA2;

/// Initializes the user LED
pub fn init(gpioa: &GPIOA, rcc: &RCC) {
    // power on GPIOA
    rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

    // configure PA2 as output
    gpioa.moder.modify(|_, w| w.moder2().bits(1));
}

impl PA2 {
    /// Turns the LED on
    pub fn on(&self) {
        unsafe {
            (*GPIOA.get()).odr.modify(|_, w| w.odr2().bit(true));
        }
    }

    /// Turns the LED off
    pub fn off(&self) {
        unsafe {
            (*GPIOA.get()).odr.modify(|_, w| w.odr2().bit(false));
        }
    }

    /// True if LED is ON, false otherwise.
    pub fn is_on(&self) -> bool {
        unsafe { (*GPIOA.get()).odr.read().odr2().bit_is_set() }
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