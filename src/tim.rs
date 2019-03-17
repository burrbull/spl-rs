use crate::hal::timer::Timer;

pub use crate::pac::tim1::ccmr1_input::IC1FR as ICFilter;
pub use crate::pac::tim1::smcr::ETFR as ExtTRGFilter;
//pub use crate::paac::tim1::smcr::SMSW as EncoderMode;



/// TIM Time Base Init structure definition
pub struct TimTimeBaseStruct {
    pub period             : u16,
    pub prescaler          : u16,
    pub counter_mode       : CounterMode,
    pub clock_division     : ClockDivision,
    pub repetition_counter : u8
}

impl TimTimeBaseStruct {
    /// Fills each time_base_initstruct member with its default value.
    /// `time_base_initstruct`: pointer to a TIM_TimeBaseInitTypeDef
    ///   structure which will be initialized.
    pub fn init() -> TimTimeBaseStruct {
        // Set the default configuration
        TimTimeBaseStruct {
            period             : 0xFFFF,
            prescaler          : 0x0000,
            counter_mode       : CounterMode::Up,
            clock_division     : ClockDivision::NODIV,
            repetition_counter : 0x0000
        }
    }
}

/// TIM Output Compare Init structure definition
pub struct TimOCStruct {
    pub oc_mode        : OCMode,
    pub output_state   : bool,
    pub output_n_state : bool,
    pub pulse          : u16,
    pub oc_polarity    : OCPolarity,
    pub ocn_polarity   : OCPolarity,
    pub oc_idle_state  : bool,
    pub ocn_idle_state : bool
}

impl TimOCStruct {
    /// Fills each oc_initstruct member with its default value.
    /// `oc_initstruct`: pointer to a TIM_OCInitTypeDef structure
    ///   which will be initialized.
    pub fn init() -> TimOCStruct {
        // Set the default configuration
        TimOCStruct {
            oc_mode        : OCMode::Frozen,
            output_state   : false,
            output_n_state : false,
            pulse          : 0x0000,
            oc_polarity    : OCPolarity::High,
            ocn_polarity   : OCPolarity::High,
            oc_idle_state  : false,
            ocn_idle_state : false
        }
    }
}
/*
/// TIM Input Capture Init structure definition  
pub struct TimICStruct {
    channel      : Channel,
    ic_polarity  : ICPolarity,
    ic_selection : ICSelection,
    ic_prescaler : ICPrescaler,
    ic_filter    : ICFilter,
}

impl TimICStruct {
    /// Fills each TIM_ICInitStruct member with its default value.
    /// `TIM_ICInitStruct`: pointer to a TIM_ICInitTypeDef structure
    ///   which will be initialized.
    pub fn init() -> TimICStruct {
        // Set the default configuration
        TimICStruct {
            channel      : Channel::C1,
            ic_polarity  : ICPolarity::Rising,
            ic_selection : ICSelection::DirectTI,
            ic_prescaler : ICPrescaler::NODIV,
            ic_filter    : ICFilter::OFF
        }
    }
}
*/
/// BDTR structure definition  
pub struct TimBDTRStruct {
    ossr_state       : bool,
    ossi_state       : bool,
    lock_level       : LOCKLevel, 
    dead_time        : u8,
    Break            : bool,
    break_polarity   : BreakPolarity,
    automatic_output : bool
}

impl TimBDTRStruct {
    /// Fills each bdtr_initstruct member with its default value.
    /// `bdtr_initstruct`: pointer to a TIM_BDTRInitTypeDef
    ///   structure which will be initialized.
    pub fn init() -> TimBDTRStruct {
        // Set the default configuration
        TimBDTRStruct {
            ossr_state       : false,
            ossi_state       : false,
            lock_level       : LOCKLevel::OFF,
            dead_time        : 0x00,
            Break            : false,
            break_polarity   : BreakPolarity::LOW,
            automatic_output : false
        }
    }
}

/*
#define IS_TIM_ALL_PERIPH(PERIPH) (((*(uint32_t*)&(PERIPH)) == TIM1_BASE) || \
                                   ((*(uint32_t*)&(PERIPH)) == TIM2_BASE) || \
                                   ((*(uint32_t*)&(PERIPH)) == TIM3_BASE) || \
                                   ((*(uint32_t*)&(PERIPH)) == TIM4_BASE) || \
                                   ((*(uint32_t*)&(PERIPH)) == TIM5_BASE) || \
                                   ((*(uint32_t*)&(PERIPH)) == TIM6_BASE) || \
                                   ((*(uint32_t*)&(PERIPH)) == TIM7_BASE) || \
                                   ((*(uint32_t*)&(PERIPH)) == TIM8_BASE))
*/

pub use crate::pac::tim1::ccmr1_output::CC1MR as OCMode;
/// TIM_Output_Compare_and_PWM_modes 
pub enum OutputCompare {
    Timing   = 0,
    Active   = 1,
    Inactive = 2,
    Toggle   = 3,
    PWM1     = 6,
    PWM2     = 7
}

/// TIM_Forced_Action 
pub enum ForcedAction {
    InActive = 4,
    Active   = 5
}

/// TIM_One_Pulse_Mode 
pub use crate::pac::tim1::cr1::OPMR as OPMode;
//#define TIM_OPMode_Single                  ((uint16_t)0x0008)
//#define TIM_OPMode_Repetitive              ((uint16_t)0x0000)

/// channel
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel {
    C1, //= 0x0,
    C2, //= 0x4,
    C3, //= 0x8,
    C4  //= 0xC
}
/*#define IS_TIM_PWMI_CHANNEL(CHANNEL) (((CHANNEL) == channel_1) || \
                                      ((CHANNEL) == channel_2))
#define IS_TIM_COMPLEMENTARY_CHANNEL(CHANNEL) (((CHANNEL) == channel_1) || \
                                               ((CHANNEL) == channel_2) || \
                                               ((CHANNEL) == channel_3)) 
#define IS_TIM_PERIPH_DMA(PERIPH, SOURCE) ((((((*(uint32_t*)&(PERIPH)) == TIM2_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM3_BASE))||\
                                            (((*(uint32_t*)&(PERIPH)) == TIM4_BASE)) || (((*(uint32_t*)&(PERIPH)) == TIM5_BASE))))&& \
                                            (((SOURCE) & (uint16_t)0xA0FF) == 0x0000) && ((SOURCE) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM1_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM8_BASE))))&& \
                                            (((SOURCE) & (uint16_t)0x80FF) == 0x0000) && ((SOURCE) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM6_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM7_BASE))))&& \
                                            (((SOURCE) & (uint16_t)0xFEFF) == 0x0000) && ((SOURCE) != 0x0000))) 
*/

/// TIM_Clock_Division_CKD 
pub use crate::pac::tim1::cr1::CKDR as ClockDivision;
//#define tim_ckd_DIV1                       ((uint16_t)0x0000)
//#define tim_ckd_DIV2                       ((uint16_t)0x0100)
//#define tim_ckd_DIV4                       ((uint16_t)0x0200)

pub use crate::pac::tim1::cr1::DIRR as CounterDirection;
pub use crate::pac::tim1::cr1::CMSR as CounterAlign;
/// TIM_Counter_Mode 
pub enum CounterMode {
    Up             = 0,
    Down           = 1,
    CenterAligned1 = 2,
    CenterAligned2 = 4,
    CenterAligned3 = 6
}
impl CounterMode {
    pub fn to_dir_cms(&self) -> (CounterDirection, CounterAlign) {
        match self {
            CounterMode::Up   => (CounterDirection::UP, CounterAlign::EDGE_ALIGNED),
            CounterMode::Down => (CounterDirection::DOWN, CounterAlign::EDGE_ALIGNED),
            CounterMode::CenterAligned1 => (CounterDirection::UP, CounterAlign::CENTER_ALIGNED1),
            CounterMode::CenterAligned2 => (CounterDirection::UP, CounterAlign::CENTER_ALIGNED2),
            CounterMode::CenterAligned3 => (CounterDirection::UP, CounterAlign::CENTER_ALIGNED3)
        }
    }
}

/// TIM_Output_Compare_Polarity 
pub enum OCPolarity {
    High = 0,
    Low  = 1
}

/// Break_Polarity 
pub use crate::pac::tim1::bdtr::BKPR as BreakPolarity;
//#define TIM_BreakPolarity_Low              ((uint16_t)0x0000)
//#define TIM_BreakPolarity_High             ((uint16_t)0x2000)

/// TIM_AOE_Bit_Set_Reset 
//#define TIM_AutomaticOutput_Enable         ((uint16_t)0x4000)
//#define TIM_AutomaticOutput_Disable        ((uint16_t)0x0000)

/// Lock_levels 
pub use crate::pac::tim1::bdtr::LOCKR as LOCKLevel;
//#define TIM_LOCKLevel_OFF                  ((uint16_t)0x0000)
//#define TIM_LOCKLevel_1                    ((uint16_t)0x0100)
//#define TIM_LOCKLevel_2                    ((uint16_t)0x0200)
//#define TIM_LOCKLevel_3                    ((uint16_t)0x0300)

/// TIM_Input_Capture_Polarity 
pub enum ICPolarity {
    Rising = 0,
    Falling = 1
}

/// TIM_Input_Capture_Selection 
pub enum ICSelection {
    DirectTI   = 1,
    IndirectTI = 2,
    TRC        = 3
}

/// TIM_Input_Capture_Prescaler 

pub use crate::pac::tim1::ccmr1_input::IC1PSCR as ICPrescaler;
//#define ic_psc_DIV1                     ((uint16_t)0x0000)
//#define ic_psc_DIV2                     ((uint16_t)0x0004)
//#define ic_psc_DIV4                     ((uint16_t)0x0008)
//#define ic_psc_DIV8                     ((uint16_t)0x000C) 


/// TIM_DMA_Base_address 
pub use crate::pac::tim1::dcr::DBAR as DMABaseAddress;
/*#define TIM_DMABase_CR1                    ((uint16_t)0x0000)
#define TIM_DMABase_CR2                    ((uint16_t)0x0001)
#define TIM_DMABase_SMCR                   ((uint16_t)0x0002)
#define TIM_DMABase_DIER                   ((uint16_t)0x0003)
#define TIM_DMABase_SR                     ((uint16_t)0x0004)
#define TIM_DMABase_EGR                    ((uint16_t)0x0005)
#define TIM_DMABase_CCMR1                  ((uint16_t)0x0006)
#define TIM_DMABase_CCMR2                  ((uint16_t)0x0007)
#define TIM_DMABase_CCER                   ((uint16_t)0x0008)
#define TIM_DMABase_CNT                    ((uint16_t)0x0009)
#define TIM_DMABase_PSC                    ((uint16_t)0x000A)
#define TIM_DMABase_ARR                    ((uint16_t)0x000B)
#define TIM_DMABase_RCR                    ((uint16_t)0x000C)
#define TIM_DMABase_CCR1                   ((uint16_t)0x000D)
#define TIM_DMABase_CCR2                   ((uint16_t)0x000E)
#define TIM_DMABase_CCR3                   ((uint16_t)0x000F)
#define TIM_DMABase_CCR4                   ((uint16_t)0x0010)
#define TIM_DMABase_BDTR                   ((uint16_t)0x0011)
#define TIM_DMABase_DCR                    ((uint16_t)0x0012)*/

