
/// I2C State Monitoring Functions
///
///This I2C driver provides three different ways for I2C state monitoring
/// depending on the application requirements and constraints:
///       
/// 
///1) Basic state monitoring:
///   Using I2C_CheckEvent() function:
///   It compares the status registers (SR1 and SR2) content to a given event
///   (can be the combination of one or more flags).
///   It returns SUCCESS if the current status includes the given flags 
///   and returns ERROR if one or more flags are missing in the current status.
///   - When to use:
///     - This function is suitable for most applications as well as for startup 
///     activity since the events are fully described in the product reference manual 
///     (RM0008).
///     - It is also suitable for users who need to define their own events.
///   - Limitations:
///     - If an error occurs (ie. error flags are set besides to the monitored flags),
///       the I2C_CheckEvent() function may return SUCCESS despite the communication
///       hold or corrupted real state. 
///       In this case, it is advised to use error interrupts to monitor the error
///       events and handle them in the interrupt IRQ handler.
///       
///       @note 
///       For error management, it is advised to use the following functions:
///         - I2C_ITConfig() to configure and enable the error interrupts (I2C_IT_ERR).
///         - I2Cx_ER_IRQHandler() which is called when the error interrupt occurs.
///           Where x is the peripheral instance (I2C1, I2C2 ...)
///         - I2C_GetFlagStatus() or I2C_GetITStatus() to be called into I2Cx_ER_IRQHandler()
///           in order to determine which error occurred.
///         - I2C_ClearFlag() or I2C_ClearITPendingBit() and/or I2C_SoftwareResetCmd()
///           and/or I2C_GenerateStop() in order to clear the error flag and source,
///           and return to correct communication status.
///           
///
/// 2) Advanced state monitoring:
///    Using the function I2C_GetLastEvent() which returns the image of both status 
///    registers in a single word (uint32_t) (Status Register 2 value is shifted left 
///    by 16 bits and concatenated to Status Register 1).
///    - When to use:
///      - This function is suitable for the same applications above but it allows to
///        overcome the limitations of I2C_GetFlagStatus() function (see below).
///        The returned value could be compared to events already defined in the 
///        library (stm32f10x_i2c.h) or to custom values defined by user.
///      - This function is suitable when multiple flags are monitored at the same time.
///      - At the opposite of I2C_CheckEvent() function, this function allows user to
///        choose when an event is accepted (when all events flags are set and no 
///        other flags are set or just when the needed flags are set like 
///        I2C_CheckEvent() function).
///    - Limitations:
///      - User may need to define his own events.
///      - Same remark concerning the error management is applicable for this 
///        function if user decides to check only regular communication flags (and 
///        ignores error flags).
///    
///
/// 3) Flag-based state monitoring:
///    Using the function I2C_GetFlagStatus() which simply returns the status of 
///    one single flag (ie. I2C_FLAG_RXNE ...). 
///    - When to use:
///       - This function could be used for specific applications or in debug phase.
///       - It is suitable when only one flag checking is needed (most I2C events 
///         are monitored through multiple flags).
///    - Limitations: 
///       - When calling this function, the Status register is accessed. Some flags are
///         cleared when the status register is accessed. So checking the status
///         of one Flag, may clear other ones.
///       - Function may need to be called twice or more in order to monitor one 
///         single event.

pub trait I2cControl {
	/// Enables or disables the specified I2C interrupts.
	/// `I2C_IT`: specifies the I2C interrupts sources to be enabled or disabled. 
	///   This parameter can be any combination of the following values:
	/// * I2C_IT_BUF: Buffer interrupt mask
	/// * I2C_IT_EVT: Event interrupt mask
	/// * I2C_IT_ERR: Error interrupt mask
	/// `new_state`: new state of the specified I2C interrupts.
	///   This parameter can be: ENABLE or DISABLE.
	fn ITConfig(&self, I2C_IT : u16, bool);
	
	/// 1) Basic state monitoring

