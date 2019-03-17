

/* CSR register bit mask */
#define CSR_CTE_Set       ((uint16_t)0x0001)
#define CSR_CTI_Set       ((uint16_t)0x0002)

void BKP_ITConfig(FunctionalState new_state : FunctionalState);
FlagStatus BKP_GetFlagStatus(void);
void BKP_ClearFlag(void);
ITStatus BKP_GetITStatus(void);
void BKP_ClearITPendingBit(void);

/// Enables or disables the Tamper Pin Interrupt.
/// `new_state : FunctionalState`: new state of the Tamper Pin Interrupt.
///   This parameter can be: ENABLE or DISABLE.
void BKP_ITConfig(FunctionalState new_state : FunctionalState) {
  *(__IO uint32_t *) CSR_TPIE_BB = (uint32_t)new_state : FunctionalState;
}

/// Checks whether the Tamper Pin Event flag is set or not.
/// @param  None
/// @retval : The new state of the Tamper Pin Event flag (SET or RESET).
FlagStatus BKP_GetFlagStatus(void) {
  return (FlagStatus)(*(__IO uint32_t *) CSR_TEF_BB);
}

/// Clears Tamper Pin Event pending flag.
/// @param  None
void BKP_ClearFlag(void) {
  /* Set CTE bit to clear Tamper Pin Event flag */
  BKP->CSR |= CSR_CTE_Set;
}

/// Checks whether the Tamper Pin Interrupt has occurred or not.
/// @param  None
/// @retval : The new state of the Tamper Pin Interrupt (SET or RESET).
ITStatus BKP_GetITStatus(void) {
  return (ITStatus)(*(__IO uint32_t *) CSR_TIF_BB);
}

/// Clears Tamper Pin Interrupt pending bit.
/// @param  None
void BKP_ClearITPendingBit(void) {
  /* Set CTI bit to clear Tamper Pin Interrupt pending bit */
  BKP->CSR |= CSR_CTI_Set;
}

