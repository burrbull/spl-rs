

/// Enables or disables the specified SPI/I2S interrupts.
///   2 or 3 in I2S mode
/// `SPI_I2S_IT`: specifies the SPI/I2S interrupt source to be 
///   enabled or disabled. 
///   This parameter can be one of the following values:
/// * SPI_I2S_IT_TXE: Tx buffer empty interrupt mask
/// * SPI_I2S_IT_RXNE: Rx buffer not empty interrupt mask
/// * SPI_I2S_IT_ERR: Error interrupt mask
/// `new_state`: new state of the specified SPI/I2S interrupt.
///   This parameter can be: ENABLE or DISABLE.
fn I2S_ITConfig(&self, uint8_t SPI_I2S_IT, new_state : FunctionalState) {
  uint16_t itpos = 0, itmask = 0 ;
  /* Get the SPI/I2S IT index */
  itpos = SPI_I2S_IT >> 4;
  /* Set the IT mask */
  itmask = (uint16_t)((uint16_t)1 << itpos);
  if (new_state != DISABLE) {
    /* Enable the selected SPI/I2S interrupt */
    SPIx->CR2 |= itmask;
  }
  else {
    /* Disable the selected SPI/I2S interrupt */
    SPIx->CR2 &= (uint16_t)~itmask;
  }
}



/// SPI_I2S_interrupts_definition
pub mod SpiI2sIT {
	pub const TXE    : u8 = 0x71;
	pub const RXNE   : u8 = 0x60;
	pub const ERR    : u8 = 0x50;
	pub const OVR    : u8 = 0x56;
}
pub mod SpiIT {
	pub const MODF   : u8 = 0x55;
	pub const CRCERR : u8 = 0x54;
}
#define I2S_IT_UDR                      ((uint8_t)0x53)
/**
/// @}
  */

/** @defgroup SPI_I2S_flags_definition 
/// @{
  */

#define SPI_I2S_FLAG_RXNE               ((uint16_t)0x0001)
#define SPI_I2S_FLAG_TXE                ((uint16_t)0x0002)
#define I2S_FLAG_CHSIDE                 ((uint16_t)0x0004)
#define I2S_FLAG_UDR                    ((uint16_t)0x0008)
#define SPI_FLAG_CRCERR                 ((uint16_t)0x0010)
#define SPI_FLAG_MODF                   ((uint16_t)0x0020)
#define SPI_I2S_FLAG_OVR                ((uint16_t)0x0040)
#define SPI_I2S_FLAG_BSY                ((uint16_t)0x0080)


/// Checks whether the specified SPI/I2S flag is set or not.
///   2 or 3 in I2S mode
/// `SPI_I2S_FLAG`: specifies the SPI/I2S flag to check. 
///   This parameter can be one of the following values:
/// * SPI_I2S_FLAG_TXE: Transmit buffer empty flag.
/// * SPI_I2S_FLAG_RXNE: Receive buffer not empty flag.
/// * SPI_I2S_FLAG_BSY: Busy flag.
/// * SPI_I2S_FLAG_OVR: Overrun flag.
/// * SPI_FLAG_MODF: Mode Fault flag.
/// * SPI_FLAG_CRCERR: CRC Error flag.
/// * I2S_FLAG_UDR: Underrun Error flag.
/// * I2S_FLAG_CHSIDE: Channel Side flag.
/// @retval : The new state of SPI_I2S_FLAG (SET or RESET).
FlagStatus SPI_I2S_GetFlagStatus(&self, uint16_t SPI_I2S_FLAG) {
  FlagStatus bitstatus = RESET;
  /* Check the status of the specified SPI/I2S flag */
  if ((SPIx->SR & SPI_I2S_FLAG) != (uint16_t)RESET) {
    /* SPI_I2S_FLAG is set */
    bitstatus = SET;
  }
  else {
    /* SPI_I2S_FLAG is reset */
    bitstatus = RESET;
  }
  /* Return the SPI_I2S_FLAG status */
  return  bitstatus;
}