	/// Checks whether the last I2Cx Event is equal to the one passed
	///   as parameter.
	/// `I2C_EVENT`: specifies the event to be checked. 
	///   This parameter can be one of the following values:
	/// * I2C_EVENT_SLAVE_TRANSMITTER_ADDRESS_MATCHED           : EV1
	/// * I2C_EVENT_SLAVE_RECEIVER_ADDRESS_MATCHED              : EV1
	/// * I2C_EVENT_SLAVE_TRANSMITTER_SECONDADDRESS_MATCHED     : EV1
	/// * I2C_EVENT_SLAVE_RECEIVER_SECONDADDRESS_MATCHED        : EV1
	/// * I2C_EVENT_SLAVE_GENERALCALLADDRESS_MATCHED            : EV1
	/// * I2C_EVENT_SLAVE_BYTE_RECEIVED                         : EV2
	/// * (I2C_EVENT_SLAVE_BYTE_RECEIVED | I2C_FLAG_DUALF)      : EV2
	/// * (I2C_EVENT_SLAVE_BYTE_RECEIVED | I2C_FLAG_GENCALL)    : EV2
	/// * I2C_EVENT_SLAVE_BYTE_TRANSMITTED                      : EV3
	/// * (I2C_EVENT_SLAVE_BYTE_TRANSMITTED | I2C_FLAG_DUALF)   : EV3
	/// * (I2C_EVENT_SLAVE_BYTE_TRANSMITTED | I2C_FLAG_GENCALL) : EV3
	/// * I2C_EVENT_SLAVE_ACK_FAILURE                           : EV3_2
	/// * I2C_EVENT_SLAVE_STOP_DETECTED                         : EV4
	/// * I2C_EVENT_MASTER_MODE_SELECT                          : EV5
	/// * I2C_EVENT_MASTER_TRANSMITTER_MODE_SELECTED            : EV6     
	/// * I2C_EVENT_MASTER_RECEIVER_MODE_SELECTED               : EV6
	/// * I2C_EVENT_MASTER_BYTE_RECEIVED                        : EV7
	/// * I2C_EVENT_MASTER_BYTE_TRANSMITTING                    : EV8
	/// * I2C_EVENT_MASTER_BYTE_TRANSMITTED                     : EV8_2
	/// * I2C_EVENT_MASTER_MODE_ADDRESS10                       : EV9
	///     
	/// @note: For detailed description of Events, please refer to section 
	///    I2C_Events in stm32f10x_i2c.h file.
	///    
	/// @retval An ErrorStatus enumeration value:
	/// - SUCCESS: Last event is equal to the I2C_EVENT
	/// - ERROR: Last event is different from the I2C_EVENT
	fn CheckEvent(&self, I2C_EVENT : u32) -> ErrorStatus;

	/// 2) Advanced state monitoring

	/// Returns the last I2Cx Event.
	///     
	/// @note: For detailed description of Events, please refer to section 
	///    I2C_Events in stm32f10x_i2c.h file.
	///    
	/// @retval The last event
	fn GetLastEvent(&self) -> u32;

	/// 3) Flag-based state monitoring

