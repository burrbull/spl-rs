
/// TIM_interrupt_sources 

#define TIM_IT_Update                      ((uint16_t)0x0001)
#define TIM_IT_CC1                         ((uint16_t)0x0002)
#define TIM_IT_CC2                         ((uint16_t)0x0004)
#define TIM_IT_CC3                         ((uint16_t)0x0008)
#define TIM_IT_CC4                         ((uint16_t)0x0010)
#define TIM_IT_COM                         ((uint16_t)0x0020)
#define TIM_IT_Trigger                     ((uint16_t)0x0040)
#define TIM_IT_Break                       ((uint16_t)0x0080)
#define IS_TIM_IT(IT) ((((IT) & (uint16_t)0xFF00) == 0x0000) && ((IT) != 0x0000))
#define IS_TIM_PERIPH_IT(PERIPH, TIM_IT) ((((((*(uint32_t*)&(PERIPH)) == TIM2_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM3_BASE))||\
                                            (((*(uint32_t*)&(PERIPH)) == TIM4_BASE)) || (((*(uint32_t*)&(PERIPH)) == TIM5_BASE))))&& \
                                            (((TIM_IT) & (uint16_t)0xFFA0) == 0x0000) && ((TIM_IT) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM1_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM8_BASE))))&& \
                                            (((TIM_IT) & (uint16_t)0xFF00) == 0x0000) && ((TIM_IT) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM6_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM7_BASE))))&& \
                                            (((TIM_IT) & (uint16_t)0xFFFE) == 0x0000) && ((TIM_IT) != 0x0000)))


/// TIM_Flags 

#define TIM_FLAG_Update                    ((uint16_t)0x0001)
#define TIM_FLAG_CC1                       ((uint16_t)0x0002)
#define TIM_FLAG_CC2                       ((uint16_t)0x0004)
#define TIM_FLAG_CC3                       ((uint16_t)0x0008)
#define TIM_FLAG_CC4                       ((uint16_t)0x0010)
#define TIM_FLAG_COM                       ((uint16_t)0x0020)
#define TIM_FLAG_Trigger                   ((uint16_t)0x0040)
#define TIM_FLAG_Break                     ((uint16_t)0x0080)
#define TIM_FLAG_CC1OF                     ((uint16_t)0x0200)
#define TIM_FLAG_CC2OF                     ((uint16_t)0x0400)
#define TIM_FLAG_CC3OF                     ((uint16_t)0x0800)
#define TIM_FLAG_CC4OF                     ((uint16_t)0x1000)
#define IS_TIM_GET_FLAG(FLAG) (((FLAG) == TIM_FLAG_Update) || \
                               ((FLAG) == TIM_FLAG_CC1) || \
                               ((FLAG) == TIM_FLAG_CC2) || \
                               ((FLAG) == TIM_FLAG_CC3) || \
                               ((FLAG) == TIM_FLAG_CC4) || \
                               ((FLAG) == TIM_FLAG_COM) || \
                               ((FLAG) == TIM_FLAG_Trigger) || \
                               ((FLAG) == TIM_FLAG_Break) || \
                               ((FLAG) == TIM_FLAG_CC1OF) || \
                               ((FLAG) == TIM_FLAG_CC2OF) || \
                               ((FLAG) == TIM_FLAG_CC3OF) || \
                               ((FLAG) == TIM_FLAG_CC4OF))
#define IS_TIM_CLEAR_FLAG(PERIPH, TIM_FLAG) ((((((*(uint32_t*)&(PERIPH)) == TIM2_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM3_BASE))||\
                                            (((*(uint32_t*)&(PERIPH)) == TIM4_BASE)) || (((*(uint32_t*)&(PERIPH)) == TIM5_BASE))))&& \
                                            (((TIM_FLAG) & (uint16_t)0xE1A0) == 0x0000) && ((TIM_FLAG) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM1_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM8_BASE))))&& \
                                            (((TIM_FLAG) & (uint16_t)0xE100) == 0x0000) && ((TIM_FLAG) != 0x0000)) ||\
                                            (((((*(uint32_t*)&(PERIPH)) == TIM6_BASE) || (((*(uint32_t*)&(PERIPH)) == TIM7_BASE))))&& \
                                            (((TIM_FLAG) & (uint16_t)0xFFFE) == 0x0000) && ((TIM_FLAG) != 0x0000)))
