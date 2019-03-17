



/// CAN_flags 

#define CAN_FLAG_EWG                ((uint32_t)0x00000001) /* Error Warning Flag */
#define CAN_FLAG_EPV                ((uint32_t)0x00000002) /* Error Passive Flag */
#define CAN_FLAG_BOF                ((uint32_t)0x00000004) /* Bus-Off Flag */


/// CAN_interrupts 

#define CAN_IT_RQCP0                ((uint32_t)0x00000005) /* Request completed mailbox 0 */
#define CAN_IT_RQCP1                ((uint32_t)0x00000006) /* Request completed mailbox 1 */
#define CAN_IT_RQCP2                ((uint32_t)0x00000007) /* Request completed mailbox 2 */
#define CAN_IT_TME                  ((uint32_t)0x00000001) /* Transmit mailbox empty */
#define CAN_IT_FMP0                 ((uint32_t)0x00000002) /* FIFO 0 message pending */
#define CAN_IT_FF0                  ((uint32_t)0x00000004) /* FIFO 0 full */
#define CAN_IT_FOV0                 ((uint32_t)0x00000008) /* FIFO 0 overrun */
#define CAN_IT_FMP1                 ((uint32_t)0x00000010) /* FIFO 1 message pending */
#define CAN_IT_FF1                  ((uint32_t)0x00000020) /* FIFO 1 full */
#define CAN_IT_FOV1                 ((uint32_t)0x00000040) /* FIFO 1 overrun */
#define CAN_IT_EWG                  ((uint32_t)0x00000100) /* Error warning */
#define CAN_IT_EPV                  ((uint32_t)0x00000200) /* Error passive */
#define CAN_IT_BOF                  ((uint32_t)0x00000400) /* Bus-off */
#define CAN_IT_LEC                  ((uint32_t)0x00000800) /* Last error code */
#define CAN_IT_ERR                  ((uint32_t)0x00008000) /* Error */
#define CAN_IT_WKU                  ((uint32_t)0x00010000) /* Wake-up */
#define CAN_IT_SLK                  ((uint32_t)0x00020000) /* Sleep */



void CAN_ITConfig(CAN_TypeDef* CANx, uint32_t CAN_IT, FunctionalState NewState);
FlagStatus CAN_GetFlagStatus(CAN_TypeDef* CANx, uint32_t CAN_FLAG);
void CAN_ClearFlag(CAN_TypeDef* CANx, uint32_t CAN_FLAG);
ITStatus CAN_GetITStatus(CAN_TypeDef* CANx, uint32_t CAN_IT);
void CAN_ClearITPendingBit(CAN_TypeDef* CANx, uint32_t CAN_IT);




/// Enables or disables the specified CAN interrupts.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `CAN_IT`: specifies the CAN interrupt sources to be enabled or
///   disabled.
///   This parameter can be: CAN_IT_TME, CAN_IT_FMP0, CAN_IT_FF0,
///   CAN_IT_FOV0, CAN_IT_FMP1, CAN_IT_FF1,
///   CAN_IT_FOV1, CAN_IT_EWG, CAN_IT_EPV,
///   CAN_IT_LEC, CAN_IT_ERR, CAN_IT_WKU or
///   CAN_IT_SLK.
/// `Newstate`: new state of the CAN interrupts.
///   This parameter can be: ENABLE or DISABLE..
void CAN_ITConfig(CAN_TypeDef* CANx, uint32_t CAN_IT, FunctionalState Newstate) {
  if (Newstate != DISABLE) {
    /* Enable the selected CAN interrupt */
    CANx->IER |= CAN_IT;
  } else {
    /* Disable the selected CAN interrupt */
    CANx->IER &= ~CAN_IT;
  }
}


/// Checks whether the specified CAN flag is set or not.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `CAN_FLAG`: specifies the flag to check.
///   This parameter can be: CAN_FLAG_EWG, CAN_FLAG_EPV or
///   CAN_FLAG_BOF.
/// @retval : The new state of CAN_FLAG (SET or RESET).
FlagStatus CAN_GetFlagStatus(CAN_TypeDef* CANx, uint32_t CAN_FLAG) {
  FlagStatus bitstatus = RESET;
  /* Check the status of the specified CAN flag */
  if ((CANx->ESR & CAN_FLAG) != (uint32_t)RESET) {
    /* CAN_FLAG is set */
    bitstatus = SET;
  } else {
    /* CAN_FLAG is reset */
    bitstatus = RESET;
  }
  /* Return the CAN_FLAG status */
  return  bitstatus;
}

/// Clears the CAN's pending flags.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `CAN_FLAG`: specifies the flag to clear..
void CAN_ClearFlag(CAN_TypeDef* CANx, uint32_t CAN_FLAG) {
  /* Clear the selected CAN flags */
  CANx->ESR &= ~CAN_FLAG;
}

