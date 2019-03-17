/// ADC Init structure
///
/// Contains the configuration information for the specified ADC peripheral.
#[derive(Clone, Copy, PartialEq)]
pub struct AdcStruct {
    pub mode: DualMode,
    pub scan_conv_mode: bool,
    pub continuous_conv_mode: bool,
    pub external_trig_conv: u8,
    pub data_align: DataAlign,
    pub nbr_of_channel: u8,
}

impl AdcStruct {
    /// Fills each ADC_InitStruct member with its default value.
    pub fn init() -> AdcStruct {
        AdcStruct {
            /* Reset ADC init structure parameters values */
            mode: DualMode::Independent,
            scan_conv_mode: false,
            continuous_conv_mode: false,
            external_trig_conv: ExternalTrigger::TIM1_CC1.into(),
            data_align: DataAlign::Right,
            nbr_of_channel: 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DualMode {
    /// Independent (non-dual) mode
    Independent,
    /// Combined regular simultaneous + injected simultaneous mode
    RegInjecSimult,
    /// Combined regular simultaneous + alternate trigger mode
    RegSimultAlterTrig,
    /// Combined injected simultaneous + fast interleaved mode
    InjecSimultFastInterl,
    /// Combined injected simultaneous + slow interleaved mode
    InjecSimultSlowInterl,
    /// Injected simultaneous mode only
    InjecSimult,
    /// Regular simultaneous mode only
    RegSimult,
    /// Fast interleaved mode only
    FastInterl,
    /// Slow interleaved mode only
    SlowInterl,
    /// Alternate trigger mode only
    AlterTrig,
}

impl From<DualMode> for u8 {
    fn from(mode: DualMode) -> u8 {
        match mode {
            DualMode::Independent => 0,
            DualMode::RegInjecSimult => 1,
            DualMode::RegSimultAlterTrig => 2,
            DualMode::InjecSimultFastInterl => 3,
            DualMode::InjecSimultSlowInterl => 4,
            DualMode::InjecSimult => 5,
            DualMode::RegSimult => 6,
            DualMode::FastInterl => 7,
            DualMode::SlowInterl => 8,
            DualMode::AlterTrig => 9,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum ExternalTrigger {
    /// Timer 1 Compare Output 1
    TIM1_CC1,
    /// Timer 1 Compare Output 2
    TIM1_CC2,
    /// Timer 1 Compare Output 3
    TIM1_CC3,
    /// Timer 2 Compare Output 2
    TIM2_CC2,
    /// Timer 3 Trigger Output
    TIM3_TRGO,
    /// Timer 4 Compare Output 4
    TIM4_CC4,
    /// External Interrupt 11
    EXTI11,
    /// Software Trigger
    SWSTART,
}

impl From<ExternalTrigger> for u8 {
    fn from(trigger: ExternalTrigger) -> u8 {
        match trigger {
            ExternalTrigger::TIM1_CC1 => 0,
            ExternalTrigger::TIM1_CC2 => 1,
            ExternalTrigger::TIM1_CC3 => 2,
            ExternalTrigger::TIM2_CC2 => 3,
            ExternalTrigger::TIM3_TRGO => 4,
            ExternalTrigger::TIM4_CC4 => 5,
            ExternalTrigger::EXTI11 => 6,
            ExternalTrigger::SWSTART => 7,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum ExternalTriggerInjected {
    /// Timer 1 Trigger Output
    TIM1_TRGO,
    /// Timer 1 Compare Output 4
    TIM1_CC4,
    /// Timer 2 Trigger Output
    TIM2_TRGO,
    /// Timer 2 Compare Output 1
    TIM2_CC1,
    /// Timer 3 Compare Output 4
    TIM3_CC4,
    /// Timer 4 Trigger Output
    TIM4_TRGO,
    /// External Interrupt 15
    EXTI15,
    /// Injected Software Trigger
    JSWSTART, /* Software start. */
}

impl From<ExternalTriggerInjected> for u8 {
    fn from(trigger: ExternalTriggerInjected) -> u8 {
        match trigger {
            ExternalTriggerInjected::TIM1_TRGO => 0,
            ExternalTriggerInjected::TIM1_CC4 => 1,
            ExternalTriggerInjected::TIM2_TRGO => 2,
            ExternalTriggerInjected::TIM2_CC1 => 3,
            ExternalTriggerInjected::TIM3_CC4 => 4,
            ExternalTriggerInjected::TIM4_TRGO => 5,
            ExternalTriggerInjected::EXTI15 => 6,
            ExternalTriggerInjected::JSWSTART => 7,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum ExternalTrigger3 {
    /// Timer 2 Compare Output 1
    TIM3_CC1 = 0,
    /// Timer 2 Compare Output 3
    TIM2_CC3 = 1,
    /// Timer 1 Compare Output 3
    TIM1_CC3 = 2,
    /// Timer 8 Compare Output 1
    TIM8_CC1 = 3,
    /// Timer 8 Trigger Output
    TIM8_TRGO = 4,
    /// Timer 5 Compare Output 1
    TIM5_CC1 = 5,
    /// Timer 5 Compare Output 3
    TIM5_CC3 = 6,
}
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq)]
pub enum ExternalTriggerInjected3 {
    /// Timer 1 Trigger Output
    TIM1_TRGO = 0,
    /// Timer 1 Compare Output 4
    TIM1_CC4 = 1,
    /// Timer 4 Compare Output 3
    TIM4_CC3 = 2,
    /// Timer 8 Compare Output 2
    TIM8_CC2 = 3,
    /// Timer 8 Compare Output 4
    TIM8_CC4 = 4,
    /// Timer 5 Trigger Output
    TIM5_TRGO = 5,
    /// Timer 5 Compare Output 4
    TIM5_CC4 = 6,
    /// Injected Software Trigger
    JSWSTART = 7, /* Software start. */
}

/*
/// ADC_extrenal_trigger_sources_for_regular_channels_conversion

/// for ADC1 and ADC2
pub enum AdcExternalTrigConv12 {
    /// Timer 1 Compare Output 1
    T1_CC1 = 0,
    /// Timer 1 Compare Output 2
    T1_CC2 = 1,
    /// Timer 2 Compare Output 2
    T2_CC2 = 3,
    /// Timer 3 Trigger Output
    T3_TRGO = 4,
    /// Timer 4 Compare Output 4
    T4_CC4 = 5,
    /// External Interrupt 11
    Ext_IT11_TIM8_TRGO = 6
}

/// for ADC1, ADC2 and ADC3
pub enum AdcExternalTrigConv123 {
    /// Timer 1 Compare Output 3
    T1_CC3 = 2,
    /// Software Trigger
    None = 7
}

/// for ADC3
pub enum AdcExternalTrigConv3 {
    /// Timer 2 Compare Output 1
    T3_CC1 = 0,
    /// Timer 2 Compare Output 3
    T2_CC3 = 1,
    /// Timer 1 Compare Output 3
    T8_CC1 = 3,
    /// Timer 8 Trigger Output
    T8_TRGO = 4,
    /// Timer 5 Compare Output 1
    T5_CC1 = 5,
    /// Timer 5 Compare Output 3
    T5_CC3 = 6
}

/// ADC_extrenal_trigger_sources_for_injected_channels_conversion 
 
    ///   This parameter can be one of the following values:
    /// * ADC_ExternalTrigInjecConv_T1_TRGO: Timer1 TRGO event 
    ///   selected (for ADC1, ADC2 and ADC3)
    /// * ADC_ExternalTrigInjecConv_T1_CC4: Timer1 capture
    ///   compare4 selected (for ADC1, ADC2 and ADC3)
    /// * ADC_ExternalTrigInjecConv_T2_TRGO: Timer2 TRGO event
    ///   selected (for ADC1 and ADC2)
    /// * ADC_ExternalTrigInjecConv_T2_CC1: Timer2 capture
    ///   compare1 selected (for ADC1 and ADC2)
    /// * ADC_ExternalTrigInjecConv_T3_CC4: Timer3 capture
    ///   compare4 selected (for ADC1 and ADC2)
    /// * ADC_ExternalTrigInjecConv_T4_TRGO: Timer4 TRGO event
    ///   selected (for ADC1 and ADC2)
    /// * ADC_ExternalTrigInjecConv_T4_CC3: Timer4 capture
    ///   compare3 selected (for ADC3 only)
    /// * ADC_ExternalTrigInjecConv_T8_CC2: Timer8 capture
    ///   compare2 selected (for ADC3 only)                         
    /// * ADC_ExternalTrigInjecConv_T8_CC4: Timer8 capture
    ///   compare4 selected (for ADC3 only)
    /// * ADC_ExternalTrigInjecConv_T5_TRGO: Timer5 TRGO event
    ///   selected (for ADC3 only)                         
    /// * ADC_ExternalTrigInjecConv_T5_CC4: Timer5 capture
    ///   compare4 selected (for ADC3 only)

/// For ADC1 and ADC2
pub enum AdcExternalTrigInjecConv12 {
    /// Timer 2 Trigger Output
    T2_TRGO = 2,
    /// Timer 2 Compare Output 1
    T2_CC1 = 3,
    /// Timer 3 Compare Output 4
    T3_CC4 = 4,
    /// Timer 4 Trigger Output
    T4_TRGO = 5,
    /// External Interrupt line 15 or Timer 8 Capture Compare 4 event selected
    Ext_IT15_TIM8_CC4 = 6,
}

/// For ADC1, ADC2 and ADC3
pub enum AdcExternalTrigInjecConv123 {
    /// Timer 1 Trigger Output
    T1_TRGO = 0,
    /// Timer 1 Compare Output 4
    T1_CC4 = 1,
    /// Injected conversion started by Software and not by External trigger
    None = 7
}

/// For ADC3
pub enum AdcExternalTrigInjecConv3 {
    /// Timer 4 Compare Output 3
    T4_CC3 = 2,
    /// Timer 8 Compare Output 2
    T8_CC2 = 3,
    /// Timer 8 Compare Output 4
    T8_CC4 = 4,
    /// Timer 5 Trigger Output
    T5_TRGO = 5,
    /// Timer 5 Compare Output 4
    T5_CC4 = 6,
}
*/

/// ADC_data_align
#[derive(Clone, Copy, PartialEq)]
pub enum DataAlign {
    Right = 0,
    Left = 1,
}

/// ADC_channels
#[derive(Clone, Copy, PartialEq)]
pub enum Channel {
    /// ADC Channel 0 selected
    C0 = 0x00,
    /// ADC Channel 1 selected
    C1 = 0x01,
    /// ADC Channel 2 selected
    C2 = 0x02,
    /// ADC Channel 3 selected
    C3 = 0x03,
    /// ADC Channel 4 selected
    C4 = 0x04,
    /// ADC Channel 5 selected
    C5 = 0x05,
    /// ADC Channel 6 selected
    C6 = 0x06,
    /// ADC Channel 7 selected
    C7 = 0x07,
    /// ADC Channel 8 selected
    C8 = 0x08,
    /// ADC Channel 9 selected
    C9 = 0x09,
    /// ADC Channel 10 selected
    C10 = 0x0A,
    /// ADC Channel 11 selected
    C11 = 0x0B,
    /// ADC Channel 12 selected
    C12 = 0x0C,
    /// ADC Channel 13 selected
    C13 = 0x0D,
    /// ADC Channel 14 selected
    C14 = 0x0E,
    /// ADC Channel 15 selected
    C15 = 0x0F,
    /// ADC Channel 16 selected
    Temperature = 0x10,
    /// ADC Channel 17 selected
    Voltage = 0x11,
}

/// ADC_injected_channel_selection
#[derive(Clone, Copy, PartialEq)]
pub enum InjectedChannel {
    /// Injected Channel 1 selected
    C1,
    /// Injected Channel 2 selected
    C2,
    /// Injected Channel 3 selected
    C3,
    /// Injected Channel 4 selected
    C4,
}

/// ADC_sampling_times
#[derive(Clone, Copy, PartialEq)]
pub enum SampleTime {
    /// Sample time equal to 1.5 cycles
    Cycles1_5,
    /// Sample time equal to 7.5 cycles
    Cycles7_5,
    /// Sample time equal to 13.5 cycles
    Cycles13_5,
    /// Sample time equal to 28.5 cycles
    Cycles28_5,
    /// Sample time equal to 41.5 cycles
    Cycles41_5,
    /// Sample time equal to 55.5 cycles
    Cycles55_5,
    /// Sample time equal to 71.5 cycles
    Cycles71_5,
    /// Sample time equal to 239.5 cycles
    Cycles239_5,
}

impl From<SampleTime> for u32 {
    fn from(time: SampleTime) -> u32 {
        match time {
            SampleTime::Cycles1_5 => 0,
            SampleTime::Cycles7_5 => 1,
            SampleTime::Cycles13_5 => 2,
            SampleTime::Cycles28_5 => 3,
            SampleTime::Cycles41_5 => 4,
            SampleTime::Cycles55_5 => 5,
            SampleTime::Cycles71_5 => 6,
            SampleTime::Cycles239_5 => 7,
        }
    }
}

/// ADC_analog_watchdog_selection
#[derive(Clone, Copy, PartialEq)]
pub enum AnalogWatchdog {
    /// Analog watchdog on a single regular channel
    SingleRegEnable,
    /// Analog watchdog on a single injected channel
    SingleInjecEnable,
    /// Analog watchdog on a single regular or injected channel
    SingleRegOrInjecEnable,
    /// Analog watchdog on all regular channel
    AllRegEnable,
    /// Analog watchdog on all injected channel
    AllInjecEnable,
    /// Analog watchdog on all regular and injected channels
    AllRegAllInjecEnable,
    /// No channel guarded by the analog watchdog
    None,
}

impl From<AnalogWatchdog> for u32 {
    fn from(time: AnalogWatchdog) -> u32 {
        match time {
            AnalogWatchdog::SingleRegEnable => 0x00800200,
            AnalogWatchdog::SingleInjecEnable => 0x00400200,
            AnalogWatchdog::SingleRegOrInjecEnable => 0x00C00200,
            AnalogWatchdog::AllRegEnable => 0x00800000,
            AnalogWatchdog::AllInjecEnable => 0x00400000,
            AnalogWatchdog::AllRegAllInjecEnable => 0x00C00000,
            AnalogWatchdog::None => 0x00000000,
        }
    }
}

pub trait AdcInit {
    /// Initializes the ADCx peripheral according to the specified parameters
    /// in the AdcStruct
    fn init(&self, adc_initstruct: &AdcStruct);
}

pub trait AdcStdExt {
    /// Enables or disables the specified ADC peripheral
    ///
    /// * `new_state: bool` - New state of the ADCx peripheral.
    fn cmd(&self, new_state: bool);

    /// Enables or disables the specified ADC DMA request
    ///
    /// * `new_state: bool` - New state of the selected ADC DMA transfer.
    fn dma_cmd(&self, new_state: bool);

    /// Resets the selected ADC calibration registers
    fn reset_calibration(&self);

    /// Gets the selected ADC reset calibration registers status
    ///
    /// Returns the new state of ADC reset calibration registers.
    fn get_reset_calibration_status(&self) -> bool;

    /// Starts the selected ADC calibration process.
    fn start_calibration(&self);

    /// Gets the selected ADC calibration status
    ///
    /// Returns the new state of ADC calibration.
    fn get_calibration_status(&self) -> bool;

    /// Enables or disables the selected ADC software start conversion
    ///
    /// * `new_state: bool` - New state of the selected ADC software start conversion.
    fn software_start_conv_cmd(&self, new_state: bool);

    /// Gets the selected ADC Software start conversion Status
    ///
    /// Returns the new state of ADC software start conversion.
    fn get_software_start_conv_status(&self) -> bool;

    /// Configures the discontinuous mode for the selected ADC regular group channel.
    ///
    /// * `number: u8` - Specifies the discontinuous mode regular channel
    ///   count value. This number must be between 1 and 8.
    fn disc_mode_channel_count_config(&self, number: u8);

    /// Enables or disables the discontinuous mode on regular group
    ///   channel for the specified ADC
    ///
    /// * `new_state: bool` - New state of the selected ADC discontinuous mode
    ///   on regular group channel.
    fn disc_mode_cmd(&self, new_state: bool);

    /// Configures for the selected ADC regular channel its corresponding
    ///   rank in the sequencer and its sample time.
    ///
    /// * `channel: Channel` - The ADC channel to configure.
    /// * `rank: u8` - The rank in the regular group sequencer. This parameter
    ///   must be between 1 to 16.
    /// * `sample_time: SampleTime` - The sample time value to be set for the
    ///   selected channel.
    fn regular_channel_config(&self, channel: Channel, rank: u8, sample_time: SampleTime);

    /// Enables or disables the ADCx conversion through external trigger.
    ///
    /// * `new_state: bool` - New state of the selected ADC external trigger
    ///   start of conversion.
    fn external_trig_conv_cmd(&self, new_state: bool);

    /// Returns the last ADCx conversion result data for regular channel.
    fn get_conversion_value(&self) -> u16;

    /// Enables or disables the selected ADC automatic injected group
    ///   conversion after regular one.
    ///
    /// * `new_state: bool` - New state of the selected ADC auto injected
    ///   conversion
    fn auto_injected_conv_cmd(&self, new_state: bool);

    /// Enables or disables the discontinuous mode for injected group
    ///   channel for the specified ADC
    ///
    /// * `new_state: bool` - New state of the selected ADC discontinuous mode
    ///   on injected group channel.
    fn injected_disc_mode_cmd(&self, new_state: bool);

    /// Configures the ADCx external trigger for injected channels conversion.
    ///
    /// * `external_trig_injec_conv: u8` - Specifies the ADC trigger to
    ///   start injected conversion.
    fn external_trig_injected_conv_config(&self, external_trig_injec_conv: u8);

    /// Enables or disables the ADCx injected channels conversion
    ///   through external trigger
    ///
    /// * `new_state: bool` - New state of the selected ADC external trigger
    ///   start of injected conversion.
    fn external_trig_injected_conv_cmd(&self, new_state: bool);

    /// Enables or disables the selected ADC start of the injected
    ///   channels conversion.
    ///
    /// * `new_state: bool` - New state of the selected ADC software start
    ///   injected conversion.
    fn software_start_injected_conv_cmd(&self, new_state: bool);

    /// Gets the selected ADC Software start injected conversion Status.
    ///
    /// Returns the new state of ADC software start injected conversion.
    fn get_software_start_injected_conv_cmd_status(&self) -> bool;

    /// Configures for the selected ADC injected channel its corresponding
    ///   rank in the sequencer and its sample time.
    ///
    /// * `channel: Channel` - The ADC channel to configure.
    /// * `rank: u8` - The rank in the injected group sequencer. This parameter
    ///   must be between 1 to 4.
    /// * `sample_time: SampleTime` - The sample time value to be set for the
    ///   selected channel.
    fn injected_channel_config(&self, channel: Channel, rank: u8, sample_time: SampleTime);

    /// Configures the sequencer length for injected channels
    ///
    /// * `length: u8` - The sequencer length.
    ///   This parameter must be a number between 1 to 4.
    fn injected_sequencer_length_config(&self, length: u8);

    /// Set the injected channels conversion value offset
    ///
    /// * `injected_channel: InjectedChannel` - The ADC injected channel to set its
    ///   offset.
    /// * `offset: u16` - The offset value for the selected ADC injected channel
    ///   This parameter must be a 12bit value.
    fn set_injected_offset(&self, injected_channel: InjectedChannel, offset: u16);

    /// Returns the ADC injected channel conversion result
    ///
    /// * `injected_channel: InjectedChannel` - The converted ADC injected channel.
    fn get_injected_conversion_value(&self, injected_channel: InjectedChannel) -> u16;

    /// Enables or disables the analog watchdog on single/all regular
    ///   or injected channels
    ///
    /// * `analog_watchdog: AnalogWatchdog` - The ADC analog watchdog configuration.
    fn analog_watchdog_cmd(&self, analog_watchdog: AnalogWatchdog);

    /// Configures the high and low thresholds of the analog watchdog.
    ///
    /// * `high_threshold` - The ADC analog watchdog High threshold value.
    ///   This parameter must be a 12bit value.
    /// * `low_threshold` - The ADC analog watchdog Low threshold value.
    ///   This parameter must be a 12bit value.
    fn analog_watchdog_thresholds_config(&self, high_threshold: u16, low_threshold: u16);

    /// Configures the analog watchdog guarded single channel
    ///
    /// * `channel : Channel` - The ADC channel to configure for the analog
    ///   watchdog.
    fn analog_watchdog_single_channel_config(&self, channel: Channel);

    /// Enables or disables the temperature sensor and Vrefint channel.
    ///
    /// * `new_state: bool` - New state of the temperature sensor.
    fn temp_sensor_vrefint_cmd(&self, new_state: bool);
}

pub trait AdcDualModeStdExt {
    /// Returns the last ADC1 and ADC2 conversion result data in dual mode.
    fn get_dual_mode_conversion_value(&self) -> u32;
}

macro_rules! impl_adc {
    ($ADCx:ty) => {
        impl AdcStdExt for $ADCx {
            fn cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.adon().bit(new_state));
            }
        
            fn dma_cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.dma().bit(new_state));
            }
        
            fn reset_calibration(&self) {
                /* Resets the selected ADC calibartion registers */
                self.cr2.modify(|_, w| w.rstcal().set_bit());
            }
        
            fn get_reset_calibration_status(&self) -> bool {
                /* Check the status of RSTCAL bit */
                self.cr2.read().rstcal().bit_is_set()
            }
        
            fn start_calibration(&self) {
                /* Enable the selected ADC calibration process */
        
                self.cr2.modify(|_, w| w.cal().set_bit());
            }
        
            fn get_calibration_status(&self) -> bool {
                /* Check the status of CAL bit */
                self.cr2.read().cal().bit_is_set()
            }
        
            fn software_start_conv_cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.swstart().bit(new_state));
            }
        
            fn get_software_start_conv_status(&self) -> bool {
                /* Check the status of SWSTART bit */
                self.cr2.read().swstart().bit_is_set()
            }
        
            fn disc_mode_channel_count_config(&self, number: u8) {
                self.cr1.modify(|_, w| unsafe { w.discnum().bits(number) });
            }
        
            fn disc_mode_cmd(&self, new_state: bool) {
                self.cr1.modify(|_, w| w.discen().bit(new_state));
            }
        
            fn regular_channel_config(&self, channel: Channel, rank: u8, sample_time: SampleTime) {
                /* if ADC_Channel_10 ... ADC_Channel_17 is selected */
                if (channel as u8) > (Channel::C9 as u8) {
                    let ch = 3 * ((channel as u32) - 10);
                    self.smpr1.modify(|r, w| unsafe {
                        w.bits(r.bits() & !(7u32 << ch) | (u32::from(sample_time) << ch))
                    });
                } else {
                    /* ADC_Channel include in ADC_Channel_[0..9] */
                    let ch = 3 * (channel as u32);
                    self.smpr2.modify(|r, w| unsafe {
                        w.bits(r.bits() & !(7u32 << ch) | (u32::from(sample_time) << ch))
                    });
                }
                /* For Rank 1 to 6 */
                if rank < 7 {
                    let rnk = 5 * (rank as u32 - 1);
                    self.sqr3.modify(|r, w| unsafe {
                        w.bits(r.bits() & !(0x1Fu32 << rnk) | ((channel as u32) << rnk))
                    });
                /* For Rank 7 to 12 */
                } else if rank < 13 {
                    let rnk = 5 * (rank as u32 - 7);
                    self.sqr2.modify(|r, w| unsafe {
                        w.bits(r.bits() & !(0x1Fu32 << rnk) | ((channel as u32) << rnk))
                    });
                /* For Rank 13 to 16 */
                } else {
                    let rnk = 5 * (rank as u32 - 13);
                    self.sqr1.modify(|r, w| unsafe {
                        w.bits(r.bits() & !(0x1Fu32 << rnk) | ((channel as u32) << rnk))
                    });
                }
            }
        
            fn external_trig_conv_cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.exttrig().bit(new_state));
            }
        
            fn get_conversion_value(&self) -> u16 {
                /* Return the selected ADC conversion value */
                self.dr.read().data().bits()
            }
        
            fn auto_injected_conv_cmd(&self, new_state: bool) {
                self.cr1.modify(|_, w| w.jauto().bit(new_state));
            }
        
            fn injected_disc_mode_cmd(&self, new_state: bool) {
                self.cr1.modify(|_, w| w.jdiscen().bit(new_state));
            }
        
            fn external_trig_injected_conv_config(&self, external_trig_injec_conv: u8) {
                self.cr2
                    .modify(|_, w| unsafe { w.jextsel().bits(external_trig_injec_conv) });
            }
        
            fn external_trig_injected_conv_cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.jexttrig().bit(new_state));
            }
        
            fn software_start_injected_conv_cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.jswstart().bit(new_state));
            }
        