/// TIM_DMA_Burst_Length 
pub use crate::pac::tim1::dcr::DBLR as DMABurstLength;
/*#define TIM_DMABurstLength_1Byte           ((uint16_t)0x0000)
#define TIM_DMABurstLength_2Bytes          ((uint16_t)0x0100)
#define TIM_DMABurstLength_3Bytes          ((uint16_t)0x0200)
#define TIM_DMABurstLength_4Bytes          ((uint16_t)0x0300)
#define TIM_DMABurstLength_5Bytes          ((uint16_t)0x0400)
#define TIM_DMABurstLength_6Bytes          ((uint16_t)0x0500)
#define TIM_DMABurstLength_7Bytes          ((uint16_t)0x0600)
#define TIM_DMABurstLength_8Bytes          ((uint16_t)0x0700)
#define TIM_DMABurstLength_9Bytes          ((uint16_t)0x0800)
#define TIM_DMABurstLength_10Bytes         ((uint16_t)0x0900)
#define TIM_DMABurstLength_11Bytes         ((uint16_t)0x0A00)
#define TIM_DMABurstLength_12Bytes         ((uint16_t)0x0B00)
#define TIM_DMABurstLength_13Bytes         ((uint16_t)0x0C00)
#define TIM_DMABurstLength_14Bytes         ((uint16_t)0x0D00)
#define TIM_DMABurstLength_15Bytes         ((uint16_t)0x0E00)
#define TIM_DMABurstLength_16Bytes         ((uint16_t)0x0F00)
#define TIM_DMABurstLength_17Bytes         ((uint16_t)0x1000)
#define TIM_DMABurstLength_18Bytes         ((uint16_t)0x1100)*/

/// TIM_DMA_sources 
pub mod DmaSource {
    pub const Update : u16 = 0x0100;
    pub const CC1    : u16 = 0x0200;
    pub const CC2    : u16 = 0x0400;
    pub const CC3    : u16 = 0x0800;
    pub const CC4    : u16 = 0x1000;
    pub const COM    : u16 = 0x2000;
    pub const Trigger: u16 = 0x4000;
}

/// TIM_External_Trigger_Prescaler 
pub use crate::pac::tim1::smcr::ETPSR as ExtTPGPrescaler;
/*#define TIM_ExtTRGPSC_OFF                  ((uint16_t)0x0000)
#define TIM_ExtTRGPSC_DIV2                 ((uint16_t)0x1000)
#define TIM_ExtTRGPSC_DIV4                 ((uint16_t)0x2000)
#define TIM_ExtTRGPSC_DIV8                 ((uint16_t)0x3000)
*/

/// TIM_Internal_Trigger_Selection 
pub use crate::pac::tim1::smcr::TSR as InputTriggerSource;
/*#define TIM_TS_ITR0                        ((uint16_t)0x0000)
#define TIM_TS_ITR1                        ((uint16_t)0x0010)
#define TIM_TS_ITR2                        ((uint16_t)0x0020)
#define TIM_TS_ITR3                        ((uint16_t)0x0030)
#define TIM_TS_TI1F_ED                     ((uint16_t)0x0040)
#define TIM_TS_TI1FP1                      ((uint16_t)0x0050)
#define TIM_TS_TI2FP2                      ((uint16_t)0x0060)
#define TIM_TS_ETRF                        ((uint16_t)0x0070)
#define IS_TIM_INTERNAL_TRIGGER_SELECTION(SELECTION) (((SELECTION) == TIM_TS_ITR0) || \
                                                      ((SELECTION) == TIM_TS_ITR1) || \
                                                      ((SELECTION) == TIM_TS_ITR2) || \
                                                      ((SELECTION) == TIM_TS_ITR3)) */

/// TIM_TIx_External_Clock_Source 
pub enum TIxExternalCLK1Source {
    TI1 = 0x0050,
    TI2 = 0x0060,
    TI1ED = 0x0040,
}

/// TIM_External_Trigger_Polarity  
pub use crate::pac::tim1::smcr::ETPW as ExtTRGPolarity;
//#define ext_trg_polarity_Inverted        ((uint16_t)0x8000)
//#define ext_trg_polarity_NonInverted     ((uint16_t)0x0000)

/// TIM_Prescaler_Reload_Mode 

pub use crate::pac::tim1::egr::UGW as PSCReloadMode;
//#define TIM_PSCReloadMode_Update           ((uint16_t)0x0000)
//#define TIM_PSCReloadMode_Immediate        ((uint16_t)0x0001) 

/// TIM_Encoder_Mode 
pub enum EncoderMode {
    TI1 = 0x0001,
    TI2 = 0x0002,
    TI12 = 0x0003
}

/// TIM_Event_Source 
pub mod EventSource {
    pub const Update  : u16 = 0x0001;
    pub const CC1     : u16 = 0x0002;
    pub const CC2     : u16 = 0x0004;
    pub const CC3     : u16 = 0x0008;
    pub const CC4     : u16 = 0x0010;
    pub const COM     : u16 = 0x0020;
    pub const Trigger : u16 = 0x0040;
    pub const Break   : u16 = 0x0080;
}
/*#define IS_TIM_EVENT_SOURCE(SOURCE) ((((SOURCE) & (uint16_t)0xFF00) == 0x0000) && ((SOURCE) != 0x0000))
#define IS_TIM_PERIPH_EVENT(PERIPH, EVENT) ((((((*(uint32_t*)&(PERIPH)) == TIM2_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM3_BASE))||\
                                            (((*(uint32_t*)&(PERIPH)) == TIM4_BASE)) || (((*(uint32_t*)&(PERIPH)) == TIM5_BASE))))&& \
                                            (((EVENT) & (uint16_t)0xFFA0) == 0x0000) && ((EVENT) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM1_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM8_BASE))))&& \
                                            (((EVENT) & (uint16_t)0xFF00) == 0x0000) && ((EVENT) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM6_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM7_BASE))))&& \
                                            (((EVENT) & (uint16_t)0xFFFE) == 0x0000) && ((EVENT) != 0x0000))) */

/// TIM_Update_Source 
pub use crate::pac::tim1::cr1::URSW as UpdateSource;
//#define TIM_UpdateSource_Global            ((uint16_t)0x0000)
//#define TIM_UpdateSource_Regular           ((uint16_t)0x0001)

/// TIM_Ouput_Compare_Preload_State 
//#define oc_preload_Enable               ((uint16_t)0x0008)
//#define oc_preload_Disable              ((uint16_t)0x0000)

/// TIM_Ouput_Compare_Fast_State 
//#define oc_fast_Enable                  ((uint16_t)0x0004)
//#define oc_fast_Disable                 ((uint16_t)0x0000)
                                      

/// TIM_Ouput_Compare_Clear_State 
//#define oc_clear_Enable                 ((uint16_t)0x0080)
//#define oc_clear_Disable                ((uint16_t)0x0000)

/// TIM_Trigger_Output_Source 
pub use crate::pac::tim1::cr2::MMSR as TRGOSource;
#[cfg(feature="tim6")]
pub use crate::pac::tim6::cr2::MMSR as TRGOSource6;
/*
#define TIM_TRGOSource_Reset               ((uint16_t)0x0000)
#define TIM_TRGOSource_Enable              ((uint16_t)0x0010)
#define TIM_TRGOSource_Update              ((uint16_t)0x0020)
#define TIM_TRGOSource_OC1                 ((uint16_t)0x0030)
#define TIM_TRGOSource_OC1Ref              ((uint16_t)0x0040)
#define TIM_TRGOSource_OC2Ref              ((uint16_t)0x0050)
#define TIM_TRGOSource_OC3Ref              ((uint16_t)0x0060)
#define TIM_TRGOSource_OC4Ref              ((uint16_t)0x0070)

*/
/// TIM_Slave_Mode 
pub enum SlaveMode {
    Reset = 0x0004,
    Gated = 0x0005,
    Trigger = 0x0006,
    External1 = 0x0007
}

/// TIM_Master_Slave_Mode 
//#define master_slave_mode_Enable         ((uint16_t)0x0080)
//#define master_slave_mode_Disable        ((uint16_t)0x0000)





/// `TIMx`: where x can be 1 to 8 to select the TIM peripheral.
pub trait TimBaseSplExt {

    /// Enables or disables the specified TIM peripheral.
    /// `new_state : bool`: new state of the TIMx peripheral.
    ///     This parameter can be: ENABLE or DISABLE.
    fn cmd(&self, new_state : bool);

    /// Configures the TIMx event to be generate by software.
    /// `TIM_EventSource`: specifies the event source.
    ///     This parameter can be one or more of the following values:
    /// * TIM_EventSource_Update: Timer update Event source
    /// * TIM_EventSource_CC1: Timer Capture Compare 1 Event source
    /// * TIM_EventSource_CC2: Timer Capture Compare 2 Event source
    /// * TIM_EventSource_CC3: Timer Capture Compare 3 Event source
    /// * TIM_EventSource_CC4: Timer Capture Compare 4 Event source
    /// * TIM_EventSource_Trigger: Timer Trigger Event source
    fn generate_event(&self, event_source : u16);

    /// Enables or disables the TIMx’s DMA Requests.
    /// `TIM_DMASource`: specifies the DMA Request sources.
    ///     This parameter can be any combination of the following values:
    /// * TIM_DMA_Update: TIM update Interrupt source
    /// * TIM_DMA_CC1: TIM Capture Compare 1 DMA source
    /// * TIM_DMA_CC2: TIM Capture Compare 2 DMA source
    /// * TIM_DMA_CC3: TIM Capture Compare 3 DMA source
    /// * TIM_DMA_CC4: TIM Capture Compare 4 DMA source
    /// * TIM_DMA_COM: TIM Commutation DMA source
    /// * TIM_DMA_Trigger: TIM Trigger DMA source
    /// `new_state : bool`: new state of the DMA Request sources.
    ///     This parameter can be: ENABLE or DISABLE.
    fn dma_cmd(&self, dma_source : u16, new_state : bool);

    /// Configures the TIMx Prescaler.
    /// `Prescaler`: specifies the Prescaler Register value
    /// `TIM_PSCReloadMode`: specifies the TIM Prescaler Reload mode
    ///     This parameter can be one of the following values:
    /// * TIM_PSCReloadMode_Update: The Prescaler is loaded at
    ///     the update event.
    /// * TIM_PSCReloadMode_Immediate: The Prescaler is loaded
    ///     immediatly.
    fn prescaler_config(&self, prescaler : u16, psc_reload_mode : PSCReloadMode);

