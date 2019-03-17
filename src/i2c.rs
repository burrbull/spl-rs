
//use super::device::RCC;
//use super::rcc::RccStd;
//use super::rcc::RccPeriph;

#[derive(Clone, Copy, PartialEq)]
pub struct I2cStruct {
	/// Specifies the clock frequency.
	/// This parameter must be set to a value lower than 400kHz
	pub clock_speed : u32,
	
	/// Specifies the I2C mode.
	/// This parameter can be a value of @ref I2C_mode
	pub mode : Mode,
	
	/// Specifies the I2C fast mode duty cycle.
	/// This parameter can be a value of @ref I2C_duty_cycle_in_fast_mode
	pub duty_cycle : DutyCycle,
	
	/// Specifies the first device own address.
	/// This parameter can be a 7-bit or 10-bit address.
	pub own_address1 : u16,
	
	/// Enables or disables the acknowledgement.
	/// This parameter can be a value of @ref I2C_acknowledgement
	pub ack : bool,
	
	/// Specifies if 7-bit or 10-bit address is acknowledged.
	/// This parameter can be a value of @ref I2C_acknowledged_address
	pub acknowledged_address : u16
}

impl I2cStruct {
	/// Fills each I2C_InitStruct member with its default value.
	/// `I2C_InitStruct`: pointer to an I2C_InitTypeDef structure which will be initialized.
	pub fn init() -> I2cStruct {
	/*---------------- Reset I2C init structure parameters values ----------------*/
		I2cStruct {
			/* initialize the I2C_ClockSpeed member */
			clock_speed          : 5000,
			/* Initialize the I2C_Mode member */
			mode                : Mode::I2C,
			/* Initialize the I2C_DutyCycle member */
			duty_cycle           : DutyCycle::Div2,
			/* Initialize the I2C_OwnAddress1 member */
			own_address1         : 0,
			/* Initialize the I2C_Ack member */
			ack                 : false,
			/* Initialize the I2C_AcknowledgedAddress member */
			acknowledged_address : AcknowledgedAddress::Bit7
		}
	}
}

///I2C_mode 
#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
	I2C,
	SMBusDevice,
	SMBusHost,
}

impl Into<(bool, bool)> for Mode {
	fn into(self) -> (bool, bool) {
		match self {
			Mode::I2C         => (false,false),
			Mode::SMBusDevice => (true, false),
			Mode::SMBusHost   => (true, true),
		}
	}
}

/// I2C_duty_cycle_in_fast_mode
#[derive(Clone, Copy, Debug, PartialEq)] 
pub enum DutyCycle {
	/// I2C fast mode Tlow/Thigh = 2
	Div2,
	/// I2C fast mode Tlow/Thigh = 16/9
	Div16_9
}

/*
/// I2C_acknowledgement
pub mod I2cAck {
	pub const Enable  : u16 = 0x0400,
	pub const Disable : u16 = 0x0000
}*/

/// I2C_transfer_direction 
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
	Transmitter, // 0x00
	Receiver     // 0x01
}

/// I2C_acknowledged_address 
pub mod AcknowledgedAddress {
	pub const Bit7  : u16 = 0x4000;
	pub const Bit10 : u16 = 0xC000;
}

/// I2C_SMBus_alert_pin_level
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMBusAlert {
	Low, // 1,
	High // 0
}

/// I2C_PEC_position
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PECPosition {
	Next,   // 1,
	Current // 0
}

/// I2C_NCAK_position 
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKPosition {
	Next,   // 1,
	Current // 0
}

pub trait I2cSplExt {
	/// Initializes the I2Cx peripheral according to the specified 
	///   parameters in the I2C_InitStruct.
	/// `I2C_InitStruct`: pointer to a I2C_InitTypeDef structure that
	///   contains the configuration information for the specified I2C peripheral.
	fn init(&self, rcc : &RCC, i2c_initstruct : &I2cStruct);

	/// Enables or disables the specified I2C peripheral.
	/// `new_state`: new state of the I2Cx peripheral. 
	///   This parameter can be: ENABLE or DISABLE.
	fn cmd(&self, new_state : bool);

	/// Enables or disables the specified I2C DMA requests.
	/// `new_state`: new state of the I2C DMA transfer.
	///   This parameter can be: ENABLE or DISABLE.
	fn dma_cmd(&self, new_state : bool);

	/// Specifies if the next DMA transfer will be the last one.
	/// `new_state`: new state of the I2C DMA last transfer.
	///   This parameter can be: ENABLE or DISABLE.
	fn dma_last_transfer_cmd(&self, new_state : bool);

	/// Generates I2Cx communication START condition.
	/// `new_state`: new state of the I2C START condition generation.
	///   This parameter can be: ENABLE or DISABLE..
	fn generate_start(&self, new_state : bool);