            fn get_software_start_injected_conv_cmd_status(&self) -> bool {
                /* Check the status of JSWSTART bit */
                self.cr2.read().jswstart().bit_is_set()
            }
        
            fn injected_channel_config(&self, channel: Channel, rank: u8, sample_time: SampleTime) {
                /* if ADC_Channel_10 ... ADC_Channel_17 is selected */
                if (channel as u8) > (Channel::C9 as u8) {
                    let ch = 3 * ((channel as u32) - 10);
                    self.smpr1.modify(|r, w| unsafe {
                        w.bits(r.bits() & !(7u32 << ch) | (u32::from(sample_time) << ch))
                    });
                } else {
                    /* ADC_Channel include in ADC_Channel_[0..9] */
                    let ch = 3 * (channel as u32);
                    self.smpr2.modify(|r, w| unsafe {
                        w.bits(r.bits() & !(7u32 << ch) | (u32::from(sample_time) << ch))
                    });
                }
                /* Rank configuration */
                let jl = self.jsqr.read().jl().bits();
                let rnk = 5 * (((rank + 3) - (jl + 1)) as u8);
                self.jsqr.modify(|r, w| unsafe {
                    w.bits(r.bits() & !(0x1Fu32 << rnk) | ((channel as u32) << rnk))
                });
            }
        
