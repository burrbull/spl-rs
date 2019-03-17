
/// IWDG_Flag 

#define IWDG_FLAG_PVU               ((uint16_t)0x0001)
#define IWDG_FLAG_RVU               ((uint16_t)0x0002)

FlagStatus IWDG_GetFlagStatus(uint16_t IWDG_FLAG);

/// Checks whether the specified IWDG flag is set or not.
/// `IWDG_FLAG`: specifies the flag to check.
///   This parameter can be one of the following values:
/// * IWDG_FLAG_PVU: Prescaler Value Update on going
/// * IWDG_FLAG_RVU: Reload Value Update on going
/// @retval : The new state of IWDG_FLAG (SET or RESET).
FlagStatus IWDG_GetFlagStatus(uint16_t IWDG_FLAG) {
  FlagStatus bitstatus = RESET;
  if ((IWDG->SR & IWDG_FLAG) != (uint32_t)RESET) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  /* Return the flag status */
  return bitstatus;
}

