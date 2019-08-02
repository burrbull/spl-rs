

/// FLASH Status  

pub enum FlashStatus { 
	BUSY = 1,
	ERROR_PG,
	ERROR_WRP,
	COMPLETE,
	TIMEOUT
}


/// FLASH_Exported_Constants

/// Flash_Latency 

#define FLASH_Latency_0                ((uint32_t)0x00000000)  /* FLASH Zero Latency cycle */
#define FLASH_Latency_1                ((uint32_t)0x00000001)  /* FLASH One Latency cycle */
#define FLASH_Latency_2                ((uint32_t)0x00000002)  /* FLASH Two Latency cycles */

/// Half_Cycle_Enable_Disable 
/*
#define FLASH_HalfCycleAccess_Enable   ((uint32_t)0x00000008)  /* FLASH Half Cycle Enable */
#define FLASH_HalfCycleAccess_Disable  ((uint32_t)0x00000000)  /* FLASH Half Cycle Disable */
*/
/// Prefetch_Buffer_Enable_Disable 
/*
#define FLASH_PrefetchBuffer_Enable    ((uint32_t)0x00000010)  /* FLASH Prefetch Buffer Enable */
#define FLASH_PrefetchBuffer_Disable   ((uint32_t)0x00000000)  /* FLASH Prefetch Buffer Disable */
*/

/// Option_Bytes_Write_Protection 