            fn injected_sequencer_length_config(&self, length: u8) {
                self.jsqr.modify(|_, w| unsafe { w.jl().bits(length) });
            }
        
            fn set_injected_offset(&self, injected_channel: InjectedChannel, offset: u16) {
                /* Set the selected injected channel data offset */
                match injected_channel {
                    InjectedChannel::C1 => {
                        self.jofr1.write(|w| w.joffset().bits(offset));
                    }
                    InjectedChannel::C2 => {
                        self.jofr2.write(|w| w.joffset().bits(offset));
                    }
                    InjectedChannel::C3 => {
                        self.jofr3.write(|w| w.joffset().bits(offset));
                    }
                    InjectedChannel::C4 => {
                        self.jofr4.write(|w| w.joffset().bits(offset));
                    }
                }
                //*((__IO uint32_t *)((*(uint32_t*)&ADCx) + ADC_InjectedChannel)) = (uint32_t)Offset;
            }
        
            fn get_injected_conversion_value(&self, injected_channel: InjectedChannel) -> u16 {
                /* Returns the selected injected channel conversion data value */
                match injected_channel {
                    InjectedChannel::C1 => self.jdr1.read().jdata().bits(),
                    InjectedChannel::C2 => self.jdr2.read().jdata().bits(),
                    InjectedChannel::C3 => self.jdr3.read().jdata().bits(),
                    InjectedChannel::C4 => self.jdr4.read().jdata().bits(),
                }
                //return (uint16_t) (*(__IO uint32_t*) (((*(uint32_t*)&ADCx) + ADC_InjectedChannel + JDR_Offset)));
            }
        
