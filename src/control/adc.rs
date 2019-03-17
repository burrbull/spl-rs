
use super::base::{ITStatus};

/// ADC_interrupts_definition 
pub mod AdcIT {
	pub const EOC  : u16 = 0x0220;
	pub const AWD  : u16 = 0x0140;
	pub const JEOC : u16 = 0x0480;
}

/// ADC_flags_definition
pub mod AdcFlag {
	pub const AWD   : u8 = 0x01;
	pub const EOC   : u8 = 0x02;
	pub const JEOC  : u8 = 0x04;
	pub const JSTRT : u8 = 0x08;
	pub const STRT  : u8 = 0x10;
}


pub trait AdcMonitoring {
	

	/// Enables or disables the specified ADC interrupts.
	/// `ADC_IT`: specifies the ADC interrupt sources to be enabled
	///   or disabled. 
	///   This parameter can be any combination of the following values:
	/// * ADC_IT_EOC: End of conversion interrupt mask
	/// * ADC_IT_AWD: Analog watchdog interrupt mask
	/// * ADC_IT_JEOC: End of injected conversion interrupt mask
	/// `new_state`: new state of the specified ADC interrupts.
	///   This parameter can be: ENABLE or DISABLE.
	fn ITConfig(&self, ADC_IT : AdcIT, new_state : bool );
	
	/// Checks whether the specified ADC flag is set or not.
	/// `adc_flag`: specifies the flag to check. 
	///   This parameter can be one of the following values:
	/// * ADC_FLAG_AWD: Analog watchdog flag
	/// * ADC_FLAG_EOC: End of conversion flag
	/// * ADC_FLAG_JEOC: End of injected group conversion flag
	/// * ADC_FLAG_JSTRT: Start of injected group conversion flag
	/// * ADC_FLAG_STRT: Start of regular group conversion flag
	/// @retval : The new state of ADC_FLAG (SET or RESET).
	fn GetFlagStatus(&self, adc_flag : u8) -> bool;

	/// Clears the ADCx's pending flags.
	/// `ADC_FLAG`: specifies the flag to clear. 
	///   This parameter can be any combination of the following values:
	/// * ADC_FLAG_AWD: Analog watchdog flag
	/// * ADC_FLAG_EOC: End of conversion flag
	/// * ADC_FLAG_JEOC: End of injected group conversion flag
	/// * ADC_FLAG_JSTRT: Start of injected group conversion flag
	/// * ADC_FLAG_STRT: Start of regular group conversion flag
	fn ClearFlag(&self, adc_flag : u8);

	/// Checks whether the specified ADC interrupt has occurred or not.
	/// `ADC_IT`: specifies the ADC interrupt source to check. 
	///   This parameter can be one of the following values:
	/// * ADC_IT_EOC: End of conversion interrupt mask
	/// * ADC_IT_AWD: Analog watchdog interrupt mask
	/// * ADC_IT_JEOC: End of injected conversion interrupt mask
	/// @retval : The new state of ADC_IT (SET or RESET).
	fn GetITStatus(&self, ADC_IT : u16) -> ITStatus;

	/// Clears the ADCx’s interrupt pending bits.
	/// `ADC_IT`: specifies the ADC interrupt pending bit to clear.
	///   This parameter can be any combination of the following values:
	/// * ADC_IT_EOC: End of conversion interrupt mask
	/// * ADC_IT_AWD: Analog watchdog interrupt mask
	/// * ADC_IT_JEOC: End of injected conversion interrupt mask
	fn ClearITPendingBit(&self, ADC_IT : u16);
}


impl AdcMonitoring for ... {

	fn ITConfig(&self, ADC_IT : AdcIT, new_state : bool ) {
	  uint8_t itmask = 0;
	  /* Get the ADC IT index */
	  itmask = (uint8_t)ADC_IT;
	  if new_state
	  {
		/* Enable the selected ADC interrupts */
		ADCx->CR1 |= itmask;
	  }
	  else
	  {
		/* Disable the selected ADC interrupts */
		ADCx->CR1 &= (~(uint32_t)itmask);
	  }
	}
	
	fn GetFlagStatus(&self, adc_flag : u8) -> bool {
		/* Check the status of the specified ADC flag */
		(self.sr.read().bits() & adc_flag as u32) != 0
	}

	fn ClearFlag(&self, adc_flag : u8) {
		/* Clear the selected ADC flags */
		self.sr       .write(|w| unsafe { w
			bits( adc_flag as u32 )
		});
	}

	fn GetITStatus(&self, ADC_IT : u16) -> ITStatus {
		ITStatus bitstatus = RESET;
		uint32_t itmask = 0, enablestatus = 0;
		/* Get the ADC IT index */
		let itmask = (ADC_IT >> 8) as u32;
		/* Get the ADC_IT enable bit status */
		let enablestatus = self.cr1.read().bits() & (ADC_IT as u8 as u32) ;
		/* Check the status of the specified ADC interrupt */
		(((self.cr1.read().bits() & itmask) != (ITStatus::RESET as u32)) && enablestatus).into()
	}

	fn ClearITPendingBit(&self, ADC_IT : u16) {
		/* Get the ADC IT index */
		let itmask = (ADC_IT >> 8) as u8;
		/* Clear the selected ADC interrupt pending bits */
		self.sr       .write(|w| unsafe { w
			.bits( !(itmask as u32) )
		});
	}
}