pub mod FlashWRProt {
	/// Values to be used with STM32F10Xxx Medium-density devices: FLASH memory density
	/// ranges between 32 and 128 Kbytes with page size equal to 1 Kbytes */
	pub mod MD {
		pub const Pages0to3     : u32 = 1 << 0; /* Write protection of page 0 to 3 */
		pub const Pages4to7     : u32 = 1 << 1; /* Write protection of page 4 to 7 */
		pub const Pages8to11    : u32 = 1 << 2; /* Write protection of page 8 to 11 */
		pub const Pages12to15   : u32 = 1 << 3; /* Write protection of page 12 to 15 */
		pub const Pages16to19   : u32 = 1 << 4; /* Write protection of page 16 to 19 */
		pub const Pages20to23   : u32 = 1 << 5; /* Write protection of page 20 to 23 */
		pub const Pages24to27   : u32 = 1 << 6; /* Write protection of page 24 to 27 */
		pub const Pages28to31   : u32 = 1 << 7; /* Write protection of page 28 to 31 */
		pub const Pages32to35   : u32 = 1 << 8; /* Write protection of page 32 to 35 */
		pub const Pages36to39   : u32 = 1 << 9; /* Write protection of page 36 to 39 */
		pub const Pages40to43   : u32 = 1 << 10; /* Write protection of page 40 to 43 */
		pub const Pages44to47   : u32 = 1 << 11; /* Write protection of page 44 to 47 */
		pub const Pages48to51   : u32 = 1 << 12; /* Write protection of page 48 to 51 */
		pub const Pages52to55   : u32 = 1 << 13; /* Write protection of page 52 to 55 */
		pub const Pages56to59   : u32 = 1 << 14; /* Write protection of page 56 to 59 */
		pub const Pages60to63   : u32 = 1 << 15; /* Write protection of page 60 to 63 */
		pub const Pages64to67   : u32 = 1 << 16; /* Write protection of page 64 to 67 */
		pub const Pages68to71   : u32 = 1 << 17; /* Write protection of page 68 to 71 */
		pub const Pages72to75   : u32 = 1 << 18; /* Write protection of page 72 to 75 */
		pub const Pages76to79   : u32 = 1 << 19; /* Write protection of page 76 to 79 */
		pub const Pages80to83   : u32 = 1 << 20; /* Write protection of page 80 to 83 */
		pub const Pages84to87   : u32 = 1 << 21; /* Write protection of page 84 to 87 */
		pub const Pages88to91   : u32 = 1 << 22; /* Write protection of page 88 to 91 */
		pub const Pages92to95   : u32 = 1 << 23; /* Write protection of page 92 to 95 */
		pub const Pages96to99   : u32 = 1 << 24; /* Write protection of page 96 to 99 */
		pub const Pages100to103 : u32 = 1 << 25; /* Write protection of page 100 to 103 */
		pub const Pages104to107 : u32 = 1 << 26; /* Write protection of page 104 to 107 */
		pub const Pages108to111 : u32 = 1 << 27; /* Write protection of page 108 to 111 */
		pub const Pages112to115 : u32 = 1 << 28; /* Write protection of page 112 to 115 */
		pub const Pages116to119 : u32 = 1 << 29; /* Write protection of page 115 to 119 */
		pub const Pages120to123 : u32 = 1 << 30; /* Write protection of page 120 to 123 */
		pub const Pages124to127 : u32 = 1 << 31; /* Write protection of page 124 to 127 */
	}
	/// Values to be used with STM32F10Xxx High-density devices: FLASH memory density
	/// ranges between 256 and 512 Kbytes with page size equal to 2 Kbytes */
	pub mod HD {
		pub const Pages0to1    : u32 = 1 << 0; /* Write protection of page 0 to 1 */
		pub const Pages2to3    : u32 = 1 << 1; /* Write protection of page 2 to 3 */
		pub const Pages4to5    : u32 = 1 << 2; /* Write protection of page 4 to 5 */
		pub const Pages6to7    : u32 = 1 << 3; /* Write protection of page 6 to 7 */
		pub const Pages8to9    : u32 = 1 << 4; /* Write protection of page 8 to 9 */
		pub const Pages10to11  : u32 = 1 << 5; /* Write protection of page 10 to 11 */
		pub const Pages12to13  : u32 = 1 << 6; /* Write protection of page 12 to 13 */
		pub const Pages14to15  : u32 = 1 << 7; /* Write protection of page 14 to 15 */
		pub const Pages16to17  : u32 = 1 << 8; /* Write protection of page 16 to 17 */
		pub const Pages18to19  : u32 = 1 << 9; /* Write protection of page 18 to 19 */
		pub const Pages20to21  : u32 = 1 << 10; /* Write protection of page 20 to 21 */
		pub const Pages22to23  : u32 = 1 << 11; /* Write protection of page 22 to 23 */
		pub const Pages24to25  : u32 = 1 << 12; /* Write protection of page 24 to 25 */
		pub const Pages26to27  : u32 = 1 << 13; /* Write protection of page 26 to 27 */
		pub const Pages28to29  : u32 = 1 << 14; /* Write protection of page 28 to 29 */
		pub const Pages30to31  : u32 = 1 << 15; /* Write protection of page 30 to 31 */
		pub const Pages32to33  : u32 = 1 << 16; /* Write protection of page 32 to 33 */
		pub const Pages34to35  : u32 = 1 << 17; /* Write protection of page 34 to 35 */
		pub const Pages36to37  : u32 = 1 << 18; /* Write protection of page 36 to 37 */
		pub const Pages38to39  : u32 = 1 << 19; /* Write protection of page 38 to 39 */
		pub const Pages40to41  : u32 = 1 << 20; /* Write protection of page 40 to 41 */
		pub const Pages42to43  : u32 = 1 << 21; /* Write protection of page 42 to 43 */
		pub const Pages44to45  : u32 = 1 << 22; /* Write protection of page 44 to 45 */
		pub const Pages46to47  : u32 = 1 << 23; /* Write protection of page 46 to 47 */
		pub const Pages48to49  : u32 = 1 << 24; /* Write protection of page 48 to 49 */
		pub const Pages50to51  : u32 = 1 << 25; /* Write protection of page 50 to 51 */
		pub const Pages52to53  : u32 = 1 << 26; /* Write protection of page 52 to 53 */
		pub const Pages54to55  : u32 = 1 << 27; /* Write protection of page 54 to 55 */
		pub const Pages56to57  : u32 = 1 << 28; /* Write protection of page 56 to 57 */
		pub const Pages58to59  : u32 = 1 << 29; /* Write protection of page 58 to 59 */
		pub const Pages60to61  : u32 = 1 << 30; /* Write protection of page 60 to 61 */
		pub const Pages62to255 : u32 = 1 << 31; /* Write protection of page 62 to 255 */
	}
	pub const AllPages     : u32 = 0xFFFFFFFF; /* Write protection of all Pages */
}

/// Option_Bytes_IWatchdog 

#define OB_IWDG_SW                     ((uint16_t)0x0001)  /* Software IWDG selected */
#define OB_IWDG_HW                     ((uint16_t)0x0000)  /* Hardware IWDG selected */


/// Option_Bytes_nRST_STOP 

#define OB_STOP_NoRST                  ((uint16_t)0x0002) /* No reset generated when entering in STOP */
#define OB_STOP_RST                    ((uint16_t)0x0000) /* Reset generated when entering in STOP */


/// Option_Bytes_nRST_STDBY 