            fn analog_watchdog_cmd(&self, analog_watchdog: AnalogWatchdog) {
                /* Clear AWDEN, AWDENJ and AWDSGL bits */
                self.cr1.modify(|_, w| {
                    w.awden()
                        .clear_bit()
                        .jawden()
                        .clear_bit()
                        .awdsgl()
                        .clear_bit()
                });
                /* Set the analog watchdog enable mode */
                self.cr1
                    .modify(|_, w| unsafe { w.bits(analog_watchdog.into()) });
            }
        
            fn analog_watchdog_thresholds_config(&self, high_threshold: u16, low_threshold: u16) {
                /* Set the ADCx high threshold */
                self.htr
                    .modify(|_, w| unsafe { w.ht().bits(high_threshold) });
                /* Set the ADCx low threshold */
                self.ltr
                    .modify(|_, w| unsafe { w.lt().bits(low_threshold) });
            }
        
            fn analog_watchdog_single_channel_config(&self, channel: Channel) {
                /* Set the Analog watchdog channel */
                self.cr1
                    .modify(|_, w| unsafe { w.awdch().bits(channel as u8) });
            }
        
            fn temp_sensor_vrefint_cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.tsvrefe().bit(new_state));
            }
        }
    };
}

macro_rules! impl_adcdualmode {
    ($ADCx:ty) => {
        impl AdcDualModeStdExt for $ADCx {
            fn get_dual_mode_conversion_value(&self) -> u32 {
                /* Return the dual mode conversion value */
                self.dr.read().bits()
            }
        }
    };
}