	/*/// Checks whether the specified I2C flag is set or not.
	/// `I2C_FLAG`: specifies the flag to check. 
	///   This parameter can be one of the following values:
	/// * I2C_FLAG_DUALF: Dual flag (Slave mode)
	/// * I2C_FLAG_SMBHOST: SMBus host header (Slave mode)
	/// * I2C_FLAG_SMBDEFAULT: SMBus default header (Slave mode)
	/// * I2C_FLAG_GENCALL: General call header flag (Slave mode)
	/// * I2C_FLAG_TRA: Transmitter/Receiver flag
	/// * I2C_FLAG_BUSY: Bus busy flag
	/// * I2C_FLAG_MSL: Master/Slave flag
	/// * I2C_FLAG_SMBALERT: SMBus Alert flag
	/// * I2C_FLAG_TIMEOUT: Timeout or Tlow error flag
	/// * I2C_FLAG_PECERR: PEC error in reception flag
	/// * I2C_FLAG_OVR: Overrun/Underrun flag (Slave mode)
	/// * I2C_FLAG_AF: Acknowledge failure flag
	/// * I2C_FLAG_ARLO: Arbitration lost flag (Master mode)
	/// * I2C_FLAG_BERR: Bus error flag
	/// * I2C_FLAG_TXE: Data register empty flag (Transmitter)
	/// * I2C_FLAG_RXNE: Data register not empty (Receiver) flag
	/// * I2C_FLAG_STOPF: Stop detection flag (Slave mode)
	/// * I2C_FLAG_ADD10: 10-bit header sent flag (Master mode)
	/// * I2C_FLAG_BTF: Byte transfer finished flag
	/// * I2C_FLAG_ADDR: Address sent flag (Master mode) "ADSL"
	///   Address matched flag (Slave mode)"ENDA"
	/// * I2C_FLAG_SB: Start bit flag (Master mode)
	/// @retval The new state of I2C_FLAG (SET or RESET).
	fn GetFlagStatus(&self, I2C_FLAG : u32) -> FlagStatus;

	/// Clears the I2Cx's pending flags.
	/// `I2C_FLAG`: specifies the flag to clear. 
	///   This parameter can be any combination of the following values:
	/// * I2C_FLAG_SMBALERT: SMBus Alert flag
	/// * I2C_FLAG_TIMEOUT: Timeout or Tlow error flag
	/// * I2C_FLAG_PECERR: PEC error in reception flag
	/// * I2C_FLAG_OVR: Overrun/Underrun flag (Slave mode)
	/// * I2C_FLAG_AF: Acknowledge failure flag
	/// * I2C_FLAG_ARLO: Arbitration lost flag (Master mode)
	/// * I2C_FLAG_BERR: Bus error flag
	///   
	/// @note
	///   - STOPF (STOP detection) is cleared by software sequence: a read operation 
	///     to I2C_SR1 register (I2C_GetFlagStatus()) followed by a write operation 
	///     to I2C_CR1 register (I2C_Cmd() to re-enable the I2C peripheral).
	///   - ADD10 (10-bit header sent) is cleared by software sequence: a read 
	///     operation to I2C_SR1 (I2C_GetFlagStatus()) followed by writing the 
	///     second byte of the address in DR register.
	///   - BTF (Byte Transfer Finished) is cleared by software sequence: a read 
	///     operation to I2C_SR1 register (I2C_GetFlagStatus()) followed by a 
	///     read/write to I2C_DR register (I2C_SendData()).
	///   - ADDR (Address sent) is cleared by software sequence: a read operation to 
	///     I2C_SR1 register (I2C_GetFlagStatus()) followed by a read operation to 
	///     I2C_SR2 register ((void)(I2Cx->SR2)).
	///   - SB (Start Bit) is cleared software sequence: a read operation to I2C_SR1
	///     register (I2C_GetFlagStatus()) followed by a write operation to I2C_DR
	///     register  (I2C_SendData()).
	fn ClearFlag(&self, I2C_FLAG : u32);*/

	/// Checks whether the specified I2C interrupt has occurred or not.
	/// `I2C_IT`: specifies the interrupt source to check. 
	///   This parameter can be one of the following values:
	/// * I2C_IT_SMBALERT: SMBus Alert flag
	/// * I2C_IT_TIMEOUT: Timeout or Tlow error flag
	/// * I2C_IT_PECERR: PEC error in reception flag
	/// * I2C_IT_OVR: Overrun/Underrun flag (Slave mode)
	/// * I2C_IT_AF: Acknowledge failure flag
	/// * I2C_IT_ARLO: Arbitration lost flag (Master mode)
	/// * I2C_IT_BERR: Bus error flag
	/// * I2C_IT_TXE: Data register empty flag (Transmitter)
	/// * I2C_IT_RXNE: Data register not empty (Receiver) flag
	/// * I2C_IT_STOPF: Stop detection flag (Slave mode)
	/// * I2C_IT_ADD10: 10-bit header sent flag (Master mode)
	/// * I2C_IT_BTF: Byte transfer finished flag
	/// * I2C_IT_ADDR: Address sent flag (Master mode) "ADSL"
	///                       Address matched flag (Slave mode)"ENDAD"
	/// * I2C_IT_SB: Start bit flag (Master mode)
	/// @retval The new state of I2C_IT (SET or RESET).
	fn GetITStatus(&self, I2C_IT : u32) -> ITStatus;

