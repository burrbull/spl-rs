#[cfg(feature="adc")]
pub use crate::adc::{AdcStruct, AdcChannel};
#[cfg(feature="adc")]
pub use crate::adc::{AdcDualModeStdExt, AdcInit, AdcStdExt, AnalogInputPin};
#[cfg(feature="crc")]
pub use crate::crc::CrcStd;
pub use crate::dbgmcu::DbgMcuStd;
pub use crate::deinit::DeInit;
#[cfg(feature="dma")]
pub use crate::dma::DmaChannel;
#[cfg(feature="dma")]
pub use crate::dma::DmaStruct;
#[cfg(feature="rtc")]
pub use crate::rtc::RtcStd;
#[cfg(feature="spi")]
pub use crate::spi::SpiStdExt;
#[cfg(feature="spi")]
pub use crate::spi::SpiStruct;