#define OB_STDBY_NoRST                 ((uint16_t)0x0004) /* No reset generated when entering in STANDBY */
#define OB_STDBY_RST                   ((uint16_t)0x0000) /* Reset generated when entering in STANDBY */


/// FLASH_Interrupts 

#define FLASH_IT_ERROR                 ((uint32_t)0x00000400)  /* FPEC error interrupt source */
#define FLASH_IT_EOP                   ((uint32_t)0x00001000)  /* End of FLASH Operation Interrupt source */


/// FLASH_Flags 

#define FLASH_FLAG_BSY                 ((uint32_t)0x00000001)  /* FLASH Busy flag */
#define FLASH_FLAG_EOP                 ((uint32_t)0x00000020)  /* FLASH End of Operation flag */
#define FLASH_FLAG_PGERR               ((uint32_t)0x00000004)  /* FLASH Program error flag */
#define FLASH_FLAG_WRPRTERR            ((uint32_t)0x00000010)  /* FLASH Write protected error flag */
#define FLASH_FLAG_OPTERR              ((uint32_t)0x00000001)  /* FLASH Option Byte error flag */



/// FLASH_Exported_Macros


/// FLASH_Exported_Functions

void FLASH_SetLatency(uint32_t FLASH_Latency);
void FLASH_HalfCycleAccessCmd(uint32_t FLASH_HalfCycleAccess);
void FLASH_PrefetchBufferCmd(uint32_t FLASH_PrefetchBuffer);
void FLASH_Unlock(void);
void FLASH_Lock(void);
FLASH_Status FLASH_ErasePage(uint32_t Page_Address);
FLASH_Status FLASH_EraseAllPages(void);
FLASH_Status FLASH_EraseOptionBytes(void);
FLASH_Status FLASH_ProgramWord(uint32_t Address, uint32_t Data);
FLASH_Status FLASH_ProgramHalfWord(uint32_t Address, uint16_t Data);
FLASH_Status FLASH_ProgramOptionByteData(uint32_t Address, uint8_t Data);
FLASH_Status FLASH_EnableWriteProtection(uint32_t FLASH_Pages);
FLASH_Status FLASH_ReadOutProtection(new_state : bool);
FLASH_Status FLASH_UserOptionByteConfig(uint16_t OB_IWDG, uint16_t OB_STOP, uint16_t OB_STDBY);
uint32_t FLASH_GetUserOptionByte(void);
uint32_t FLASH_GetWriteProtectionOptionByte(void);
FlagStatus FLASH_GetReadOutProtectionStatus(void);
FlagStatus FLASH_GetPrefetchBufferStatus(void);
void FLASH_ITConfig(uint16_t FLASH_IT, new_state : bool);
FlagStatus FLASH_GetFlagStatus(uint16_t FLASH_FLAG);
void FLASH_ClearFlag(uint16_t FLASH_FLAG);
FLASH_Status FLASH_GetStatus(void);
FLASH_Status FLASH_WaitForLastOperation(uint32_t Timeout);




/* Flash Access Control Register bits */
#define ACR_LATENCY_Mask         ((uint32_t)0x00000038)
#define ACR_HLFCYA_Mask          ((uint32_t)0xFFFFFFF7)
#define ACR_PRFTBE_Mask          ((uint32_t)0xFFFFFFEF)

/* Flash Access Control Register bits */
#define ACR_PRFTBS_Mask          ((uint32_t)0x00000020) 

/* Flash Control Register bits */
#define CR_PG_Set                ((uint32_t)0x00000001)
#define CR_PG_Reset              ((uint32_t)0x00001FFE) 
#define CR_PER_Set               ((uint32_t)0x00000002)
#define CR_PER_Reset             ((uint32_t)0x00001FFD)
#define CR_MER_Set               ((uint32_t)0x00000004)
#define CR_MER_Reset             ((uint32_t)0x00001FFB)
#define CR_OPTPG_Set             ((uint32_t)0x00000010)
#define CR_OPTPG_Reset           ((uint32_t)0x00001FEF)
#define CR_OPTER_Set             ((uint32_t)0x00000020)
#define CR_OPTER_Reset           ((uint32_t)0x00001FDF)
#define CR_STRT_Set              ((uint32_t)0x00000040)
#define CR_LOCK_Set              ((uint32_t)0x00000080)

/* FLASH Mask */
#define RDPRT_Mask               ((uint32_t)0x00000002)
#define WRP0_Mask                ((uint32_t)0x000000FF)
#define WRP1_Mask                ((uint32_t)0x0000FF00)
#define WRP2_Mask                ((uint32_t)0x00FF0000)
#define WRP3_Mask                ((uint32_t)0xFF000000)

