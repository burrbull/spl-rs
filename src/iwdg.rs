use crate::pac::IWDG;

/// Write_access_to_IWDG_PR_and_IWDG_RLR_registers 
#[derive(Clone, Copy, PartialEq)]
pub enum WriteAccess {
    Enable,
    Disable
}

impl From<WriteAccess> for u16 {
    fn from( access : WriteAccess ) -> u16 {
        match access {
            WriteAccess::Enable  => 0x5555,
            WriteAccess::Disable => 0x0000,
        }
    }
}

/// IWDG_prescaler 
pub use crate::pac::iwdg::pr::PRW as Prescaler;

/* KR register bit mask */
pub const KR_KEY_RELOAD : u16 = 0xAAAA;
pub const KR_KEY_ENABLE : u16 = 0xCCCC;


pub trait IwdgStd {
    /// Enables or disables write access to IWDG_PR and IWDG_RLR
    ///   registers.
    /// `IWDG_WriteAccess`: new state of write access to IWDG_PR and
    ///   IWDG_RLR registers.
    ///   This parameter can be one of the following values:
    /// * IWDG_WriteAccess_Enable: Enable write access to 
    ///   IWDG_PR and IWDG_RLR registers
    /// * IWDG_WriteAccess_Disable: Disable write access to
    ///   IWDG_PR and IWDG_RLR registers
    fn write_access_cmd(&self, write_access : WriteAccess);

    /// Sets IWDG Prescaler value.
    /// `IWDG_Prescaler`: specifies the IWDG Prescaler value.
    ///   This parameter can be one of the following values:
    /// * IWDG_Prescaler_4: IWDG prescaler set to 4
    /// * IWDG_Prescaler_8: IWDG prescaler set to 8
    /// * IWDG_Prescaler_16: IWDG prescaler set to 16
    /// * IWDG_Prescaler_32: IWDG prescaler set to 32
    /// * IWDG_Prescaler_64: IWDG prescaler set to 64
    /// * IWDG_Prescaler_128: IWDG prescaler set to 128
    /// * IWDG_Prescaler_256: IWDG prescaler set to 256
    fn set_prescaler(&self, prescaler : Prescaler);

    /// Sets IWDG Reload value.
    /// `Reload`: specifies the IWDG Reload value.
    ///   This parameter must be a number between 0 and 0x0FFF.
    fn set_reload(&self, reload : u16);

    /// Reloads IWDG counter with value defined in the reload register
    ///   (write access to IWDG_PR and IWDG_RLR registers disabled).
    fn reload_counter(&self);

    /// Enables IWDG (write access to IWDG_PR and IWDG_RLR registers
    ///   disabled).
    fn enable(&self);
}

impl IwdgStd for IWDG {
    fn write_access_cmd(&self, write_access : WriteAccess) {
        self.kr   .write(|w| unsafe { w
            .key()   .bits( write_access.into() )
        });
    }

    fn set_prescaler(&self, prescaler : Prescaler) {
        self.pr   .write(|w| w
            .pr()   .variant( prescaler )
        );
    }

    fn set_reload(&self, reload : u16) {
        self.rlr   .write(|w| w
            .rl()   .bits( reload )
        );
    }

    fn reload_counter(&self) {
        self.kr   .write(|w| unsafe { w
            .key()   .bits( KR_KEY_RELOAD )
        });
    }

    fn enable(&self) {
        self.kr   .write(|w| unsafe { w
            .key()   .bits( KR_KEY_ENABLE )
        });
    }
}


