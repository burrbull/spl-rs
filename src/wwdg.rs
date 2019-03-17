
use crate::device::WWDG;

/// WWDG_Prescaler
pub use crate::device::wwdg::cfr::WDGTBW as WwdgPrescaler;


pub trait WwdgStd {
	/// Sets the WWDG Prescaler.
	/// `WWDG_Prescaler`: specifies the WWDG Prescaler.
	///   This parameter can be one of the following values:
	/// * WWDG_Prescaler_1: WWDG counter clock = (PCLK1/4096)/1
	/// * WWDG_Prescaler_2: WWDG counter clock = (PCLK1/4096)/2
	/// * WWDG_Prescaler_4: WWDG counter clock = (PCLK1/4096)/4
	/// * WWDG_Prescaler_8: WWDG counter clock = (PCLK1/4096)/8
	fn set_prescaler(&self, wwdg_prescaler : WwdgPrescaler);

	/// Sets the WWDG window value.
	/// `WindowValue`: specifies the window value to be compared to
	///   the downcounter.
	///   This parameter value must be lower than 0x80.
	fn set_window_value(&self, window_value : u8);

	/// Enables the WWDG Early Wakeup interrupt(EWI).
	fn enable_it(&self);

	/// Sets the WWDG counter value.
	/// `Counter`: specifies the watchdog counter value.
	///   This parameter must be a number between 0x40 and 0x7F.
	fn set_counter(&self, counter : u8);

	/// Enables WWDG and load the counter value.                  
	/// `Counter`: specifies the watchdog counter value.
	///   This parameter must be a number between 0x40 and 0x7F.
	fn enable(&self, counter : u8);
}


impl WwdgStd for WWDG {
	fn set_prescaler(&self, wwdg_prescaler : WwdgPrescaler) {
		/* Set WDGTB[1:0] bits according to WWDG_Prescaler value */
		self.cfr       .write(|w| w
			.wdgtb()   .variant( wwdg_prescaler )
		);
	}

	fn set_window_value(&self, window_value : u8) {
		/* Set W[6:0] bits according to WindowValue value */
		self.cfr       .modify(|_,w| unsafe { w
			.w()       .bits( window_value )
		});
	}

	fn enable_it(&self) {
		self.cfr       .write(|w| w
			.ewi()     .set_bit()
		);
	}
	
	fn set_counter(&self, counter : u8) {
		/* Write to T[6:0] bits to configure the counter value, no need to do
		 a read-modify-write; writing a 0 to WDGA bit does nothing */
		self.cr        .write(|w| unsafe { w
			.t()       .bits( counter )
		});
	}
	
	fn enable(&self, counter : u8) {
		self.cr        .write(|w| unsafe { w
			.wdga()    .set_bit()
			.t()       .bits( counter )
		});
	}
}


