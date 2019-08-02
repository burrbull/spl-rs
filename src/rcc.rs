
use crate::base::{ITStatus, FlagStatus, FunctionalState, ErrorStatus};

use crate::device::{RCC};



/// PLL_multiplication_factor 
pub use crate::device::rcc::cfgr::PLLMULW as RccPLLMul;
/// System_clock_source 
pub use crate::device::rcc::cfgr::SWW as RccSYSCLKSource;
/// ADC_clock_source 
pub use crate::device::rcc::cfgr::ADCPREW as RccPCLK2;
/// AHB_clock_source
pub use crate::device::rcc::cfgr::HPREW as RccSYSCLK;
/// APB1_APB2_clock_source
pub use crate::device::rcc::cfgr::PPRE1W as RccHCLK;
pub use crate::device::rcc::cfgr::PPRE2W as RccHCLK2;


/// RTC_clock_source 
pub use crate::device::rcc::bdcr::RTCSELW as RccRTCCLKSource;

/// USB_clock_source
pub use crate::device::rcc::cfgr::USBPREW as RccUSBCLKSource_PLLCLK;


pub struct RccClocks {
    pub SYSCLK_Frequency : u32,
    pub HCLK_Frequency   : u32,
    pub PCLK1_Frequency  : u32,
    pub PCLK2_Frequency  : u32,
    pub ADCCLK_Frequency : u32
}

/// HSE_configuration 
pub enum RccHSE {
    OFF,
    ON,
    Bypass
}

/// LSE_configuration 
pub enum RccLSE {
    OFF,
    ON,
    Bypass
}

/// PLL_entry_clock_source 
pub enum RccPLLSource {
    HSI_Div2 = 0,
    HSE_Div1 = 1,
    HSE_Div2 = 3
}



/// RCC_Interrupt_source 
pub mod RccIT {
    pub const LSIRDY : u8 = 0x01;
    pub const LSERDY : u8 = 0x02;
    pub const HSIRDY : u8 = 0x04;
    pub const HSERDY : u8 = 0x08;
    pub const PLLRDY : u8 = 0x10;
    pub const CSS    : u8 = 0x80;
}




pub mod RccPeriph {
    /// AHB_peripheral
    pub mod ahb {
        pub const DMA1  : u32 = 0x00000001;
        pub const DMA2  : u32 = 0x00000002;
        pub const SRAM  : u32 = 0x00000004;
        pub const FLITF : u32 = 0x00000010;
        pub const CRC   : u32 = 0x00000040;
        pub const FSMC  : u32 = 0x00000100;
        pub const SDIO  : u32 = 0x00000400;
    }
    /// APB2_peripheral
    pub mod apb2 {
        pub const AFIO  : u32 = 0x00000001;
        pub const GPIOA : u32 = 0x00000004;
        pub const GPIOB : u32 = 0x00000008;
        pub const GPIOC : u32 = 0x00000010;
        pub const GPIOD : u32 = 0x00000020;
        pub const GPIOE : u32 = 0x00000040;
        pub const GPIOF : u32 = 0x00000080;
        pub const GPIOG : u32 = 0x00000100;
        pub const ADC1  : u32 = 0x00000200;
        pub const ADC2  : u32 = 0x00000400;
        pub const TIM1  : u32 = 0x00000800;
        pub const SPI1  : u32 = 0x00001000;
        pub const TIM8  : u32 = 0x00002000;
        pub const USART1: u32 = 0x00004000;
        pub const ADC3  : u32 = 0x00008000;
        pub const ALL   : u32 = 0x0000FFFD;
    }
    /// APB1_peripheral
    pub mod apb1 {
        pub const TIM2  : u32 = 0x00000001;
        pub const TIM3  : u32 = 0x00000002;
        pub const TIM4  : u32 = 0x00000004;
        pub const TIM5  : u32 = 0x00000008;
        pub const TIM6  : u32 = 0x00000010;
        pub const TIM7  : u32 = 0x00000020;
        pub const WWDG  : u32 = 0x00000800;
        pub const SPI2  : u32 = 0x00004000;
        pub const SPI3  : u32 = 0x00008000;
        pub const USART2: u32 = 0x00020000;
        pub const USART3: u32 = 0x00040000;
        pub const UART4 : u32 = 0x00080000;
        pub const UART5 : u32 = 0x00100000;
        pub const I2C1  : u32 = 0x00200000;
        pub const I2C2  : u32 = 0x00400000;
        pub const USB   : u32 = 0x00800000;
        pub const CAN1  : u32 = 0x02000000;
        pub const BKP   : u32 = 0x08000000;
        pub const PWR   : u32 = 0x10000000;
        pub const DAC   : u32 = 0x20000000;
        pub const ALL   : u32 = 0x3AFEC83F;
    }
}

/// Clock_source_to_output_on_MCO_pin
pub enum RccMco {
    NoClock     = 0x00,
    SYSCLK      = 0x04,
    HSI         = 0x05,
    HSE         = 0x06,
    PLLCLK_Div2 = 0x07
}