    /// Enables or Disables the TIMx Update event.
    /// `new_state : bool`: new state of the TIMx UDIS bit
    ///     This parameter can be: ENABLE or DISABLE.
    fn update_disable_config(&self, new_state : bool);

    /// Configures the TIMx Update Request Interrupt source.
    /// `TIM_UpdateSource`: specifies the Update source.
    ///     This parameter can be one of the following values:
    /// * TIM_UpdateSource_Regular
    /// * TIM_UpdateSource_Global
    fn update_request_config(&self, update_source : UpdateSource);

    /// Selects the TIMx’s One Pulse Mode.
    /// `TIM_OPMode`: specifies the OPM Mode to be used.
    ///     This parameter can be one of the following values:
    /// * TIM_OPMode_Single
    /// * TIM_OPMode_Repetitive
    fn select_one_pulse_mode(&self, op_mode : OPMode);

    /// Sets the TIMx Counter Register value
    /// `Counter`: specifies the Counter register new value.
    fn set_counter(&self, counter : u16);

    /// Sets the TIMx Autoreload Register value
    /// `Autoreload`: specifies the Autoreload register new value.
    fn set_autoreload(&self, autoreload : u16);

    /// Gets the TIMx Counter value.
    /// @retval : Counter Register value.
    fn get_counter(&self) -> u16;

    /// Gets the TIMx Prescaler value.
    /// @retval : Prescaler Register value.
    fn get_prescaler(&self) -> u16;
}


/// `TIMx`: where x can be 6 to 7 to select the TIM peripheral.
pub trait TimBaseOnlySplExt {
    /// Selects the TIMx Trigger Output Mode.
    /// `TIM_TRGOSource`: specifies the Trigger Output source.
    ///     This paramter can be as follow:
    /// * TIM_TRGOSource_Reset 
    /// * TIM_TRGOSource_Enable
    /// * TIM_TRGOSource_Update
    fn select_output_trigger(&self, trgo_source : TRGOSource6);
}

/// `TIMx`: where x can be    1 or 8 to select the TIM 
pub trait TimAdvancedSplExt {

    /// Configures the: Break feature, dead time, Lock level, the OSSI,
    ///     the OSSR State and the AOE(automatic output enable).
    /// `bdtr_initstruct`: pointer to a TIM_BDTRInitTypeDef
    ///     structure that contains the BDTR Register configuration
    ///     information for the TIM peripheral.
    fn bdtr_config(&self, bdtr_initstruct : &TimBDTRStruct);

    /// Enables or disables the TIM peripheral Main Outputs.
    /// `new_state : bool`: new state of the TIM peripheral Main Outputs.
    ///     This parameter can be: ENABLE or DISABLE.
    fn ctrl_pwm_outputs(&self, new_state : bool);

    /// Selects the TIM peripheral Commutation event.
    /// `new_state : bool`: new state of the Commutation event.
    ///     This parameter can be: ENABLE or DISABLE.
    fn select_com(&self, new_state : bool);

    /// Sets or Resets the TIM peripheral Capture Compare Preload 
    ///     Control bit.
    /// `new_state : bool`: new state of the Capture Compare Preload Control bit
    ///     This parameter can be: ENABLE or DISABLE.
    fn cc_preload_control(&self, new_state : bool);

    /// Configures the TIMx Channel 1N polarity.
    /// `ocn_polarity`: specifies the OC1N Polarity
    ///     This parmeter can be one of the following values:
    /// * ocn_polarity_High: Output Compare active high
    /// * ocn_polarity_Low: Output Compare active low
    fn oc1n_polarity_config(&self, ocn_polarity : OCPolarity);

    /// Configures the TIMx Channel 2N polarity.
    /// `ocn_polarity`: specifies the OC2N Polarity
    ///     This parmeter can be one of the following values:
    /// * ocn_polarity_High: Output Compare active high
    /// * ocn_polarity_Low: Output Compare active low
    fn oc2n_polarity_config(&self, ocn_polarity : OCPolarity);

    /// Configures the TIMx Channel 3N polarity.
    /// `ocn_polarity`: specifies the OC3N Polarity
    ///     This parmeter can be one of the following values:
    /// * ocn_polarity_High: Output Compare active high
    /// * ocn_polarity_Low: Output Compare active low
    fn oc3n_polarity_config(&self, ocn_polarity : OCPolarity);

    /// Enables or disables the TIM Capture Compare Channel xN.
    /// `channel`: specifies the TIM Channel
    ///     This parmeter can be one of the following values:
    /// * channel_1: TIM Channel 1
    /// * channel_2: TIM Channel 2
    /// * channel_3: TIM Channel 3
    /// `ccxn`: specifies the TIM Channel CCxNE bit new state.
    ///     This parameter can be: ccxn_Enable or ccxn_Disable. 
    fn ccxn_cmd(&self, channel : Channel, ccxn : bool);
}


pub trait TimInitSplExt {
    /// Initializes the TIMx Time Base Unit peripheral according to 
    ///     the specified parameters in the time_base_initstruct.
    /// `time_base_initstruct`: pointer to a TIM_TimeBaseInitTypeDef
    ///     structure that contains the configuration information for
    ///     the specified TIM peripheral.
    fn time_base_init(&self, time_base_initstruct : &TimTimeBaseStruct);

    /// Initializes the TIMx Channel1 according to the specified
    ///     parameters in the oc_initstruct.
    /// `oc_initstruct`: pointer to a TIM_OCInitTypeDef structure
    ///     that contains the configuration information for the specified
    ///     TIM peripheral.
    fn oc1_init(&self, oc_initstruct : &TimOCStruct);

    /// Initializes the TIMx Channel2 according to the specified
    ///     parameters in the oc_initstruct.
    /// `oc_initstruct`: pointer to a TIM_OCInitTypeDef structure
    ///     that contains the configuration information for the specified
    ///     TIM peripheral.
    fn oc2_init(&self, oc_initstruct : &TimOCStruct);

    /// Initializes the TIMx Channel3 according to the specified
    ///     parameters in the oc_initstruct.
    /// `oc_initstruct`: pointer to a TIM_OCInitTypeDef structure
    ///     that contains the configuration information for the specified
    ///     TIM peripheral.
    fn oc3_init(&self, oc_initstruct : &TimOCStruct);

    /// Initializes the TIMx Channel4 according to the specified
    ///     parameters in the oc_initstruct.
    /// `oc_initstruct`: pointer to a TIM_OCInitTypeDef structure
    ///     that contains the configuration information for the specified
    ///     TIM peripheral.
    fn oc4_init(&self, oc_initstruct : &TimOCStruct);
}


/// `TIMx`: where x can be 1, 2, 3, 4, 5 or 8 to select the TIM peripheral.
pub trait TimGeneralSplExt {

    /// Selects the TIMx Trigger Output Mode.
    /// `TIM_TRGOSource`: specifies the Trigger Output source.
    ///     This paramter can be as follow:
    /// * TIM_TRGOSource_Reset 
    /// * TIM_TRGOSource_Enable
    /// * TIM_TRGOSource_Update
    /// * TIM_TRGOSource_OC1
    /// * TIM_TRGOSource_OC1Ref
    /// * TIM_TRGOSource_OC2Ref
    /// * TIM_TRGOSource_OC3Ref
    /// * TIM_TRGOSource_OC4Ref
    fn select_output_trigger(&self, trgo_source : TRGOSource);

    /// Initializes the TIM peripheral according to the specified
    ///     parameters in the TIM_ICInitStruct.
    /// `TIM_ICInitStruct`: pointer to a TIM_ICInitTypeDef structure
    ///     that contains the configuration information for the specified
    ///     TIM peripheral.
    fn ic_init(&self, ic_initstruct : &TimICStruct);

    /// Configures the TIM peripheral according to the specified
    ///     parameters in the TIM_ICInitStruct to measure an external PWM
    ///     signal.
    /// `TIM_ICInitStruct`: pointer to a TIM_ICInitTypeDef structure
    ///     that contains the configuration information for the specified
    ///     TIM peripheral.
    fn pwmi_config(&self, ic_initstruct : &TimICStruct);

    /// Configures the TIMx’s DMA interface.
    /// `TIM_DMABase`: DMA Base address.
    ///     This parameter can be one of the following values:
    /// * TIM_DMABase_CR, TIM_DMABase_CR2, TIM_DMABase_SMCR,
    ///     TIM_DMABase_DIER, TIM1_DMABase_SR, TIM_DMABase_EGR,
    ///     TIM_DMABase_CCMR1, TIM_DMABase_CCMR2, TIM_DMABase_CCER,
    ///     TIM_DMABase_CNT, TIM_DMABase_PSC, TIM_DMABase_ARR,
    ///     TIM_DMABase_RCR, TIM_DMABase_CCR1, TIM_DMABase_CCR2,
    ///     TIM_DMABase_CCR3, TIM_DMABase_CCR4, TIM_DMABase_BDTR,
    ///     TIM_DMABase_DCR.
    /// `TIM_DMABurstLength`: DMA Burst length.
    ///     This parameter can be one value between:
    ///     TIM_DMABurstLength_1Byte and TIM_DMABurstLength_18Bytes.
    fn dma_config(&self, dma_base : DMABase, dma_burst_length : DMABurstLength);

    /// Configures the TIMx interrnal Clock
    fn internal_clock_config(&self);

    /// Configures the TIMx Internal Trigger as External Clock
    /// `TIM_ITRSource`: Trigger source.
    ///     This parameter can be one of the following values:
    /// `TIM_TS_ITR0`: Internal Trigger 0
    /// `TIM_TS_ITR1`: Internal Trigger 1
    /// `TIM_TS_ITR2`: Internal Trigger 2
    /// `TIM_TS_ITR3`: Internal Trigger 3
    fn itrx_external_clock_config(&self, input_trigger_source : InputTriggerSource);

    /// Configures the TIMx Trigger as External Clock
    /// `tix_external_clk_source`: Trigger source.
    ///     This parameter can be one of the following values:
    /// * TIM_TIxExternalCLK1Source_TI1ED: TI1 Edge Detector
    /// * TIM_TIxExternalCLK1Source_TI1: Filtered Timer Input 1
    /// * TIM_TIxExternalCLK1Source_TI2: Filtered Timer Input 2
    /// `ic_polarity`: specifies the TIx Polarity.
    ///     This parameter can be:
    /// * ic_polarity_Rising
    /// * ic_polarity_Falling
    /// `ICFilter`: specifies the filter value.
    ///     This parameter must be a value between 0x0 and 0xF.
    fn tix_external_clock_config(&self, tix_external_clk_source : TIxExternalCLK1Source,
                        ic_polarity : ICPolarity, ic_filter : ICFilter);