/// Checks whether the specified CAN interrupt has occurred or 
///   not.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `CAN_IT`: specifies the CAN interrupt source to check.
///   This parameter can be: CAN_IT_RQCP0, CAN_IT_RQCP1, CAN_IT_RQCP2,
///   CAN_IT_FF0, CAN_IT_FOV0, CAN_IT_FF1,
///   CAN_IT_FOV1, CAN_IT_EWG, CAN_IT_EPV, 
///   CAN_IT_BOF, CAN_IT_WKU or CAN_IT_SLK.
/// @retval : The new state of CAN_IT (SET or RESET).
ITStatus CAN_GetITStatus(CAN_TypeDef* CANx, uint32_t CAN_IT) {
  ITStatus pendingbitstatus = RESET;
  switch (CAN_IT) {
    case CAN_IT_RQCP0:
      pendingbitstatus = CheckITStatus(CANx->TSR, TSR_RQCP0);
      break;
    case CAN_IT_RQCP1:
      pendingbitstatus = CheckITStatus(CANx->TSR, TSR_RQCP1);
      break;
    case CAN_IT_RQCP2:
      pendingbitstatus = CheckITStatus(CANx->TSR, TSR_RQCP2);
      break;
    case CAN_IT_FF0:
      pendingbitstatus = CheckITStatus(CANx->RF0R, RF0R_FULL0);
      break;
    case CAN_IT_FOV0:
      pendingbitstatus = CheckITStatus(CANx->RF0R, RF0R_FOVR0);
      break;
    case CAN_IT_FF1:
      pendingbitstatus = CheckITStatus(CANx->RF1R, RF1R_FULL1);
      break;
    case CAN_IT_FOV1:
      pendingbitstatus = CheckITStatus(CANx->RF1R, RF1R_FOVR1);
      break;
    case CAN_IT_EWG:
      pendingbitstatus = CheckITStatus(CANx->ESR, ESR_EWGF);
      break;
    case CAN_IT_EPV:
      pendingbitstatus = CheckITStatus(CANx->ESR, ESR_EPVF);
      break;
    case CAN_IT_BOF:
      pendingbitstatus = CheckITStatus(CANx->ESR, ESR_BOFF);
      break;
    case CAN_IT_SLK:
      pendingbitstatus = CheckITStatus(CANx->MSR, MSR_SLAKI);
      break;
    case CAN_IT_WKU:
      pendingbitstatus = CheckITStatus(CANx->MSR, MSR_WKUI);
      break;
    default :
      pendingbitstatus = RESET;
      break;
  }
  /* Return the CAN_IT status */
  return  pendingbitstatus;
}

/// Clears the CAN’s interrupt pending bits.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `CAN_IT`: specifies the interrupt pending bit to clear..
void CAN_ClearITPendingBit(CAN_TypeDef* CANx, uint32_t CAN_IT) {
  switch (CAN_IT) {
    case CAN_IT_RQCP0:
      CANx->TSR = TSR_RQCP0; /* rc_w1*/
      break;
    case CAN_IT_RQCP1:
      CANx->TSR = TSR_RQCP1; /* rc_w1*/
      break;
    case CAN_IT_RQCP2:
      CANx->TSR = TSR_RQCP2; /* rc_w1*/
      break;
    case CAN_IT_FF0:
      CANx->RF0R = RF0R_FULL0; /* rc_w1*/
      break;
    case CAN_IT_FOV0:
      CANx->RF0R = RF0R_FOVR0; /* rc_w1*/
      break;
    case CAN_IT_FF1:
      CANx->RF1R = RF1R_FULL1; /* rc_w1*/
      break;
    case CAN_IT_FOV1:
      CANx->RF1R = RF1R_FOVR1; /* rc_w1*/
      break;
    case CAN_IT_EWG:
      CANx->ESR &= ~ ESR_EWGF; /* rw */
      break;
    case CAN_IT_EPV:
      CANx->ESR &= ~ ESR_EPVF; /* rw */
      break;
    case CAN_IT_BOF:
      CANx->ESR &= ~ ESR_BOFF; /* rw */
      break;
    case CAN_IT_WKU:
      CANx->MSR = MSR_WKUI;  /* rc_w1*/
      break;
    case CAN_IT_SLK:
      CANx->MSR = MSR_SLAKI;  /* rc_w1*/
      break;
    default :
      break;
  }
}

/// Checks whether the CAN interrupt has occurred or not.
/// `CAN_Reg`: specifies the CAN interrupt register to check.
///   It_Bit: specifies the interrupt source bit to check.
/// @retval : The new state of the CAN Interrupt (SET or RESET).
static ITStatus CheckITStatus(uint32_t CAN_Reg, uint32_t It_Bit) {
  ITStatus pendingbitstatus = RESET;
  
  if ((CAN_Reg & It_Bit) != (uint32_t)RESET) {
    /* CAN_IT is set */
    pendingbitstatus = SET;
  } else {
    /* CAN_IT is reset */
    pendingbitstatus = RESET;
  }
  return pendingbitstatus;
}