/// RCC_Flag
pub mod RccFlag {
    pub const HSIRDY  : u8 = 0x21;
    pub const HSERDY  : u8 = 0x31;
    pub const PLLRDY  : u8 = 0x39;
    pub const LSERDY  : u8 = 0x41;
    pub const LSIRDY  : u8 = 0x61;
    pub const PINRST  : u8 = 0x7A;
    pub const PORRST  : u8 = 0x7B;
    pub const SFTRST  : u8 = 0x7C;
    pub const IWDGRST : u8 = 0x7D;
    pub const WWDGRST : u8 = 0x7E;
    pub const LPWRRST : u8 = 0x7F;
    pub const MASK    : u8 = 0x1F;
}


/* Typical Value of the HSI in Hz */
pub const HSI_VALUE : u32 = 8000000;


//#ifndef HSEStartUp_TimeOut
/* Time out for HSE start up */
static HSESTARTUP_TIMEOUT : u16 = 0x0500;
//#endif

// ??????
static HSE_VALUE : u32 = 8000000;

const APBAHB_PRESCTABLE : [u8;16] = [0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 6, 7, 8, 9];
const ADC_PRESCTABLE : [u8;4] = [2, 4, 6, 8];


pub trait RccStd {
    /// Resets the RCC clock configuration to the default reset state.
    fn DeInit(&self);

    /// Configures the External High Speed oscillator (HSE).
    ///   HSE can not be stopped if it is used directly or through the 
    ///   PLL as system clock.
    /// `rcc_hse`: specifies the new state of the HSE.
    ///   This parameter can be one of the following values:
    /// * RCC_HSE_OFF: HSE oscillator OFF
    /// * RCC_HSE_ON: HSE oscillator ON
    /// * RCC_HSE_Bypass: HSE oscillator bypassed with external
    ///   clock
    fn HSEConfig(&self, rcc_hse : RccHSE);

    /// Waits for HSE start-up.
    /// @retval : An ErrorStatus enumuration value:
    /// - SUCCESS: HSE oscillator is stable and ready to use
    /// - ERROR: HSE oscillator not yet ready
    fn WaitForHSEStartUp(&self) -> ErrorStatus;

    /// Adjusts the Internal High Speed oscillator (HSI) calibration
    ///   value.
    /// `hsicalibrationvalue`: specifies the calibration trimming value.
    ///   This parameter must be a number between 0 and 0x1F.
    fn AdjustHSICalibrationValue(&self, hsicalibrationvalue : u8);

    /// Enables or disables the Internal High Speed oscillator (HSI).
    ///   HSI can not be stopped if it is used directly or through the 
    ///   PLL as system clock.
    /// `new_state`: new state of the HSI.
    ///   This parameter can be: ENABLE or DISABLE.
    fn HSICmd(&self, new_state : bool);

    /// Configures the PLL clock source and multiplication factor.
    ///   This function must be used only when the PLL is disabled.
    /// `rcc_pllsource`: specifies the PLL entry clock source.
    ///   This parameter can be one of the following values:
    /// * RCC_PLLSource_HSI_Div2: HSI oscillator clock divided
    ///   by 2 selected as PLL clock entry
    /// * RCC_PLLSource_HSE_Div1: HSE oscillator clock selected
    ///   as PLL clock entry
    /// * RCC_PLLSource_HSE_Div2: HSE oscillator clock divided
    ///   by 2 selected as PLL clock entry
    /// `rcc_pllmul`: specifies the PLL multiplication factor.
    ///   This parameter can be RCC_PLLMul_x where x:[2,16]
    fn PLLConfig(&self, rcc_pllsource : RccPLLSource, rcc_pllmul : RccPLLMul);

    /// Enables or disables the PLL.
    ///   The PLL can not be disabled if it is used as system clock.
    /// `new_state`: new state of the PLL.
    ///   This parameter can be: ENABLE or DISABLE.
    fn PLLCmd(&self, new_state : bool);

    /// Configures the system clock (SYSCLK).
    /// `rcc_sysclksource`: specifies the clock source used as system
    ///   clock. This parameter can be one of the following values:
    /// * RCC_SYSCLKSource_HSI: HSI selected as system clock
    /// * RCC_SYSCLKSource_HSE: HSE selected as system clock
    /// * RCC_SYSCLKSource_PLLCLK: PLL selected as system clock
    fn SYSCLKConfig(&self, rcc_sysclksource : RccSYSCLKSource);

    /// Returns the clock source used as system clock.
    /// @retval : The clock source used as system clock. The returned value can
    ///   be one of the following:
    /// - 0x00: HSI used as system clock
    /// - 0x04: HSE used as system clock
    /// - 0x08: PLL used as system clock
    fn GetSYSCLKSource(&self) -> u8;

