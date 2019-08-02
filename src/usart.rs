

/// USART Init Structure definition   
  
typedef struct {
  uint32_t USART_BaudRate;
  uint16_t USART_WordLength;
  uint16_t USART_StopBits;
  uint16_t USART_Parity;
  uint16_t USART_Mode;
  uint16_t USART_HardwareFlowControl;  
} USART_InitTypeDef;



/// Fills each USART_InitStruct member with its default value.
/// `USART_InitStruct`: pointer to a USART_InitTypeDef structure
///   which will be initialized.
fn StructInit(USART_InitTypeDef* USART_InitStruct) {
  /* USART_InitStruct members default value */
  USART_InitStruct->USART_BaudRate = 9600;
  USART_InitStruct->USART_WordLength = USART_WordLength_8b;
  USART_InitStruct->USART_StopBits = USART_StopBits_1;
  USART_InitStruct->USART_Parity = USART_Parity_No ;
  USART_InitStruct->USART_Mode = USART_Mode_Rx | USART_Mode_Tx;
  USART_InitStruct->USART_HardwareFlowControl = USART_HardwareFlowControl_None;  
}


/// USART Clock Init Structure definition   
  
typedef struct {
  uint16_t USART_Clock;
  uint16_t USART_CPOL;
  uint16_t USART_CPHA;
  uint16_t USART_LastBit;
} USART_ClockInitTypeDef;
 

/// Initializes the USARTx peripheral Clock according to the 
///   specified parameters in the USART_ClockInitStruct .
/// `USARTx`: where x can be 1, 2, 3 to select the USART peripheral.
/// `USART_ClockInitStruct`: pointer to a USART_ClockInitTypeDef
///   structure that contains the configuration information for the specified 
///   USART peripheral.  
/// @note The Smart Card mode is not available for UART4 and UART5.
fn ClockInit(&self, USART_ClockInitTypeDef* USART_ClockInitStruct) {
  uint32_t tmpreg = 0x00;
  /* Check the parameters */
  assert_param(IS_USART_123_PERIPH(USARTx));
  
/*---------------------------- USART CR2 Configuration -----------------------*/
  tmpreg = USARTx->CR2;
  /* Clear CLKEN, CPOL, CPHA and LBCL bits */
  tmpreg &= CR2_CLOCK_CLEAR_Mask;
  /* Configure the USART Clock, CPOL, CPHA and LastBit ------------*/
  /* Set CLKEN bit according to USART_Clock value */
  /* Set CPOL bit according to USART_CPOL value */
  /* Set CPHA bit according to USART_CPHA value */
  /* Set LBCL bit according to USART_LastBit value */
  tmpreg |= (uint32_t)USART_ClockInitStruct->USART_Clock | USART_ClockInitStruct->USART_CPOL | 
                 USART_ClockInitStruct->USART_CPHA | USART_ClockInitStruct->USART_LastBit;
  /* Write to USART CR2 */
  USARTx->CR2 = (uint16_t)tmpreg;
}

/// USART_Exported_Constants 
  
#define IS_USART_ALL_PERIPH(PERIPH) (((*(uint32_t*)&(PERIPH)) == USART1_BASE) || \
                                     ((*(uint32_t*)&(PERIPH)) == USART2_BASE) || \
                                     ((*(uint32_t*)&(PERIPH)) == USART3_BASE) || \
                                     ((*(uint32_t*)&(PERIPH)) == UART4_BASE) || \
                                     ((*(uint32_t*)&(PERIPH)) == UART5_BASE))
#define IS_USART_123_PERIPH(PERIPH) (((*(uint32_t*)&(PERIPH)) == USART1_BASE) || \
                                     ((*(uint32_t*)&(PERIPH)) == USART2_BASE) || \
                                     ((*(uint32_t*)&(PERIPH)) == USART3_BASE))
#define IS_USART_1234_PERIPH(PERIPH) (((*(uint32_t*)&(PERIPH)) == USART1_BASE) || \
                                      ((*(uint32_t*)&(PERIPH)) == USART2_BASE) || \
                                      ((*(uint32_t*)&(PERIPH)) == USART3_BASE) || \
                                      ((*(uint32_t*)&(PERIPH)) == UART4_BASE))
