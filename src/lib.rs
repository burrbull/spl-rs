//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]

pub use embedded_hal as hal;

#[cfg(feature = "stm32f103")]
pub use stm32f1xx_hal::{self as device_hal, device};

pub mod prelude;

pub mod deinit;

pub mod adc;
//pub mod base;
//pub mod dac;
pub mod dbgmcu;
//pub mod rcc;
//pub mod i2c;
pub mod crc;
//pub mod wwdg;
pub mod rtc;
//pub mod iwdg;

pub mod spi;

pub mod dma;

//pub mod tim;