    /// Configures the AHB clock (HCLK).
    /// `rcc_sysclk`: defines the AHB clock divider. This clock is derived from 
    ///                    the system clock (SYSCLK).
    ///   This parameter can be one of the following values:
    /// * RCC_SYSCLK_Div1: AHB clock = SYSCLK
    /// * RCC_SYSCLK_Div2: AHB clock = SYSCLK/2
    /// * RCC_SYSCLK_Div4: AHB clock = SYSCLK/4
    /// * RCC_SYSCLK_Div8: AHB clock = SYSCLK/8
    /// * RCC_SYSCLK_Div16: AHB clock = SYSCLK/16
    /// * RCC_SYSCLK_Div64: AHB clock = SYSCLK/64
    /// * RCC_SYSCLK_Div128: AHB clock = SYSCLK/128
    /// * RCC_SYSCLK_Div256: AHB clock = SYSCLK/256
    /// * RCC_SYSCLK_Div512: AHB clock = SYSCLK/512
    fn HCLKConfig(&self, rcc_sysclk : RccSYSCLK);

    /// Configures the Low Speed APB clock (PCLK1).
    /// `rcc_hclk`: defines the APB1 clock divider. This clock is derived from 
    ///                  the AHB clock (HCLK).
    ///   This parameter can be one of the following values:
    /// * RCC_HCLK_Div1: APB1 clock = HCLK
    /// * RCC_HCLK_Div2: APB1 clock = HCLK/2
    /// * RCC_HCLK_Div4: APB1 clock = HCLK/4
    /// * RCC_HCLK_Div8: APB1 clock = HCLK/8
    /// * RCC_HCLK_Div16: APB1 clock = HCLK/16
    fn PCLK1Config(&self, rcc_hclk : RccHCLK);

    /// Configures the High Speed APB clock (PCLK2).
    /// `rcc_hclk`: defines the APB2 clock divider. This clock is derived from 
    ///                  the AHB clock (HCLK).
    ///   This parameter can be one of the following values:
    /// * RCC_HCLK_Div1: APB2 clock = HCLK
    /// * RCC_HCLK_Div2: APB2 clock = HCLK/2
    /// * RCC_HCLK_Div4: APB2 clock = HCLK/4
    /// * RCC_HCLK_Div8: APB2 clock = HCLK/8
    /// * RCC_HCLK_Div16: APB2 clock = HCLK/16
    fn PCLK2Config(&self, rcc_hclk : RccHCLK2);

    /*/// Enables or disables the specified RCC interrupts.
    /// `rcc_it`: specifies the RCC interrupt sources to be enabled or disabled.
    ///   This parameter can be any combination of the following values:
    /// * RCC_IT_LSIRDY: LSI ready interrupt
    /// * RCC_IT_LSERDY: LSE ready interrupt
    /// * RCC_IT_HSIRDY: HSI ready interrupt
    /// * RCC_IT_HSERDY: HSE ready interrupt
    /// * RCC_IT_PLLRDY: PLL ready interrupt
    /// `new_state`: new state of the specified RCC interrupts.
    ///   This parameter can be: ENABLE or DISABLE.
    fn ITConfig(&self, rcc_it : u8, new_state : bool);*/

    /// Configures the USB clock (USBCLK).
    /// `rcc_usbclksource`: specifies the USB clock source. This clock is 
    ///                          derived from the PLL output.
    ///   This parameter can be one of the following values:
    /// * RCC_USBCLKSource_PLLCLK_1Div5: PLL clock divided by 1,5 selected as USB 
    ///                                     clock source
    /// * RCC_USBCLKSource_PLLCLK_Div1: PLL clock selected as USB clock source
    fn USBCLKConfig(&self, rcc_usbclksource : RccUSBCLKSource_PLLCLK);
    /// Configures the ADC clock (ADCCLK).
    /// `rcc_pclk2`: defines the ADC clock divider. This clock is derived from 
    ///                   the APB2 clock (PCLK2).
    ///   This parameter can be one of the following values:
    /// * RCC_PCLK2_Div2: ADC clock = PCLK2/2
    /// * RCC_PCLK2_Div4: ADC clock = PCLK2/4
    /// * RCC_PCLK2_Div6: ADC clock = PCLK2/6
    /// * RCC_PCLK2_Div8: ADC clock = PCLK2/8
    fn ADCCLKConfig(&self, rcc_pclk2 : RccPCLK2);

    /// Configures the External Low Speed oscillator (LSE).
    /// `rcc_lse`: specifies the new state of the LSE.
    ///   This parameter can be one of the following values:
    /// * RCC_LSE_OFF: LSE oscillator OFF
    /// * RCC_LSE_ON: LSE oscillator ON
    /// * RCC_LSE_Bypass: LSE oscillator bypassed with external
    ///   clock
    fn LSEConfig(&self, rcc_lse : RccLSE);

    /// Enables or disables the Internal Low Speed oscillator (LSI).
    ///   LSI can not be disabled if the IWDG is running.
    /// `new_state`: new state of the LSI.
    ///   This parameter can be: ENABLE or DISABLE.
    fn LSICmd(&self, new_state : bool);