/* FLASH Keys */
#define RDP_Key                  ((uint16_t)0x00A5)
#define FLASH_KEY1               ((uint32_t)0x45670123)
#define FLASH_KEY2               ((uint32_t)0xCDEF89AB)

/* Delay definition */   
#define EraseTimeout             ((uint32_t)0x00000FFF)
#define ProgramTimeout           ((uint32_t)0x0000000F)



static void delay(void);

/// FLASH_Private_Functions

/// Sets the code latency value.
/// `FLASH_Latency`: specifies the FLASH Latency value.
///   This parameter can be one of the following values:
/// * FLASH_Latency_0: FLASH Zero Latency cycle
/// * FLASH_Latency_1: FLASH One Latency cycle
/// * FLASH_Latency_2: FLASH Two Latency cycles
fn SetLatency(&self, uint32_t FLASH_Latency) {
  uint32_t tmpreg = 0;
  
  
  /* Read the ACR register */
  tmpreg = FLASH->ACR;  
  
  /* Sets the Latency value */
  tmpreg &= ACR_LATENCY_Mask;
  tmpreg |= FLASH_Latency;
  
  /* Write the ACR register */
  FLASH->ACR = tmpreg;
}

/// Enables or disables the Half cycle flash access.
/// `FLASH_HalfCycleAccess`: specifies the FLASH Half cycle Access mode.
///   This parameter can be one of the following values:
/// * FLASH_HalfCycleAccess_Enable: FLASH Half Cycle Enable
/// * FLASH_HalfCycleAccess_Disable: FLASH Half Cycle Disable
fn HalfCycleAccessCmd(&self, uint32_t FLASH_HalfCycleAccess : bool) {
  
  /* Enable or disable the Half cycle access */
  FLASH->ACR &= ACR_HLFCYA_Mask;
  FLASH->ACR |= FLASH_HalfCycleAccess;
}

/// Enables or disables the Prefetch Buffer.
/// `FLASH_PrefetchBuffer`: specifies the Prefetch buffer status.
///   This parameter can be one of the following values:
/// * FLASH_PrefetchBuffer_Enable: FLASH Prefetch Buffer Enable
/// * FLASH_PrefetchBuffer_Disable: FLASH Prefetch Buffer Disable
fn PrefetchBufferCmd(&self, uint32_t FLASH_PrefetchBuffer : bool) {
  
  /* Enable or disable the Prefetch Buffer */
  FLASH->ACR &= ACR_PRFTBE_Mask;
  FLASH->ACR |= FLASH_PrefetchBuffer;
}

/// Unlocks the FLASH Program Erase Controller.
void FLASH_Unlock(void) {
  /* Authorize the FPEC Access */
  FLASH->KEYR = FLASH_KEY1;
  FLASH->KEYR = FLASH_KEY2;
}

/// Locks the FLASH Program Erase Controller.
void FLASH_Lock(void) {
  /* Set the Lock Bit to lock the FPEC and the FCR */
  FLASH->CR |= CR_LOCK_Set;
}

/// Erases a specified FLASH page.
/// `Page_Address`: The page address to be erased.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT.
fn ErasePage(&self, uint32_t Page_Address) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE;
  /* Wait for last operation to be completed */
  status = FLASH_WaitForLastOperation(EraseTimeout);
  
  if(status == FLASH_COMPLETE) { 
    /* if the previous operation is completed, proceed to erase the page */
    FLASH->CR|= CR_PER_Set;
    FLASH->AR = Page_Address; 
    FLASH->CR|= CR_STRT_Set;
    
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(EraseTimeout);
    if(status != FLASH_BUSY) {
      /* if the erase operation is completed, disable the PER Bit */
      FLASH->CR &= CR_PER_Reset;
    }
  }
  /* Return the Erase Status */
  return status;
}

/// Erases all FLASH pages.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT.
fn EraseAllPages(&self) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE;
  /* Wait for last operation to be completed */
  status = FLASH_WaitForLastOperation(EraseTimeout);
  
  if(status == FLASH_COMPLETE) {
    /* if the previous operation is completed, proceed to erase all pages */
     FLASH->CR |= CR_MER_Set;
     FLASH->CR |= CR_STRT_Set;
    
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(EraseTimeout);
    if(status != FLASH_BUSY) {
      /* if the erase operation is completed, disable the MER Bit */
      FLASH->CR &= CR_MER_Reset;
    }
  }	   
  /* Return the Erase Status */
  return status;
}