    /// Configures the External clock Mode1
    /// `ext_trg_prescaler`: The external Trigger Prescaler.
    ///     It can be one of the following values:
    /// * TIM_ExtTRGPSC_OFF
    /// * TIM_ExtTRGPSC_DIV2
    /// * TIM_ExtTRGPSC_DIV4
    /// * TIM_ExtTRGPSC_DIV8.
    /// `ext_trg_polarity`: The external Trigger Polarity.
    ///     It can be one of the following values:
    /// * ext_trg_polarity_Inverted
    /// * ext_trg_polarity_NonInverted
    /// `ext_trg_filter`: External Trigger Filter.
    ///     This parameter must be a value between 0x00 and 0x0F
    fn etr_clock_mode1_config(&self, ext_trg_prescaler : ExtTRGPrescaler,
                                    ext_trg_polarity : ExtTRGPolarity,
                                    ext_trg_filter : ExtTRGFilter);

    /// Configures the External clock Mode2
    /// `ext_trg_prescaler`: The external Trigger Prescaler.
    ///     It can be one of the following values:
    /// * TIM_ExtTRGPSC_OFF
    /// * TIM_ExtTRGPSC_DIV2
    /// * TIM_ExtTRGPSC_DIV4
    /// * TIM_ExtTRGPSC_DIV8
    /// `ext_trg_polarity`: The external Trigger Polarity.
    ///     It can be one of the following values:
    /// * ext_trg_polarity_Inverted
    /// * ext_trg_polarity_NonInverted
    /// `ext_trg_filter`: External Trigger Filter.
    ///     This parameter must be a value between 0x00 and 0x0F
    fn etr_clock_mode2_config(&self, ext_trg_prescaler : ExtTRGPrescaler,
                                    ext_trg_polarity : ExtTRGPolarity,
                                    ext_trg_filter : ExtTRGFilter);

    /// Configures the TIMx External Trigger (ETR).
    /// `ext_trg_prescaler`: The external Trigger Prescaler.
    ///     This parameter can be one of the following values:
    /// * TIM_ExtTRGPSC_OFF
    /// * TIM_ExtTRGPSC_DIV2
    /// * TIM_ExtTRGPSC_DIV4
    /// * TIM_ExtTRGPSC_DIV8
    /// `ext_trg_polarity`: The external Trigger Polarity.
    ///     This parameter can be one of the following values:
    /// * ext_trg_polarity_Inverted
    /// * ext_trg_polarity_NonInverted
    /// `ext_trg_filter`: External Trigger Filter.
    ///     This parameter must be a value between 0x00 and 0x0F.
    fn etr_config(&self, ext_trg_prescaler : ExtTRGPrescaler,
                        ext_trg_polarity : ExtTRGPolarity,
                        ext_trg_filter : ExtTRGFilter);

    /// Specifies the TIMx Counter Mode to be used.
    /// `counter_mode`: specifies the Counter Mode to be used
    ///     This parameter can be one of the following values:
    /// * counter_mode_Up: TIM Up Counting Mode
    /// * counter_mode_Down: TIM Down Counting Mode
    /// * counter_mode_CenterAligned1: TIM Center Aligned Mode1
    /// * counter_mode_CenterAligned2: TIM Center Aligned Mode2
    /// * counter_mode_CenterAligned3: TIM Center Aligned Mode3
    fn counter_mode_config(&self, counter_mode : CounterMode);

    /// Selects the Input Trigger source
    /// `input_trigger_source`: The Input Trigger source.
    ///     This parameter can be one of the following values:
    /// * TIM_TS_ITR0: Internal Trigger 0
    /// * TIM_TS_ITR1: Internal Trigger 1
    /// * TIM_TS_ITR2: Internal Trigger 2
    /// * TIM_TS_ITR3: Internal Trigger 3
    /// * TIM_TS_TI1F_ED: TI1 Edge Detector
    /// * TIM_TS_TI1FP1: Filtered Timer Input 1
    /// * TIM_TS_TI2FP2: Filtered Timer Input 2
    /// * TIM_TS_ETRF: External Trigger input
    fn select_input_trigger(&self, input_trigger_source : InputTriggerSource);

    /// Configures the TIMx Encoder Interface.
    /// `encoder_mode`: specifies the TIMx Encoder Mode.
    ///     This parameter can be one of the following values:
    /// * encoder_mode_TI1: Counter counts on TI1FP1 edge
    ///     depending on TI2FP2 level.
    /// * encoder_mode_TI2: Counter counts on TI2FP2 edge
    ///     depending on TI1FP1 level.
    /// * encoder_mode_TI12: Counter counts on both TI1FP1 and
    ///     TI2FP2 edges depending on the level of the other input.
    /// `ic1_polarity`: specifies the IC1 Polarity
    ///     This parmeter can be one of the following values:
    /// * ic_polarity_Falling: IC Falling edge.
    /// * ic_polarity_Rising: IC Rising edge.
    /// `ic2_polarity`: specifies the IC2 Polarity
    ///     This parmeter can be one of the following values:
    /// * ic_polarity_Falling: IC Falling edge.
    /// * ic_polarity_Rising: IC Rising edge.
    fn encoder_interface_config(&self, encoder_mode : EncoderMode,
                            ic1_polarity : ICPolarity,
                            ic2_polarity : ICPolarity);

    /// Forces the TIMx output 1 waveform to active or inactive level.
    /// `forced_action`: specifies the forced Action to be set to
    ///     the output waveform.
    ///     This parameter can be one of the following values:
    /// * forced_action_Active: Force active level on OC1REF
    /// * forced_action_InActive: Force inactive level on
    ///     OC1REF.
    fn forced_oc1_config(&self, forced_action : ForcedAction);

    /// Forces the TIMx output 2 waveform to active or inactive level.
    /// `forced_action`: specifies the forced Action to be set to
    ///     the output waveform.
    ///     This parameter can be one of the following values:
    /// * forced_action_Active: Force active level on OC2REF
    /// * forced_action_InActive: Force inactive level on
    ///     OC2REF.
    fn forced_oc2_config(&self, forced_action : ForcedAction);

    /// Forces the TIMx output 3 waveform to active or inactive level.
    /// `forced_action`: specifies the forced Action to be set to
    ///     the output waveform.
    ///     This parameter can be one of the following values:
    /// * forced_action_Active: Force active level on OC3REF
    /// * forced_action_InActive: Force inactive level on
    ///     OC3REF.
    fn forced_oc3_config(&self, forced_action : ForcedAction);

    /// Forces the TIMx output 4 waveform to active or inactive level.
    /// `forced_action`: specifies the forced Action to be set to
    ///     the output waveform.
    ///     This parameter can be one of the following values:
    /// * forced_action_Active: Force active level on OC4REF
    /// * forced_action_InActive: Force inactive level on
    ///     OC4REF.
    fn forced_oc4_config(&self, forced_action : ForcedAction);

    /// Enables or disables TIMx peripheral Preload register on ARR.
    /// `new_state : bool`: new state of the TIMx peripheral Preload register
    ///     This parameter can be: ENABLE or DISABLE.
    fn arr_preload_config(&self, new_state : bool);

    /// Selects the TIMx peripheral Capture Compare DMA source.
    /// `new_state : bool`: new state of the Capture Compare DMA source
    ///     This parameter can be: ENABLE or DISABLE.
    fn select_ccdma(&self, new_state : bool);

    /// Enables or disables the TIMx peripheral Preload register on CCR1.
    /// `oc_preload`: new state of the TIMx peripheral Preload
    ///     register
    ///     This parameter can be one of the following values:
    /// * oc_preload_Enable
    /// * oc_preload_Disable
    fn oc1_preload_config(&self, oc_preload : bool);

    /// Enables or disables the TIMx peripheral Preload register on CCR2.
    /// `oc_preload`: new state of the TIMx peripheral Preload
    ///     register
    ///     This parameter can be one of the following values:
    /// * oc_preload_Enable
    /// * oc_preload_Disable
    fn oc2_preload_config(&self, oc_preload : bool);

    /// Enables or disables the TIMx peripheral Preload register on CCR3.
    /// `oc_preload`: new state of the TIMx peripheral Preload
    ///     register
    ///     This parameter can be one of the following values:
    /// * oc_preload_Enable
    /// * oc_preload_Disable
    fn oc3_preload_config(&self, oc_preload : bool);

    /// Enables or disables the TIMx peripheral Preload register on CCR4.
    /// `oc_preload`: new state of the TIMx peripheral Preload
    ///     register
    ///     This parameter can be one of the following values:
    /// * oc_preload_Enable
    /// * oc_preload_Disable
    fn oc4_preload_config(&self, oc_preload : bool);

    /// Configures the TIMx Output Compare 1 Fast feature.
    /// `oc_fast`: new state of the Output Compare Fast Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_fast_Enable: TIM output compare fast enable
    /// * oc_fast_Disable: TIM output compare fast disable
    fn oc1_fast_config(&self, oc_fast : bool);

    /// Configures the TIMx Output Compare 2 Fast feature.
    /// `oc_fast`: new state of the Output Compare Fast Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_fast_Enable: TIM output compare fast enable
    /// * oc_fast_Disable: TIM output compare fast disable
    fn oc2_fast_config(&self, oc_fast : bool);

    /// Configures the TIMx Output Compare 3 Fast feature.
    /// `oc_fast`: new state of the Output Compare Fast Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_fast_Enable: TIM output compare fast enable
    /// * oc_fast_Disable: TIM output compare fast disable
    fn oc3_fast_config(&self, oc_fast : bool);

    /// Configures the TIMx Output Compare 4 Fast feature.
    /// `oc_fast`: new state of the Output Compare Fast Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_fast_Enable: TIM output compare fast enable
    /// * oc_fast_Disable: TIM output compare fast disable
    fn oc4_fast_config(&self, oc_fast : bool);

    /// Clears or safeguards the OCREF1 signal on an external event
    /// `oc_clear`: new state of the Output Compare Clear Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_clear_Enable: TIM Output clear enable
    /// * oc_clear_Disable: TIM Output clear disable
    fn clear_oc1_ref(&self, oc_clear : bool);

    /// Clears or safeguards the OCREF2 signal on an external event
    /// `oc_clear`: new state of the Output Compare Clear Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_clear_Enable: TIM Output clear enable
    /// * oc_clear_Disable: TIM Output clear disable
    fn clear_oc2_ref(&self, oc_clear : bool);

    /// Clears or safeguards the OCREF3 signal on an external event
    /// `oc_clear`: new state of the Output Compare Clear Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_clear_Enable: TIM Output clear enable
    /// * oc_clear_Disable: TIM Output clear disable
    fn clear_oc3_ref(&self, oc_clear : bool);

    /// Clears or safeguards the OCREF4 signal on an external event
    /// `oc_clear`: new state of the Output Compare Clear Enable Bit.
    ///     This parameter can be one of the following values:
    /// * oc_clear_Enable: TIM Output clear enable
    /// * oc_clear_Disable: TIM Output clear disable
    fn clear_oc4_ref(&self, oc_clear : bool);

    /// Configures the TIMx channel 1 polarity.
    /// `oc_polarity`: specifies the OC1 Polarity
    ///     This parmeter can be one of the following values:
    /// * oc_polarity_High: Output Compare active high
    /// * oc_polarity_Low: Output Compare active low
    fn oc1_polarity_config(&self, oc_polarity: OCPolarity);