	/*/// Clears the I2Cx’s interrupt pending bits.
	/// `I2C_IT`: specifies the interrupt pending bit to clear. 
	///   This parameter can be any combination of the following values:
	/// * I2C_IT_SMBALERT: SMBus Alert interrupt
	/// * I2C_IT_TIMEOUT: Timeout or Tlow error interrupt
	/// * I2C_IT_PECERR: PEC error in reception  interrupt
	/// * I2C_IT_OVR: Overrun/Underrun interrupt (Slave mode)
	/// * I2C_IT_AF: Acknowledge failure interrupt
	/// * I2C_IT_ARLO: Arbitration lost interrupt (Master mode)
	/// * I2C_IT_BERR: Bus error interrupt
	///   
	/// @note
	///   - STOPF (STOP detection) is cleared by software sequence: a read operation 
	///     to I2C_SR1 register (I2C_GetITStatus()) followed by a write operation to 
	///     I2C_CR1 register (I2C_Cmd() to re-enable the I2C peripheral).
	///   - ADD10 (10-bit header sent) is cleared by software sequence: a read 
	///     operation to I2C_SR1 (I2C_GetITStatus()) followed by writing the second 
	///     byte of the address in I2C_DR register.
	///   - BTF (Byte Transfer Finished) is cleared by software sequence: a read 
	///     operation to I2C_SR1 register (I2C_GetITStatus()) followed by a 
	///     read/write to I2C_DR register (I2C_SendData()).
	///   - ADDR (Address sent) is cleared by software sequence: a read operation to 
	///     I2C_SR1 register (I2C_GetITStatus()) followed by a read operation to 
	///     I2C_SR2 register ((void)(I2Cx->SR2)).
	///   - SB (Start Bit) is cleared by software sequence: a read operation to 
	///     I2C_SR1 register (I2C_GetITStatus()) followed by a write operation to 
	///     I2C_DR register (I2C_SendData()).
	fn ClearITPendingBit(&self, I2C_IT : u32);*/
}




/// I2C_interrupts_definition 
pub mod I2cIT {
	/// I2C Interrupt Enable mask
	pub const Mask : u32 = 0x07000000;
	pub mod config {
		pub const BUF : u16 = 0x0400;
		pub const EVT : u16 = 0x0200;
		pub const ERR : u16 = 0x0100;
	}
	
	pub mod get {
		pub const SMBALERT : u32 = 0x01008000;
		pub const TIMEOUT  : u32 = 0x01004000;
		pub const PECERR   : u32 = 0x01001000;
		pub const OVR      : u32 = 0x01000800;
		pub const AF       : u32 = 0x01000400;
		pub const ARLO     : u32 = 0x01000200;
		pub const BERR     : u32 = 0x01000100;
		pub const TXE      : u32 = 0x06000080;
		pub const RXNE     : u32 = 0x06000040;
		pub const STOPF    : u32 = 0x02000010;
		pub const ADD10    : u32 = 0x02000008;
		pub const BTF      : u32 = 0x02000004;
		pub const ADDR     : u32 = 0x02000002;
		pub const SB       : u32 = 0x02000001;
	}
}

/// I2C_flags_definition 
pub mod I2cFlag {
	/* I2C FLAG mask */
	pub const Mask : u32 = 0x00FFFFFF;
	/// SR2 register flags  
	pub mod sr2 {
		pub const DUALF    : u32 = 0x00800000;
		pub const SMBHOST  : u32 = 0x00400000;
		pub const SMBDEFAULT:u32 = 0x00200000;
		pub const GENCALL  : u32 = 0x00100000;
		pub const TRA      : u32 = 0x00040000;
		pub const BUSY     : u32 = 0x00020000;
		pub const MSL      : u32 = 0x00010000;
	}
	/// SR1 register flags 
	pub mod sr1 { 
		pub const SMBALERT : u32 = 0x10008000;
		pub const TIMEOUT  : u32 = 0x10004000;
		pub const PECERR   : u32 = 0x10001000;
		pub const OVR      : u32 = 0x10000800;
		pub const AF       : u32 = 0x10000400;
		pub const ARLO     : u32 = 0x10000200;
		pub const BERR     : u32 = 0x10000100;
		pub const TXE      : u32 = 0x10000080;
		pub const RXNE     : u32 = 0x10000040;
		pub const STOPF    : u32 = 0x10000010;
		pub const ADD10    : u32 = 0x10000008;
		pub const BTF      : u32 = 0x10000004;
		pub const ADDR     : u32 = 0x10000002;
		pub const SB       : u32 = 0x10000001;
	}
}