/// Erases the FLASH option bytes.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT.
fn EraseOptionBytes(&self) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE;
  
  /* Wait for last operation to be completed */
  status = FLASH_WaitForLastOperation(EraseTimeout);
  if(status == FLASH_COMPLETE) {
    /* Authorize the small information block programming */
    FLASH->OPTKEYR = FLASH_KEY1;
    FLASH->OPTKEYR = FLASH_KEY2;
    
    /* if the previous operation is completed, proceed to erase the option bytes */
    FLASH->CR |= CR_OPTER_Set;
    FLASH->CR |= CR_STRT_Set;
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(EraseTimeout);
    
    if(status == FLASH_COMPLETE) {
      /* if the erase operation is completed, disable the OPTER Bit */
      FLASH->CR &= CR_OPTER_Reset;
       
      /* Enable the Option Bytes Programming operation */
      FLASH->CR |= CR_OPTPG_Set;
      /* Enable the readout access */
      OB->RDP= RDP_Key; 
      /* Wait for last operation to be completed */
      status = FLASH_WaitForLastOperation(ProgramTimeout);
 
      if(status != FLASH_BUSY) {
        /* if the program operation is completed, disable the OPTPG Bit */
        FLASH->CR &= CR_OPTPG_Reset;
      }
    } else {
      if (status != FLASH_BUSY) {
        /* Disable the OPTPG Bit */
        FLASH->CR &= CR_OPTPG_Reset;
      }
    }  
  }
  /* Return the erase status */
  return status;
}

/// Programs a word at a specified address.
/// `Address`: specifies the address to be programmed.
/// `Data`: specifies the data to be programmed.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT. 
fn ProgramWord(&self, uint32_t Address, uint32_t Data) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE;
  /* Wait for last operation to be completed */
  status = FLASH_WaitForLastOperation(ProgramTimeout);
  
  if(status == FLASH_COMPLETE) {
    /* if the previous operation is completed, proceed to program the new first 
    half word */
    FLASH->CR |= CR_PG_Set;
  
    *(__IO uint16_t*)Address = (uint16_t)Data;
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(ProgramTimeout);
 
    if(status == FLASH_COMPLETE) {
      /* if the previous operation is completed, proceed to program the new second 
      half word */
      *(__IO uint16_t*)(Address + 2) = Data >> 16;
    
      /* Wait for last operation to be completed */
      status = FLASH_WaitForLastOperation(ProgramTimeout);
        
      if(status != FLASH_BUSY) {
        /* Disable the PG Bit */
        FLASH->CR &= CR_PG_Reset;
      }
    } else {
      if (status != FLASH_BUSY) {
        /* Disable the PG Bit */
        FLASH->CR &= CR_PG_Reset;
      }
     }
  }
  /* Return the Program Status */
  return status;
}

/// Programs a half word at a specified address.
/// `Address`: specifies the address to be programmed.
/// `Data`: specifies the data to be programmed.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT. 
fn ProgramHalfWord(&self, uint32_t Address, uint16_t Data) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE;
  /* Wait for last operation to be completed */
  status = FLASH_WaitForLastOperation(ProgramTimeout);
  
  if(status == FLASH_COMPLETE) {
    /* if the previous operation is completed, proceed to program the new data */
    FLASH->CR |= CR_PG_Set;
  
    *(__IO uint16_t*)Address = Data;
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(ProgramTimeout);
    if(status != FLASH_BUSY) {
      /* if the program operation is completed, disable the PG Bit */
      FLASH->CR &= CR_PG_Reset;
    }
  } 
  /* Return the Program Status */
  return status;
}

/// Programs a half word at a specified Option Byte Data address.
/// `Address`: specifies the address to be programmed.
///   This parameter can be 0x1FFFF804 or 0x1FFFF806. 
/// `Data`: specifies the data to be programmed.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT. 
fn ProgramOptionByteData(&self, uint32_t Address, uint8_t Data) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE;
  status = FLASH_WaitForLastOperation(ProgramTimeout);
  if(status == FLASH_COMPLETE) {
    /* Authorize the small information block programming */
    FLASH->OPTKEYR = FLASH_KEY1;
    FLASH->OPTKEYR = FLASH_KEY2;
    /* Enables the Option Bytes Programming operation */
    FLASH->CR |= CR_OPTPG_Set; 
    *(__IO uint16_t*)Address = Data;
    
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(ProgramTimeout);
    if(status != FLASH_BUSY) {
      /* if the program operation is completed, disable the OPTPG Bit */
      FLASH->CR &= CR_OPTPG_Reset;
    }
  }    
  /* Return the Option Byte Data Program Status */
  return status;
}