    /// Configures the TIMx channel 2 polarity.
    /// `oc_polarity`: specifies the OC2 Polarity
    ///     This parmeter can be one of the following values:
    /// * oc_polarity_High: Output Compare active high
    /// * oc_polarity_Low: Output Compare active low
    fn oc2_polarity_config(&self, oc_polarity: OCPolarity);

    /// Configures the TIMx channel 3 polarity.
    /// `oc_polarity`: specifies the OC3 Polarity
    ///     This parmeter can be one of the following values:
    /// * oc_polarity_High: Output Compare active high
    /// * oc_polarity_Low: Output Compare active low
    fn oc3_polarity_config(&self, oc_polarity: OCPolarity);

    /// Configures the TIMx channel 4 polarity.
    /// `oc_polarity`: specifies the OC4 Polarity
    ///     This parmeter can be one of the following values:
    /// * oc_polarity_High: Output Compare active high
    /// * oc_polarity_Low: Output Compare active low
    fn oc4_polarity_config(&self, oc_polarity: OCPolarity);

    /// Enables or disables the TIM Capture Compare Channel x.
    /// `channel`: specifies the TIM Channel
    ///     This parmeter can be one of the following values:
    /// * channel_1: TIM Channel 1
    /// * channel_2: TIM Channel 2
    /// * channel_3: TIM Channel 3
    /// * channel_4: TIM Channel 4
    /// `ccx`: specifies the TIM Channel CCxE bit new state.
    ///     This parameter can be: ccx_Enable or ccx_Disable. 
    fn ccx_cmd(&self, channel : Channel, ccx : bool);

    /// Selects the TIM Ouput Compare Mode.
    ///     This function disables the selected channel before changing 
    ///     the Ouput Compare Mode. User has to enable this channel using
    ///     ccxCmd and ccxnCmd functions.
    /// `channel`: specifies the TIM Channel
    ///     This parmeter can be one of the following values:
    /// * channel_1: TIM Channel 1
    /// * channel_2: TIM Channel 2
    /// * channel_3: TIM Channel 3
    /// * channel_4: TIM Channel 4
    /// `oc_mode`: specifies the TIM Output Compare Mode.
    ///     This paramter can be one of the following values:
    /// * oc_mode_Timing
    /// * oc_mode_Active
    /// * oc_mode_Toggle
    /// * oc_mode_PWM1
    /// * oc_mode_PWM2
    /// * forced_action_Active
    /// * forced_action_InActive
    fn select_ocxm(&self, channel : Channel, oc_mode : OCMode);

    /// Enables or disables the TIMx’s Hall sensor interface.
    /// `new_state : bool`: new state of the TIMx Hall sensor interface.
    ///     This parameter can be: ENABLE or DISABLE.
    fn select_hall_sensor(&self, new_state : bool);

    /// Selects the TIMx Slave Mode.
    /// `slave_mode`: specifies the Timer Slave Mode.
    ///     This paramter can be one of the following values:
    /// * slave_mode_Reset
    /// * slave_mode_Gated
    /// * slave_mode_Trigger
    /// * slave_mode_External1
    fn select_slave_mode(&self, slave_mode : SlaveMode);

    /// Sets or Resets the TIMx Master/Slave Mode.
    /// `master_slave_mode`: specifies the Timer Master Slave Mode.
    ///     This paramter can be one of the following values:
    /// * master_slave_mode_Enable: synchronization between the
    ///     current timer and its slaves (through TRGO).
    /// * master_slave_mode_Disable: No action
    fn select_master_slave_mode(&self, master_slave_mode : bool );

    /// Sets the TIMx Capture Compare1 Register value
    /// `compare1`: specifies the Capture Compare1 register new value.
    fn set_compare1(&self, compare1 : u16);
    /// Sets the TIMx Capture Compare2 Register value
    /// `compare2`: specifies the Capture Compare2 register new value.
    fn set_compare2(&self, compare2 : u16);

    /// Sets the TIMx Capture Compare3 Register value
    /// `compare3`: specifies the Capture Compare3 register new value.
    fn set_compare3(&self, compare3 : u16);

    /// Sets the TIMx Capture Compare4 Register value
    /// `compare4`: specifies the Capture Compare4 register new value.
    fn set_compare4(&self, compare4 : u16);

    /// Sets the TIMx Input Capture 1 prescaler.
    /// `ic_psc`: specifies the Input Capture1 prescaler
    ///     new value.
    ///     This parameter can be one of the following values:
    /// * ic_psc_DIV1: no prescaler
    /// * ic_psc_DIV2: capture is done once every 2 events
    /// * ic_psc_DIV4: capture is done once every 4 events
    /// * ic_psc_DIV8: capture is done once every 8 events
    fn set_ic1_prescaler(&self, ic_psc : ICPrescaler);

    /// Sets the TIMx Input Capture 2 prescaler.
    /// `ic_psc`: specifies the Input Capture2 prescaler
    ///     new value.
    ///     This parameter can be one of the following values:
    /// * ic_psc_DIV1: no prescaler
    /// * ic_psc_DIV2: capture is done once every 2 events
    /// * ic_psc_DIV4: capture is done once every 4 events
    /// * ic_psc_DIV8: capture is done once every 8 events
    fn set_ic2_prescaler(&self, ic_psc : ICPrescaler);

    /// Sets the TIMx Input Capture 3 prescaler.
    /// `ic_psc`: specifies the Input Capture3 prescaler
    ///     new value.
    ///     This parameter can be one of the following values:
    /// * ic_psc_DIV1: no prescaler
    /// * ic_psc_DIV2: capture is done once every 2 events
    /// * ic_psc_DIV4: capture is done once every 4 events
    /// * ic_psc_DIV8: capture is done once every 8 events
    fn set_ic3_prescaler(&self, ic_psc : ICPrescaler);

    /// Sets the TIMx Input Capture 4 prescaler.
    /// `ic_psc`: specifies the Input Capture4 prescaler
    ///     new value.
    ///     This parameter can be one of the following values:
    /// * ic_psc_DIV1: no prescaler
    /// * ic_psc_DIV2: capture is done once every 2 events
    /// * ic_psc_DIV4: capture is done once every 4 events
    /// * ic_psc_DIV8: capture is done once every 8 events
    fn set_ic4_prescaler(&self, ic_psc : ICPrescaler);

    /// Sets the TIMx Clock Division value.
    /// `tim_ckd`: specifies the clock division value.
    ///     This parameter can be one of the following value:
    /// * tim_ckd_DIV1: TDTS = Tck_tim
    /// * tim_ckd_DIV2: TDTS = 2*Tck_tim
    /// * tim_ckd_DIV4: TDTS = 4*Tck_tim
    fn set_clock_division(&self, ckd : ClockDivision);

    /// Gets the TIMx Input Capture 1 value.
    /// @retval : Capture Compare 1 Register value.
    fn get_capture1(&self) -> u16;

    /// Gets the TIMx Input Capture 2 value.
    /// @retval : Capture Compare 2 Register value.
    fn get_capture2(&self) -> u16;

    /// Gets the TIMx Input Capture 3 value.
    /// @retval : Capture Compare 3 Register value.
    fn get_capture3(&self) -> u16;

    /// Gets the TIMx Input Capture 4 value.
    /// @retval : Capture Compare 4 Register value.
    fn get_capture4(&self) -> u16;


    /// Configure the TI1 as Input.
    /// `ic_polarity`: The Input Polarity.
    ///     This parameter can be one of the following values:
    /// * ic_polarity_Rising
    /// * ic_polarity_Falling
    /// `ic_selection`: specifies the input to be used.
    ///     This parameter can be one of the following values:
    /// * ic_selection_DirectTI: TIM Input 1 is selected to
    ///     be connected to IC1.
    /// * ic_selection_IndirectTI: TIM Input 1 is selected to
    ///     be connected to IC2.
    /// * ic_selection_TRC: TIM Input 1 is selected to be
    ///     connected to TRC.
    /// `ic_filter`: Specifies the Input Capture Filter.
    ///     This parameter must be a value between 0x00 and 0x0F.
    fn ti1_config(&self, ic_polarity : ICPolarity,
                        ic_selection : ICSelection,
                        ic_filter : ICFilter);

    /// Configure the TI2 as Input.
    /// `ic_polarity`: The Input Polarity.
    ///     This parameter can be one of the following values:
    /// * ic_polarity_Rising
    /// * ic_polarity_Falling
    /// `ic_selection`: specifies the input to be used.
    ///     This parameter can be one of the following values:
    /// * ic_selection_DirectTI: TIM Input 2 is selected to
    ///     be connected to IC2.
    /// * ic_selection_IndirectTI: TIM Input 2 is selected to
    ///     be connected to IC1.
    /// * ic_selection_TRC: TIM Input 2 is selected to be
    ///     connected to TRC.
    /// `ic_filter`: Specifies the Input Capture Filter.
    ///     This parameter must be a value between 0x00 and 0x0F.
    fn ti2_config(&self, ic_polarity : ICPolarity,
                        ic_selection : ICSelection,
                        ic_filter : ICFilter);

    /// Configure the TI3 as Input.
    /// `ic_polarity`: The Input Polarity.
    ///     This parameter can be one of the following values:
    /// * ic_polarity_Rising
    /// * ic_polarity_Falling
    /// `ic_selection`: specifies the input to be used.
    ///     This parameter can be one of the following values:
    /// * ic_selection_DirectTI: TIM Input 3 is selected to
    ///     be connected to IC3.
    /// * ic_selection_IndirectTI: TIM Input 3 is selected to
    ///     be connected to IC4.
    /// * ic_selection_TRC: TIM Input 3 is selected to be
    ///     connected to TRC.
    /// `ic_filter`: Specifies the Input Capture Filter.
    ///     This parameter must be a value between 0x00 and 0x0F.
    fn ti3_config(&self, ic_polarity : ICPolarity,
                        ic_selection : ICSelection,
                        ic_filter : ICFilter);

    /// Configure the TI4 as Input.
    /// `ic_polarity`: The Input Polarity.
    ///     This parameter can be one of the following values:
    /// * ic_polarity_Rising
    /// * ic_polarity_Falling
    /// `ic_selection`: specifies the input to be used.
    ///     This parameter can be one of the following values:
    /// * ic_selection_DirectTI: TIM Input 4 is selected to
    ///     be connected to IC4.
    /// * ic_selection_IndirectTI: TIM Input 4 is selected to
    ///     be connected to IC3.
    /// * ic_selection_TRC: TIM Input 4 is selected to be
    ///     connected to TRC.
    /// `ic_filter`: Specifies the Input Capture Filter.
    ///     This parameter must be a value between 0x00 and 0x0F.
    fn ti4_config(&self, ic_polarity : ICPolarity,
                        ic_selection : ICSelection,
                        ic_filter : ICFilter);
}