	/// Generates I2Cx communication STOP condition.
	/// `new_state`: new state of the I2C STOP condition generation.
	///   This parameter can be: ENABLE or DISABLE..
	fn generate_stop(&self, new_state : bool);

	/// Enables or disables the specified I2C acknowledge feature.
	/// `new_state`: new state of the I2C Acknowledgement.
	///   This parameter can be: ENABLE or DISABLE..
	fn acknowledge_config(&self, new_state : bool);

	/// Configures the specified I2C own address2.
	/// `Address`: specifies the 7bit I2C own address2..
	fn own_address2_config(&self, address : u8);

	/// Enables or disables the specified I2C dual addressing mode.
	/// `new_state`: new state of the I2C dual addressing mode.
	///   This parameter can be: ENABLE or DISABLE.
	fn dual_address_cmd(&self, new_state : bool);

	/// Enables or disables the specified I2C general call feature.
	/// `new_state`: new state of the I2C General call.
	///   This parameter can be: ENABLE or DISABLE.
	fn general_call_cmd(&self, new_state : bool);

	/// Sends a data byte through the I2Cx peripheral.
	/// `Data`: Byte to be transmitted..
	fn send_data(&self, data : u8);

	/// Returns the most recent received data by the I2Cx peripheral.
	/// @retval The value of the received data.
	fn receive_data(&self) -> u8;

	/// Transmits the address byte to select the slave device.
	/// `Address`: specifies the slave address which will be transmitted
	/// `I2C_Direction`: specifies whether the I2C device will be a
	///   Transmitter or a Receiver. This parameter can be one of the following values
	/// * I2C_Direction_Transmitter: Transmitter mode
	/// * I2C_Direction_Receiver: Receiver mode.
	fn send_7bit_address(&self, address : u8, direction : Direction);


	/// Enables or disables the specified I2C software reset.
	/// `new_state`: new state of the I2C software reset.
	///   This parameter can be: ENABLE or DISABLE.
	fn software_reset_cmd(&self, new_state : bool);

	/// Selects the specified I2C NACK position in master receiver mode.
	///         This function is useful in I2C Master Receiver mode when the number
	///         of data to be received is equal to 2. In this case, this function 
	///         should be called (with parameter I2C_NACKPosition_Next) before data 
	///         reception starts,as described in the 2-byte reception procedure 
	///         recommended in Reference Manual in Section: Master receiver.                
	/// `I2C_NACKPosition`: specifies the NACK position. 
	///   This parameter can be one of the following values:
	/// * I2C_NACKPosition_Next: indicates that the next byte will be the last
	///          received byte.  
	/// * I2C_NACKPosition_Current: indicates that current byte is the last 
	///          received byte.
	///            
	/// @note    This function configures the same bit (POS) as I2C_PECPositionConfig() 
	///          but is intended to be used in I2C mode while I2C_PECPositionConfig() 
	///          is intended to used in SMBUS mode.
	fn nack_position_config(&self, nack_position : NACKPosition);

	/// Drives the SMBusAlert pin high or low for the specified I2C.
	/// `I2C_SMBusAlert`: specifies SMBAlert pin level. 
	///   This parameter can be one of the following values:
	/// * I2C_SMBusAlert_Low: SMBAlert pin driven low
	/// * I2C_SMBusAlert_High: SMBAlert pin driven high
	fn smbus_alert_config(&self, smbus_alert : SMBusAlert);

	/// Enables or disables the specified I2C PEC transfer.
	/// `new_state`: new state of the I2C PEC transmission.
	///   This parameter can be: ENABLE or DISABLE.
	fn transmit_pec(&self, new_state : bool);

	/// Selects the specified I2C PEC position.
	/// `I2C_PECPosition`: specifies the PEC position. 
	///   This parameter can be one of the following values:
	/// * I2C_PECPosition_Next: indicates that the next byte is PEC
	/// * I2C_PECPosition_Current: indicates that current byte is PEC
	///       
	/// @note    This function configures the same bit (POS) as I2C_NACKPositionConfig()
	///          but is intended to be used in SMBUS mode while I2C_NACKPositionConfig() 
	///          is intended to used in I2C mode.
	fn pec_position_config(&self, pec_position : PECPosition);

	/// Enables or disables the PEC value calculation of the transferred bytes.
	/// `new_state`: new state of the I2Cx PEC value calculation.
	///   This parameter can be: ENABLE or DISABLE.
	fn calculate_pec(&self, new_state : bool);

	/// Returns the PEC value for the specified I2C.
	/// @retval The PEC value.
	fn get_pec(&self) -> u8;