#define IS_TIM_PERIPH_FLAG(PERIPH, TIM_FLAG)  (((((*(uint32_t*)&(PERIPH))==TIM2_BASE) || ((*(uint32_t*)&(PERIPH)) == TIM3_BASE) ||\
                                                 ((*(uint32_t*)&(PERIPH)) == TIM4_BASE) || ((*(uint32_t*)&(PERIPH))==TIM5_BASE) || \
                                                 ((*(uint32_t*)&(PERIPH))==TIM1_BASE) || ((*(uint32_t*)&(PERIPH))==TIM8_BASE)) &&\
                                                 (((TIM_FLAG) == TIM_FLAG_CC1) || ((TIM_FLAG) == TIM_FLAG_CC2) ||\
                                                 ((TIM_FLAG) == TIM_FLAG_CC3) || ((TIM_FLAG) == TIM_FLAG_CC4) || \
                                                 ((TIM_FLAG) == TIM_FLAG_Trigger))) ||\
                                                 ((((*(uint32_t*)&(PERIPH))==TIM2_BASE) || ((*(uint32_t*)&(PERIPH)) == TIM3_BASE) || \
                                                 ((*(uint32_t*)&(PERIPH)) == TIM4_BASE) || ((*(uint32_t*)&(PERIPH))==TIM5_BASE) ||\
                                                 ((*(uint32_t*)&(PERIPH))==TIM1_BASE)|| ((*(uint32_t*)&(PERIPH))==TIM8_BASE) || \
                                                 ((*(uint32_t*)&(PERIPH))==TIM7_BASE) || ((*(uint32_t*)&(PERIPH))==TIM6_BASE)) && \
                                                 (((TIM_FLAG) == TIM_FLAG_Update))) ||\
                                                 ((((*(uint32_t*)&(PERIPH))==TIM1_BASE) || ((*(uint32_t*)&(PERIPH)) == TIM8_BASE)) &&\
                                                 (((TIM_FLAG) == TIM_FLAG_COM) || ((TIM_FLAG) == TIM_FLAG_Break))) ||\
                                                 ((((*(uint32_t*)&(PERIPH))==TIM2_BASE) || ((*(uint32_t*)&(PERIPH)) == TIM3_BASE) || \
                                                 ((*(uint32_t*)&(PERIPH)) == TIM4_BASE) || ((*(uint32_t*)&(PERIPH))==TIM5_BASE) || \
                                                 ((*(uint32_t*)&(PERIPH))==TIM1_BASE) || ((*(uint32_t*)&(PERIPH))==TIM8_BASE)) &&\
                                                 (((TIM_FLAG) == TIM_FLAG_CC1OF) || ((TIM_FLAG) == TIM_FLAG_CC2OF) ||\
                                                 ((TIM_FLAG) == TIM_FLAG_CC3OF) || ((TIM_FLAG) == TIM_FLAG_CC4OF))))
 




void TIM_ITConfig(TIM_TypeDef* TIMx, uint16_t TIM_IT, FunctionalState NewState);
FlagStatus TIM_GetFlagStatus(TIM_TypeDef* TIMx, uint16_t TIM_FLAG);
void TIM_ClearFlag(TIM_TypeDef* TIMx, uint16_t TIM_FLAG);
ITStatus TIM_GetITStatus(TIM_TypeDef* TIMx, uint16_t TIM_IT);
void TIM_ClearITPendingBit(TIM_TypeDef* TIMx, uint16_t TIM_IT);

/// Enables or disables the specified TIM interrupts.
/// `TIMx`: where x can be 1 to 8 to select the TIMx peripheral.
/// `TIM_IT`: specifies the TIM interrupts sources to be enabled
///   or disabled.
///   This parameter can be any combination of the following values:
/// * TIM_IT_Update: TIM update Interrupt source
/// * TIM_IT_CC1: TIM Capture Compare 1 Interrupt source
/// * TIM_IT_CC2: TIM Capture Compare 2 Interrupt source
/// * TIM_IT_CC3: TIM Capture Compare 3 Interrupt source
/// * TIM_IT_CC4: TIM Capture Compare 4 Interrupt source
/// * TIM_IT_COM: TIM Commutation Interrupt source
/// * TIM_IT_Trigger: TIM Trigger Interrupt source
/// * TIM_IT_Break: TIM Break Interrupt source
/// `NewState`: new state of the TIM interrupts.
///   This parameter can be: ENABLE or DISABLE.
void TIM_ITConfig(TIM_TypeDef* TIMx, uint16_t TIM_IT, FunctionalState NewState) {  
  /* Check the parameters */
  assert_param(IS_TIM_ALL_PERIPH(TIMx));
  
  if (NewState != DISABLE) {
    /* Enable the Interrupt sources */
    TIMx->DIER |= TIM_IT;
  } else {
    /* Disable the Interrupt sources */
    TIMx->DIER &= (uint16_t)~TIM_IT;
  }
}