/// Write protects the desired pages
/// `FLASH_Pages`: specifies the address of the pages to be 
///   write protected. This parameter can be:
/// * For STM32F10Xxx Medium-density devices (FLASH page size equal to 1 KB)
/// A value between FLASH_WRProt_Pages0to3 and FLASH_WRProt_Pages124to127
/// * For STM32F10Xxx High-density devices (FLASH page size equal to 2 KB) 
/// A value between FLASH_WRProt_Pages0to1 and  FLASH_WRProt_Pages60to61 
/// or FLASH_WRProt_Pages62to255 
/// * FLASH_WRProt_AllPages
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT.
fn EnableWriteProtection(&self, uint32_t FLASH_Pages) -> FlashStatus {
  uint16_t WRP0_Data = 0xFFFF, WRP1_Data = 0xFFFF, WRP2_Data = 0xFFFF, WRP3_Data = 0xFFFF;
  
  FLASH_Status status = FLASH_COMPLETE;
  
  FLASH_Pages = (uint32_t)(~FLASH_Pages);
  WRP0_Data = (uint16_t)(FLASH_Pages & WRP0_Mask);
  WRP1_Data = (uint16_t)((FLASH_Pages & WRP1_Mask) >> 8);
  WRP2_Data = (uint16_t)((FLASH_Pages & WRP2_Mask) >> 16);
  WRP3_Data = (uint16_t)((FLASH_Pages & WRP3_Mask) >> 24);
  
  /* Wait for last operation to be completed */
  status = FLASH_WaitForLastOperation(ProgramTimeout);
  
  if(status == FLASH_COMPLETE) {
    /* Authorizes the small information block programming */
    FLASH->OPTKEYR = FLASH_KEY1;
    FLASH->OPTKEYR = FLASH_KEY2;
    FLASH->CR |= CR_OPTPG_Set;
    if(WRP0_Data != 0xFF) {
      OB->WRP0 = WRP0_Data;
      
      /* Wait for last operation to be completed */
      status = FLASH_WaitForLastOperation(ProgramTimeout);
    }
    if((status == FLASH_COMPLETE) && (WRP1_Data != 0xFF)) {
      OB->WRP1 = WRP1_Data;
      
      /* Wait for last operation to be completed */
      status = FLASH_WaitForLastOperation(ProgramTimeout);
    }
    if((status == FLASH_COMPLETE) && (WRP2_Data != 0xFF)) {
      OB->WRP2 = WRP2_Data;
      
      /* Wait for last operation to be completed */
      status = FLASH_WaitForLastOperation(ProgramTimeout);
    }
    
    if((status == FLASH_COMPLETE)&& (WRP3_Data != 0xFF)) {
      OB->WRP3 = WRP3_Data;
     
      /* Wait for last operation to be completed */
      status = FLASH_WaitForLastOperation(ProgramTimeout);
    }
          
    if(status != FLASH_BUSY) {
      /* if the program operation is completed, disable the OPTPG Bit */
      FLASH->CR &= CR_OPTPG_Reset;
    }
  } 
  /* Return the write protection operation Status */
  return status;       
}

/// Enables or disables the read out protection.
///   If the user has already programmed the other option bytes before 
///   calling this function, he must re-program them since this 
///   function erases all option bytes.
/// `Newstate`: new state of the ReadOut Protection.
///   This parameter can be: ENABLE or DISABLE.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT.
fn ReadOutProtection(&self, new_state : bool) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE;
  status = FLASH_WaitForLastOperation(EraseTimeout);
  if(status == FLASH_COMPLETE) {
    /* Authorizes the small information block programming */
    FLASH->OPTKEYR = FLASH_KEY1;
    FLASH->OPTKEYR = FLASH_KEY2;
    FLASH->CR |= CR_OPTER_Set;
    FLASH->CR |= CR_STRT_Set;
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(EraseTimeout);
    if(status == FLASH_COMPLETE) {
      /* if the erase operation is completed, disable the OPTER Bit */
      FLASH->CR &= CR_OPTER_Reset;
      /* Enable the Option Bytes Programming operation */
      FLASH->CR |= CR_OPTPG_Set; 
      if(new_state : FunctionalState != DISABLE) {
        OB->RDP = 0x00;
      } else {
        OB->RDP = RDP_Key;  
      }
      /* Wait for last operation to be completed */
      status = FLASH_WaitForLastOperation(EraseTimeout); 
    
      if(status != FLASH_BUSY) {
        /* if the program operation is completed, disable the OPTPG Bit */
        FLASH->CR &= CR_OPTPG_Reset;
      }
    } else  {
      if(status != FLASH_BUSY) {
        /* Disable the OPTER Bit */
        FLASH->CR &= CR_OPTER_Reset;
      }
    }
  }
  /* Return the protection operation Status */
  return status;      
}