macro_rules! impl_timbase {
    ($TIMX:ty) => (

    impl TimBaseSplExt for Timer<$TIMX> {

        fn cmd(&self, new_state : bool) {
            self.tim .cr1     .modify(|_,w| w
                .cen()  .bit( new_state )
            );
        }

        fn generate_event(&self, event_source : u16) { 
            // Set the event sources
            self.tim .egr      .write(|w| unsafe { w
                .bits( event_source as u32 )
            });
        }

        fn dma_cmd(&self, dma_source : u16, new_state : bool) {
            //const MASK: u8 = 0x7f;
            //const OFFSET: u8 = 8;
            self.tim .dier      .modify(|r,w| unsafe { w.bits(
                if new_state {
                    // Enable the DMA sources
                    r.bits() | (dma_source as u32)
                } else {
                    // Disable the DMA sources
                    r.bits() & !(dma_source as u32)
                }
            )});
        }

        fn prescaler_config(&self, prescaler : u16, psc_reload_mode : PSCReloadMode) {
            // Set the Prescaler value
            self.tim .psc      .write(|w| unsafe { w
                .psc()    .bits( prescaler )
            });
            // Set or reset the UG Bit
            self.tim .egr      .write(|w| unsafe { w
                .ug()    .variant( psc_reload_mode )
            });
        }

        fn update_disable_config(&self, new_state : bool) {
            self.tim .cr1     .modify(|_,w| w
                .udis()  .bit( new_state )
            );
        }

        fn update_request_config(&self, update_source : UpdateSource) {
            self.tim .cr1     .modify(|_,w| w
                .udis()  .variant( update_source )
            );
        }

        fn select_one_pulse_mode(&self, op_mode : OPMode) {
            self.tim .cr1     .modify(|_,w| w
                .opm()   .variant(op_mode)
            );
        }

        fn set_counter(&self, counter : u16) {
            self.tim .cnt     .write(|w| unsafe { w
                .cnt()    .bits( counter )
            });
        }

        fn set_autoreload(&self, autoreload : u16) {
            self.tim .arr     .write(|w| unsafe { w
                .arr()    .bits( autoreload )
            });
        }

        fn get_counter(&self) -> u16 {
            self.tim .cnt.read().cnt().bits()
        }

        fn get_prescaler(&self) -> u16 {
            self.tim .psc.read().psc().bits()
        }
    }
)}



macro_rules! impl_timbaseonly{
    ($TIMX:ty) => (

    impl  TimBaseOnlySplExt for Timer<$TIMX> {
        
        fn select_output_trigger(&self, trgo_source : TRGOSource6) {
            self.tim .cr2     .modify(|_,w| w
                .mms()   .variant( trgo_source )
            );
        }
    }
)}