/// USART_Word_Length  
  
#define USART_WordLength_8b                  ((uint16_t)0x0000)
#define USART_WordLength_9b                  ((uint16_t)0x1000)

/// USART_Stop_Bits  
  
#define USART_StopBits_1                     ((uint16_t)0x0000)
#define USART_StopBits_0_5                   ((uint16_t)0x1000)
#define USART_StopBits_2                     ((uint16_t)0x2000)
#define USART_StopBits_1_5                   ((uint16_t)0x3000) 

/// USART_Parity  
  
#define USART_Parity_No                      ((uint16_t)0x0000)
#define USART_Parity_Even                    ((uint16_t)0x0400)
#define USART_Parity_Odd                     ((uint16_t)0x0600)  

/// USART_Mode  
  
#define USART_Mode_Rx                        ((uint16_t)0x0004)
#define USART_Mode_Tx                        ((uint16_t)0x0008)

/// USART_Hardware_Flow_Control  
#define USART_HardwareFlowControl_None       ((uint16_t)0x0000)
#define USART_HardwareFlowControl_RTS        ((uint16_t)0x0100)
#define USART_HardwareFlowControl_CTS        ((uint16_t)0x0200)
#define USART_HardwareFlowControl_RTS_CTS    ((uint16_t)0x0300)

#define IS_USART_PERIPH_HFC(PERIPH, HFC) ((((*(uint32_t*)&(PERIPH)) != UART4_BASE) && \
                                          ((*(uint32_t*)&(PERIPH)) != UART5_BASE)) \
                                          || ((HFC) == USART_HardwareFlowControl_None)) 

/// USART_Clock  
#define USART_Clock_Disable                  ((uint16_t)0x0000)
#define USART_Clock_Enable                   ((uint16_t)0x0800)

/// USART_Clock_Polarity 
  
#define USART_CPOL_Low                       ((uint16_t)0x0000)
#define USART_CPOL_High                      ((uint16_t)0x0400)
 

/// USART_Clock_Phase

#define USART_CPHA_1Edge                     ((uint16_t)0x0000)
#define USART_CPHA_2Edge                     ((uint16_t)0x0200)


/// USART_Last_Bit

#define USART_LastBit_Disable                ((uint16_t)0x0000)
#define USART_LastBit_Enable                 ((uint16_t)0x0100)


/// USART_DMA_Requests 

#define USART_DMAReq_Tx                      ((uint16_t)0x0080)
#define USART_DMAReq_Rx                      ((uint16_t)0x0040)
#define IS_USART_DMAREQ(DMAREQ) ((((DMAREQ) & (uint16_t)0xFF3F) == 0x00) && ((DMAREQ) != (uint16_t)0x00))
 

/// USART_WakeUp_methods

#define USART_WakeUp_IdleLine                ((uint16_t)0x0000)
#define USART_WakeUp_AddressMark             ((uint16_t)0x0800)

/// USART_LIN_Break_Detection_Length 
  
#define USART_LINBreakDetectLength_10b      ((uint16_t)0x0000)
#define USART_LINBreakDetectLength_11b      ((uint16_t)0x0020)

/// USART_IrDA_Low_Power 

#define USART_IrDAMode_LowPower              ((uint16_t)0x0004)
#define USART_IrDAMode_Normal                ((uint16_t)0x0000) 




fn DeInit(&self);
fn Init(&self, USART_InitTypeDef* USART_InitStruct);
fn StructInit(USART_InitTypeDef* USART_InitStruct);
fn ClockInit(&self, USART_ClockInitTypeDef* USART_ClockInitStruct);
fn ClockStructInit(USART_ClockInitTypeDef* USART_ClockInitStruct);
fn Cmd(&self, new_state : bool);
fn DMACmd(&self, uint16_t USART_DMAReq, new_state : bool);
fn SetAddress(&self, uint8_t USART_Address);
fn WakeUpConfig(&self, uint16_t USART_WakeUp);
fn ReceiverWakeUpCmd(&self, new_state : bool);
fn LINBreakDetectLengthConfig(&self, uint16_t USART_LINBreakDetectLength);
fn LINCmd(&self, new_state : bool);
fn SendData(&self, uint16_t Data);
uint16_t USART_ReceiveData(&self);
fn SendBreak(&self);
fn SetGuardTime(&self, uint8_t USART_GuardTime);
fn SetPrescaler(&self, uint8_t USART_Prescaler);
fn SmartCardCmd(&self, new_state : bool);
fn SmartCardNACKCmd(&self, new_state : bool);
fn HalfDuplexCmd(&self, new_state : bool);
fn IrDAConfig(&self, uint16_t USART_IrDAMode);
fn IrDACmd(&self, new_state : bool);