    /// Configures the RTC clock (RTCCLK).
    ///   Once the RTC clock is selected it can’t be changed unless the
    ///   Backup domain is reset.
    /// `rcc_rtcclksource`: specifies the RTC clock source.
    ///   This parameter can be one of the following values:
    /// * RCC_RTCCLKSource_LSE: LSE selected as RTC clock
    /// * RCC_RTCCLKSource_LSI: LSI selected as RTC clock
    /// * RCC_RTCCLKSource_HSE_Div128: HSE clock divided by 128
    ///   selected as RTC clock
    fn RTCCLKConfig(&self, rcc_rtcclksource : RccRTCCLKSource);

    /// Enables or disables the RTC clock.
    ///   This function must be used only after the RTC clock was
    ///   selected using the RCC_RTCCLKConfig function.
    /// `new_state`: new state of the RTC clock.
    ///   This parameter can be: ENABLE or DISABLE.
    fn RTCCLKCmd(&self, new_state : bool);

    /// Returns the frequencies of different on chip clocks.
    /// `rcc_clocks`: pointer to a RCC_ClocksTypeDef structure which
    ///   will hold the clocks frequencies.
    fn GetClocksFreq(&self) -> RccClocks;

    /// Enables or disables the AHB peripheral clock.
    /// `rcc_ahbperiph`: specifies the AHB peripheral to gates its clock.
    ///   This parameter can be any combination of the following values:
    /// * RCC_AHBPeriph_DMA1
    /// * RCC_AHBPeriph_DMA2
    /// * RCC_AHBPeriph_SRAM
    /// * RCC_AHBPeriph_FLITF
    /// * RCC_AHBPeriph_CRC
    /// * RCC_AHBPeriph_FSMC
    /// * RCC_AHBPeriph_SDIO
    ///   SRAM and FLITF clock can be disabled only during sleep mode.
    /// `new_state`: new state of the specified peripheral clock.
    ///   This parameter can be: ENABLE or DISABLE.
    fn AHBPeriphClockCmd(&self, rcc_ahbperiph : u32, new_state : bool);

    /// Enables or disables the High Speed APB (APB2) peripheral clock.
    /// `rcc_apb2periph`: specifies the APB2 peripheral to gates its
    ///   clock.
    ///   This parameter can be any combination of the following values:
    /// * RCC_APB2Periph_AFIO, RCC_APB2Periph_GPIOA, RCC_APB2Periph_GPIOB,
    ///   RCC_APB2Periph_GPIOC, RCC_APB2Periph_GPIOD, RCC_APB2Periph_GPIOE,
    ///   RCC_APB2Periph_GPIOF, RCC_APB2Periph_GPIOG, RCC_APB2Periph_ADC1,
    ///   RCC_APB2Periph_ADC2, RCC_APB2Periph_TIM1, RCC_APB2Periph_SPI1,
    ///   RCC_APB2Periph_TIM8, RCC_APB2Periph_USART1, RCC_APB2Periph_ADC3,
    ///   RCC_APB2Periph_ALL
    /// `new_state`: new state of the specified peripheral clock.
    ///   This parameter can be: ENABLE or DISABLE.
    fn APB2PeriphClockCmd(&self, rcc_apb2periph : u32, new_state : bool);

    /// Enables or disables the Low Speed APB (APB1) peripheral clock.
    /// `rcc_apb1periph`: specifies the APB1 peripheral to gates its
    ///   clock.
    ///   This parameter can be any combination of the following values:
    /// * RCC_APB1Periph_TIM2, RCC_APB1Periph_TIM3, RCC_APB1Periph_TIM4,
    ///   RCC_APB1Periph_TIM5, RCC_APB1Periph_TIM6, RCC_APB1Periph_TIM7,
    ///   RCC_APB1Periph_WWDG, RCC_APB1Periph_SPI2, RCC_APB1Periph_SPI3,
    ///   RCC_APB1Periph_USART2, RCC_APB1Periph_USART3, RCC_APB1Periph_USART4, 
    ///   RCC_APB1Periph_USART5, RCC_APB1Periph_I2C1, RCC_APB1Periph_I2C2,
    ///   RCC_APB1Periph_USB, RCC_APB1Periph_CAN1, RCC_APB1Periph_BKP,
    ///   RCC_APB1Periph_PWR, RCC_APB1Periph_DAC, RCC_APB1Periph_ALL
    /// `new_state`: new state of the specified peripheral clock.
    ///   This parameter can be: ENABLE or DISABLE.
    fn APB1PeriphClockCmd(&self, rcc_apb1periph : u32, new_state : bool);

    /// Forces or releases High Speed APB (APB2) peripheral reset.
    /// `rcc_apb2periph`: specifies the APB2 peripheral to reset.
    ///   This parameter can be any combination of the following values:
    /// * RCC_APB2Periph_AFIO, RCC_APB2Periph_GPIOA, RCC_APB2Periph_GPIOB,
    ///   RCC_APB2Periph_GPIOC, RCC_APB2Periph_GPIOD, RCC_APB2Periph_GPIOE,
    ///   RCC_APB2Periph_GPIOF, RCC_APB2Periph_GPIOG, RCC_APB2Periph_ADC1,
    ///   RCC_APB2Periph_ADC2, RCC_APB2Periph_TIM1, RCC_APB2Periph_SPI1,
    ///   RCC_APB2Periph_TIM8, RCC_APB2Periph_USART1, RCC_APB2Periph_ADC3,
    ///   RCC_APB2Periph_ALL
    /// `new_state`: new state of the specified peripheral reset.
    ///   This parameter can be: ENABLE or DISABLE.
    fn APB2PeriphResetCmd(&self, rcc_apb2periph : u32, new_state : bool);