/// Clears the SPIx CRC Error (CRCERR) flag.
/// `SPI_I2S_FLAG`: specifies the SPI flag to clear. 
///   This function clears only CRCERR flag.
/// @note
///   - OVR (OverRun error) flag is cleared by software sequence: a read 
///     operation to SPI_DR register (SPI_I2S_ReceiveData()) followed by a read 
///     operation to SPI_SR register (SPI_I2S_GetFlagStatus()).
///   - UDR (UnderRun error) flag is cleared by a read operation to 
///     SPI_SR register (SPI_I2S_GetFlagStatus()).
///   - MODF (Mode Fault) flag is cleared by software sequence: a read/write 
///     operation to SPI_SR register (SPI_I2S_GetFlagStatus()) followed by a 
///     write operation to SPI_CR1 register (SPI_Cmd() to enable the SPI).
fn I2S_ClearFlag(&self, uint16_t SPI_I2S_FLAG) {
    
    /* Clear the selected SPI CRC Error (CRCERR) flag */
    SPIx->SR = (uint16_t)~SPI_I2S_FLAG;
}

/// Checks whether the specified SPI/I2S interrupt has occurred or not.
///   2 or 3 in I2S mode
/// `SPI_I2S_IT`: specifies the SPI/I2S interrupt source to check. 
///   This parameter can be one of the following values:
/// * SPI_I2S_IT_TXE: Transmit buffer empty interrupt.
/// * SPI_I2S_IT_RXNE: Receive buffer not empty interrupt.
/// * SPI_I2S_IT_OVR: Overrun interrupt.
/// * SPI_IT_MODF: Mode Fault interrupt.
/// * SPI_IT_CRCERR: CRC Error interrupt.
/// * I2S_IT_UDR: Underrun Error interrupt.
/// @retval : The new state of SPI_I2S_IT (SET or RESET).
ITStatus SPI_I2S_GetITStatus(&self, uint8_t SPI_I2S_IT) {
  ITStatus bitstatus = RESET;
  uint16_t itpos = 0, itmask = 0, enablestatus = 0;
  /* Get the SPI/I2S IT index */
  itpos = (uint16_t)((uint16_t)0x01 << (SPI_I2S_IT & (uint8_t)0x0F));
  /* Get the SPI/I2S IT mask */
  itmask = SPI_I2S_IT >> 4;
  /* Set the IT mask */
  itmask = (uint16_t)((uint16_t)0x01 << itmask);
  /* Get the SPI_I2S_IT enable bit status */
  enablestatus = (SPIx->CR2 & itmask) ;
  /* Check the status of the specified SPI/I2S interrupt */
  if (((SPIx->SR & itpos) != (uint16_t)RESET) && enablestatus) {
    /* SPI_I2S_IT is set */
    bitstatus = SET;
  }
  else {
    /* SPI_I2S_IT is reset */
    bitstatus = RESET;
  }
  /* Return the SPI_I2S_IT status */
  return bitstatus;
}

/// Clears the SPIx CRC Error (CRCERR) interrupt pending bit.
/// `SPI_I2S_IT`: specifies the SPI interrupt pending bit to clear.
///   This function clears only CRCERR intetrrupt pending bit.   
/// @note
///   - OVR (OverRun Error) interrupt pending bit is cleared by software 
///     sequence: a read operation to SPI_DR register (SPI_I2S_ReceiveData()) 
///     followed by a read operation to SPI_SR register (SPI_I2S_GetITStatus()).
///   - UDR (UnderRun Error) interrupt pending bit is cleared by a read 
///     operation to SPI_SR register (SPI_I2S_GetITStatus()).
///   - MODF (Mode Fault) interrupt pending bit is cleared by software sequence:
///     a read/write operation to SPI_SR register (SPI_I2S_GetITStatus()) 
///     followed by a write operation to SPI_CR1 register (SPI_Cmd() to enable 
///     the SPI).
fn I2S_ClearITPendingBit(&self, uint8_t SPI_I2S_IT) {
  uint16_t itpos = 0;
  /* Get the SPI IT index */
  itpos = (uint16_t)((uint16_t)0x01 << (SPI_I2S_IT & (uint8_t)0x0F));
  /* Clear the selected SPI CRC Error (CRCERR) interrupt pending bit */
  SPIx->SR = (uint16_t)~itpos;
}