	/// Enables or disables the specified I2C ARP.
	/// `new_state`: new state of the I2Cx ARP. 
	///   This parameter can be: ENABLE or DISABLE.
	fn arp_cmd(&self, new_state : bool);

	/// Enables or disables the specified I2C Clock stretching.
	/// `new_state`: new state of the I2Cx Clock stretching.
	///   This parameter can be: ENABLE or DISABLE.
	fn stretch_clock_cmd(&self, new_state : bool);

	/// Selects the specified I2C fast mode duty cycle.
	/// `I2C_DutyCycle`: specifies the fast mode duty cycle.
	///   This parameter can be one of the following values:
	/// * I2C_DutyCycle_2: I2C fast mode Tlow/Thigh = 2
	/// * I2C_DutyCycle_16_9: I2C fast mode Tlow/Thigh = 16/9
	fn fast_mode_duty_cycle_config(&self, duty_cycle : DutyCycle);
}


macro_rules! impl_i2c {
    ($I2Cx:ty) => (

	impl I2cSplExt for $I2Cx {
	
		fn init(&self, rcc : &RCC, i2c_initstruct : &I2cStruct) {
			let mut result;// = 0x04u16;
			//uint32_t pclk1 = 8000000;

			/*---------------------------- I2Cx CR2 Configuration ------------------------*/
			/* Get pclk1 frequency value */
			let rcc_clocks = rcc.GetClocksFreq();
			let pclk1 = rcc_clocks.PCLK1_Frequency;
			/* Set frequency bits depending on pclk1 value */
			let freqrange = (pclk1 / 1_000_000) as u16;
			/* Write to I2Cx CR2 */
			self.cr2     .modify(|_,w| unsafe { w
				.freq()  .bits( freqrange as u8 )
			});

			/*---------------------------- I2Cx CCR Configuration ------------------------*/
			/* Disable the selected I2C peripheral to configure TRISE */
			self.cr1        .modify(|_,w| w
				.pe()    .clear_bit()
			);
			/* Reset tmpreg value */
			
			/* Configure speed in standard mode */
			if i2c_initstruct.clock_speed <= 100000 {
				/* Standard mode speed calculate */
				result = (pclk1 / (i2c_initstruct.clock_speed << 1)) as u16;
				/* Test if CCR value is under 0x4*/
				if result < 0x04 {
				  /* Set minimum allowed value */
				  result = 0x04;  
				}
				/* Set Maximum Rise Time for standard mode */
				self.trise     .modify(|_,w| unsafe { w
					.trise()  .bits( (freqrange + 1 ) as u8 )
				});
				/* Write to I2Cx CCR */
				self.ccr       .write(|w| unsafe { w
					.ccr()     .bits( result )
				});
			} /* Configure speed in fast mode */
			else { /*(I2C_InitStruct->I2C_ClockSpeed <= 400000)*/
				if i2c_initstruct.duty_cycle == DutyCycle::Div2 {
					/* Fast mode speed calculate: Tlow/Thigh = 2 */
					result = (pclk1 / (i2c_initstruct.clock_speed * 3)) as u16;
				}
				else /*I2C_InitStruct->I2C_DutyCycle == I2C_DutyCycle_16_9*/
				{
					/* Fast mode speed calculate: Tlow/Thigh = 16/9 */
					result = (pclk1 / (i2c_initstruct.clock_speed * 25)) as u16;
				}
				
				/* Test if CCR value is under 0x1*/
				if result == 0 {
					/* Set minimum allowed value */
					result =0x0001;  
				}
				
				/* Set Maximum Rise Time for fast mode */
				self.trise     .modify(|_,w| unsafe { w
					.trise()  .bits( (((freqrange * 300u16) / 1000u16) + 1u16) as u8 )
				});
				
				/* Write to I2Cx CCR */
				self.ccr       .write(|w| unsafe { w
					/* Set speed value and set F/S bit for fast mode */
					.f_s()     .set_bit()
					.ccr()     .bits( result )
					.duty()    .bit( i2c_initstruct.duty_cycle == DutyCycle::Div16_9 )
				});
			}
			/* Enable the selected I2C peripheral */
			self.cr1        .modify(|_,w| w
				.pe()    .set_bit()
			);

			/*---------------------------- I2Cx CR1 Configuration ------------------------*/
			self.cr1        .modify(|_,w| w
				.smbus()    .clear_bit()
				.smbtype()  .clear_bit()
				.ack()      .clear_bit()
			);
			/* Configure I2Cx: mode and acknowledgement */
			/* Set SMBTYPE and SMBUS bits according to I2C_Mode value */
			/* Set ACK bit according to I2C_Ack value */
			let mode : (bool, bool) = i2c_initstruct.mode.into();
			self.cr1        .modify(|_,w| w
				.smbus()    .bit( mode.0 )
				.smbtype()  .bit( mode.1 )
				.ack()      .bit( i2c_initstruct.ack )
			);

			/*---------------------------- I2Cx OAR1 Configuration -----------------------*/
			/* Set I2Cx Own Address1 and acknowledged address */
			self.oar1       .modify(|r,w| unsafe { w
				.bits( r.bits() | i2c_initstruct.acknowledged_address as u32 | i2c_initstruct.own_address1 as u32 )
			});
		}

		fn cmd(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.pe()    .bit(new_state)
			);
		}

		fn dma_cmd(&self, new_state : bool) {
			self.cr2        .modify(|_,w| w
				.dmaen()    .bit(new_state)
			);
		}

		fn dma_last_transfer_cmd(&self, new_state : bool) {
			self.cr2        .modify(|_,w| w
				.last()    .bit(new_state)
			);
		}

		fn generate_start(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.start()    .bit(new_state)
			);
		}

		fn generate_stop(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.stop()    .bit(new_state)
			);
		}

		fn acknowledge_config(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.ack()    .bit(new_state)
			);
		}

		fn own_address2_config(&self, address : u8) {
			self.oar2         .modify(|_,w| unsafe { w
				.add2()       .bits( address >> 1 )
			});
			/*
		  uint16_t tmpreg = 0;

		  /* Check the parameters */
		  assert_param(IS_I2C_ALL_PERIPH(I2Cx));

		  /* Get the old register value */
		  tmpreg = I2Cx->OAR2;

		  /* Reset I2Cx Own address2 bit [7:1] */
		  tmpreg &= OAR2_ADD2_Reset;

		  /* Set I2Cx Own address2 */
		  tmpreg |= (uint16_t)((uint16_t)address & (uint16_t)0x00FE);

		  /* Store the new register value */
		  I2Cx->OAR2 = tmpreg;*/
		}

		fn dual_address_cmd(&self, new_state : bool) {
			self.oar2        .modify(|_,w| w
				.endual()    .bit(new_state)
			);
		}

		fn general_call_cmd(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.engc()    .bit(new_state)
			);
		}

		fn send_data(&self, data : u8) {
			/* Write in the DR register the data to be sent */
			self.dr    .write(|w| unsafe { w
				.dr()	.bits(data)
			});
		}

		fn receive_data(&self) -> u8 {
			/* Return the data in the DR register */
			self.dr.read().dr().bits()
		}

		fn send_7bit_address(&self, address : u8, direction : Direction) {
			/* Test on the direction to set/reset the read/write bit */
			self.oar1     .modify(|_,w| w
				/* Set the address bit0 for read/write */
				.add0()   .bit( direction == Direction::Receiver )
			); 
			/* Send the address */
			self.dr    .write(|w| unsafe { w
				.dr()	.bits(address)
			});
		}

		fn software_reset_cmd(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.swrst()    .bit(new_state)
			);
		}

		fn nack_position_config(&self, nack_position : NACKPosition) {
			self.cr1       .modify(|_,w| w
				.pos()      .bit( nack_position == NACKPosition::Next )
			);
		}

		fn smbus_alert_config(&self, i2c_smbusalert : SMBusAlert) {
			self.cr1       .modify(|_,w| w
				.alert()   .bit( i2c_smbusalert == SMBusAlert::Low )
			);
		}

		fn transmit_pec(&self, new_state : bool) {
			self.cr1      .modify(|_,w| w
				.pec()    .bit(new_state)
			);
		}

		fn pec_position_config(&self, pec_position : PECPosition) {
			self.cr1       .modify(|_,w| w
				.pos()     .bit( pec_position == PECPosition::Next )
			);
		}

		fn calculate_pec(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.enpec()    .bit(new_state)
			);
		}

		fn get_pec(&self) -> u8 {
			/* Return the selected I2C PEC value */
			self.sr2.read()  .pec().bits()
		}

		fn arp_cmd(&self, new_state : bool) {
			self.cr1        .modify(|_,w| w
				.enarp()    .bit(new_state)
			);
		}

		fn stretch_clock_cmd(&self, new_state : bool) {
			let state : bool = new_state;
			self.cr1        .modify(|_,w| w
				.nostretch() .bit(!state)
			);
		}

		fn fast_mode_duty_cycle_config(&self, duty_cycle : DutyCycle) {
			self.ccr       .modify(|_,w| w
				.duty()     .bit( duty_cycle == DutyCycle::Div16_9 )
			);
		}
	}

)}


use super::device::{I2C1,I2C2};

impl_i2c!(I2C1);
impl_i2c!(I2C2);