macro_rules! impl_timgeneral {
    ($TIMX:ty) => (

    impl TimGeneralSplExt for Timer<$TIMX> {
        fn select_output_trigger(&self, trgo_source : TRGOSource) {
            self.tim .cr2     .modify(|_,w| w
                .mms()   .variant( trgo_source )
            );
        }
        
        fn ic_init(&self, ic_initstruct : &TimICStruct) {
            match ic_initstruct.Channel {
                Channel::C1 => {
                    // TI1 Configuration
                    self.ti1_config(ic_initstruct.ic_polarity,
                                    ic_initstruct.ic_selection,
                                    ic_initstruct.ic_filter);
                    // Set the Input Capture Prescaler value
                    self.set_ic1_prescaler(ic_initstruct.ic_prescaler);
                },
                Channel::C2 => {
                    // TI2 Configuration
                    self.ti2_config(ic_initstruct.ic_polarity,
                                    ic_initstruct.ic_selection,
                                    ic_initstruct.ic_filter);
                    // Set the Input Capture Prescaler value
                    self.set_ic2_prescaler(ic_initstruct.ic_prescaler);
                }
                Channel::C3 => {
                    // TI3 Configuration
                    self.ti3_config(ic_initstruct.ic_polarity,
                                    ic_initstruct.ic_selection,
                                    ic_initstruct.ic_filter);
                    // set the input capture prescaler value
                    self.set_ic3_prescaler(ic_initstruct.ic_prescaler);
                },
                Channel::C4 => {
                    // TI4 Configuration
                    self.ti4_config(ic_initstruct.ic_polarity,
                                    ic_initstruct.ic_selection,
                                    ic_initstruct.ic_filter);
                    // set the input capture prescaler value
                    self.set_ic4_prescaler(ic_initstruct.ic_prescaler);
                }
            }
        }
        
        fn pwmi_config(&self, ic_initstruct : &TimICStruct) {
            // Select the Opposite Input Polarity
            let ic_opposite_polarity = match ic_initstruct.ic_polarity {
                ICPolarity::Rising  => ICPolarity::Falling,
                ICPolarity::Falling => ICPolarity::Rising
            };
            // Select the Opposite Input
            let ic_opposite_selection = match ic_initstruct.ic_selection {
                ICSelection::DirectTI   => ICSelection::IndirectTI,
                _                       => ICSelection::DirectTI
            };
            match TIM_ICInitStruct.Channel {
                Channel::C1 => {
                    // TI1 Configuration
                    self.ti1_config(ic_initstruct.ic_polarity,
                                    ic_initstruct.ic_selection,
                                    ic_initstruct.ic_filter);
                    // set the input capture prescaler value
                    self.set_ic1_prescaler(ic_initstruct.ic_prescaler);
                    // ti2 configuration
                    self.ti2_config(ic_opposite_polarity,
                                    ic_opposite_selection,
                                    ic_initstruct.ic_filter);
                    // set the input capture prescaler value
                    self.set_ic2_prescaler(ic_initstruct.ic_prescaler);
                },
                Channel::C2 => {
                    // TI2 Configuration
                    self.ti2_config(ic_initstruct.ic_polarity,
                                    ic_initstruct.ic_selection,
                                    ic_initstruct.ic_filter);
                    // set the input capture prescaler value
                    self.set_ic2_prescaler(ic_initstruct.ic_prescaler);
                    // ti1 configuration
                    self.ti1_config(icoppositepolarity,
                                    icoppositeselection,
                                    ic_initstruct.ic_filter);
                    // set the input capture prescaler value
                    self.set_ic1_prescaler(ic_initstruct.ic_prescaler);
                },
                _ => unreachable!()
            }
        }
        
        fn dma_config(&self, dma_base : DMABase, dma_burst_length : DMABurstLength) {
            // Set the DMA Base and the DMA Burst Length
            self.tim .dcr       .write(|w| w
                .dba()     .variant( dma_base )
                .dbl()     .variant( tim_dmaburstLength )
            );
        }
        
        fn internal_clock_config(&self) {
            // Disable slave mode to clock the prescaler directly with the internal clock
            self.tim .smcr      .modify(|_,w| w
                .sms()     .disabled()
            );
        }

        fn itrx_external_clock_config(&self, input_trigger_source : InputTriggerSource) {
            // Select the Internal Trigger
            self.SelectInputTrigger(input_trigger_source);
            // Select the External clock mode1
            self.tim .smcr      .modify(|_,w| unsafe { w
                .sms()     .bits( SlaveMode::External1 as u8 )
            });
        }

        fn tix_external_clock_config(&self, tix_external_clk_source : TIxExternalCLK1Source,
                            ic_polarity : ICPolarity, ICFilter : ICFilter) {
            // Configure the Timer Input Clock Source
            if tix_external_clk_source == TIxExternalCLK1Source::TI2 {
                self.TI2_Config(ic_polarity, ICSelection::DirectTI, ICFilter);
            } else {
                self.TI1_Config(ic_polarity, ICSelection::DirectTI, ICFilter);
            }
            // Select the Trigger source
            self.SelectInputTrigger(tix_external_clk_source);
            // Select the External clock mode1
            self.tim .smcr      .modify(|_,w| unsafe { w
                .sms()     .bits( SlaveMode::External1 as u8 )
            });
        }
        
        fn etr_clock_mode1_config(&self, ext_trg_prescaler : ExtTRGPrescaler,
                                    ext_trg_polarity : ExtTRGPolarity,
                                    ext_trg_filter : ExtTRGFilter) {
            // Configure the ETR Clock source
            self.ETRConfig(ext_trg_prescaler, ext_trg_polarity, ext_trg_filter);
            
            self.tim .smcr      .modify(|_,w| unsafe { w
                // Select the External clock mode1
                .sms()     .bits( SlaveMode::External1 as u8 )
                // Select the Trigger selection : ETRF
                .ts()      .etrf()
            });
        }

        fn etr_clock_mode2_config(&self, ext_trg_prescaler : ExtTRGPrescaler,
                                    ext_trg_polarity : ExtTRGPolarity,
                                    ext_trg_filter : ExtTRGFilter) {
            // Configure the ETR Clock source
            self.ETRConfig(ext_trg_prescaler, ext_trg_polarity, ext_trg_filter);
            // Enable the External clock mode2
            self.tim .smcr       .modify(|_,w| w
                .ece()      .set_bit()
            );
        }

        fn etr_config(&self, ext_trg_prescaler : ExtTRGPrescaler,
                            ext_trg_polarity : ExtTRGPolarity,
                            ext_trg_filter : ExtTRGFilter) {
            self.tim .smcr       .modify(|_,w| w
                // Reset the ETR Bits
                .ece()    .clear_bit()
                // Set the Prescaler, the Filter value and the Polarity
                .etps()   .variant( ext_trg_prescaler )
                .etp()    .variant( ext_trg_polarity )
                .etp()    .variant( ext_trg_filter )
            );
        }

        fn counter_mode_config(&self, counter_mode : CounterMode) {
            let dir, cms = counter_mode.to_dir_cms();
            self.tim .cr1     .modify(|_,w| w
                .dir()     .variant( dir )
                .cms()     .variant( cms )
            );
        }

        fn select_input_trigger(&self, input_trigger_source : InputTriggerSource) {
            self.tim .smcr     .modify(|_,w| w
                .ts()     .variant( InputTriggerSource )
            );
        }

        fn encoder_interface_config(&self, encoder_mode : EncoderMode,
                                ic1_polarity : ICPolarity,
                                ic2_polarity : ICPolarity) {
            // Set the encoder Mode
            self.tim .smcr    .modify(|_,w| w
                .sms()   .variant( encoder_mode )
            );
            // Select the Capture Compare 1 and the Capture Compare 2 as input
            (unsafe { self.tim .ccmr1.input })  .modify(|_,w| w
                .cc1s()   .input_direct_ti()
                .cc2s()   .input_direct_ti()
            );
            // Set the TI1 and the TI2 Polarities
            self.tim .ccer    .modify(|_,w| w
                .cc1p()   .variant( ic1_polarity )
                .cc2p()   .variant( ic2_polarity )
            );
        }

        fn forced_oc1_config(&self, forced_action : ForcedAction) {
            (unsafe { self.tim .ccmr1.output }) .modify(|_,w| w
                .cc1s()       .bits( 0 )
                .oc1m()      .bit( forced_action as u8 )
            );
        }

        fn forced_oc2_config(&self, forced_action : ForcedAction) {
            (unsafe { self.tim .ccmr1.output }) .modify(|_,w| w
                .cc2s()       .bits( 0 )
                .oc2m()      .bit( forced_action as u8 )
            );
        }

        fn forced_oc3_config(&self, forced_action : ForcedAction) {
            (unsafe { self.tim .ccmr2.output }) .modify(|_,w| w
                .cc3s()       .bits( 0 )
                .oc3m()      .bit( forced_action as u8 )
            );
        }

        fn forced_oc4_config(&self, forced_action : ForcedAction) {
            (unsafe { self.tim .ccmr1.output }) .modify(|_,w| w
                .cc4s()       .bits( 0 )
                .oc4m()      .bit( forced_action as u8 )
            );
        }

        fn arr_preload_config(&self, new_state : bool) {
            self.tim .cr1     .modify(|_,w| w
                .arpe()  .bit( oc_polarity.into() )
            );
        }

        fn select_ccdma(&self, new_state : bool) {
            self.tim .cr2     .modify(|_,w| w
                .ccds()  .bit( oc_polarity.into() )
            );
        }

        fn oc1_preload_config(&self, oc_preload : bool) {
            (unsafe { self.tim .ccmr1.output })    .modify(|_,w| w
                .oc1pe()    .bit( oc_preload )
            );
        }

        fn oc2_preload_config(&self, oc_preload : bool) {
            (unsafe { self.tim .ccmr1.output })    .modify(|_,w| w
                .oc2pe()    .bit( oc_preload )
            );
        }

        fn oc3_preload_config(&self, oc_preload : bool) {
            (unsafe { self.tim .ccmr2.output })   .modify(|_,w| w
                .oc3pe()    .bit( oc_preload )
            );
        }

        fn oc4_preload_config(&self, oc_preload : bool) {
            (unsafe { self.tim .ccmr2.output })   .modify(|_,w| w
                .oc4pe()    .bit( oc_preload )
            );
        }

        fn oc1_fast_config(&self, oc_fast : bool) {
            (unsafe { self.tim .ccmr1.output })   .modify(|_,w| w
                .oc1fe()    .bit( oc_fast )
            );
        }

        fn oc2_fast_config(&self, oc_fast : bool) {
            (unsafe { self.tim .ccmr1.output })   .modify(|_,w| w
                .oc2fe()    .bit( oc_fast )
            );
        }

        fn oc3_fast_config(&self, oc_fast : bool) {
            (unsafe { self.tim .ccmr2.output })   .modify(|_,w| w
                .oc3fe()    .bit( oc_fast )
            );
        }

        fn oc4_fast_config(&self, oc_fast : bool) {
            (unsafe { self.tim .ccmr2.output })   .modify(|_,w| w
                .oc4fe()    .bit( oc_fast )
            );
        }

        fn clear_oc1_ref(&self, oc_clear : bool) {
            (unsafe { self.tim .ccmr1.output }) .modify(|_,w| w
                .oc1ce()  .bit( oc_clear )
            );
        }

        fn clear_oc2_ref(&self, oc_clear : bool) {
            (unsafe { self.tim .ccmr1.output }) .modify(|_,w| w
                .oc2ce()  .bit( oc_clear )
            );
        }

        fn clear_oc3_ref(&self, oc_clear : bool) {
            (unsafe { self.tim .ccmr2.output })  .modify(|_,w| w
                .oc3ce()  .bit( oc_clear )
            );
        }

        fn clear_oc4_ref(&self, oc_clear : bool) {
            (unsafe { self.tim .ccmr4.output }) .modify(|_,w| w
                .oc4ce()  .bit( oc_clear )
            );
        }

        fn oc1_polarity_config(&self, oc_polarity: OCPolarity) {
            self.tim .ccer     .modify(|_,w| w
                .cc1p()  .bit( oc_polarity.into() )
            );
        }

        fn oc2_polarity_config(&self, oc_polarity: OCPolarity) {
            self.tim .ccer     .modify(|_,w| w
                .cc2p()  .bit( oc_polarity.into() )
            );
        }

        fn oc3_polarity_config(&self, oc_polarity: OCPolarity) {
            self.tim .ccer     .modify(|_,w| w
                .cc3p()  .bit( oc_polarity.into() )
            );
        }

        fn oc4_polarity_config(&self, oc_polarity: OCPolarity) {
            self.tim .ccer     .modify(|_,w| w
                .cc4p()  .bit( oc_polarity.into() )
            );
        }

        fn ccx_cmd(&self, channel : Channel, ccx : bool) {
            self.tim .ccer       .modify(|_,w|
                match channel {
                    Channel::C1 => w.cc1e().bit( ccx ),
                    Channel::C2 => w.cc2e().bit( ccx ),
                    Channel::C3 => w.cc3e().bit( ccx ),
                    Channel::C4 => w.cc4e().bit( ccx )
                }
            );
        }

        fn select_ocxm(&self, channel : Channel, oc_mode : OCMode) {
            // Disable the Channel: Reset the CCxE Bit
            self.CCxCmd( channel, TimCCx::Disable );
            // Configure the OCxM bits in the CCMRx register
            match channel {
                Channel::C1 => {
                    (unsafe { self.tim .ccmr1.output }) .modify(|_,w| w
                        .oc1m()   .variant( oc_mode )
                    );
                },
                Channel::C2 => {
                    (unsafe { self.tim .ccmr1.output }) .modify(|_,w| w
                        .oc2m()   .variant( oc_mode )
                    );
                },
                Channel::C3 => {
                    (unsafe { self.tim .ccmr2.output }) .modify(|_,w| w
                        .oc3m()   .variant( oc_mode )
                    );
                },
                Channel::C4 => {
                    (unsafe { self.tim .ccmr2.output }) .modify(|_,w| w
                        .oc4m()   .variant( oc_mode )
                    );
                }
            }
        }

        fn select_hall_sensor(&self, new_state : bool) {
            self.tim .cr2      .modify(|_,w| w
                .ti1s()   .bit( new_state )
            );
        }

        fn select_slave_mode(&self, slave_mode : SlaveMode) {
            self.tim .smcr     .modify(|_,w| unsafe { w
                .sms()    .bits( slave_mode as u8 )
            });
        }

        fn select_master_slave_mode(&self, master_slave_mode : bool ) {
            self.tim .smcr     .modify(|_,w| w
                .sms()    .bit( master_slave_mode )
            );
        }

        fn set_compare1(&self, compare1 : u16) {
            self.tim .ccr1         .write(|w| unsafe { w
                .ccr1()   .bits( compare1 )
            });
        }

        fn set_compare2(&self, compare2 : u16) {
            self.tim .ccr2         .write(|w| unsafe { w
                .ccr2()   .bits( compare2 )
            });
        }

        fn set_compare3(&self, compare3 : u16) {
            self.tim .ccr3         .write(|w| unsafe { w
                .ccr3()   .bits( Compare3 )
            });
        }

        fn set_compare4(&self, compare4 : u16) {
            self.tim .ccr4         .write(|w| unsafe { w
                .ccr4()   .bits( compare4 )
            });
        }

        fn set_ic1_prescaler(&self, ic_psc : ICPrescaler) {
            (unsafe { self.tim .ccmr1.input })  .modify(|_,w| w
                .ic1psc()   .variant( ic_psc )
            );
        }

        fn set_ic2_prescaler(&self, ic_psc : ICPrescaler) {
            (unsafe { self.tim .ccmr1.input }) .modify(|_,w| w
                .ic2psc()   .variant( ic_psc )
            );
        }

        fn set_ic3_prescaler(&self, ic_psc : ICPrescaler) {
            (unsafe { self.tim .ccmr2.input }) .modify(|_,w| w
                .ic3psc()   .variant( ic_psc )
            );
        }

        fn set_ic4_prescaler(&self, ic_psc : ICPrescaler) {
            (unsafe { self.tim .ccmr2.input }) .modify(|_,w| w
                .ic4psc()   .variant( ic_psc )
            );
        }

        fn set_clock_division(&self, ckd : ClockDivision) {
            self.tim .cr1     .modify(|_,w| w
                .ckd()   .variant( ckd )
            );
        }

        fn get_capture1(&self) -> u16 {
            self.tim .ccr1.read().ccr1().bits()
        }

        fn get_capture2(&self) -> u16 {
            self.tim .ccr2.read().ccr2().bits()
        }

        fn get_capture3(&self) -> u16 {
            self.tim .ccr3.read().ccr3().bits()
        }

        fn get_capture4(&self) -> u16 {
            self.tim .ccr4.read().ccr4().bits()
        }

        fn ti1_config(&self, ic_polarity : ICPolarity,
                            ic_selection : ICSelection,
                            ic_filter : ICFilter) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc1e()    .clear_bit()
            );
            // Select the Input and set the filter
            (unsafe { self.tim .ccmr1.input })  .modify(|_,w| w
                .cc1s()   .bits( ic_selection as u8 )
                .ic1f()   .variant( ic_filter )
            );
            // Select the Polarity and set the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc1p()    .bit( ic_polarity.into() )
                .cc1e()    .set_bit()
            );
        }

        fn ti2_config(&self, ic_polarity : ICPolarity,
                            ic_selection : ICSelection,
                            ic_filter : ICFilter) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc2e()    .clear_bit()
            );
            // Select the Input and set the filter
            (unsafe { self.tim .ccmr1.input })  .modify(|_,w| w
                .cc2s()   .bits( ic_selection as u8 )
                .ic2f()   .variant( ic_filter )
            );
            // Select the Polarity and set the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc2p()    .bit( ic_polarity.into() )
                .cc2e()    .set_bit()
            );
        }

        fn ti3_config(&self, ic_polarity : ICPolarity,
                            ic_selection : ICSelection,
                            ic_filter : ICFilter) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc3e()    .clear_bit()
            );
            // Select the Input and set the filter
            (unsafe { self.tim .ccmr2.input })  .modify(|_,w| w
                .cc3s()   .bits( ic_selection as u8 )
                .ic3f()   .variant( ic_filter )
            );
            // Select the Polarity and set the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc3p()    .bit( ic_polarity.into() )
                .cc3e()    .set_bit()
            );
        }

        fn ti4_config(&self, ic_polarity : ICPolarity,
                            ic_selection : ICSelection,
                            ic_filter : ICFilter) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc4e()    .clear_bit()
            );
            // Select the Input and set the filter
            (unsafe { self.tim .ccmr2.input })  .modify(|_,w| w
                .cc4s()   .bits( ic_selection as u8 )
                .ic4f()   .variant( ic_filter )
            );
            // Select the Polarity and set the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc4p()    .bit( ic_polarity.into() )
                .cc4e()    .set_bit()
            );
        }
    }

)}