/// Checks whether the specified TIM flag is set or not.
/// `TIMx`: where x can be 1 to 8 to select the TIM peripheral.
/// `TIM_FLAG`: specifies the flag to check.
///   This parameter can be one of the following values:
/// * TIM_FLAG_Update: TIM update Flag
/// * TIM_FLAG_CC1: TIM Capture Compare 1 Flag
/// * TIM_FLAG_CC2: TIM Capture Compare 2 Flag
/// * TIM_FLAG_CC3: TIM Capture Compare 3 Flag
/// * TIM_FLAG_CC4: TIM Capture Compare 4 Flag
/// * TIM_FLAG_COM: TIM Commutation Flag
/// * TIM_FLAG_Trigger: TIM Trigger Flag
/// * TIM_FLAG_Break: TIM Break Flag
/// * TIM_FLAG_CC1OF: TIM Capture Compare 1 overcapture Flag
/// * TIM_FLAG_CC2OF: TIM Capture Compare 2 overcapture Flag
/// * TIM_FLAG_CC3OF: TIM Capture Compare 3 overcapture Flag
/// * TIM_FLAG_CC4OF: TIM Capture Compare 4 overcapture Flag
/// @retval : The new state of TIM_FLAG (SET or RESET).
FlagStatus TIM_GetFlagStatus(TIM_TypeDef* TIMx, uint16_t TIM_FLAG) { 
  ITStatus bitstatus = RESET;  
  /* Check the parameters */
  assert_param(IS_TIM_ALL_PERIPH(TIMx));
  
  if ((TIMx->SR & TIM_FLAG) != (uint16_t)RESET) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the TIMx's pending flags.
/// `TIMx`: where x can be 1 to 8 to select the TIM peripheral.
/// `TIM_FLAG`: specifies the flag bit to clear.
///   This parameter can be any combination of the following values:
/// * TIM_FLAG_Update: TIM update Flag
/// * TIM_FLAG_CC1: TIM Capture Compare 1 Flag
/// * TIM_FLAG_CC2: TIM Capture Compare 2 Flag
/// * TIM_FLAG_CC3: TIM Capture Compare 3 Flag
/// * TIM_FLAG_CC4: TIM Capture Compare 4 Flag
/// * TIM_FLAG_COM: TIM Commutation Flag
/// * TIM_FLAG_Trigger: TIM Trigger Flag
/// * TIM_FLAG_Break: TIM Break Flag
/// * TIM_FLAG_CC1OF: TIM Capture Compare 1 overcapture Flag
/// * TIM_FLAG_CC2OF: TIM Capture Compare 2 overcapture Flag
/// * TIM_FLAG_CC3OF: TIM Capture Compare 3 overcapture Flag
/// * TIM_FLAG_CC4OF: TIM Capture Compare 4 overcapture Flag
void TIM_ClearFlag(TIM_TypeDef* TIMx, uint16_t TIM_FLAG) {  
  /* Check the parameters */
  assert_param(IS_TIM_ALL_PERIPH(TIMx));
   
  /* Clear the flags */
  TIMx->SR = (uint16_t)~TIM_FLAG;
}

/// Checks whether the TIM interrupt has occurred or not.
/// `TIMx`: where x can be 1 to 8 to select the TIM peripheral.
/// `TIM_IT`: specifies the TIM interrupt source to check.
///   This parameter can be one of the following values:
/// * TIM_IT_Update: TIM update Interrupt source
/// * TIM_IT_CC1: TIM Capture Compare 1 Interrupt source
/// * TIM_IT_CC2: TIM Capture Compare 2 Interrupt source
/// * TIM_IT_CC3: TIM Capture Compare 3 Interrupt source
/// * TIM_IT_CC4: TIM Capture Compare 4 Interrupt source
/// * TIM_IT_COM: TIM Commutation Interrupt
///   source
/// * TIM_IT_Trigger: TIM Trigger Interrupt source
/// * TIM_IT_Break: TIM Break Interrupt source
/// @retval : The new state of the TIM_IT(SET or RESET).
ITStatus TIM_GetITStatus(TIM_TypeDef* TIMx, uint16_t TIM_IT) {
  ITStatus bitstatus = RESET;  
  uint16_t itstatus = 0x0, itenable = 0x0;
  /* Check the parameters */
  assert_param(IS_TIM_ALL_PERIPH(TIMx));
   
  itstatus = TIMx->SR & TIM_IT;
  
  itenable = TIMx->DIER & TIM_IT;
  if ((itstatus != (uint16_t)RESET) && (itenable != (uint16_t)RESET)) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the TIMx's interrupt pending bits.
/// `TIMx`: where x can be 1 to 8 to select the TIM peripheral.
/// `TIM_IT`: specifies the pending bit to clear.
///   This parameter can be any combination of the following values:
/// * TIM_IT_Update: TIM1 update Interrupt source
/// * TIM_IT_CC1: TIM Capture Compare 1 Interrupt source
/// * TIM_IT_CC2: TIM Capture Compare 2 Interrupt source
/// * TIM_IT_CC3: TIM Capture Compare 3 Interrupt source
/// * TIM_IT_CC4: TIM Capture Compare 4 Interrupt source
/// * TIM_IT_COM: TIM Commutation Interrupt
///   source
/// * TIM_IT_Trigger: TIM Trigger Interrupt source
/// * TIM_IT_Break: TIM Break Interrupt source
void TIM_ClearITPendingBit(TIM_TypeDef* TIMx, uint16_t TIM_IT) {
  /* Check the parameters */
  assert_param(IS_TIM_ALL_PERIPH(TIMx));
  /* Clear the IT pending Bit */
  TIMx->SR = (uint16_t)~TIM_IT;
}

