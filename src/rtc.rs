use super::device::RTC;

const RTC_LSB_MASK: u32 = 0x0000FFFF; /* RTC LSB Mask */
const PRLH_MSB_MASK: u32 = 0x000F0000; /* RTC Prescaler MSB Mask */

pub trait RtcStd {
    /// Enters the RTC configuration mode.
    fn enter_config_mode(&self);

    /// Exits from the RTC configuration mode.
    fn exit_config_mode(&self);

    /// Gets the RTC counter value.
    /// @retval : RTC counter value.
    fn get_counter(&self) -> u32;

    /// Sets the RTC counter value.
    /// `CounterValue`: RTC counter new value.
    fn set_counter(&self, counter_value: u32);

    /// Sets the RTC prescaler value.
    /// `PrescalerValue`: RTC prescaler new value.
    fn set_prescaler(&self, prescaler_value: u32);

    /// Sets the RTC alarm value.
    /// `AlarmValue`: RTC alarm new value.
    fn set_alarm(&self, alarm_value: u32);

    /// Gets the RTC divider value.
    /// @retval : RTC Divider value.
    fn get_divider(&self) -> u32;

    /// Waits until last write operation on RTC registers has finished.
    ///   This function must be called before any write to RTC registers.
    fn wait_for_last_task(&self);

    /// Waits until the RTC registers (RTC_CNT, RTC_ALR and RTC_PRL)
    ///   are synchronized with RTC APB clock.
    ///   This function must be called before any read operation after
    ///   an APB reset or an APB clock stop.
    fn wait_for_synchro(&self);
}

impl RtcStd for RTC {
    fn enter_config_mode(&self) {
        /* Set the CNF flag to enter in the Configuration Mode */
        self.crl.modify(|_, w| w.cnf().set_bit());
    }

    fn exit_config_mode(&self) {
        /* Reset the CNF flag to exit from the Configuration Mode */
        self.crl.modify(|_, w| w.cnf().clear_bit());
    }

    fn get_counter(&self) -> u32 {
        ((self.cnth.read().cnth().bits() as u32) << 16) | (self.cntl.read().cntl().bits() as u32)
    }

    fn set_counter(&self, counter_value: u32) {
        self.enter_config_mode();
        /* Set RTC COUNTER MSB word */
        self.cnth
            .write(|w| unsafe { w.cnth().bits((counter_value >> 16) as u16) });
        /* Set RTC COUNTER LSB word */
        self.cntl
            .write(|w| unsafe { w.cntl().bits((counter_value & RTC_LSB_MASK) as u16) });
        self.exit_config_mode();
    }

    fn set_prescaler(&self, prescaler_value: u32) {
        self.enter_config_mode();
        /* Set RTC PRESCALER MSB word */
        self.prlh.write(|w| unsafe {
            w.prlh()
                .bits(((prescaler_value & PRLH_MSB_MASK) >> 16) as u8)
        });
        /* Set RTC PRESCALER LSB word */
        self.prll
            .write(|w| unsafe { w.prll().bits((prescaler_value & RTC_LSB_MASK) as u16) });
        self.exit_config_mode();
    }

    fn set_alarm(&self, alarm_value: u32) {
        self.enter_config_mode();
        /* Set the ALARM MSB word */
        self.alrh
            .write(|w| unsafe { w.alrh().bits((alarm_value >> 16) as u16) });
        /* Set the ALARM LSB word */
        self.alrl
            .write(|w| unsafe { w.alrl().bits((alarm_value & RTC_LSB_MASK) as u16) });
        self.exit_config_mode();
    }

    fn get_divider(&self) -> u32 {
        ((self.divh.read().divh().bits() as u32) << 16) | (self.divl.read().divl().bits() as u32)
    }

    fn wait_for_last_task(&self) {
        /* Loop until RTOFF flag is set */
        while self.crl.read().rtoff().bit_is_clear() {}
    }

    fn wait_for_synchro(&self) {
        /* Clear RSF flag */
        self.crl.modify(|_, w| w.rsf().clear_bit());
        /* Loop until RSF flag is set */
        while self.crl.read().rsf().bit_is_clear() {}
    }
}