    /// Forces or releases Low Speed APB (APB1) peripheral reset.
    /// `rcc_apb1periph`: specifies the APB1 peripheral to reset.
    ///   This parameter can be any combination of the following values:
    /// * RCC_APB1Periph_TIM2, RCC_APB1Periph_TIM3, RCC_APB1Periph_TIM4,
    ///   RCC_APB1Periph_TIM5, RCC_APB1Periph_TIM6, RCC_APB1Periph_TIM7,
    ///   RCC_APB1Periph_WWDG, RCC_APB1Periph_SPI2, RCC_APB1Periph_SPI3,
    ///   RCC_APB1Periph_USART2, RCC_APB1Periph_USART3, RCC_APB1Periph_USART4, 
    ///   RCC_APB1Periph_USART5, RCC_APB1Periph_I2C1, RCC_APB1Periph_I2C2,
    ///   RCC_APB1Periph_USB, RCC_APB1Periph_CAN1, RCC_APB1Periph_BKP,
    ///   RCC_APB1Periph_PWR, RCC_APB1Periph_DAC, RCC_APB1Periph_ALL
    /// `new_state`: new state of the specified peripheral clock.
    ///   This parameter can be: ENABLE or DISABLE.
    fn APB1PeriphResetCmd(&self, rcc_apb1periph : u32, new_state : bool);

    /// Forces or releases the Backup domain reset.
    /// `new_state`: new state of the Backup domain reset.
    ///   This parameter can be: ENABLE or DISABLE.
    fn BackupResetCmd(&self, new_state : bool);

    /// Enables or disables the Clock Security System.
    /// `new_state`: new state of the Clock Security System..
    ///   This parameter can be: ENABLE or DISABLE.
    fn ClockSecuritySystemCmd(&self, new_state : bool);

    /// Selects the clock source to output on MCO pin.
    /// `rcc_mco`: specifies the clock source to output.
    ///   This parameter can be one of the following values:
    /// * RCC_MCO_NoClock: No clock selected
    /// * RCC_MCO_SYSCLK: System clock selected
    /// * RCC_MCO_HSI: HSI oscillator clock selected
    /// * RCC_MCO_HSE: HSE oscillator clock selected
    /// * RCC_MCO_PLLCLK_Div2: PLL clock divided by 2 selected
    fn MCOConfig(&self, rcc_mco : RccMco);

    /// Checks whether the specified RCC flag is set or not.
    /// `rcc_flag`: specifies the flag to check.
    ///   This parameter can be one of the following values:
    /// * RCC_FLAG_HSIRDY: HSI oscillator clock ready
    /// * RCC_FLAG_HSERDY: HSE oscillator clock ready
    /// * RCC_FLAG_PLLRDY: PLL clock ready
    /// * RCC_FLAG_LSERDY: LSE oscillator clock ready
    /// * RCC_FLAG_LSIRDY: LSI oscillator clock ready
    /// * RCC_FLAG_PINRST: Pin reset
    /// * RCC_FLAG_PORRST: POR/PDR reset
    /// * RCC_FLAG_SFTRST: Software reset
    /// * RCC_FLAG_IWDGRST: Independent Watchdog reset
    /// * RCC_FLAG_WWDGRST: Window Watchdog reset
    /// * RCC_FLAG_LPWRRST: Low Power reset
    /// @retval : The new state of RCC_FLAG (SET or RESET).
    fn GetFlagStatus(&self, rcc_flag : u8) -> FlagStatus;

    /// Clears the RCC reset flags.
    ///   The reset flags are: RCC_FLAG_PINRST, RCC_FLAG_PORRST,
    ///   RCC_FLAG_SFTRST, RCC_FLAG_IWDGRST, RCC_FLAG_WWDGRST,
    ///   RCC_FLAG_LPWRRST
    fn ClearFlag(&self);

    /// Checks whether the specified RCC interrupt has occurred or not.
    /// `rcc_it`: specifies the RCC interrupt source to check.
    ///   This parameter can be one of the following values:
    /// * RCC_IT_LSIRDY: LSI ready interrupt
    /// * RCC_IT_LSERDY: LSE ready interrupt
    /// * RCC_IT_HSIRDY: HSI ready interrupt
    /// * RCC_IT_HSERDY: HSE ready interrupt
    /// * RCC_IT_PLLRDY: PLL ready interrupt
    /// * RCC_IT_CSS: Clock Security System interrupt
    /// @retval : The new state of RCC_IT (SET or RESET).
    fn GetITStatus(&self, rcc_it : u8) -> ITStatus;