/* USART UE Mask */
#define CR1_UE_Set                ((uint16_t)0x2000)  /* USART Enable Mask */
#define CR1_UE_Reset              ((uint16_t)0xDFFF)  /* USART Disable Mask */

/* USART WakeUp Method  */
#define CR1_WAKE_Mask             ((uint16_t)0xF7FF)  /* USART WakeUp Method Mask */

/* USART RWU Mask */
#define CR1_RWU_Set               ((uint16_t)0x0002)  /* USART mute mode Enable Mask */
#define CR1_RWU_Reset             ((uint16_t)0xFFFD)  /* USART mute mode Enable Mask */
#define CR1_SBK_Set               ((uint16_t)0x0001)  /* USART Break Character send Mask */
#define CR1_CLEAR_Mask            ((uint16_t)0xE9F3)  /* USART CR1 Mask */
#define CR2_Address_Mask          ((uint16_t)0xFFF0)  /* USART address Mask */

/* USART LIN Mask */
#define CR2_LINEN_Set              ((uint16_t)0x4000)  /* USART LIN Enable Mask */
#define CR2_LINEN_Reset            ((uint16_t)0xBFFF)  /* USART LIN Disable Mask */

/* USART LIN Break detection */
#define CR2_LBDL_Mask             ((uint16_t)0xFFDF)  /* USART LIN Break detection Mask */
#define CR2_STOP_CLEAR_Mask       ((uint16_t)0xCFFF)  /* USART CR2 STOP Bits Mask */
#define CR2_CLOCK_CLEAR_Mask      ((uint16_t)0xF0FF)  /* USART CR2 Clock Mask */

/* USART SC Mask */
#define CR3_SCEN_Set              ((uint16_t)0x0020)  /* USART SC Enable Mask */
#define CR3_SCEN_Reset            ((uint16_t)0xFFDF)  /* USART SC Disable Mask */

/* USART SC NACK Mask */
#define CR3_NACK_Set              ((uint16_t)0x0010)  /* USART SC NACK Enable Mask */
#define CR3_NACK_Reset            ((uint16_t)0xFFEF)  /* USART SC NACK Disable Mask */

/* USART Half-Duplex Mask */
#define CR3_HDSEL_Set             ((uint16_t)0x0008)  /* USART Half-Duplex Enable Mask */
#define CR3_HDSEL_Reset           ((uint16_t)0xFFF7)  /* USART Half-Duplex Disable Mask */

/* USART IrDA Mask */
#define CR3_IRLP_Mask             ((uint16_t)0xFFFB)  /* USART IrDA LowPower mode Mask */
#define CR3_CLEAR_Mask            ((uint16_t)0xFCFF)  /* USART CR3 Mask */

/* USART IrDA Mask */
#define CR3_IREN_Set              ((uint16_t)0x0002)  /* USART IrDA Enable Mask */
#define CR3_IREN_Reset            ((uint16_t)0xFFFD)  /* USART IrDA Disable Mask */
#define GTPR_LSB_Mask             ((uint16_t)0x00FF)  /* Guard Time Register LSB Mask */
#define GTPR_MSB_Mask             ((uint16_t)0xFF00)  /* Guard Time Register MSB Mask */
#define IT_Mask                   ((uint16_t)0x001F)  /* USART Interrupt Mask */