#[cfg(feature="adc1")]
impl_adc!(crate::pac::ADC1);
#[cfg(feature="adc2")]
impl_adc!(crate::pac::ADC2);

#[cfg(feature="adc1")]
impl_adcdualmode!(crate::pac::ADC1);

#[cfg(feature="adc1")]
impl AdcInit for crate::pac::ADC1 {
    fn init(&self, adc_initstruct: &AdcStruct) {
        /*---------------------------- ADCx CR1 Configuration -----------------*/
        /* Configure ADCx: Dual mode and scan conversion mode */
        /* Set DUALMOD bits according to ADC_Mode value */
        /* Set SCAN bit according to ADC_ScanConvMode value */
        self.cr1.modify(|_, w| unsafe { w
            .dualmod() .bits(adc_initstruct.mode as u8)
            .scan()    .bit(adc_initstruct.scan_conv_mode)
        });

        /*---------------------------- ADCx CR2 Configuration -----------------*/
        /* Configure ADCx: external trigger event and continuous conversion mode */
        /* Set ALIGN bit according to ADC_DataAlign value */
        /* Set EXTSEL bits according to ADC_ExternalTrigConv value */
        /* Set CONT bit according to ADC_ContinuousConvMode value */
        self.cr2.modify(|_, w| unsafe { w
            .align() .bit(adc_initstruct.data_align == DataAlign::Left)
            .extsel().bits(adc_initstruct.external_trig_conv)
            .cont()  .bit(adc_initstruct.continuous_conv_mode)
        });

        /*---------------------------- ADCx SQR1 Configuration -----------------*/
        /* Configure ADCx: regular channel sequence length */
        /* Set L bits according to ADC_NbrOfChannel value */
        self.sqr1
            .modify(|_, w| unsafe { w.l().bits(adc_initstruct.nbr_of_channel - 1) });
    }
}