    /*/// Clears the RCC’s interrupt pending bits.
    /// `rcc_it`: specifies the interrupt pending bit to clear.
    ///   This parameter can be any combination of the following values:
    /// * RCC_IT_LSIRDY: LSI ready interrupt
    /// * RCC_IT_LSERDY: LSE ready interrupt
    /// * RCC_IT_HSIRDY: HSI ready interrupt
    /// * RCC_IT_HSERDY: HSE ready interrupt
    /// * RCC_IT_PLLRDY: PLL ready interrupt
    /// * RCC_IT_CSS: Clock Security System interrupt
    fn ClearITPendingBit(&self, rcc_it : u8);*/
}


impl RccStd for RCC {
    fn DeInit(&self) {
        /* Set HSION bit */
        self.cr      .modify(|_,w| w
            .hsion() .set_bit()
        );
        /* Reset SW[1:0], HPRE[3:0], PPRE1[2:0], PPRE2[2:0], ADCPRE[1:0] and MCO[2:0] bits */
        self.cfgr      .modify(|_,w| unsafe { w
            .sw()    .bits( 0 )
            .hpre()  .bits( 0 )
            .ppre1() .bits( 0 )
            .ppre2() .bits( 0 )
            .adcpre().bits( 0 )
            .mco()   .bits( 0 )
        });
        /* Reset HSEON, CSSON and PLLON bits */
        self.cr      .modify(|_,w| w
            .hseon() .clear_bit()
            .csson() .clear_bit()
            .pllon() .clear_bit()
        );
        /* Reset HSEBYP bit */
        self.cr      .modify(|_,w| w
            .hsebyp() .clear_bit()
        );
        /* Reset PLLSRC, PLLXTPRE, PLLMUL[3:0] and USBPRE bits */
        self.cfgr      .modify(|_,w| unsafe { w
            .pllsrc()  .clear_bit()
            .pllxtpre().clear_bit()
            .pllmul()  .bits( 0 )
        });
        /* Disable all interrupts */
        self.cir.reset();
    }
    
    fn HSEConfig(&self, rcc_hse : RccHSE) {
        /* Reset HSEON and HSEBYP bits before configuring the HSE ------------------*/
        self.cr     .modify(|_,w| w
            .hseon()   .clear_bit()
            .hsebyp()   .clear_bit()
        );
        /* Configure HSE (RCC_HSE_OFF is already covered by the code section above) */
        match rcc_hse {
            RccHSE::ON => {
                /* Set HSEON bit */
                self.cr     .modify(|_,w| w
                    .hseon()   .set_bit()
                );
            },
            RccHSE::Bypass => {
                /* Set HSEBYP and HSEON bits */
                self.cr     .modify(|_,w| w
                    .hsebyp()   .set_bit()
                    .hseon()   .set_bit()
                );
            },
            _ => {}
        }
    }
    
    fn WaitForHSEStartUp(&self) -> ErrorStatus {
        let mut start_up_counter = 0u32;
        let mut hse_status : FlagStatus;// = FlagStatus::RESET;
        
        /* Wait till HSE is ready and if Time out is reached exit */
        loop {
            hse_status = self.GetFlagStatus(RccFlag::HSERDY);
            start_up_counter += 1;
            if hse_status.into() || (start_up_counter == HSESTARTUP_TIMEOUT as u32) {
                break; }
        }
        if self.GetFlagStatus(RccFlag::HSERDY).into() {
            ErrorStatus::SUCCESS
        } else {
            ErrorStatus::ERROR
        }
    }
    
    fn AdjustHSICalibrationValue(&self, hsicalibrationvalue : u8) {
        self.cr      .modify(|_,w| unsafe { w
            .hsitrim()   .bits( hsicalibrationvalue )
        });
    }
    
    fn HSICmd(&self, new_state : bool) {
        self.cr       .modify(|_,w| w
            .hsion()    .bit( new_state )
        );
    }
    
    fn PLLConfig(&self, rcc_pllsource : RccPLLSource, rcc_pllmul : RccPLLMul) {
        let source = rcc_pllsource as u8;
        let xtpre = source >> 1;
        self.cfgr      .modify(|_,w| w
            .pllxtpre()   .bit( xtpre != 0 )
            .pllsrc()     .bit( (source - (xtpre << 1)) != 0 )
            .pllmul()     .variant( rcc_pllmul )
        );
    }
    
    fn PLLCmd(&self, new_state : bool) {
        self.cr       .modify(|_,w| w
            .pllon()    .bit( new_state )
        );
    }
    
    fn SYSCLKConfig(&self, rcc_sysclksource : RccSYSCLKSource) {
        self.cfgr      .modify(|_,w| w
            .sw()      .variant( rcc_sysclksource )
        );
    }
    
    fn GetSYSCLKSource(&self) -> u8 {
        self.cfgr.read().sws().bits()
    }
    
    fn HCLKConfig(&self, rcc_sysclk : RccSYSCLK) {
        self.cfgr      .modify(|_,w| w
            .hpre()      .variant( rcc_sysclk )
        );
    }
    