/// Initializes the USARTx peripheral according to the specified
///   parameters in the USART_InitStruct .
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_InitStruct`: pointer to a USART_InitTypeDef structure
///   that contains the configuration information for the
///   specified USART peripheral.
fn Init(&self, USART_InitTypeDef* USART_InitStruct) {
  uint32_t tmpreg = 0x00, apbclock = 0x00;
  uint32_t integerdivider = 0x00;
  uint32_t fractionaldivider = 0x00;
  uint32_t usartxbase = 0;
  RCC_ClocksTypeDef RCC_ClocksStatus;
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  /* The hardware flow control is available only for USART1, USART2 and USART3 */
  assert_param(IS_USART_PERIPH_HFC(USARTx, USART_InitStruct->USART_HardwareFlowControl));

  usartxbase = (*(uint32_t*)&USARTx);
/*---------------------------- USART CR2 Configuration -----------------------*/
  tmpreg = USARTx->CR2;
  /* Clear STOP[13:12] bits */
  tmpreg &= CR2_STOP_CLEAR_Mask;
  /* Configure the USART Stop Bits, Clock, CPOL, CPHA and LastBit ------------*/
  /* Set STOP[13:12] bits according to USART_StopBits value */
  tmpreg |= (uint32_t)USART_InitStruct->USART_StopBits;
  
  /* Write to USART CR2 */
  USARTx->CR2 = (uint16_t)tmpreg;
/*---------------------------- USART CR1 Configuration -----------------------*/
  tmpreg = USARTx->CR1;
  /* Clear M, PCE, PS, TE and RE bits */
  tmpreg &= CR1_CLEAR_Mask;
  /* Configure the USART Word Length, Parity and mode ----------------------- */
  /* Set the M bits according to USART_WordLength value */
  /* Set PCE and PS bits according to USART_Parity value */
  /* Set TE and RE bits according to USART_Mode value */
  tmpreg |= (uint32_t)USART_InitStruct->USART_WordLength | USART_InitStruct->USART_Parity |
            USART_InitStruct->USART_Mode;
  /* Write to USART CR1 */
  USARTx->CR1 = (uint16_t)tmpreg;
/*---------------------------- USART CR3 Configuration -----------------------*/  
  tmpreg = USARTx->CR3;
  /* Clear CTSE and RTSE bits */
  tmpreg &= CR3_CLEAR_Mask;
  /* Configure the USART HFC -------------------------------------------------*/
  /* Set CTSE and RTSE bits according to USART_HardwareFlowControl value */
  tmpreg |= USART_InitStruct->USART_HardwareFlowControl;
  /* Write to USART CR3 */
  USARTx->CR3 = (uint16_t)tmpreg;
/*---------------------------- USART BRR Configuration -----------------------*/
  /* Configure the USART Baud Rate -------------------------------------------*/
  RCC_GetClocksFreq(&RCC_ClocksStatus);
  if (usartxbase == USART1_BASE) {
    apbclock = RCC_ClocksStatus.PCLK2_Frequency;
  } else {
    apbclock = RCC_ClocksStatus.PCLK1_Frequency;
  }
  /* Determine the integer part */
  integerdivider = ((0x19 * apbclock) / (0x04 * (USART_InitStruct->USART_BaudRate)));
  tmpreg = (integerdivider / 0x64) << 0x04;
  /* Determine the fractional part */
  fractionaldivider = integerdivider - (0x64 * (tmpreg >> 0x04));
  tmpreg |= ((((fractionaldivider * 0x10) + 0x32) / 0x64)) & ((uint8_t)0x0F);
  /* Write to USART BRR */
  USARTx->BRR = (uint16_t)tmpreg;
}

/// Fills each USART_ClockInitStruct member with its default value.
/// `USART_ClockInitStruct`: pointer to a USART_ClockInitTypeDef
///   structure which will be initialized.
fn ClockStructInit(USART_ClockInitTypeDef* USART_ClockInitStruct) {
  /* USART_ClockInitStruct members default value */
  USART_ClockInitStruct->USART_Clock = USART_Clock_Disable;
  USART_ClockInitStruct->USART_CPOL = USART_CPOL_Low;
  USART_ClockInitStruct->USART_CPHA = USART_CPHA_1Edge;
  USART_ClockInitStruct->USART_LastBit = USART_LastBit_Disable;
}

/// Enables or disables the specified USART peripheral.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `new_state : FunctionalState`: new state of the USARTx peripheral.
///   This parameter can be: ENABLE or DISABLE.
fn Cmd(&self, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the selected USART by setting the UE bit in the CR1 register */
    USARTx->CR1 |= CR1_UE_Set;
  } else {
    /* Disable the selected USART by clearing the UE bit in the CR1 register */
    USARTx->CR1 &= CR1_UE_Reset;
  }
}


/// Enables or disables the USART’s DMA interface.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3 or UART4.  
/// `USART_DMAReq`: specifies the DMA request.
///   This parameter can be any combination of the following values:
/// * USART_DMAReq_Tx: USART DMA transmit request
/// * USART_DMAReq_Rx: USART DMA receive request
/// `new_state : FunctionalState`: new state of the DMA Request sources.
///   This parameter can be: ENABLE or DISABLE.
/// @note The DMA mode is not available for UART5.  
fn DMACmd(&self, uint16_t USART_DMAReq, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_1234_PERIPH(USARTx));
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the DMA transfer for selected requests by setting the DMAT and/or
       DMAR bits in the USART CR3 register */
    USARTx->CR3 |= USART_DMAReq;
  } else {
    /* Disable the DMA transfer for selected requests by clearing the DMAT and/or
       DMAR bits in the USART CR3 register */
    USARTx->CR3 &= (uint16_t)~USART_DMAReq;
  }
}

/// Sets the address of the USART node.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_Address`: Indicates the address of the USART node.
fn SetAddress(&self, uint8_t USART_Address) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
    
  /* Clear the USART address */
  USARTx->CR2 &= CR2_Address_Mask;
  /* Set the USART address node */
  USARTx->CR2 |= USART_Address;
}

