//#![deny(missing_docs)]
//#![deny(warnings)]
#![no_std]

pub use embedded_hal as ehal;

#[cfg(feature = "stm32f103")]
pub use stm32f1xx_hal::{self as hal, pac};

pub mod prelude;

pub mod deinit;

#[cfg(feature="adc")]
pub mod adc;
//pub mod base;
#[cfg(feature="dac")]
pub mod dac;
pub mod dbgmcu;
//pub mod rcc;
//#[cfg(feature="i2c")]
//pub mod i2c;
#[cfg(feature="crc")]
pub mod crc;
pub mod wwdg;
#[cfg(feature="rtc")]
pub mod rtc;
#[cfg(feature="iwdg")]
pub mod iwdg;

#[cfg(feature="spi")]
pub mod spi;

#[cfg(feature="dma")]
pub mod dma;

//#[cfg(feature="timer")]
//pub mod tim;