macro_rules! impl_timadvanced {
    ($TIMX:ty) => (

    impl TimAdvancedSplExt for Timer<$TIMX> {

        fn bdtr_config(&self, bdtr_initstruct : &TimBDTRStruct) {
            /* Set the Lock level, the Break enable Bit and the Ploarity, the OSSR State,
                 the OSSI State, the dead time value and the Automatic Output Enable Bit */
            self.tim .bdtr    .write(|w| unsafe { w
                .ossr()  .bit( bdtr_initstruct.ossr_state )
                .ossi()  .bit( bdtr_initstruct.ossi_state )
                .lock()  .variant( bdtr_initstruct.lock_level )
                .dtg()   .bits( bdtr_initstruct.dead_timeime )
                .bke()   .bit( bdtr_initstruct.Break )
                .bkp()   .variant( bdtr_initstruct.break_polarity )
                .aoe()   .bit( bdtr_initstruct.automatic_output )
            });
        }

        fn ctrl_pwm_outputs(&self, new_state : bool) {
            self.tim .bdtr     .modify(|_,w| w
                .moe()    .bit( new_state )
            );
        }

        fn select_com(&self, new_state : bool) {
            self.tim .cr2      .modify(|_,w| w
                .ccus()   .bit( new_state )
            );
        }

        fn cc_preload_control(&self, new_state : bool) {
            self.tim .cr2      .modify(|_,w| w
                .ccpc()   .bit( new_state )
            );
        }

        fn oc1n_polarity_config(&self, ocn_polarity : OCPolarity) {
            self.tim .ccer     .modify(|_,w| w
                .cc1np()  .bit( ocn_polarity.into() )
            );
        }

        fn oc2n_polarity_config(&self, ocn_polarity : OCPolarity) {
            self.tim .ccer     .modify(|_,w| w
                .cc2np()  .bit( ocn_polarity.into() )
            );
        }

        fn oc3n_polarity_config(&self, ocn_polarity : OCPolarity) {
            self.tim .ccer     .modify(|_,w| w
                .cc3np()  .bit( ocn_polarity.into() )
            );
        }
 
        fn ccxn_cmd(&self, channel : Channel, ccxn : bool) {
            self.tim .ccer       .modify(|_,w|
                match channel {
                    Channel::C1 => w.cc1ne().bit( ccxn ),
                    Channel::C2 => w.cc2ne().bit( ccxn ),
                    Channel::C3 => w.cc3ne().bit( ccxn ),
                    Channel::C4 => w.cc4ne().bit( ccxn )
                }
            );
        }

    }

)}






/// `TIMx`: where x can be    1 or 8 to select the TIM peripheral.
macro_rules! impl_timinitadvanced {
    ($TIMX:ty) => (

    impl TimInitSplExt for Timer<$TIMX> {
        
        fn time_base_init(&self, time_base_initstruct : &TimTimeBaseStruct) {
            // Select the Counter Mode and set the clock division
            let (dir, cms) = time_base_initstruct.counter_mode.to_dir_cms();
            self.tim .cr1     .modify(|_,w| w
                .ckd()   .variant( time_base_initstruct.clock_division )
                .dir()     .variant( dir )
                .cms()     .variant( cms )
            );
            // Set the Autoreload value
            self.tim .arr      .write(|w| unsafe { w
                .arr()    .bits( time_base_initstruct.period )
            });
            // Set the Prescaler value
            self.tim .psc      .write(|w| unsafe { w
                .psc()    .bits( time_base_initstruct.prescaler )
            });
            // Set the Repetition Counter value
            self.tim .rcr      .write(|w| unsafe { w
                .rcr()    .bits( time_base_initstruct.repetition_counter )
            });
            // Generate an update event to reload the Prescaler value immediatly
            self.tim .egr       .write(|w| w
                .ug()      .immediate()
            );
        }

        fn oc1_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc1e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr1  .write(|w| unsafe { w
                .ccr1()   .bits( oc_initstruct.pulse )
            });
            self.tim .cr2      .modify(|_,w| w
                // Set the Output Idle state
                .ois1()  .bit( oc_initstruct.oc_idle_state )
                // Set the Output N Idle state
                .ois1n()  .bit( oc_initstruct.ocn_idle_state )
            );
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr1.output })  .modify(|_,w| w
                .oc1m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc1p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc1e()  .bit( oc_initstruct.output_state )
                // Set the Output N Polarity
                .cc1np() .bit(oc_initstruct.ocn_polarity.into() )
                // Set the Output N State
                .cc1ne()  .bit( oc_initstruct.output_n_state )
            );
        }

        fn oc2_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc2e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr2  .write(|w| unsafe { w
                .ccr2()   .bits( oc_initstruct.pulse )
            });
            self.tim .cr2      .modify(|_,w| w
                // Set the Output Idle state
                .ois2()  .bit( oc_initstruct.oc_idle_state )
                // Set the Output N Idle state
                .ois2n()  .bit( oc_initstruct.ocn_idle_state )
            );
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr1.output })  .modify(|_,w| w
                .oc2m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc2p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc2e()  .bit( oc_initstruct.output_state )
                // Set the Output N Polarity
                .cc2np() .bit(oc_initstruct.ocn_polarity.into() )
                // Set the Output N State
                .cc2ne()  .bit( oc_initstruct.output_n_state )
            );
        }

        fn oc3_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc3e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr3  .write(|w| unsafe { w
                .ccr3()   .bits( oc_initstruct.pulse )
            });
            self.tim .cr2      .modify(|_,w| w
                // Set the Output Idle state
                .ois2()  .bit( oc_initstruct.oc_idle_state )
                // Set the Output N Idle state
                .ois2n()  .bit( oc_initstruct.ocn_idle_state )
            );
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr2.output })  .modify(|_,w| w
                .oc2m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc3p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc3e()  .bit( oc_initstruct.output_state )
                // Set the Output N Polarity
                .cc3np() .bit(oc_initstruct.ocn_polarity.into() )
                // Set the Output N State
                .cc3ne()  .bit( oc_initstruct.output_n_state )
            );
        }

        fn oc4_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc4e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr4  .write(|w| unsafe { w
                .ccr4()   .bits( oc_initstruct.pulse )
            });
            self.tim .cr2      .modify(|_,w| w
                // Set the Output Idle state
                .ois4()  .bit( oc_initstruct.oc_idle_state )
                // Set the Output N Idle state
                .ois4n()  .bit( oc_initstruct.ocn_idle_state )
            );
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr2.output })  .modify(|_,w| w
                .oc4m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc4p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc4e()  .bit( oc_initstruct.output_state )
            );
        }
    }
)}



/// `TIMx`: where x can be    2, 3, 4, 5 to select the TIM peripheral.
macro_rules! impl_timinitgeneral{
    ($TIMX:ty) => (

    impl TimInitSplExt for Timer<$TIMX> {

        fn time_base_init(&self, time_base_initstruct : &TimTimeBaseStruct) {
            // Select the Counter Mode and set the clock division
            let (dir, cms) = time_base_initstruct.counter_mode.to_dir_cms();
            self.tim .cr1     .modify(|_,w| w
                .ckd()   .variant( time_base_initstruct.clock_division )
                .dir()     .variant( dir )
                .cms()     .variant( cms )
            );
            // Set the Autoreload value
            self.tim .arr      .write(|w| unsafe { w
                .arr()    .bits( time_base_initstruct.period )
            });
            // Set the Prescaler value
            self.tim .psc      .write(|w| unsafe { w
                .psc()    .bits( time_base_initstruct.prescaler )
            });
            // Generate an update event to reload the Prescaler value immediatly
            self.tim .egr       .write(|w| w
                .ug()      .immediate()
            );
        }
        
        fn oc1_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc1e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr1  .write(|w| unsafe { w
                .ccr1()   .bits( oc_initstruct.pulse )
            });
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr1.output })  .modify(|_,w| w
                .oc1m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc1p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc1e()  .bit( oc_initstruct.output_state )
            );
        }
        
        fn oc2_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc2e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr2  .write(|w| unsafe { w
                .ccr2()   .bits( oc_initstruct.pulse )
            });
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr1.output })  .modify(|_,w| w
                .oc2m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc2p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc2e()  .bit( oc_initstruct.output_state )
            );
        }
        
        fn oc3_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc3e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr3  .write(|w| unsafe { w
                .ccr3()   .bits( oc_initstruct.pulse )
            });
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr2.output })  .modify(|_,w| w
                .oc2m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc3p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc3e()  .bit( oc_initstruct.output_state )
            );
        }
        
        fn oc4_init(&self, oc_initstruct : &TimOCStruct) {
            // Disable the Channel 1: Reset the CC1E Bit
            self.tim .ccer      .modify(|_,w| w
                .cc4e()    .clear_bit()
            );
            // Set the Capture Compare Register value
            self.tim .ccr4  .write(|w| unsafe { w
                .ccr4()   .bits( oc_initstruct.pulse )
            });
            // Select the Output Compare Mode
            (unsafe { self.tim .ccmr2.output })  .modify(|_,w| w
                .oc4m()    .variant( oc_initstruct.oc_mode )
            );
            self.tim .ccer     .modify(|_,w| w
                // Set the Output Compare Polarity
                .cc4p()  .bit( oc_initstruct.oc_polarity.into() )
                // Set the Output State
                .cc4e()  .bit( oc_initstruct.output_state )
            );
        }
    }
)}


#[cfg(feature="tim1")]
impl_timbase!(crate::pac::TIM1);
#[cfg(feature="tim2")]
impl_timbase!(crate::pac::TIM2);
#[cfg(feature="tim3")]
impl_timbase!(crate::pac::TIM3);
#[cfg(feature="tim4")]
impl_timbase!(crate::pac::TIM4);
#[cfg(feature="tim5")]
impl_timbase!(crate::pac::TIM5);
#[cfg(feature="tim6")]
impl_timbase!(crate::pac::TIM6);
#[cfg(feature="tim7")]
impl_timbase!(crate::pac::TIM7);
#[cfg(feature="tim8")]
impl_timbase!(crate::pac::TIM8);

#[cfg(feature="tim6")]
impl_timbaseonly!(crate::pac::TIM6);
#[cfg(feature="tim7")]
impl_timbaseonly!(crate::pac::TIM7);

#[cfg(feature="tim1")]
impl_timadvanced!(crate::pac::TIM1);
#[cfg(feature="tim8")]
impl_timadvanced!(crate::pac::TIM8);

#[cfg(feature="tim1")]
impl_timinitadvanced!(crate::pac::TIM1);
#[cfg(feature="tim8")]
impl_timinitadvanced!(crate::pac::TIM8);

#[cfg(feature="tim2")]
impl_timinitgeneral!(crate::pac::TIM2);
#[cfg(feature="tim3")]
impl_timinitgeneral!(crate::pac::TIM3);
#[cfg(feature="tim4")]
impl_timinitgeneral!(crate::pac::TIM4);
#[cfg(feature="tim5")]
impl_timinitgeneral!(crate::pac::TIM5);
