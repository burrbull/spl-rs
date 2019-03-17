

/// USART_Interrupt_definition 
  
#define USART_IT_PE                          ((uint16_t)0x0028)
#define USART_IT_TXE                         ((uint16_t)0x0727)
#define USART_IT_TC                          ((uint16_t)0x0626)
#define USART_IT_RXNE                        ((uint16_t)0x0525)
#define USART_IT_IDLE                        ((uint16_t)0x0424)
#define USART_IT_LBD                         ((uint16_t)0x0846)
#define USART_IT_CTS                         ((uint16_t)0x096A)
#define USART_IT_ERR                         ((uint16_t)0x0060)
#define USART_IT_ORE                         ((uint16_t)0x0360)
#define USART_IT_NE                          ((uint16_t)0x0260)
#define USART_IT_FE                          ((uint16_t)0x0160)

/// USART_Flags 

#define USART_FLAG_CTS                       ((uint16_t)0x0200)
#define USART_FLAG_LBD                       ((uint16_t)0x0100)
#define USART_FLAG_TXE                       ((uint16_t)0x0080)
#define USART_FLAG_TC                        ((uint16_t)0x0040)
#define USART_FLAG_RXNE                      ((uint16_t)0x0020)
#define USART_FLAG_IDLE                      ((uint16_t)0x0010)
#define USART_FLAG_ORE                       ((uint16_t)0x0008)
#define USART_FLAG_NE                        ((uint16_t)0x0004)
#define USART_FLAG_FE                        ((uint16_t)0x0002)
#define USART_FLAG_PE                        ((uint16_t)0x0001)



void USART_ITConfig(USART_TypeDef* USARTx, uint16_t USART_IT, FunctionalState NewState);
FlagStatus USART_GetFlagStatus(USART_TypeDef* USARTx, uint16_t USART_FLAG);
void USART_ClearFlag(USART_TypeDef* USARTx, uint16_t USART_FLAG);
ITStatus USART_GetITStatus(USART_TypeDef* USARTx, uint16_t USART_IT);
void USART_ClearITPendingBit(USART_TypeDef* USARTx, uint16_t USART_IT);


/// Enables or disables the specified USART interrupts.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_IT`: specifies the USART interrupt sources to be
///   enabled or disabled.
///   This parameter can be one of the following values:
/// * USART_IT_CTS:  CTS change interrupt (not available for
///   UART4 and UART5)
/// * USART_IT_LBD:  LIN Break detection interrupt
/// * USART_IT_TXE:  Tansmit Data Register empty interrupt
/// * USART_IT_TC:   Transmission complete interrupt
/// * USART_IT_RXNE: Receive Data register not empty 
///   interrupt
/// * USART_IT_IDLE: Idle line detection interrupt
/// * USART_IT_PE:   Parity Error interrupt
/// * USART_IT_ERR:  Error interrupt(Frame error, noise
///   error, overrun error)
/// `NewState`: new state of the specified USARTx interrupts.
///   This parameter can be: ENABLE or DISABLE.
void USART_ITConfig(USART_TypeDef* USARTx, uint16_t USART_IT, FunctionalState NewState) {
  uint32_t usartreg = 0x00, itpos = 0x00, itmask = 0x00;
  uint32_t usartxbase = 0x00;
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  assert_param(IS_USART_PERIPH_IT(USARTx, USART_IT)); /* The CTS interrupt is not available for UART4 and UART5 */
  usartxbase = (*(uint32_t*)&(USARTx));
  /* Get the USART register index */
  usartreg = (((uint8_t)USART_IT) >> 0x05);
  /* Get the interrupt position */
  itpos = USART_IT & IT_Mask;
  itmask = (((uint32_t)0x01) << itpos);
    
  if (usartreg == 0x01) /* The IT is in CR1 register */ {
    usartxbase += 0x0C;
  } else if (usartreg == 0x02) /* The IT is in CR2 register */ {
    usartxbase += 0x10;
  } else /* The IT is in CR3 register */ {
    usartxbase += 0x14; 
  }
  if (NewState != DISABLE) {
    *(__IO uint32_t*)usartxbase  |= itmask;
  } else {
    *(__IO uint32_t*)usartxbase &= ~itmask;
  }
}