/// Programs the FLASH User Option Byte: IWDG_SW / RST_STOP /
///   RST_STDBY.
/// `OB_IWDG`: Selects the IWDG mode
///   This parameter can be one of the following values:
/// * OB_IWDG_SW: Software IWDG selected
/// * OB_IWDG_HW: Hardware IWDG selected
/// `OB_STOP`: Reset event when entering STOP mode.
///   This parameter can be one of the following values:
/// * OB_STOP_NoRST: No reset generated when entering in STOP
/// * OB_STOP_RST: Reset generated when entering in STOP
/// `OB_STDBY`: Reset event when entering Standby mode.
///   This parameter can be one of the following values:
/// * OB_STDBY_NoRST: No reset generated when entering in STANDBY
/// * OB_STDBY_RST: Reset generated when entering in STANDBY
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT.
fn UserOptionByteConfig(&self, uint16_t OB_IWDG, uint16_t OB_STOP, uint16_t OB_STDBY) -> FlashStatus {
  FLASH_Status status = FLASH_COMPLETE; 
  /* Authorize the small information block programming */
  FLASH->OPTKEYR = FLASH_KEY1;
  FLASH->OPTKEYR = FLASH_KEY2;
  
  /* Wait for last operation to be completed */
  status = FLASH_WaitForLastOperation(ProgramTimeout);
  
  if(status == FLASH_COMPLETE) {  
    /* Enable the Option Bytes Programming operation */
    FLASH->CR |= CR_OPTPG_Set; 
           
    OB->USER = ( OB_IWDG | OB_STOP |OB_STDBY) | (uint16_t)0xF8; 
  
    /* Wait for last operation to be completed */
    status = FLASH_WaitForLastOperation(ProgramTimeout);
    if(status != FLASH_BUSY) {
      /* if the program operation is completed, disable the OPTPG Bit */
      FLASH->CR &= CR_OPTPG_Reset;
    }
  }    
  /* Return the Option Byte program Status */
  return status;
}

/// Returns the FLASH User Option Bytes values.
/// @retval : The FLASH User Option Bytes values:IWDG_SW(Bit0), RST_STOP(Bit1)
///   and RST_STDBY(Bit2).
fn GetUserOptionByte(&self) -> u32 {
  /* Return the User Option Byte */
  return (uint32_t)(FLASH->OBR >> 2);
}

/// Returns the FLASH Write Protection Option Bytes Register value.
/// @retval : The FLASH Write Protection  Option Bytes Register value
fn GetWriteProtectionOptionByte(&self) -> u32 {
  /* Return the Falsh write protection Register value */
  return (uint32_t)(FLASH->WRPR);
}

/// Checks whether the FLASH Read Out Protection Status is set 
///   or not.
/// @retval : FLASH ReadOut Protection Status(SET or RESET)
fn GetReadOutProtectionStatus(&self) -> FlagStatus {
  FlagStatus readoutstatus = RESET;
  if ((FLASH->OBR & RDPRT_Mask) != (uint32_t)RESET) {
    readoutstatus = SET;
  } else {
    readoutstatus = RESET;
  }
  return readoutstatus;
}

/// Checks whether the FLASH Prefetch Buffer status is set or not.
/// @retval : FLASH Prefetch Buffer Status (SET or RESET).
fn GetPrefetchBufferStatus(&self) -> FlagStatus{
  FlagStatus bitstatus = RESET;
  
  if ((FLASH->ACR & ACR_PRFTBS_Mask) != (uint32_t)RESET) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  /* Return the new state of FLASH Prefetch Buffer Status (SET or RESET) */
  return bitstatus; 
}

