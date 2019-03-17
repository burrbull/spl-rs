
FlagStatus WWDG_GetFlagStatus(void);
void WWDG_ClearFlag(void);


/// Checks whether the Early Wakeup interrupt flag is set or not.
/// @param  None
/// @retval : The new state of the Early Wakeup interrupt flag (SET or RESET)
FlagStatus WWDG_GetFlagStatus(void) {
  return (FlagStatus)(WWDG->SR);
}

/// Clears Early Wakeup interrupt flag.
/// @param  None
void WWDG_ClearFlag(void) {
  WWDG->SR = (uint32_t)RESET;
}