    fn PCLK1Config(&self, rcc_hclk : RccHCLK) {
        self.cfgr      .modify(|_,w| w
            .ppre1()   .variant( rcc_hclk )
        );
    }
    
    fn PCLK2Config(&self, rcc_hclk : RccHCLK2) {
        self.cfgr      .modify(|_,w| w
            .ppre2()   .variant( rcc_hclk )
        );
    }
    
    /*fn ITConfig(&self, rcc_it : u8, new_state : bool) {
        if new_state {
            /* Perform Byte access to RCC_CIR[12:8] bits to enable the selected interrupts */
            self.cir      .modify(|r,w| unsafe { w
                .bits( r | (rcc_it << 8) )
            });
            write_volatile
            // *(__IO uint8_t *) CIR_BYTE2_ADDRESS |= RCC_IT;
        } else {
            /* Perform Byte access to RCC_CIR[12:8] bits to disable the selected interrupts */
            self.cir      .modify(|r,w| unsafe { w
                .bits( r & !(rcc_it << 8) )
            });
            // *(__IO uint8_t *) CIR_BYTE2_ADDRESS &= (uint8_t)~rcc_it;
        }
    }*/
    
    fn USBCLKConfig(&self, rcc_usbclksource : RccUSBCLKSource_PLLCLK ) {
        self.cfgr      .modify(|_,w| w
            .usbpre()   .variant( rcc_usbclksource )
        );
    }
    
    fn ADCCLKConfig(&self, rcc_pclk2 : RccPCLK2) {
        self.cfgr      .modify(|_,w| w
            .adcpre()   .variant( rcc_pclk2 )
        );
    }
    
    fn LSEConfig(&self, rcc_lse : RccLSE) {
        /* Reset LSEON and LSEBYP bits before configuring the LSE ------------------*/
        self.bdcr     .modify(|_,w| w
            .lseon()   .clear_bit()
            .lsebyp()   .clear_bit()
        );
        /* Configure LSE (RCC_LSE_OFF is already covered by the code section above) */
        match rcc_lse {
            RccLSE::ON => {
                /* Set LSEON bit */
                self.bdcr     .modify(|_,w| w
                    .lseon()   .set_bit()
                );
            },
            RccLSE::Bypass => {
                /* Set LSEBYP and LSEON bits */
                self.bdcr     .modify(|_,w| w
                    .lsebyp()   .set_bit()
                    .lseon()   .set_bit()
                );
            },
            _ => {}
        }
    }
    
    fn LSICmd(&self, new_state : bool) {
        self.csr       .modify(|_,w| w
            .lsion()    .bit( new_state )
        );
    }
    
    fn RTCCLKConfig(&self, rcc_rtcclksource : RccRTCCLKSource) {
        self.bdcr      .modify(|_,w| w
            .rtcsel()   .variant( rcc_rtcclksource )
        );
    }
    
    fn RTCCLKCmd(&self, new_state : bool) {
        self.bdcr       .modify(|_,w| w
            .rtcen()    .bit( new_state )
        );
    }
    
    fn GetClocksFreq(&self) -> RccClocks {
        /* Get SYSCLK source -------------------------------------------------------*/
        let SYSCLK_Frequency : u32;
        use crate::device::rcc::cfgr::SWSR as Source;
        match self.cfgr.read().sws() {
            /* HSI used as system clock */
            Source::HSI => { SYSCLK_Frequency = HSI_VALUE; },
            /* HSE used as system clock */
            Source::HSE => { SYSCLK_Frequency = HSE_VALUE; },
            /* PLL used as system clock */
            Source::PLL => { 
                /* Get PLL clock source and multiplication factor ----------------------*/
                let pllmull = (self.cfgr.read().pllmul().bits() + 2) as u32;
                if self.cfgr.read().pllsrc().bit_is_clear() {
                    /* HSI oscillator clock divided by 2 selected as PLL clock entry */
                    SYSCLK_Frequency = (HSI_VALUE >> 1) * pllmull;
                } else {
                    /* HSE selected as PLL clock entry */
                    if self.cfgr.read().pllxtpre().bit_is_set() {
                        /* HSE oscillator clock divided by 2 */
                        SYSCLK_Frequency = (HSE_VALUE >> 1) * pllmull;
                    } else {
                        SYSCLK_Frequency = HSE_VALUE * pllmull;
                    }
                }
            },
            _  => { SYSCLK_Frequency = HSI_VALUE; }
        }
        let mut presc : u32;
        /* Compute HCLK, PCLK1, PCLK2 and ADCCLK clocks frequencies ----------------*/
        /* Get HCLK prescaler */
        presc = APBAHB_PRESCTABLE[self.cfgr.read().hpre().bits() as usize] as u32;
        /* HCLK clock frequency */
        let HCLK_Frequency = SYSCLK_Frequency >> presc;
        /* Get PCLK1 prescaler */
        presc = APBAHB_PRESCTABLE[self.cfgr.read().ppre1().bits() as usize] as u32;
        /* PCLK1 clock frequency */
        let PCLK1_Frequency = HCLK_Frequency >> presc;
        /* Get PCLK2 prescaler */
        presc = APBAHB_PRESCTABLE[self.cfgr.read().ppre2().bits() as usize] as u32;
        /* PCLK2 clock frequency */
        let PCLK2_Frequency = HCLK_Frequency >> presc;
        /* Get ADCCLK prescaler */
        presc = ADC_PRESCTABLE[self.cfgr.read().adcpre().bits() as usize] as u32;
        /* ADCCLK clock frequency */
        let ADCCLK_Frequency = PCLK2_Frequency / presc;
        RccClocks { SYSCLK_Frequency,
                    HCLK_Frequency,
                    PCLK1_Frequency,
                    PCLK2_Frequency,
                    ADCCLK_Frequency
        }
    }
    
    fn AHBPeriphClockCmd(&self, rcc_ahbperiph : u32, new_state : bool) {
        if new_state {
            self.ahbenr     .modify(|r,w| unsafe { w
                .bits( r.bits() | rcc_ahbperiph )
            });
        } else {
            self.ahbenr     .modify(|r,w| unsafe { w
                .bits( r.bits() & !rcc_ahbperiph )
            });
        }
    }
    
    fn APB2PeriphClockCmd(&self, rcc_apb2periph : u32, new_state : bool) {
        if new_state {
            self.apb2enr     .modify(|r,w| unsafe { w
                .bits( r.bits() | rcc_apb2periph )
            });
        } else {
            self.apb2enr     .modify(|r,w| unsafe { w
                .bits( r.bits() & !rcc_apb2periph )
            });
        }
    }
    
    fn APB1PeriphClockCmd(&self, rcc_apb1periph : u32, new_state : bool) {
        if new_state {
            self.apb1enr     .modify(|r,w| unsafe { w
                .bits( r.bits() | rcc_apb1periph )
            });
        } else {
            self.apb1enr     .modify(|r,w| unsafe { w
                .bits( r.bits() & !rcc_apb1periph )
            });
        }
    }
    
    fn APB2PeriphResetCmd(&self, rcc_apb2periph : u32, new_state : bool) {
        if new_state {
            self.apb2rstr     .modify(|r,w| unsafe { w
                .bits( r.bits() | rcc_apb2periph )
            });
        } else {
            self.apb2rstr     .modify(|r,w| unsafe { w
                .bits( r.bits() & !rcc_apb2periph )
            });
        }
    }
    
    fn APB1PeriphResetCmd(&self, rcc_apb1periph : u32, new_state : bool) {
        if new_state {
            self.apb1rstr     .modify(|r,w| unsafe { w
                .bits( r.bits() | rcc_apb1periph )
            });
        } else {
            self.apb1rstr     .modify(|r,w| unsafe { w
                .bits( r.bits() & !rcc_apb1periph )
            });
        }
    }
    
    fn BackupResetCmd(&self, new_state : bool) {
        self.bdcr       .modify(|_,w| w
            .bdrst()    .bit( new_state )
        );
    }
    
    fn ClockSecuritySystemCmd(&self, new_state : bool) {
        self.cr        .modify(|_,w| w
            .csson()   .bit( new_state )
        );
    }
    
    fn MCOConfig(&self, rcc_mco : RccMco) {
        self.cfgr     .modify(|_,w| unsafe { w
            .mco()    .bits( rcc_mco as u8 )
        });
    }
    
    fn GetFlagStatus(&self, rcc_flag : u8) -> FlagStatus {
        /* Get the RCC register index */
        let statusreg = match rcc_flag >> 5 {
            /* The flag to check is in CR register */
            1 => { self.cr.read().bits() },
            /* The flag to check is in BDCR register */
            2 => { self.bdcr.read().bits() },
            /* The flag to check is in CSR register */
            _ => { self.csr.read().bits() }
        };
        /* Get the flag position */
        let tmp = rcc_flag & RccFlag::MASK;
        ((statusreg & (1u32 << tmp)) != (ITStatus::RESET as u32)).into()
    }

    fn ClearFlag(&self) {
        /* Set RMVF bit to clear the reset flags */
        self.csr      .modify(|_,w| w
            .rmvf()    .set_bit()
        );
    }
    
    fn GetITStatus(&self, rcc_it : u8) -> ITStatus {
        /* Check the status of the specified RCC interrupt */
        if (self.cir.read().bits() & rcc_it as u32) != (ITStatus::RESET as u32) {
            ITStatus::SET
        } else {
            ITStatus::RESET
        }
    }
    
    /*fn ClearITPendingBit(&self, rcc_it : u8) {
        /* Perform Byte access to RCC_CIR[23:16] bits to clear the selected interrupt
            pending bits */
        self.cir      .modify(|r,w| unsafe { w
            .bits( r | (rcc_it << 16) )
        });
        // *(__IO uint8_t *) CIR_BYTE3_ADDRESS = rcc_it;
    }*/

}