/// Returns the FLASH Status.
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP or FLASH_COMPLETE
fn GetStatus(&self) -> FlashStatus {
  FLASH_Status flashstatus = FLASH_COMPLETE;
  
  if((FLASH->SR & FLASH_FLAG_BSY) == FLASH_FLAG_BSY)  {
    flashstatus = FLASH_BUSY;
  } else  {  
    if(FLASH->SR & FLASH_FLAG_PGERR) { 
      flashstatus = FLASH_ERROR_PG;
    } else  {
      if(FLASH->SR & FLASH_FLAG_WRPRTERR) {
        flashstatus = FLASH_ERROR_WRP;
      } else {
        flashstatus = FLASH_COMPLETE;
      }
    }
  }
  /* Return the Flash Status */
  return flashstatus;
}

/// Waits for a Flash operation to complete or a TIMEOUT to occur.
/// `Timeout`: FLASH progamming Timeout
/// @retval : FLASH Status: The returned value can be: FLASH_BUSY, 
///   FLASH_ERROR_PG, FLASH_ERROR_WRP, FLASH_COMPLETE or 
///   FLASH_TIMEOUT.
fn WaitForLastOperation(&self, uint32_t Timeout) -> FlashStatus { 
  FLASH_Status status = FLASH_COMPLETE;
   
  /* Check for the Flash Status */
  status = FLASH_GetStatus();
  /* Wait for a Flash operation to complete or a TIMEOUT to occur */
  while((status == FLASH_BUSY) && (Timeout != 0x00)) {
    delay();
    status = FLASH_GetStatus();
    Timeout--;
  }
  if(Timeout == 0x00 ) {
    status = FLASH_TIMEOUT;
  }
  /* Return the operation status */
  return status;
}

/// Inserts a time delay.
static void delay(void) {
  __IO uint32_t i = 0;
  for(i = 0xFF; i != 0; i--) {
  }
}














/// Enables or disables the specified FLASH interrupts.
/// `FLASH_IT`: specifies the FLASH interrupt sources to be 
///   enabled or disabled.
///   This parameter can be any combination of the following values:
/// * FLASH_IT_ERROR: FLASH Error Interrupt
/// * FLASH_IT_EOP: FLASH end of operation Interrupt
/// `new_state : FunctionalState`: new state of the specified Flash interrupts.
///   This parameter can be: ENABLE or DISABLE.       
fn ITConfig(&self, uint16_t FLASH_IT, new_state : bool) {
  if(new_state : FunctionalState != DISABLE) {
    /* Enable the interrupt sources */
    FLASH->CR |= FLASH_IT;
  } else {
    /* Disable the interrupt sources */
    FLASH->CR &= ~(uint32_t)FLASH_IT;
  }
}


/// Checks whether the specified FLASH flag is set or not.
/// `FLASH_FLAG`: specifies the FLASH flag to check.
///   This parameter can be one of the following values:
/// * FLASH_FLAG_BSY: FLASH Busy flag           
/// * FLASH_FLAG_PGERR: FLASH Program error flag       
/// * FLASH_FLAG_WRPRTERR: FLASH Write protected error flag      
/// * FLASH_FLAG_EOP: FLASH End of Operation flag           
/// * FLASH_FLAG_OPTERR:  FLASH Option Byte error flag     
/// @retval : The new state of FLASH_FLAG (SET or RESET).
fn GetFlagStatus(&self, uint16_t FLASH_FLAG) -> FlagStatus {
  FlagStatus bitstatus = RESET;
  if(FLASH_FLAG == FLASH_FLAG_OPTERR)  {
    if((FLASH->OBR & FLASH_FLAG_OPTERR) != (uint32_t)RESET) {
      bitstatus = SET;
    } else {
      bitstatus = RESET;
    }
  } else {
   if((FLASH->SR & FLASH_FLAG) != (uint32_t)RESET) {
      bitstatus = SET;
    } else {
      bitstatus = RESET;
    }
  }
  /* Return the new state of FLASH_FLAG (SET or RESET) */
  return bitstatus;
}



/// Clears the FLASH’s pending flags.
/// `FLASH_FLAG`: specifies the FLASH flags to clear.
///   This parameter can be any combination of the following values:
/// * FLASH_FLAG_BSY: FLASH Busy flag           
/// * FLASH_FLAG_PGERR: FLASH Program error flag       
/// * FLASH_FLAG_WRPRTERR: FLASH Write protected error flag      
/// * FLASH_FLAG_EOP: FLASH End of Operation flag           
fn ClearFlag(&self, uint16_t FLASH_FLAG) {
  
  /* Clear the flags */
  FLASH->SR = FLASH_FLAG;
}