pub mod I2cEvent {
	/// I2C Master Events (Events grouped in order of communication)
	pub mod Master {

		/// Communication start
		/// 
		/// After sending the START condition (I2C_GenerateSTART() function) the master 
		/// has to wait for this event. It means that the Start condition has been correctly 
		/// released on the I2C bus (the bus is free, no other devices is communicating).
		/* --EV5 */
		pub const MODE_SELECT : u32 = 0x00030001;  /* BUSY, MSL and SB flag */

		/// Address Acknowledge
		/// 
		/// After checking on EV5 (start condition correctly released on the bus), the 
		/// master sends the address of the slave(s) with which it will communicate 
		/// (I2C_Send7bitAddress() function, it also determines the direction of the communication: 
		/// Master transmitter or Receiver). Then the master has to wait that a slave acknowledges 
		/// his address. If an acknowledge is sent on the bus, one of the following events will 
		/// be set:
		/// 
		///  1) In case of Master Receiver (7-bit addressing): the I2C_EVENT_MASTER_RECEIVER_MODE_SELECTED 
		///     event is set.
		///  
		///  2) In case of Master Transmitter (7-bit addressing): the I2C_EVENT_MASTER_TRANSMITTER_MODE_SELECTED 
		///     is set
		///  
		///  3) In case of 10-Bit addressing mode, the master (just after generating the START 
		///  and checking on EV5) has to send the header of 10-bit addressing mode (I2C_SendData() 
		///  function). Then master should wait on EV9. It means that the 10-bit addressing 
		///  header has been correctly sent on the bus. Then master should send the second part of 
		///  the 10-bit address (LSB) using the function I2C_Send7bitAddress(). Then master 
		///  should wait for event EV6.
		/* --EV6 */
		pub const TRANSMITTER_MODE_SELECTED : u32 = 0x00070082;  /* BUSY, MSL, ADDR, TXE and TRA flags */
		pub const RECEIVER_MODE_SELECTED    : u32 = 0x00030002;  /* BUSY, MSL and ADDR flags */
		/* --EV9 */
		pub const MODE_ADDRESS10  : u32 = 0x00030008;  /* BUSY, MSL and ADD10 flags */

		/// Communication events
		/// 
		/// If a communication is established (START condition generated and slave address 
		/// acknowledged) then the master has to check on one of the following events for 
		/// communication procedures:
		///  
		/// 1) Master Receiver mode: The master has to wait on the event EV7 then to read 
		///    the data received from the slave (I2C_ReceiveData() function).
		/// 
		/// 2) Master Transmitter mode: The master has to send data (I2C_SendData() 
		///    function) then to wait on event EV8 or EV8_2.
		///    These two events are similar: 
		///     - EV8 means that the data has been written in the data register and is 
		///       being shifted out.
		///     - EV8_2 means that the data has been physically shifted out and output 
		///       on the bus.
		///     In most cases, using EV8 is sufficient for the application.
		///     Using EV8_2 leads to a slower communication but ensure more reliable test.
		///     EV8_2 is also more suitable than EV8 for testing on the last data transmission 
		///     (before Stop condition generation).
		///     
		///  @note In case the  user software does not guarantee that this event EV7 is 
		///  managed before the current byte end of transfer, then user may check on EV7 
		///  and BTF flag at the same time (ie. (I2C_EVENT_MASTER_BYTE_RECEIVED | I2C_FLAG_BTF)).
		///  In this case the communication may be slower.
		/// 

		/* Master RECEIVER mode -----------------------------*/ 
		/* --EV7 */
		pub const BYTE_RECEIVED  : u32 = 0x00030040;  /* BUSY, MSL and RXNE flags */

		/* Master TRANSMITTER mode --------------------------*/
		/* --EV8 */
		pub const BYTE_TRANSMITTING  : u32 = 0x00070080; /* TRA, BUSY, MSL, TXE flags */
		/* --EV8_2 */
		pub const BYTE_TRANSMITTED  : u32 = 0x00070084;  /* TRA, BUSY, MSL, TXE and BTF flags */
	}
	
	/// I2C Slave Events (Events grouped in order of communication)
	pub mod Slave {