#[cfg(feature="adc2")]
impl AdcInit for crate::pac::ADC2 {
    fn init(&self, adc_initstruct: &AdcStruct) {
        /*---------------------------- ADCx CR1 Configuration -----------------*/
        /* Configure ADCx: Dual mode and scan conversion mode */
        /* Set DUALMOD bits according to ADC_Mode value */
        /* Set SCAN bit according to ADC_ScanConvMode value */
        self.cr1.modify(|_, w| {
            w
                //.dualmod() .bits( adc_initstruct.mode as u8 )
                .scan()
                .bit(adc_initstruct.scan_conv_mode)
        });

        /*---------------------------- ADCx CR2 Configuration -----------------*/
        /* Configure ADCx: external trigger event and continuous conversion mode */
        /* Set ALIGN bit according to ADC_DataAlign value */
        /* Set EXTSEL bits according to ADC_ExternalTrigConv value */
        /* Set CONT bit according to ADC_ContinuousConvMode value */
        self.cr2.modify(|_, w| unsafe {
            w.align()
                .bit(adc_initstruct.data_align == DataAlign::Left)
                .extsel()
                .bits(adc_initstruct.external_trig_conv)
                .cont()
                .bit(adc_initstruct.continuous_conv_mode)
        });

        /*---------------------------- ADCx SQR1 Configuration -----------------*/
        /* Configure ADCx: regular channel sequence length */
        /* Set L bits according to ADC_NbrOfChannel value */
        self.sqr1
            .modify(|_, w| unsafe { w.l().bits(adc_initstruct.nbr_of_channel - 1) });
    }
}

