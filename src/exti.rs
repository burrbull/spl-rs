

/// EXTI mode enumeration  

typedef enum {
  EXTI_Mode_Interrupt = 0x00,
  EXTI_Mode_Event = 0x04
}EXTIMode_TypeDef;

#define IS_EXTI_MODE(MODE) (((MODE) == EXTI_Mode_Interrupt) || ((MODE) == EXTI_Mode_Event))

/// EXTI Trigger enumeration  

typedef enum {
  EXTI_Trigger_Rising = 0x08,
  EXTI_Trigger_Falling = 0x0C,  
  EXTI_Trigger_Rising_Falling = 0x10
}EXTITrigger_TypeDef;


/// EXTI Init Structure definition  

typedef struct {
  uint32_t EXTI_Line;
  EXTIMode_TypeDef EXTI_Mode;
  EXTITrigger_TypeDef EXTI_Trigger;
  FunctionalState EXTI_LineCmd;
}EXTI_InitTypeDef;


/// Fills each EXTI_InitStruct member with its reset value.
/// `EXTI_InitStruct`: pointer to a EXTI_InitTypeDef structure
///   which will be initialized.
void EXTI_StructInit(EXTI_InitTypeDef* EXTI_InitStruct) {
  EXTI_InitStruct->EXTI_Line = EXTI_LineNone;
  EXTI_InitStruct->EXTI_Mode = EXTI_Mode_Interrupt;
  EXTI_InitStruct->EXTI_Trigger = EXTI_Trigger_Falling;
  EXTI_InitStruct->EXTI_LineCmd = DISABLE;
}


/// EXTI_Lines

#define EXTI_Line0       ((uint32_t)0x00001)  /* External interrupt line 0 */
#define EXTI_Line1       ((uint32_t)0x00002)  /* External interrupt line 1 */
#define EXTI_Line2       ((uint32_t)0x00004)  /* External interrupt line 2 */
#define EXTI_Line3       ((uint32_t)0x00008)  /* External interrupt line 3 */
#define EXTI_Line4       ((uint32_t)0x00010)  /* External interrupt line 4 */
#define EXTI_Line5       ((uint32_t)0x00020)  /* External interrupt line 5 */
#define EXTI_Line6       ((uint32_t)0x00040)  /* External interrupt line 6 */
#define EXTI_Line7       ((uint32_t)0x00080)  /* External interrupt line 7 */
#define EXTI_Line8       ((uint32_t)0x00100)  /* External interrupt line 8 */
#define EXTI_Line9       ((uint32_t)0x00200)  /* External interrupt line 9 */
#define EXTI_Line10      ((uint32_t)0x00400)  /* External interrupt line 10 */
#define EXTI_Line11      ((uint32_t)0x00800)  /* External interrupt line 11 */
#define EXTI_Line12      ((uint32_t)0x01000)  /* External interrupt line 12 */
#define EXTI_Line13      ((uint32_t)0x02000)  /* External interrupt line 13 */
#define EXTI_Line14      ((uint32_t)0x04000)  /* External interrupt line 14 */
#define EXTI_Line15      ((uint32_t)0x08000)  /* External interrupt line 15 */
#define EXTI_Line16      ((uint32_t)0x10000)  /* External interrupt line 16
                                                 Connected to the PVD Output */
#define EXTI_Line17      ((uint32_t)0x20000)  /* External interrupt line 17 
                                                 Connected to the RTC Alarm event */
#define EXTI_Line18      ((uint32_t)0x40000)  /* External interrupt line 18 
                                                 Connected to the USB Wakeup from 
                                                 suspend event */



void EXTI_DeInit(void);
void EXTI_Init(EXTI_InitTypeDef* EXTI_InitStruct);
void EXTI_StructInit(EXTI_InitTypeDef* EXTI_InitStruct);
void EXTI_GenerateSWInterrupt(uint32_t EXTI_Line);
FlagStatus EXTI_GetFlagStatus(uint32_t EXTI_Line);
void EXTI_ClearFlag(uint32_t EXTI_Line);
ITStatus EXTI_GetITStatus(uint32_t EXTI_Line);
void EXTI_ClearITPendingBit(uint32_t EXTI_Line);

#endif /* __STM32F10x_EXTI_H */



#define EXTI_LineNone    ((uint32_t)0x00000)  /* No interrupt selected */