/// Selects the USART WakeUp method.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_WakeUp`: specifies the USART wakeup method.
///   This parameter can be one of the following values:
/// * USART_WakeUp_IdleLine: WakeUp by an idle line detection
/// * USART_WakeUp_AddressMark: WakeUp by an address mark
fn WakeUpConfig(&self, uint16_t USART_WakeUp) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  USARTx->CR1 &= CR1_WAKE_Mask;
  USARTx->CR1 |= USART_WakeUp;
}

/// Determines if the USART is in mute mode or not.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `new_state : FunctionalState`: new state of the USART mute mode.
///   This parameter can be: ENABLE or DISABLE.
fn ReceiverWakeUpCmd(&self, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx)); 
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the USART mute mode  by setting the RWU bit in the CR1 register */
    USARTx->CR1 |= CR1_RWU_Set;
  } else {
    /* Disable the USART mute mode by clearing the RWU bit in the CR1 register */
    USARTx->CR1 &= CR1_RWU_Reset;
  }
}

/// Sets the USART LIN Break detection length.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_LINBreakDetectLength`: specifies the LIN break
///   detection length.
///   This parameter can be one of the following values:
/// * USART_LINBreakDetectLength_10b: 10-bit break detection
/// * USART_LINBreakDetectLength_11b: 11-bit break detection
fn LINBreakDetectLengthConfig(&self, uint16_t USART_LINBreakDetectLength) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  USARTx->CR2 &= CR2_LBDL_Mask;
  USARTx->CR2 |= USART_LINBreakDetectLength;  
}

/// Enables or disables the USART’s LIN mode.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `new_state : FunctionalState`: new state of the USART LIN mode.
///   This parameter can be: ENABLE or DISABLE.
fn LINCmd(&self, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the LIN mode by setting the LINEN bit in the CR2 register */
    USARTx->CR2 |= CR2_LINEN_Set;
  } else {
    /* Disable the LIN mode by clearing the LINEN bit in the CR2 register */
    USARTx->CR2 &= CR2_LINEN_Reset;
  }
}

/// Transmits single data through the USARTx peripheral.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `Data`: the data to transmit.
fn SendData(&self, uint16_t Data) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
    
  /* Transmit Data */
  USARTx->DR = (Data & (uint16_t)0x01FF);
}

/// Returns the most recent received data by the USARTx peripheral.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// @retval : The received data.
uint16_t USART_ReceiveData(&self) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  /* Receive Data */
  return (uint16_t)(USARTx->DR & (uint16_t)0x01FF);
}

/// Transmits break characters.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
fn SendBreak(&self) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  /* Send break characters */
  USARTx->CR1 |= CR1_SBK_Set;
}

