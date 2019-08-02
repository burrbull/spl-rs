use crate::pac::DBGMCU;

/// DBGMCU_Exported_Constants
pub mod constant {
    pub const SLEEP: u32 = 1 << 0;
    pub const STOP: u32 = 1 << 1;
    pub const STANDBY: u32 = 1 << 2;
    pub const IWDG_STOP: u32 = 1 << 8;
    pub const WWDG_STOP: u32 = 1 << 9;
    pub const TIM1_STOP: u32 = 1 << 10;
    pub const TIM2_STOP: u32 = 1 << 11;
    pub const TIM3_STOP: u32 = 1 << 12;
    pub const TIM4_STOP: u32 = 1 << 13;
    pub const CAN1_STOP: u32 = 1 << 14;
    pub const I2C1_SMBUS_TIMEOUT: u32 = 1 << 15;
    pub const I2C2_SMBUS_TIMEOUT: u32 = 1 << 16;
    pub const TIM8_STOP: u32 = 1 << 17;
    pub const TIM5_STOP: u32 = 1 << 18;
    pub const TIM6_STOP: u32 = 1 << 19;
    pub const TIM7_STOP: u32 = 1 << 20;
}

pub trait DbgMcuStd {
    /// Returns the device revision identifier.
    /// @retval : Device revision identifier
    fn get_rev_id(&self) -> u16;

    /// Returns the device identifier.
    /// @retval : Device identifier
    fn get_dev_id(&self) -> u16;

    /// Configures the specified peripheral and low power mode behavior
    ///   when the MCU under Debug mode.
    /// `DBGMCU_Periph`: specifies the peripheral and low power mode.
    ///   This parameter can be any combination of the following values:
    /// * DBGMCU_SLEEP: Keep debugger connection during SLEEP mode              
    /// * DBGMCU_STOP: Keep debugger connection during STOP mode               
    /// * DBGMCU_STANDBY: Keep debugger connection during STANDBY mode            
    /// * DBGMCU_IWDG_STOP: Debug IWDG stopped when Core is halted          
    /// * DBGMCU_WWDG_STOP: Debug WWDG stopped when Core is halted          
    /// * DBGMCU_TIM1_STOP: TIM1 counter stopped when Core is halted          
    /// * DBGMCU_TIM2_STOP: TIM2 counter stopped when Core is halted          
    /// * DBGMCU_TIM3_STOP: TIM3 counter stopped when Core is halted          
    /// * DBGMCU_TIM4_STOP: TIM4 counter stopped when Core is halted          
    /// * DBGMCU_CAN1_STOP: Debug CAN 1 stopped when Core is halted           
    /// * DBGMCU_I2C1_SMBUS_TIMEOUT: I2C1 SMBUS timeout mode stopped when Core is
    ///                                 halted
    /// * DBGMCU_I2C2_SMBUS_TIMEOUT: I2C2 SMBUS timeout mode stopped when Core is
    ///                                 halted
    /// * DBGMCU_TIM5_STOP: TIM5 counter stopped when Core is halted          
    /// * DBGMCU_TIM6_STOP: TIM6 counter stopped when Core is halted          
    /// * DBGMCU_TIM7_STOP: TIM7 counter stopped when Core is halted          
    /// * DBGMCU_TIM8_STOP: TIM8 counter stopped when Core is halted          
    /// `new_state : FunctionalState`: new state of the specified peripheral in Debug mode.
    ///   This parameter can be: ENABLE or DISABLE.
    fn config(&self, dbgmcu_periph: u32, new_state: bool);
}

impl DbgMcuStd for DBGMCU {
    fn get_rev_id(&self) -> u16 {
        self.idcode.read().rev_id().bits()
    }

    fn get_dev_id(&self) -> u16 {
        self.idcode.read().dev_id().bits()
    }

    fn config(&self, dbgmcu_periph: u32, new_state: bool) {
        self.cr.modify(|r, w| unsafe {
            w.bits(if new_state {
                r.bits() | dbgmcu_periph
            } else {
                r.bits() & !dbgmcu_periph
            })
        });
    }
}