		/// Communication start events
		/// 
		/// Wait on one of these events at the start of the communication. It means that 
		/// the I2C peripheral detected a Start condition on the bus (generated by master 
		/// device) followed by the peripheral address. The peripheral generates an ACK 
		/// condition on the bus (if the acknowledge feature is enabled through function 
		/// I2C_AcknowledgeConfig()) and the events listed above are set :
		///  
		/// 1) In normal case (only one address managed by the slave), when the address 
		///   sent by the master matches the own address of the peripheral (configured by 
		///   I2C_OwnAddress1 field) the I2C_EVENT_SLAVE_XXX_ADDRESS_MATCHED event is set 
		///   (where XXX could be TRANSMITTER or RECEIVER).
		///    
		/// 2) In case the address sent by the master matches the second address of the 
		///   peripheral (configured by the function I2C_OwnAddress2Config() and enabled 
		///   by the function I2C_DualAddressCmd()) the events I2C_EVENT_SLAVE_XXX_SECONDADDRESS_MATCHED 
		///   (where XXX could be TRANSMITTER or RECEIVER) are set.
		///   
		/// 3) In case the address sent by the master is General Call (address 0x00) and 
		///   if the General Call is enabled for the peripheral (using function I2C_GeneralCallCmd()) 
		///   the following event is set I2C_EVENT_SLAVE_GENERALCALLADDRESS_MATCHED.   
		/* --EV1  (all the events below are variants of EV1) */   
		/* 1) Case of One Single Address managed by the slave */
		pub const RECEIVER_ADDRESS_MATCHED  : u32 = 0x00020002; /* BUSY and ADDR flags */
		pub const TRANSMITTER_ADDRESS_MATCHED  : u32 = 0x00060082; /* TRA, BUSY, TXE and ADDR flags */

		/* 2) Case of Dual address managed by the slave */
		pub const RECEIVER_SECONDADDRESS_MATCHED  : u32 = 0x00820000;  /* DUALF and BUSY flags */
		pub const TRANSMITTER_SECONDADDRESS_MATCHED  : u32 = 0x00860080;  /* DUALF, TRA, BUSY and TXE flags */

		/* 3) Case of General Call enabled for the slave */
		pub const GENERALCALLADDRESS_MATCHED  : u32 = 0x00120000;  /* GENCALL and BUSY flags */
		 
		/// Communication events
		/// 
		/// Wait on one of these events when EV1 has already been checked and: 
		/// 
		/// - Slave RECEIVER mode:
		///     - EV2: When the application is expecting a data byte to be received. 
		///     - EV4: When the application is expecting the end of the communication: master 
		///       sends a stop condition and data transmission is stopped.
		///    
		/// - Slave Transmitter mode:
		///    - EV3: When a byte has been transmitted by the slave and the application is expecting 
		///      the end of the byte transmission. The two events I2C_EVENT_SLAVE_BYTE_TRANSMITTED and
		///      I2C_EVENT_SLAVE_BYTE_TRANSMITTING are similar. The second one can optionally be 
		///      used when the user software doesn't guarantee the EV3 is managed before the
		///      current byte end of transfer.
		///    - EV3_2: When the master sends a NACK in order to tell slave that data transmission 
		///      shall end (before sending the STOP condition). In this case slave has to stop sending 
		///      data bytes and expect a Stop condition on the bus.
		///      
		///  @note In case the  user software does not guarantee that the event EV2 is 
		///  managed before the current byte end of transfer, then user may check on EV2 
		///  and BTF flag at the same time (ie. (I2C_EVENT_SLAVE_BYTE_RECEIVED | I2C_FLAG_BTF)).
		/// In this case the communication may be slower.
		/* Slave RECEIVER mode --------------------------*/ 
		/* --EV2 */
		pub const BYTE_RECEIVED  : u32 = 0x00020040;  /* BUSY and RXNE flags */
		/* --EV4  */
		pub const STOP_DETECTED  : u32 = 0x00000010;  /* STOPF flag */

		/* Slave TRANSMITTER mode -----------------------*/
		/* --EV3 */
		pub const BYTE_TRANSMITTED  : u32 = 0x00060084;  /* TRA, BUSY, TXE and BTF flags */
		pub const BYTE_TRANSMITTING  : u32 = 0x00060080;  /* TRA, BUSY and TXE flags */
		/* --EV3_2 */
		pub const ACK_FAILURE  : u32 = 0x00000400;  /* AF flag */
	}
	
}