/// Sets the specified USART guard time.
/// `USARTx`: where x can be 1, 2 or 3 to select the USART
///   peripheral.
/// `USART_GuardTime`: specifies the guard time.
/// @note The guard time bits are not available for UART4 and UART5.   
fn SetGuardTime(&self, uint8_t USART_GuardTime) {    
  /* Check the parameters */
  assert_param(IS_USART_123_PERIPH(USARTx));
  
  /* Clear the USART Guard time */
  USARTx->GTPR &= GTPR_LSB_Mask;
  /* Set the USART guard time */
  USARTx->GTPR |= (uint16_t)((uint16_t)USART_GuardTime << 0x08);
}

/// Sets the system clock prescaler.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_Prescaler`: specifies the prescaler clock.  
/// @note The function is used for IrDA mode with UART4 and UART5.
 /// @retval : None
fn SetPrescaler(&self, uint8_t USART_Prescaler) { 
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  /* Clear the USART prescaler */
  USARTx->GTPR &= GTPR_MSB_Mask;
  /* Set the USART prescaler */
  USARTx->GTPR |= USART_Prescaler;
}

/// Enables or disables the USART’s Smart Card mode.
/// `USARTx`: where x can be 1, 2 or 3 to select the USART
///   peripheral.
/// `new_state : FunctionalState`: new state of the Smart Card mode.
///   This parameter can be: ENABLE or DISABLE.     
/// @note The Smart Card mode is not available for UART4 and UART5. 
fn SmartCardCmd(&self, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_123_PERIPH(USARTx));
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the SC mode by setting the SCEN bit in the CR3 register */
    USARTx->CR3 |= CR3_SCEN_Set;
  } else {
    /* Disable the SC mode by clearing the SCEN bit in the CR3 register */
    USARTx->CR3 &= CR3_SCEN_Reset;
  }
}

/// Enables or disables NACK transmission.
/// `USARTx`: where x can be 1, 2 or 3 to select the USART
///   peripheral. 
/// `new_state : FunctionalState`: new state of the NACK transmission.
///   This parameter can be: ENABLE or DISABLE.  
/// @note The Smart Card mode is not available for UART4 and UART5.
fn SmartCardNACKCmd(&self, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_123_PERIPH(USARTx));
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the NACK transmission by setting the NACK bit in the CR3 register */
    USARTx->CR3 |= CR3_NACK_Set;
  } else {
    /* Disable the NACK transmission by clearing the NACK bit in the CR3 register */
    USARTx->CR3 &= CR3_NACK_Reset;
  }
}

/// Enables or disables the USART’s Half Duplex communication.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `new_state : FunctionalState`: new state of the USART Communication.
///   This parameter can be: ENABLE or DISABLE.
fn HalfDuplexCmd(&self, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the Half-Duplex mode by setting the HDSEL bit in the CR3 register */
    USARTx->CR3 |= CR3_HDSEL_Set;
  } else {
    /* Disable the Half-Duplex mode by clearing the HDSEL bit in the CR3 register */
    USARTx->CR3 &= CR3_HDSEL_Reset;
  }
}

/// Configures the USART’s IrDA interface.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `USART_IrDAMode`: specifies the IrDA mode.
///   This parameter can be one of the following values:
/// * USART_IrDAMode_LowPower
/// * USART_IrDAMode_Normal
fn IrDAConfig(&self, uint16_t USART_IrDAMode) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
    
  USARTx->CR3 &= CR3_IRLP_Mask;
  USARTx->CR3 |= USART_IrDAMode;
}

/// Enables or disables the USART’s IrDA interface.
/// `USARTx`: Select the USART or the UART peripheral. 
///   This parameter can be one of the following values:
///   USART1, USART2, USART3, UART4 or UART5.
/// `new_state : FunctionalState`: new state of the IrDA mode.
///   This parameter can be: ENABLE or DISABLE.
fn IrDACmd(&self, new_state : bool) {
  /* Check the parameters */
  assert_param(IS_USART_ALL_PERIPH(USARTx));
    
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the IrDA mode by setting the IREN bit in the CR3 register */
    USARTx->CR3 |= CR3_IREN_Set;
  } else {
    /* Disable the IrDA mode by clearing the IREN bit in the CR3 register */
    USARTx->CR3 &= CR3_IREN_Reset;
  }
}