/// Checks whether the specified USART flag is set or not.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_FLAG`: specifies the flag to check.
///   This parameter can be one of the following values:
/// * USART_FLAG_CTS:  CTS Change flag (not available for UART4 and UART5)
/// * USART_FLAG_LBD:  LIN Break detection flag
/// * USART_FLAG_TXE:  Transmit data register empty flag
/// * USART_FLAG_TC:   Transmission Complete flag
/// * USART_FLAG_RXNE: Receive data register not empty flag
/// * USART_FLAG_IDLE: Idle Line detection flag
/// * USART_FLAG_ORE:  OverRun Error flag
/// * USART_FLAG_NE:   Noise Error flag
/// * USART_FLAG_FE:   Framing Error flag
/// * USART_FLAG_PE:   Parity Error flag
/// @retval : The new state of USART_FLAG (SET or RESET).
FlagStatus USART_GetFlagStatus(USART_TypeDef* USARTx, uint16_t USART_FLAG) {
  FlagStatus bitstatus = RESET;
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  assert_param(IS_USART_PERIPH_FLAG(USARTx, USART_FLAG)); /* The CTS flag is not available for UART4 and UART5 */   
  if ((USARTx->SR & USART_FLAG) != (uint16_t)RESET) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the USARTx's pending flags.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_FLAG`: specifies the flag to clear.
///   This parameter can be any combination of the following values:
/// * USART_FLAG_CTS:  CTS Change flag (not available for UART4 and UART5).
/// * USART_FLAG_LBD:  LIN Break detection flag.
/// * USART_FLAG_TC:   Transmission Complete flag.
/// * USART_FLAG_RXNE: Receive data register not empty flag.
///   
/// @note
///   - PE (Parity error), FE (Framing error), NE (Noise error), ORE (OverRun 
///     error) and IDLE (Idle line detected) flags are cleared by software 
///     sequence: a read operation to USART_SR register (USART_GetFlagStatus()) 
///     followed by a read operation to USART_DR register (USART_ReceiveData()).
///   - RXNE flag can be also cleared by a read to the USART_DR register 
///     (USART_ReceiveData()).
///   - TC flag can be also cleared by software sequence: a read operation to 
///     USART_SR register (USART_GetFlagStatus()) followed by a write operation
///     to USART_DR register (USART_SendData()).
///   - TXE flag is cleared only by a write to the USART_DR register 
///     (USART_SendData()).
void USART_ClearFlag(USART_TypeDef* USARTx, uint16_t USART_FLAG) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  assert_param(IS_USART_PERIPH_FLAG(USARTx, USART_FLAG)); /* The CTS flag is not available for UART4 and UART5 */   
   
  USARTx->SR = (uint16_t)~USART_FLAG;
}

/// Checks whether the specified USART interrupt has occurred or not.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_IT`: specifies the USART interrupt source to check.
///   This parameter can be one of the following values:
/// * USART_IT_CTS:  CTS change interrupt (not available for UART4 and UART5)
/// * USART_IT_LBD:  LIN Break detection interrupt
/// * USART_IT_TXE:  Tansmit Data Register empty interrupt
/// * USART_IT_TC:   Transmission complete interrupt
/// * USART_IT_RXNE: Receive Data register not empty interrupt
/// * USART_IT_IDLE: Idle line detection interrupt
/// * USART_IT_ORE:  OverRun Error interrupt
/// * USART_IT_NE:   Noise Error interrupt
/// * USART_IT_FE:   Framing Error interrupt
/// * USART_IT_PE:   Parity Error interrupt
/// @retval : The new state of USART_IT (SET or RESET).
ITStatus USART_GetITStatus(USART_TypeDef* USARTx, uint16_t USART_IT) {
  uint32_t bitpos = 0x00, itmask = 0x00, usartreg = 0x00;
  ITStatus bitstatus = RESET;
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  assert_param(IS_USART_PERIPH_IT(USARTx, USART_IT)); /* The CTS interrupt is not available for UART4 and UART5 */  
  
  /* Get the USART register index */
  usartreg = (((uint8_t)USART_IT) >> 0x05);
  /* Get the interrupt position */
  itmask = USART_IT & IT_Mask;
  itmask = (uint32_t)0x01 << itmask;
  
  if (usartreg == 0x01) /* The IT  is in CR1 register */ {
    itmask &= USARTx->CR1;
  } else if (usartreg == 0x02) /* The IT  is in CR2 register */ {
    itmask &= USARTx->CR2;
  } else /* The IT  is in CR3 register */ {
    itmask &= USARTx->CR3;
  }
  
  bitpos = USART_IT >> 0x08;
  bitpos = (uint32_t)0x01 << bitpos;
  bitpos &= USARTx->SR;
  if ((itmask != (uint16_t)RESET)&&(bitpos != (uint16_t)RESET)) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  
  return bitstatus;  
}

/// Clears the USARTx’s interrupt pending bits.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_IT`: specifies the interrupt pending bit to clear.
///   This parameter can be one of the following values:
/// * USART_IT_CTS:  CTS change interrupt (not available for UART4 and UART5)
/// * USART_IT_LBD:  LIN Break detection interrupt
/// * USART_IT_TC:   Transmission complete interrupt. 
/// * USART_IT_RXNE: Receive Data register not empty interrupt.
///   
/// @note
///   - PE (Parity error), FE (Framing error), NE (Noise error), ORE (OverRun 
///     error) and IDLE (Idle line detected) pending bits are cleared by 
///     software sequence: a read operation to USART_SR register 
///     (USART_GetITStatus()) followed by a read operation to USART_DR register 
///     (USART_ReceiveData()).
///   - RXNE pending bit can be also cleared by a read to the USART_DR register 
///     (USART_ReceiveData()).
///   - TC pending bit can be also cleared by software sequence: a read 
///     operation to USART_SR register (USART_GetITStatus()) followed by a write 
///     operation to USART_DR register (USART_SendData()).
///   - TXE pending bit is cleared only by a write to the USART_DR register 
///     (USART_SendData()).
void USART_ClearITPendingBit(USART_TypeDef* USARTx, uint16_t USART_IT) {
  uint16_t bitpos = 0x00, itmask = 0x00;
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  assert_param(IS_USART_PERIPH_IT(USARTx, USART_IT)); /* The CTS interrupt is not available for UART4 and UART5 */
  
  bitpos = USART_IT >> 0x08;
  itmask = (uint16_t)((uint16_t)0x01 << bitpos);
  USARTx->SR = (uint16_t)~itmask;
}