/******************************************************/

use crate::hal::gpio::Analog;

/// ADC_smp
#[derive(Clone, Copy, PartialEq)]
pub enum Smp {
    Smpr1,
    Smpr2,
}


pub trait AdcChannel {
    /// Channel ID type
    ///
    /// A type used to identify this ADC channel. For example, if the ADC has eight channels, this
    /// might be a `u8`. If the ADC has multiple banks of channels, it could be a tuple, like
    /// `(u8: bank_id, u8: channel_id)`.
    type ID;
    /// The specific ID that identifies this channel, for example `0` for the first ADC channel.
    const CHANNEL: Self::ID;

    fn channel(&self) -> Self::ID {
        Self::CHANNEL
    }
}

pub trait AnalogInputPin {
    const SMPR: Smp;
}

macro_rules! analog_input {
    ($PXi:ident, $CHx:ident, $SMPRx:ident) => {
        impl AdcChannel for $PXi<Analog> {
            type ID = Channel;
            const CHANNEL: Channel = Channel::$CHx;
        }
        impl AnalogInputPin for $PXi<Analog> {
            const SMPR: Smp = Smp::$SMPRx;
        }
    };
}

#[cfg(feature="gpioa")]
use crate::hal::gpio::gpioa::{PA0, PA1, PA2, PA3, PA4, PA5, PA6, PA7};
#[cfg(feature="gpioa")]
analog_input!(PA0, C0, Smpr2);
#[cfg(feature="gpioa")]
analog_input!(PA1, C1, Smpr2);
#[cfg(feature="gpioa")]
analog_input!(PA2, C2, Smpr2);
#[cfg(feature="gpioa")]
analog_input!(PA3, C3, Smpr2);
#[cfg(feature="gpioa")]
analog_input!(PA4, C4, Smpr2);
#[cfg(feature="gpioa")]
analog_input!(PA5, C5, Smpr2);
#[cfg(feature="gpioa")]
analog_input!(PA6, C6, Smpr2);
#[cfg(feature="gpioa")]
analog_input!(PA7, C7, Smpr2);
#[cfg(feature="gpiob")]
use crate::hal::gpio::gpiob::{PB0, PB1};
#[cfg(feature="gpiob")]
analog_input!(PB0, C8, Smpr2);
#[cfg(feature="gpiob")]
analog_input!(PB1, C9, Smpr2);
/*#[cfg(feature="gpioc")]
use crate::hal::gpio::gpioc::{PC0,PC1,PC2,PC3,PC4,PC5};
#[cfg(feature="gpioc")]
analog_input!(PC0, C10, Smpr1);
#[cfg(feature="gpioc")]
analog_input!(PC1, C11, Smpr1);
#[cfg(feature="gpioc")]
analog_input!(PC2, C12, Smpr1);
#[cfg(feature="gpioc")]
analog_input!(PC3, C13, Smpr1);
#[cfg(feature="gpioc")]
analog_input!(PC4, C14, Smpr1);
#[cfg(feature="gpioc")]
analog_input!(PC5, C15, Smpr1);
*/

/******************************************************/