/// Deinitializes the EXTI peripheral registers to their default 
///   reset values.
/// @param  None
void EXTI_DeInit(void) {
  EXTI->IMR = 0x00000000;
  EXTI->EMR = 0x00000000;
  EXTI->RTSR = 0x00000000; 
  EXTI->FTSR = 0x00000000; 
  EXTI->PR = 0x0007FFFF;
}

/// Initializes the EXTI peripheral according to the specified
///   parameters in the EXTI_InitStruct.
/// `EXTI_InitStruct`: pointer to a EXTI_InitTypeDef structure
///   that contains the configuration information for the EXTI
///   peripheral.
void EXTI_Init(EXTI_InitTypeDef* EXTI_InitStruct) {
     
  if (EXTI_InitStruct->EXTI_LineCmd != DISABLE) {
    /* Clear EXTI line configuration */
    EXTI->IMR &= ~EXTI_InitStruct->EXTI_Line;
    EXTI->EMR &= ~EXTI_InitStruct->EXTI_Line;
    
    *(__IO uint32_t *)(EXTI_BASE + (uint32_t)EXTI_InitStruct->EXTI_Mode)|= EXTI_InitStruct->EXTI_Line;
    /* Clear Rising Falling edge configuration */
    EXTI->RTSR &= ~EXTI_InitStruct->EXTI_Line;
    EXTI->FTSR &= ~EXTI_InitStruct->EXTI_Line;
    
    /* Select the trigger for the selected external interrupts */
    if (EXTI_InitStruct->EXTI_Trigger == EXTI_Trigger_Rising_Falling) {
      /* Rising Falling edge */
      EXTI->RTSR |= EXTI_InitStruct->EXTI_Line;
      EXTI->FTSR |= EXTI_InitStruct->EXTI_Line;
    } else {
      *(__IO uint32_t *)(EXTI_BASE + (uint32_t)EXTI_InitStruct->EXTI_Trigger)|= EXTI_InitStruct->EXTI_Line;
    }
  } else {
    /* Disable the selected external lines */
    *(__IO uint32_t *)(EXTI_BASE + (uint32_t)EXTI_InitStruct->EXTI_Mode)&= ~EXTI_InitStruct->EXTI_Line;
  }
}

/// Generates a Software interrupt.
/// `EXTI_Line`: specifies the EXTI lines to be enabled or
///   disabled.
///   This parameter can be any combination of EXTI_Linex where 
///   x can be (0..18).
void EXTI_GenerateSWInterrupt(uint32_t EXTI_Line) {
  
  EXTI->SWIER |= EXTI_Line;
}

/// Checks whether the specified EXTI line flag is set or not.
/// `EXTI_Line`: specifies the EXTI line flag to check.
///   This parameter can be:
/// * EXTI_Linex: External interrupt line x where x(0..18)
/// @retval : The new state of EXTI_Line (SET or RESET).
FlagStatus EXTI_GetFlagStatus(uint32_t EXTI_Line) {
  FlagStatus bitstatus = RESET;
  
  if ((EXTI->PR & EXTI_Line) != (uint32_t)RESET) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the EXTI’s line pending flags.
/// `EXTI_Line`: specifies the EXTI lines flags to clear.
///   This parameter can be any combination of EXTI_Linex where 
///   x can be (0..18).
void EXTI_ClearFlag(uint32_t EXTI_Line) {
  
  EXTI->PR = EXTI_Line;
}

/// Checks whether the specified EXTI line is asserted or not.
/// `EXTI_Line`: specifies the EXTI line to check.
///   This parameter can be:
/// * EXTI_Linex: External interrupt line x where x(0..18)
/// @retval : The new state of EXTI_Line (SET or RESET).
ITStatus EXTI_GetITStatus(uint32_t EXTI_Line) {
  ITStatus bitstatus = RESET;
  uint32_t enablestatus = 0;
  
  enablestatus =  EXTI->IMR & EXTI_Line;
  if (((EXTI->PR & EXTI_Line) != (uint32_t)RESET) && (enablestatus != (uint32_t)RESET)) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the EXTI’s line pending bits.
/// `EXTI_Line`: specifies the EXTI lines to clear.
///   This parameter can be any combination of EXTI_Linex where 
///   x can be (0..18).
void EXTI_ClearITPendingBit(uint32_t EXTI_Line) {
  
  EXTI->PR = EXTI_Line;
}

