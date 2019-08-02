

/// FSMC_Interrupt_sources 

#define FSMC_IT_RisingEdge                              ((uint32_t)0x00000008)
#define FSMC_IT_Level                                   ((uint32_t)0x00000010)
#define FSMC_IT_FallingEdge                             ((uint32_t)0x00000020)

/// FSMC_Flags 

#define FSMC_FLAG_RisingEdge                            ((uint32_t)0x00000001)
#define FSMC_FLAG_Level                                 ((uint32_t)0x00000002)
#define FSMC_FLAG_FallingEdge                           ((uint32_t)0x00000004)
#define FSMC_FLAG_FEMPT                                 ((uint32_t)0x00000040)


void FSMC_ITConfig(uint32_t FSMC_Bank, uint32_t FSMC_IT, FunctionalState NewState);
FlagStatus FSMC_GetFlagStatus(uint32_t FSMC_Bank, uint32_t FSMC_FLAG);
void FSMC_ClearFlag(uint32_t FSMC_Bank, uint32_t FSMC_FLAG);
ITStatus FSMC_GetITStatus(uint32_t FSMC_Bank, uint32_t FSMC_IT);
void FSMC_ClearITPendingBit(uint32_t FSMC_Bank, uint32_t FSMC_IT);




/// Enables or disables the specified FSMC interrupts.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// * FSMC_Bank4_PCCARD: FSMC Bank4 PCCARD
/// `FSMC_IT`: specifies the FSMC interrupt sources to be
///   enabled or disabled.
///   This parameter can be any combination of the following values:
/// * FSMC_IT_RisingEdge: Rising edge detection interrupt. 
/// * FSMC_IT_Level: Level edge detection interrupt.
/// * FSMC_IT_FallingEdge: Falling edge detection interrupt.
/// `NewState`: new state of the specified FSMC interrupts.
///   This parameter can be: ENABLE or DISABLE.
void FSMC_ITConfig(uint32_t FSMC_Bank, uint32_t FSMC_IT, FunctionalState NewState) {
  
  if (NewState != DISABLE) {
    /* Enable the selected FSMC_Bank2 interrupts */
    if(FSMC_Bank == FSMC_Bank2_NAND) {
      FSMC_Bank2->SR2 |= FSMC_IT;
    }
    /* Enable the selected FSMC_Bank3 interrupts */
    else if (FSMC_Bank == FSMC_Bank3_NAND) {
      FSMC_Bank3->SR3 |= FSMC_IT;
    }
    /* Enable the selected FSMC_Bank4 interrupts */
    else {
      FSMC_Bank4->SR4 |= FSMC_IT;    
    }
  } else {
    /* Disable the selected FSMC_Bank2 interrupts */
    if(FSMC_Bank == FSMC_Bank2_NAND) {
      
      FSMC_Bank2->SR2 &= (uint32_t)~FSMC_IT;
    }
    /* Disable the selected FSMC_Bank3 interrupts */
    else if (FSMC_Bank == FSMC_Bank3_NAND) {
      FSMC_Bank3->SR3 &= (uint32_t)~FSMC_IT;
    }
    /* Disable the selected FSMC_Bank4 interrupts */
    else {
      FSMC_Bank4->SR4 &= (uint32_t)~FSMC_IT;    
    }
  }
}

/// Checks whether the specified FSMC flag is set or not.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// * FSMC_Bank4_PCCARD: FSMC Bank4 PCCARD
/// `FSMC_FLAG`: specifies the flag to check.
///   This parameter can be one of the following values:
/// * FSMC_FLAG_RisingEdge: Rising egde detection Flag.
/// * FSMC_FLAG_Level: Level detection Flag.
/// * FSMC_FLAG_FallingEdge: Falling egde detection Flag.
/// * FSMC_FLAG_FEMPT: Fifo empty Flag. 
/// @retval : The new state of FSMC_FLAG (SET or RESET).
FlagStatus FSMC_GetFlagStatus(uint32_t FSMC_Bank, uint32_t FSMC_FLAG) {
  FlagStatus bitstatus = RESET;
  uint32_t tmpsr = 0x00000000;
  
  
  if(FSMC_Bank == FSMC_Bank2_NAND) {
    tmpsr = FSMC_Bank2->SR2;
  }  
  else if(FSMC_Bank == FSMC_Bank3_NAND) {
    tmpsr = FSMC_Bank3->SR3;
  }
  /* FSMC_Bank4_PCCARD*/
  else {
    tmpsr = FSMC_Bank4->SR4;
  } 
  
  /* Get the flag status */
  if ((tmpsr & FSMC_FLAG) != (uint16_t)RESET ) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  /* Return the flag status */
  return bitstatus;
}

