

#define RTC_IT_OW            ((uint16_t)0x0004)  /* Overflow interrupt */
#define RTC_IT_ALR           ((uint16_t)0x0002)  /* Alarm interrupt */
#define RTC_IT_SEC           ((uint16_t)0x0001)  /* Second interrupt */
#define IS_RTC_IT(IT) ((((IT) & (uint16_t)0xFFF8) == 0x00) && ((IT) != 0x00))
#define IS_RTC_GET_IT(IT) (((IT) == RTC_IT_OW) || ((IT) == RTC_IT_ALR) || \
                           ((IT) == RTC_IT_SEC))


#define RTC_FLAG_RTOFF       ((uint16_t)0x0020)  /* RTC Operation OFF flag */
#define RTC_FLAG_RSF         ((uint16_t)0x0008)  /* Registers Synchronized flag */
#define RTC_FLAG_OW          ((uint16_t)0x0004)  /* Overflow flag */
#define RTC_FLAG_ALR         ((uint16_t)0x0002)  /* Alarm flag */
#define RTC_FLAG_SEC         ((uint16_t)0x0001)  /* Second flag */
#define IS_RTC_CLEAR_FLAG(FLAG) ((((FLAG) & (uint16_t)0xFFF0) == 0x00) && ((FLAG) != 0x00))
#define IS_RTC_GET_FLAG(FLAG) (((FLAG) == RTC_FLAG_RTOFF) || ((FLAG) == RTC_FLAG_RSF) || \
                               ((FLAG) == RTC_FLAG_OW) || ((FLAG) == RTC_FLAG_ALR) || \
                               ((FLAG) == RTC_FLAG_SEC))
#define IS_RTC_PRESCALER(PRESCALER) ((PRESCALER) <= 0xFFFFF)




/// Enables or disables the specified RTC interrupts.
/// `RTC_IT`: specifies the RTC interrupts sources to be enabled or 
///                disabled.
///   This parameter can be any combination of the following values:
/// * RTC_IT_OW: Overflow interrupt
/// * RTC_IT_ALR: Alarm interrupt
/// * RTC_IT_SEC: Second interrupt
/// `NewState`: new state of the specified RTC interrupts.
///   This parameter can be: ENABLE or DISABLE.
void RTC_ITConfig(uint16_t RTC_IT, FunctionalState NewState) {
  
  if (NewState != DISABLE) {
    RTC->CRH |= RTC_IT;
  } else {
    RTC->CRH &= (uint16_t)~RTC_IT;
  }
}


/// Checks whether the specified RTC flag is set or not.
/// `RTC_FLAG`: specifies the flag to check.
///   This parameter can be one the following values:
/// * RTC_FLAG_RTOFF: RTC Operation OFF flag
/// * RTC_FLAG_RSF: Registers Synchronized flag
/// * RTC_FLAG_OW: Overflow flag
/// * RTC_FLAG_ALR: Alarm flag
/// * RTC_FLAG_SEC: Second flag
/// @retval : The new state of RTC_FLAG (SET or RESET).
FlagStatus RTC_GetFlagStatus(uint16_t RTC_FLAG) {
  FlagStatus bitstatus = RESET;
  
  
  if ((RTC->CRL & RTC_FLAG) != (uint16_t)RESET) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the RTC’s pending flags.
/// `RTC_FLAG`: specifies the flag to clear.
///   This parameter can be any combination of the following values:
/// * RTC_FLAG_RSF: Registers Synchronized flag. This flag
///   is cleared only after an APB reset or an APB Clock stop.
/// * RTC_FLAG_OW: Overflow flag
/// * RTC_FLAG_ALR: Alarm flag
/// * RTC_FLAG_SEC: Second flag
void RTC_ClearFlag(uint16_t RTC_FLAG) {
    
  /* Clear the coressponding RTC flag */
  RTC->CRL &= (uint16_t)~RTC_FLAG;
}

/// Checks whether the specified RTC interrupt has occured or not.
/// `RTC_IT`: specifies the RTC interrupts sources to check.
///   This parameter can be one of the following values:
/// * RTC_IT_OW: Overflow interrupt
/// * RTC_IT_ALR: Alarm interrupt
/// * RTC_IT_SEC: Second interrupt
/// @retval : The new state of the RTC_IT (SET or RESET).
ITStatus RTC_GetITStatus(uint16_t RTC_IT) {
  ITStatus bitstatus = RESET;
  
  bitstatus = (ITStatus)(RTC->CRL & RTC_IT);
  if (((RTC->CRH & RTC_IT) != (uint16_t)RESET) && (bitstatus != (uint16_t)RESET)) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the RTC’s interrupt pending bits.
/// `RTC_IT`: specifies the interrupt pending bit to clear.
///   This parameter can be any combination of the following values:
/// * RTC_IT_OW: Overflow interrupt
/// * RTC_IT_ALR: Alarm interrupt
/// * RTC_IT_SEC: Second interrupt
void RTC_ClearITPendingBit(uint16_t RTC_IT) { 
  
  /* Clear the coressponding RTC pending bit */
  RTC->CRL &= (uint16_t)~RTC_IT;
}