impl<I : Deref<Target = i2c1::RegisterBlock>> I2cControl for I {
	fn ITConfig(&self, I2C_IT : u16, bool) {
		self.cr2      .modify(|r,w| unsafe { w.bits (
			if new_state { /* Enable the selected I2C interrupts */
				r.bits() | (I2C_IT as u32)
			} else { /* Disable the selected I2C interrupts */
				r.bits() & (!I2C_IT as u32)
			}
		)});
	}
	
	fn CheckEvent(&self, I2C_EVENT : u32) -> ErrorStatus {
		let mut status = ErrorStatus::ERROR;

		/* Read the I2Cx status register */
		let flag1 = self.sr1.read().bits();
		let flag2 = self.sr2.read().bits() << 16;

		/* Get the last event value from I2C status register */
		let lastevent = (flag1 | flag2) & I2cFlag::Mask;

		/* Check whether the last event contains the I2C_EVENT */
		((lastevent & I2C_EVENT) == I2C_EVENT).into()
	}
	
	fn GetLastEvent(&self) -> u32 {
		/* Read the I2Cx status register */
		let flag1 = self.sr1.read().bits();
		let flag2 = self.sr2.read().bits() << 16;

		/* Get the last event value from I2C status register */
		(flag1 | flag2) & I2cFlag::Mask
	}
	
	/*fn GetFlagStatus(&self, I2C_FLAG : u32) -> FlagStatus {
	  let mut bitstatus = FlagStatus::RESET;
	  __IO uint32_t i2creg = 0, i2cxbase = 0;

	  /* Check the parameters */
	  assert_param(IS_I2C_ALL_PERIPH(I2Cx));
	  assert_param(IS_I2C_GET_FLAG(I2C_FLAG));

	  /* Get the I2Cx peripheral base address */
	  i2cxbase = (uint32_t)I2Cx;
	  
	  /* Read flag register index */
	  i2creg = I2C_FLAG >> 28;
	  
	  /* Get bit[23:0] of the flag */
	  I2C_FLAG &= I2cFlag::Mask;
	  
	  if i2creg != 0 {
		/* Get the I2Cx SR1 register address */
		i2cxbase += 0x14;
	  } else {
		/* Flag in I2Cx SR2 Register */
		I2C_FLAG = (uint32_t)(I2C_FLAG >> 16);
		/* Get the I2Cx SR2 register address */
		i2cxbase += 0x18;
	  }
	  
	  if(((*(__IO uint32_t *)i2cxbase) & I2C_FLAG) != (uint32_t)RESET) {
		/* I2C_FLAG is set */
		bitstatus = FlagStatus::SET;
	  } else {
		/* I2C_FLAG is reset */
		bitstatus = FlagStatus::RESET;
	  }
	  
	  /* Return the I2C_FLAG status */
	  return  bitstatus;
	}

	fn ClearFlag(&self, I2C_FLAG : u32) {
		/* Get the I2C flag position */
		let flagpos = I2C_FLAG & I2cFlag::Mask;
		/* Clear the selected I2C flag */
		self.sr1         .write(|w| unsafe { w
			.bits( (!flagpos) as u16 )
		});
	}*/

	fn GetITStatus(&self, I2C_IT : u32) -> ITStatus {
		/* Check if the interrupt source is enabled or not */
		let enablestatus = ( ((I2C_IT & I2cIT::Mask) >> 16) & self.cr2.read().bits() ) != 0 ;
		
		/* Get bit[23:0] of the flag */
		I2C_IT &= I2cFlag::Mask;
		
		/* Check the status of the specified I2C flag */
		(((self.sr1.read().bits() & I2C_IT) != (ITStatus::RESET as u32)) && enablestatus).into()
	}
	
	/*fn ClearITPendingBit(&self, I2C_IT : u32) {
		/* Get the I2C flag position */
		let flagpos = I2C_IT & I2cFlag::Mask;
		/* Clear the selected I2C flag */
		self.sr1         .write(|w| unsafe { w
			.bits( (!flagpos) as u16 )
		});
	}*/
}

