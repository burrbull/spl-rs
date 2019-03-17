use super::device::AFIO;
use super::device::CAN1;
use super::device::DAC;
use super::device::RCC;
use super::device::WWDG;
use super::device::{ADC1, ADC2, ADC3};
use super::device::{GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, GPIOG};
use super::device::{I2C1, I2C2};
use super::device::{SPI1, SPI2, SPI3};
use super::device::{TIM1, TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM8};
use super::device::{UART4, UART5};
use super::device::{USART1, USART2, USART3};

pub trait DeInit {
    /// Deinitializes peripheral registers to their default reset values.
    fn deinit(&self);
}

impl DeInit for ADC1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.adc1rst().set_bit());
        rst.modify(|_, w| w.adc1rst().clear_bit());
    }
}

impl DeInit for ADC2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.adc2rst().set_bit());
        rst.modify(|_, w| w.adc2rst().clear_bit());
    }
}

impl DeInit for ADC3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.adc3rst().set_bit());
        rst.modify(|_, w| w.adc3rst().clear_bit());
    }
}

impl DeInit for SPI1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.spi1rst().set_bit());
        rst.modify(|_, w| w.spi1rst().clear_bit());
    }
}

impl DeInit for SPI2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.spi2rst().set_bit());
        rst.modify(|_, w| w.spi2rst().clear_bit());
    }
}

impl DeInit for SPI3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.spi3rst().set_bit());
        rst.modify(|_, w| w.spi3rst().clear_bit());
    }
}

impl DeInit for USART1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.usart1rst().set_bit());
        rst.modify(|_, w| w.usart1rst().clear_bit());
    }
}
impl DeInit for USART2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.usart2rst().set_bit());
        rst.modify(|_, w| w.usart2rst().clear_bit());
    }
}
impl DeInit for USART3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.usart3rst().set_bit());
        rst.modify(|_, w| w.usart3rst().clear_bit());
    }
}
impl DeInit for UART4 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.uart4rst().set_bit());
        rst.modify(|_, w| w.uart4rst().clear_bit());
    }
}
impl DeInit for UART5 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.uart5rst().set_bit());
        rst.modify(|_, w| w.uart5rst().clear_bit());
    }
}

impl DeInit for TIM1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.tim1rst().set_bit());
        rst.modify(|_, w| w.tim1rst().clear_bit());
    }
}
impl DeInit for TIM2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim2rst().set_bit());
        rst.modify(|_, w| w.tim2rst().clear_bit());
    }
}
impl DeInit for TIM3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim3rst().set_bit());
        rst.modify(|_, w| w.tim3rst().clear_bit());
    }
}
impl DeInit for TIM4 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim4rst().set_bit());
        rst.modify(|_, w| w.tim4rst().clear_bit());
    }
}
impl DeInit for TIM5 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim5rst().set_bit());
        rst.modify(|_, w| w.tim5rst().clear_bit());
    }
}
impl DeInit for TIM6 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim6rst().set_bit());
        rst.modify(|_, w| w.tim6rst().clear_bit());
    }
}
impl DeInit for TIM7 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim7rst().set_bit());
        rst.modify(|_, w| w.tim7rst().clear_bit());
    }
}
impl DeInit for TIM8 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.tim8rst().set_bit());
        rst.modify(|_, w| w.tim8rst().clear_bit());
    }
}

impl DeInit for I2C1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.i2c1rst().set_bit());
        rst.modify(|_, w| w.i2c1rst().clear_bit());
    }
}
impl DeInit for I2C2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.i2c2rst().set_bit());
        rst.modify(|_, w| w.i2c2rst().clear_bit());
    }
}

impl DeInit for GPIOA {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.ioparst().set_bit());
        rst.modify(|_, w| w.ioparst().clear_bit());
    }
}
impl DeInit for GPIOB {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopbrst().set_bit());
        rst.modify(|_, w| w.iopbrst().clear_bit());
    }
}
impl DeInit for GPIOC {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopcrst().set_bit());
        rst.modify(|_, w| w.iopcrst().clear_bit());
    }
}
impl DeInit for GPIOD {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopdrst().set_bit());
        rst.modify(|_, w| w.iopdrst().clear_bit());
    }
}
impl DeInit for GPIOE {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.ioperst().set_bit());
        rst.modify(|_, w| w.ioperst().clear_bit());
    }
}
impl DeInit for GPIOF {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopfrst().set_bit());
        rst.modify(|_, w| w.iopfrst().clear_bit());
    }
}
impl DeInit for GPIOG {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopgrst().set_bit());
        rst.modify(|_, w| w.iopgrst().clear_bit());
    }
}

impl DeInit for AFIO {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.afiorst().set_bit());
        rst.modify(|_, w| w.afiorst().clear_bit());
    }
}

impl DeInit for DAC {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.dacrst().set_bit());
        rst.modify(|_, w| w.dacrst().clear_bit());
    }
}

impl DeInit for CAN1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.canrst().set_bit());
        rst.modify(|_, w| w.canrst().clear_bit());
    }
}

impl DeInit for WWDG {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.wwdgrst().set_bit());
        rst.modify(|_, w| w.wwdgrst().clear_bit());
    }
}
