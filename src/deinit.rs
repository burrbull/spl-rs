use crate::pac::RCC;

pub trait DeInit {
    /// Deinitializes peripheral registers to their default reset values.
    fn deinit(&self);
}

#[cfg(feature="adc1")]
impl DeInit for crate::pac::ADC1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.adc1rst().set_bit());
        rst.modify(|_, w| w.adc1rst().clear_bit());
    }
}

#[cfg(feature="adc2")]
impl DeInit for crate::pac::ADC2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.adc2rst().set_bit());
        rst.modify(|_, w| w.adc2rst().clear_bit());
    }
}

#[cfg(feature="adc3")]
impl DeInit for crate::pac::ADC3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.adc3rst().set_bit());
        rst.modify(|_, w| w.adc3rst().clear_bit());
    }
}

#[cfg(feature="spi1")]
impl DeInit for crate::pac::SPI1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.spi1rst().set_bit());
        rst.modify(|_, w| w.spi1rst().clear_bit());
    }
}

#[cfg(feature="spi2")]
impl DeInit for crate::pac::SPI2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.spi2rst().set_bit());
        rst.modify(|_, w| w.spi2rst().clear_bit());
    }
}

#[cfg(feature="spi3")]
impl DeInit for crate::pac::SPI3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.spi3rst().set_bit());
        rst.modify(|_, w| w.spi3rst().clear_bit());
    }
}

#[cfg(feature="usart1")]
impl DeInit for crate::pac::USART1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.usart1rst().set_bit());
        rst.modify(|_, w| w.usart1rst().clear_bit());
    }
}
#[cfg(feature="usart2")]
impl DeInit for crate::pac::USART2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.usart2rst().set_bit());
        rst.modify(|_, w| w.usart2rst().clear_bit());
    }
}
#[cfg(feature="usart3")]
impl DeInit for crate::pac::USART3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.usart3rst().set_bit());
        rst.modify(|_, w| w.usart3rst().clear_bit());
    }
}
#[cfg(feature="uart4")]
impl DeInit for crate::pac::UART4 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.uart4rst().set_bit());
        rst.modify(|_, w| w.uart4rst().clear_bit());
    }
}
#[cfg(feature="uart5")]
impl DeInit for crate::pac::UART5 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.uart5rst().set_bit());
        rst.modify(|_, w| w.uart5rst().clear_bit());
    }
}

#[cfg(feature="tim1")]
impl DeInit for crate::pac::TIM1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.tim1rst().set_bit());
        rst.modify(|_, w| w.tim1rst().clear_bit());
    }
}
#[cfg(feature="tim2")]
impl DeInit for crate::pac::TIM2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim2rst().set_bit());
        rst.modify(|_, w| w.tim2rst().clear_bit());
    }
}
#[cfg(feature="tim3")]
impl DeInit for crate::pac::TIM3 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim3rst().set_bit());
        rst.modify(|_, w| w.tim3rst().clear_bit());
    }
}
#[cfg(feature="tim4")]
impl DeInit for crate::pac::TIM4 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim4rst().set_bit());
        rst.modify(|_, w| w.tim4rst().clear_bit());
    }
}
#[cfg(feature="tim5")]
impl DeInit for crate::pac::TIM5 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim5rst().set_bit());
        rst.modify(|_, w| w.tim5rst().clear_bit());
    }
}
#[cfg(feature="tim6")]
impl DeInit for crate::pac::TIM6 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim6rst().set_bit());
        rst.modify(|_, w| w.tim6rst().clear_bit());
    }
}
#[cfg(feature="tim7")]
impl DeInit for crate::pac::TIM7 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.tim7rst().set_bit());
        rst.modify(|_, w| w.tim7rst().clear_bit());
    }
}
#[cfg(feature="tim8")]
impl DeInit for crate::pac::TIM8 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.tim8rst().set_bit());
        rst.modify(|_, w| w.tim8rst().clear_bit());
    }
}

#[cfg(feature="i2c1")]
impl DeInit for crate::pac::I2C1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.i2c1rst().set_bit());
        rst.modify(|_, w| w.i2c1rst().clear_bit());
    }
}
#[cfg(feature="i2c2")]
impl DeInit for crate::pac::I2C2 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.i2c2rst().set_bit());
        rst.modify(|_, w| w.i2c2rst().clear_bit());
    }
}

#[cfg(feature="gpioa")]
impl DeInit for crate::pac::GPIOA {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.ioparst().set_bit());
        rst.modify(|_, w| w.ioparst().clear_bit());
    }
}
#[cfg(feature="gpiob")]
impl DeInit for crate::pac::GPIOB {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopbrst().set_bit());
        rst.modify(|_, w| w.iopbrst().clear_bit());
    }
}
#[cfg(feature="gpioc")]
impl DeInit for crate::pac::GPIOC {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopcrst().set_bit());
        rst.modify(|_, w| w.iopcrst().clear_bit());
    }
}
#[cfg(feature="gpiod")]
impl DeInit for crate::pac::GPIOD {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopdrst().set_bit());
        rst.modify(|_, w| w.iopdrst().clear_bit());
    }
}
#[cfg(feature="gpioe")]
impl DeInit for crate::pac::GPIOE {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.ioperst().set_bit());
        rst.modify(|_, w| w.ioperst().clear_bit());
    }
}
#[cfg(feature="gpiof")]
impl DeInit for crate::pac::GPIOF {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopfrst().set_bit());
        rst.modify(|_, w| w.iopfrst().clear_bit());
    }
}
#[cfg(feature="gpiog")]
impl DeInit for crate::pac::GPIOG {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.iopgrst().set_bit());
        rst.modify(|_, w| w.iopgrst().clear_bit());
    }
}

#[cfg(feature="afio")]
impl DeInit for crate::pac::AFIO {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb2rstr;
        rst.modify(|_, w| w.afiorst().set_bit());
        rst.modify(|_, w| w.afiorst().clear_bit());
    }
}

#[cfg(feature="dac")]
impl DeInit for crate::pac::DAC {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.dacrst().set_bit());
        rst.modify(|_, w| w.dacrst().clear_bit());
    }
}

#[cfg(feature="can1")]
impl DeInit for crate::pac::CAN1 {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.canrst().set_bit());
        rst.modify(|_, w| w.canrst().clear_bit());
    }
}

impl DeInit for crate::pac::WWDG {
    fn deinit(&self) {
        let rst = &(unsafe { &*RCC::ptr() }).apb1rstr;
        rst.modify(|_, w| w.wwdgrst().set_bit());
        rst.modify(|_, w| w.wwdgrst().clear_bit());
    }
}