/// Clears the FSMC’s pending flags.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// * FSMC_Bank4_PCCARD: FSMC Bank4 PCCARD
/// `FSMC_FLAG`: specifies the flag to clear.
///   This parameter can be any combination of the following values:
/// * FSMC_FLAG_RisingEdge: Rising egde detection Flag.
/// * FSMC_FLAG_Level: Level detection Flag.
/// * FSMC_FLAG_FallingEdge: Falling egde detection Flag.
void FSMC_ClearFlag(uint32_t FSMC_Bank, uint32_t FSMC_FLAG) {
    
  if(FSMC_Bank == FSMC_Bank2_NAND) {
    FSMC_Bank2->SR2 &= ~FSMC_FLAG; 
  }  
  else if(FSMC_Bank == FSMC_Bank3_NAND) {
    FSMC_Bank3->SR3 &= ~FSMC_FLAG;
  }
  /* FSMC_Bank4_PCCARD*/
  else {
    FSMC_Bank4->SR4 &= ~FSMC_FLAG;
  }
}

/// Checks whether the specified FSMC interrupt has occurred or not.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// * FSMC_Bank4_PCCARD: FSMC Bank4 PCCARD
/// `FSMC_IT`: specifies the FSMC interrupt source to check.
///   This parameter can be one of the following values:
/// * FSMC_IT_RisingEdge: Rising edge detection interrupt. 
/// * FSMC_IT_Level: Level edge detection interrupt.
/// * FSMC_IT_FallingEdge: Falling edge detection interrupt. 
/// @retval : The new state of FSMC_IT (SET or RESET).
ITStatus FSMC_GetITStatus(uint32_t FSMC_Bank, uint32_t FSMC_IT) {
  ITStatus bitstatus = RESET;
  uint32_t tmpsr = 0x0, itstatus = 0x0, itenable = 0x0; 
  
  if(FSMC_Bank == FSMC_Bank2_NAND) {
    tmpsr = FSMC_Bank2->SR2;
  }  
  else if(FSMC_Bank == FSMC_Bank3_NAND) {
    tmpsr = FSMC_Bank3->SR3;
  }
  /* FSMC_Bank4_PCCARD*/
  else {
    tmpsr = FSMC_Bank4->SR4;
  } 
  
  itstatus = tmpsr & FSMC_IT;
  
  itenable = tmpsr & (FSMC_IT >> 3);
  if ((itstatus != (uint32_t)RESET)  && (itenable != (uint32_t)RESET)) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus; 
}

/// Clears the FSMC’s interrupt pending bits.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// * FSMC_Bank4_PCCARD: FSMC Bank4 PCCARD
/// `FSMC_IT`: specifies the interrupt pending bit to clear.
///   This parameter can be any combination of the following values:
/// * FSMC_IT_RisingEdge: Rising edge detection interrupt. 
/// * FSMC_IT_Level: Level edge detection interrupt.
/// * FSMC_IT_FallingEdge: Falling edge detection interrupt.
void FSMC_ClearITPendingBit(uint32_t FSMC_Bank, uint32_t FSMC_IT) {
    
  if(FSMC_Bank == FSMC_Bank2_NAND) {
    FSMC_Bank2->SR2 &= ~(FSMC_IT >> 3); 
  }  
  else if(FSMC_Bank == FSMC_Bank3_NAND) {
    FSMC_Bank3->SR3 &= ~(FSMC_IT >> 3);
  }
  /* FSMC_Bank4_PCCARD*/
  else {
    FSMC_Bank4->SR4 &= ~(FSMC_IT >> 3);
  }
}

