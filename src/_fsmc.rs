

/// Timing parameters For NOR/SRAM Banks  
pub struct FsmcNORSRAMTimingStruct {
  uint32_t FSMC_AddressSetupTime;
  uint32_t FSMC_AddressHoldTime;
  uint32_t FSMC_DataSetupTime;
  uint32_t FSMC_BusTurnAroundDuration;
  uint32_t FSMC_CLKDivision;
  uint32_t FSMC_DataLatency;
  uint32_t FSMC_AccessMode;
}

/// FSMC NOR/SRAM Init structure definition
pub struct FsmcNORSRAMStruct {
  uint32_t FSMC_Bank;
  uint32_t FSMC_DataAddressMux;
  uint32_t FSMC_MemoryType;
  uint32_t FSMC_MemoryDataWidth;
  uint32_t FSMC_BurstAccessMode;
  uint32_t FSMC_WaitSignalPolarity;
  uint32_t FSMC_WrapMode;
  uint32_t FSMC_WaitSignalActive;
  uint32_t FSMC_WriteOperation;
  uint32_t FSMC_WaitSignal;
  uint32_t FSMC_ExtendedMode;
  uint32_t FSMC_WriteBurst;  
	FSMC_ReadWriteTimingStruct : FsmcNORSRAMTimingStruct,/* Timing Parameters for write and read access if the  ExtendedMode is not used*/
	FSMC_WriteTimingStruct     : FsmcNORSRAMTimingStruct/* Timing Parameters for write access if the  ExtendedMode is used*/
}


/// Fills each FSMC_NORSRAMInitStruct member with its default value.
/// `FSMC_NORSRAMInitStruct`: pointer to a FSMC_NORSRAMInitTypeDef 
///   structure which will be initialized.
void FSMC_NORSRAMStructInit(FSMC_NORSRAMInitTypeDef* FSMC_NORSRAMInitStruct) {  
  /* Reset NOR/SRAM Init structure parameters values */
  FSMC_NORSRAMInitStruct->FSMC_Bank = FSMC_Bank1_NORSRAM1;
  FSMC_NORSRAMInitStruct->FSMC_DataAddressMux = FSMC_DataAddressMux_Enable;
  FSMC_NORSRAMInitStruct->FSMC_MemoryType = FSMC_MemoryType_SRAM;
  FSMC_NORSRAMInitStruct->FSMC_MemoryDataWidth = FSMC_MemoryDataWidth_8b;
  FSMC_NORSRAMInitStruct->FSMC_BurstAccessMode = FSMC_BurstAccessMode_Disable;
  FSMC_NORSRAMInitStruct->FSMC_WaitSignalPolarity = FSMC_WaitSignalPolarity_Low;
  FSMC_NORSRAMInitStruct->FSMC_WrapMode = FSMC_WrapMode_Disable;
  FSMC_NORSRAMInitStruct->FSMC_WaitSignalActive = FSMC_WaitSignalActive_BeforeWaitState;
  FSMC_NORSRAMInitStruct->FSMC_WriteOperation = FSMC_WriteOperation_Enable;
  FSMC_NORSRAMInitStruct->FSMC_WaitSignal = FSMC_WaitSignal_Enable;
  FSMC_NORSRAMInitStruct->FSMC_ExtendedMode = FSMC_ExtendedMode_Disable;
  FSMC_NORSRAMInitStruct->FSMC_WriteBurst = FSMC_WriteBurst_Disable;
  FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_AddressSetupTime = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_AddressHoldTime = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_DataSetupTime = 0xFF;
  FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_BusTurnAroundDuration = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_CLKDivision = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_DataLatency = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_AccessMode = FSMC_AccessMode_A; 
  FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_AddressSetupTime = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_AddressHoldTime = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_DataSetupTime = 0xFF;
  FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_BusTurnAroundDuration = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_CLKDivision = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_DataLatency = 0xF;
  FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_AccessMode = FSMC_AccessMode_A;
}

/// Timing parameters For FSMC NAND and PCCARD Banks
pub struct FsmcNAND_PCCARDTimingStruct {
  uint32_t FSMC_SetupTime;
  uint32_t FSMC_WaitSetupTime;
  uint32_t FSMC_HoldSetupTime;
  uint32_t FSMC_HiZSetupTime;
}

/// FSMC NAND Init structure definition
pub struct FsmcNANDStruct {
  uint32_t FSMC_Bank;
  uint32_t FSMC_Waitfeature;
  uint32_t FSMC_MemoryDataWidth;
  uint32_t FSMC_ECC;
  uint32_t FSMC_ECCPageSize;
  uint32_t FSMC_TCLRSetupTime;
  uint32_t FSMC_TARSetupTime;  
	FSMC_CommonSpaceTimingStruct    : FsmcNAND_PCCARDTimingStruct,/* FSMC Common Space Timing */ 
	FSMC_AttributeSpaceTimingStruct : FsmcNAND_PCCARDTimingStruct/* FSMC Attribute Space Timing */
}

/// FSMC PCCARD Init structure definition
pub struct FsmcPCCARDStruct {
  uint32_t FSMC_Waitfeature;
  uint32_t FSMC_TCLRSetupTime;
  uint32_t FSMC_TARSetupTime;  
	FSMC_CommonSpaceTimingStruct    : FsmcNAND_PCCARDTimingStruct,/* FSMC Common Space Timing */
	FSMC_AttributeSpaceTimingStruct : FsmcNAND_PCCARDTimingStruct,  /* FSMC Attribute Space Timing */
	FSMC_IOSpaceTimingStruct        : FsmcNAND_PCCARDTimingStruct /* FSMC IO Space Timing */
}


/// Fills each FSMC_NANDInitStruct member with its default value.
/// `FSMC_NANDInitStruct`: pointer to a FSMC_NANDInitTypeDef 
///   structure which will be initialized.
void FSMC_NANDStructInit(FSMC_NANDInitTypeDef* FSMC_NANDInitStruct) { 
  /* Reset NAND Init structure parameters values */
  FSMC_NANDInitStruct->FSMC_Bank = FSMC_Bank2_NAND;
  FSMC_NANDInitStruct->FSMC_Waitfeature = FSMC_Waitfeature_Disable;
  FSMC_NANDInitStruct->FSMC_MemoryDataWidth = FSMC_MemoryDataWidth_8b;
  FSMC_NANDInitStruct->FSMC_ECC = FSMC_ECC_Disable;
  FSMC_NANDInitStruct->FSMC_ECCPageSize = FSMC_ECCPageSize_256Bytes;
  FSMC_NANDInitStruct->FSMC_TCLRSetupTime = 0x0;
  FSMC_NANDInitStruct->FSMC_TARSetupTime = 0x0;
  FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_SetupTime = 0xFC;
  FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_WaitSetupTime = 0xFC;
  FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HoldSetupTime = 0xFC;
  FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HiZSetupTime = 0xFC;
  FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_SetupTime = 0xFC;
  FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_WaitSetupTime = 0xFC;
  FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HoldSetupTime = 0xFC;
  FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HiZSetupTime = 0xFC;	  
}

/// Fills each FSMC_PCCARDInitStruct member with its default value.
/// `FSMC_PCCARDInitStruct`: pointer to a FSMC_PCCARDInitTypeDef 
///   structure which will be initialized.
void FSMC_PCCARDStructInit(FSMC_PCCARDInitTypeDef* FSMC_PCCARDInitStruct) {
  /* Reset PCCARD Init structure parameters values */
  FSMC_PCCARDInitStruct->FSMC_Waitfeature = FSMC_Waitfeature_Disable;
  FSMC_PCCARDInitStruct->FSMC_TCLRSetupTime = 0x0;
  FSMC_PCCARDInitStruct->FSMC_TARSetupTime = 0x0;
  FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_SetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_WaitSetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HoldSetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HiZSetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_SetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_WaitSetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HoldSetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HiZSetupTime = 0xFC;	
  FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_SetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_WaitSetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_HoldSetupTime = 0xFC;
  FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_HiZSetupTime = 0xFC;
}

/// FSMC_Exported_Constants

/// FSMC_Banks_definitions 

#define FSMC_Bank1_NORSRAM1                             ((uint32_t)0x00000000)
#define FSMC_Bank1_NORSRAM2                             ((uint32_t)0x00000002)
#define FSMC_Bank1_NORSRAM3                             ((uint32_t)0x00000004)
#define FSMC_Bank1_NORSRAM4                             ((uint32_t)0x00000006)
#define FSMC_Bank2_NAND                                 ((uint32_t)0x00000010)
#define FSMC_Bank3_NAND                                 ((uint32_t)0x00000100)
#define FSMC_Bank4_PCCARD                               ((uint32_t)0x00001000)

#define IS_FSMC_NORSRAM_BANK(BANK) (((BANK) == FSMC_Bank1_NORSRAM1) || \
                                    ((BANK) == FSMC_Bank1_NORSRAM2) || \
                                    ((BANK) == FSMC_Bank1_NORSRAM3) || \
                                    ((BANK) == FSMC_Bank1_NORSRAM4))

#define IS_FSMC_NAND_BANK(BANK) (((BANK) == FSMC_Bank2_NAND) || \
                                 ((BANK) == FSMC_Bank3_NAND))

#define IS_FSMC_GETFLAG_BANK(BANK) (((BANK) == FSMC_Bank2_NAND) || \
                                    ((BANK) == FSMC_Bank3_NAND) || \
                                    ((BANK) == FSMC_Bank4_PCCARD))

#define IS_FSMC_IT_BANK(BANK) (((BANK) == FSMC_Bank2_NAND) || \
                               ((BANK) == FSMC_Bank3_NAND) || \
                               ((BANK) == FSMC_Bank4_PCCARD))

/// NOR_SRAM_Banks 

/// FSMC_Data_Address_Bus_Multiplexing 

#define FSMC_DataAddressMux_Disable                       ((uint32_t)0x00000000)
#define FSMC_DataAddressMux_Enable                        ((uint32_t)0x00000002)


/// FSMC_Memory_Type 

#define FSMC_MemoryType_SRAM                            ((uint32_t)0x00000000)
#define FSMC_MemoryType_PSRAM                           ((uint32_t)0x00000004)
#define FSMC_MemoryType_NOR                             ((uint32_t)0x00000008)


/// FSMC_Data_Width 

#define FSMC_MemoryDataWidth_8b                         ((uint32_t)0x00000000)
#define FSMC_MemoryDataWidth_16b                        ((uint32_t)0x00000010)


/// FSMC_Burst_Access_Mode 

#define FSMC_BurstAccessMode_Disable                    ((uint32_t)0x00000000) 
#define FSMC_BurstAccessMode_Enable                     ((uint32_t)0x00000100)

/// FSMC_Wait_Signal_Polarity 

#define FSMC_WaitSignalPolarity_Low                     ((uint32_t)0x00000000)
#define FSMC_WaitSignalPolarity_High                    ((uint32_t)0x00000200)


/// FSMC_Wrap_Mode 

#define FSMC_WrapMode_Disable                           ((uint32_t)0x00000000)
#define FSMC_WrapMode_Enable                            ((uint32_t)0x00000400)


/// FSMC_Wait_Timing 

#define FSMC_WaitSignalActive_BeforeWaitState           ((uint32_t)0x00000000)
#define FSMC_WaitSignalActive_DuringWaitState           ((uint32_t)0x00000800) 


/// FSMC_Write_Operation 

#define FSMC_WriteOperation_Disable                     ((uint32_t)0x00000000)
#define FSMC_WriteOperation_Enable                      ((uint32_t)0x00001000)
                              

/// FSMC_Wait_Signal 

#define FSMC_WaitSignal_Disable                         ((uint32_t)0x00000000)
#define FSMC_WaitSignal_Enable                          ((uint32_t)0x00002000)

/// FSMC_Extended_Mode 

#define FSMC_ExtendedMode_Disable                       ((uint32_t)0x00000000)
#define FSMC_ExtendedMode_Enable                        ((uint32_t)0x00004000)


/// FSMC_Write_Burst 

#define FSMC_WriteBurst_Disable                         ((uint32_t)0x00000000)
#define FSMC_WriteBurst_Enable                          ((uint32_t)0x00080000)

/// FSMC_Address_Setup_Time 

#define IS_FSMC_ADDRESS_SETUP_TIME(TIME) ((TIME) <= 0xF)


/// FSMC_Address_Hold_Time 

#define IS_FSMC_ADDRESS_HOLD_TIME(TIME) ((TIME) <= 0xF)


/// FSMC_Data_Setup_Time 

#define IS_FSMC_DATASETUP_TIME(TIME) (((TIME) > 0) && ((TIME) <= 0xFF))


/// FSMC_Bus_Turn_around_Duration 

#define IS_FSMC_TURNAROUND_TIME(TIME) ((TIME) <= 0xF)


/// FSMC_CLK_Division 

#define IS_FSMC_CLK_DIV(DIV) ((DIV) <= 0xF)


/// FSMC_Data_Latency 

#define IS_FSMC_DATA_LATENCY(LATENCY) ((LATENCY) <= 0xF)


/// FSMC_Access_Mode 

#define FSMC_AccessMode_A                               ((uint32_t)0x00000000)
#define FSMC_AccessMode_B                               ((uint32_t)0x10000000) 
#define FSMC_AccessMode_C                               ((uint32_t)0x20000000)
#define FSMC_AccessMode_D                               ((uint32_t)0x30000000)


  
/// NAND_and_PCCARD_Banks 

/// FSMC_Wait_feature 

#define FSMC_Waitfeature_Disable                        ((uint32_t)0x00000000)
#define FSMC_Waitfeature_Enable                         ((uint32_t)0x00000002)


/// FSMC_Memory_Data_Width  
#define FSMC_MemoryDataWidth_8b                         ((uint32_t)0x00000000)
#define FSMC_MemoryDataWidth_16b                        ((uint32_t)0x00000010)


/// FSMC_ECC 

#define FSMC_ECC_Disable                                ((uint32_t)0x00000000)
#define FSMC_ECC_Enable                                 ((uint32_t)0x00000040)

/// FSMC_ECC_Page_Size 

#define FSMC_ECCPageSize_256Bytes                       ((uint32_t)0x00000000)
#define FSMC_ECCPageSize_512Bytes                       ((uint32_t)0x00020000)
#define FSMC_ECCPageSize_1024Bytes                      ((uint32_t)0x00040000)
#define FSMC_ECCPageSize_2048Bytes                      ((uint32_t)0x00060000)
#define FSMC_ECCPageSize_4096Bytes                      ((uint32_t)0x00080000)
#define FSMC_ECCPageSize_8192Bytes                      ((uint32_t)0x000A0000)




/// FSMC_Exported_Macros


/// FSMC_Exported_Functions

void FSMC_NORSRAMDeInit(uint32_t FSMC_Bank);
void FSMC_NANDDeInit(uint32_t FSMC_Bank);
void FSMC_PCCARDDeInit(void);
void FSMC_NORSRAMInit(FSMC_NORSRAMInitTypeDef* FSMC_NORSRAMInitStruct);
void FSMC_NANDInit(FSMC_NANDInitTypeDef* FSMC_NANDInitStruct);
void FSMC_PCCARDInit(FSMC_PCCARDInitTypeDef* FSMC_PCCARDInitStruct);
void FSMC_NORSRAMStructInit(FSMC_NORSRAMInitTypeDef* FSMC_NORSRAMInitStruct);
void FSMC_NANDStructInit(FSMC_NANDInitTypeDef* FSMC_NANDInitStruct);
void FSMC_PCCARDStructInit(FSMC_PCCARDInitTypeDef* FSMC_PCCARDInitStruct);
void FSMC_NORSRAMCmd(uint32_t FSMC_Bank, FunctionalState new_state : FunctionalState);
void FSMC_NANDCmd(uint32_t FSMC_Bank, FunctionalState new_state : FunctionalState);
void FSMC_PCCARDCmd(FunctionalState new_state : FunctionalState);
void FSMC_NANDECCCmd(uint32_t FSMC_Bank, FunctionalState new_state : FunctionalState);
uint32_t FSMC_GetECC(uint32_t FSMC_Bank);

#endif /*__STM32F10x_FSMC_H */



/* --------------------- FSMC registers bit mask ---------------------------- */

/* FSMC BCRx Mask */
#define BCR_MBKEN_Set                       ((uint32_t)0x00000001)
#define BCR_MBKEN_Reset                     ((uint32_t)0x000FFFFE)
#define BCR_FACCEN_Set                      ((uint32_t)0x00000040)

/* FSMC PCRx Mask */
#define PCR_PBKEN_Set                       ((uint32_t)0x00000004)
#define PCR_PBKEN_Reset                     ((uint32_t)0x000FFFFB)
#define PCR_ECCEN_Set                       ((uint32_t)0x00000040)
#define PCR_ECCEN_Reset                     ((uint32_t)0x000FFFBF)
#define PCR_MemoryType_NAND                 ((uint32_t)0x00000008)


/// Deinitializes the FSMC NOR/SRAM Banks registers to their default 
///   reset values.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank1_NORSRAM1: FSMC Bank1 NOR/SRAM1  
/// * FSMC_Bank1_NORSRAM2: FSMC Bank1 NOR/SRAM2 
/// * FSMC_Bank1_NORSRAM3: FSMC Bank1 NOR/SRAM3 
/// * FSMC_Bank1_NORSRAM4: FSMC Bank1 NOR/SRAM4 
void FSMC_NORSRAMDeInit(uint32_t FSMC_Bank) {
  
  /* FSMC_Bank1_NORSRAM1 */
  if(FSMC_Bank == FSMC_Bank1_NORSRAM1) {
    FSMC_Bank1->BTCR[FSMC_Bank] = 0x000030DB;    
  }
  /* FSMC_Bank1_NORSRAM2,  FSMC_Bank1_NORSRAM3 or FSMC_Bank1_NORSRAM4 */
  else {   
    FSMC_Bank1->BTCR[FSMC_Bank] = 0x000030D2; 
  }
  FSMC_Bank1->BTCR[FSMC_Bank + 1] = 0x0FFFFFFF;
  FSMC_Bank1E->BWTR[FSMC_Bank] = 0x0FFFFFFF;  
}

/// Deinitializes the FSMC NAND Banks registers to their default 
///   reset values.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND 
void FSMC_NANDDeInit(uint32_t FSMC_Bank) {
  
  if(FSMC_Bank == FSMC_Bank2_NAND) {
    /* Set the FSMC_Bank2 registers to their reset values */
    FSMC_Bank2->PCR2 = 0x00000018;
    FSMC_Bank2->SR2 = 0x00000040;
    FSMC_Bank2->PMEM2 = 0xFCFCFCFC;
    FSMC_Bank2->PATT2 = 0xFCFCFCFC;  
  }
  /* FSMC_Bank3_NAND */  
  else {
    /* Set the FSMC_Bank3 registers to their reset values */
    FSMC_Bank3->PCR3 = 0x00000018;
    FSMC_Bank3->SR3 = 0x00000040;
    FSMC_Bank3->PMEM3 = 0xFCFCFCFC;
    FSMC_Bank3->PATT3 = 0xFCFCFCFC; 
  }  
}

/// Deinitializes the FSMC PCCARD Bank registers to their default 
///   reset values.
/// @param  None                       
void FSMC_PCCARDDeInit(void) {
  /* Set the FSMC_Bank4 registers to their reset values */
  FSMC_Bank4->PCR4 = 0x00000018; 
  FSMC_Bank4->SR4 = 0x00000000;	
  FSMC_Bank4->PMEM4 = 0xFCFCFCFC;
  FSMC_Bank4->PATT4 = 0xFCFCFCFC;
  FSMC_Bank4->PIO4 = 0xFCFCFCFC;
}

/// Initializes the FSMC NOR/SRAM Banks according to the 
///   specified parameters in the FSMC_NORSRAMInitStruct.
/// `FSMC_NORSRAMInitStruct`: pointer to a FSMC_NORSRAMInitTypeDef
///   structure that contains the configuration information for 
///   the FSMC NOR/SRAM specified Banks.                       
void FSMC_NORSRAMInit(FSMC_NORSRAMInitTypeDef* FSMC_NORSRAMInitStruct) { 
  
  /* Bank1 NOR/SRAM control register configuration */ 
  FSMC_Bank1->BTCR[FSMC_NORSRAMInitStruct->FSMC_Bank] = 
            (uint32_t)FSMC_NORSRAMInitStruct->FSMC_DataAddressMux |
            FSMC_NORSRAMInitStruct->FSMC_MemoryType |
            FSMC_NORSRAMInitStruct->FSMC_MemoryDataWidth |
            FSMC_NORSRAMInitStruct->FSMC_BurstAccessMode |
            FSMC_NORSRAMInitStruct->FSMC_WaitSignalPolarity |
            FSMC_NORSRAMInitStruct->FSMC_WrapMode |
            FSMC_NORSRAMInitStruct->FSMC_WaitSignalActive |
            FSMC_NORSRAMInitStruct->FSMC_WriteOperation |
            FSMC_NORSRAMInitStruct->FSMC_WaitSignal |
            FSMC_NORSRAMInitStruct->FSMC_ExtendedMode |
            FSMC_NORSRAMInitStruct->FSMC_WriteBurst;
  if(FSMC_NORSRAMInitStruct->FSMC_MemoryType == FSMC_MemoryType_NOR) {
    FSMC_Bank1->BTCR[FSMC_NORSRAMInitStruct->FSMC_Bank] |= (uint32_t)BCR_FACCEN_Set;
  }
  /* Bank1 NOR/SRAM timing register configuration */
  FSMC_Bank1->BTCR[FSMC_NORSRAMInitStruct->FSMC_Bank+1] = 
            (uint32_t)FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_AddressSetupTime |
            (FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_AddressHoldTime << 4) |
            (FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_DataSetupTime << 8) |
            (FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_BusTurnAroundDuration << 16) |
            (FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_CLKDivision << 20) |
            (FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_DataLatency << 24) |
             FSMC_NORSRAMInitStruct->FSMC_ReadWriteTimingStruct->FSMC_AccessMode;
            
    
  /* Bank1 NOR/SRAM timing register for write configuration, if extended mode is used */
  if(FSMC_NORSRAMInitStruct->FSMC_ExtendedMode == FSMC_ExtendedMode_Enable) {
    FSMC_Bank1E->BWTR[FSMC_NORSRAMInitStruct->FSMC_Bank] = 
              (uint32_t)FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_AddressSetupTime |
              (FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_AddressHoldTime << 4 )|
              (FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_DataSetupTime << 8) |
              (FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_CLKDivision << 20) |
              (FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_DataLatency << 24) |
               FSMC_NORSRAMInitStruct->FSMC_WriteTimingStruct->FSMC_AccessMode;
  } else {
    FSMC_Bank1E->BWTR[FSMC_NORSRAMInitStruct->FSMC_Bank] = 0x0FFFFFFF;
  }
}

/// Initializes the FSMC NAND Banks according to the specified 
///   parameters in the FSMC_NANDInitStruct.
/// `FSMC_NANDInitStruct`: pointer to a FSMC_NANDInitTypeDef 
///   structure that contains the configuration information for 
///   the FSMC NAND specified Banks.                       
void FSMC_NANDInit(FSMC_NANDInitTypeDef* FSMC_NANDInitStruct) {
  uint32_t tmppcr = 0x00000000, tmppmem = 0x00000000, tmppatt = 0x00000000; 
    
  
  /* Set the tmppcr value according to FSMC_NANDInitStruct parameters */
  tmppcr = (uint32_t)FSMC_NANDInitStruct->FSMC_Waitfeature |
            PCR_MemoryType_NAND |
            FSMC_NANDInitStruct->FSMC_MemoryDataWidth |
            FSMC_NANDInitStruct->FSMC_ECC |
            FSMC_NANDInitStruct->FSMC_ECCPageSize |
            (FSMC_NANDInitStruct->FSMC_TCLRSetupTime << 9 )|
            (FSMC_NANDInitStruct->FSMC_TARSetupTime << 13);
            
  /* Set tmppmem value according to FSMC_CommonSpaceTimingStructure parameters */
  tmppmem = (uint32_t)FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_SetupTime |
            (FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_WaitSetupTime << 8) |
            (FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HoldSetupTime << 16)|
            (FSMC_NANDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HiZSetupTime << 24); 
            
  /* Set tmppatt value according to FSMC_AttributeSpaceTimingStructure parameters */
  tmppatt = (uint32_t)FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_SetupTime |
            (FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_WaitSetupTime << 8) |
            (FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HoldSetupTime << 16)|
            (FSMC_NANDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HiZSetupTime << 24);
  
  if(FSMC_NANDInitStruct->FSMC_Bank == FSMC_Bank2_NAND) {
    /* FSMC_Bank2_NAND registers configuration */
    FSMC_Bank2->PCR2 = tmppcr;
    FSMC_Bank2->PMEM2 = tmppmem;
    FSMC_Bank2->PATT2 = tmppatt;
  } else {
    /* FSMC_Bank3_NAND registers configuration */
    FSMC_Bank3->PCR3 = tmppcr;
    FSMC_Bank3->PMEM3 = tmppmem;
    FSMC_Bank3->PATT3 = tmppatt;
  }
}

/// Initializes the FSMC PCCARD Bank according to the specified 
///   parameters in the FSMC_PCCARDInitStruct.
/// `FSMC_PCCARDInitStruct`: pointer to a FSMC_PCCARDInitTypeDef
///   structure that contains the configuration information for 
///   the FSMC PCCARD Bank.                       
void FSMC_PCCARDInit(FSMC_PCCARDInitTypeDef* FSMC_PCCARDInitStruct) {
  
  /* Set the PCR4 register value according to FSMC_PCCARDInitStruct parameters */
  FSMC_Bank4->PCR4 = (uint32_t)FSMC_PCCARDInitStruct->FSMC_Waitfeature |
                     FSMC_MemoryDataWidth_16b |  
                     (FSMC_PCCARDInitStruct->FSMC_TCLRSetupTime << 9) |
                     (FSMC_PCCARDInitStruct->FSMC_TARSetupTime << 13);
            
  /* Set PMEM4 register value according to FSMC_CommonSpaceTimingStructure parameters */
  FSMC_Bank4->PMEM4 = (uint32_t)FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_SetupTime |
                      (FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_WaitSetupTime << 8) |
                      (FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HoldSetupTime << 16)|
                      (FSMC_PCCARDInitStruct->FSMC_CommonSpaceTimingStruct->FSMC_HiZSetupTime << 24); 
            
  /* Set PATT4 register value according to FSMC_AttributeSpaceTimingStructure parameters */
  FSMC_Bank4->PATT4 = (uint32_t)FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_SetupTime |
                      (FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_WaitSetupTime << 8) |
                      (FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HoldSetupTime << 16)|
                      (FSMC_PCCARDInitStruct->FSMC_AttributeSpaceTimingStruct->FSMC_HiZSetupTime << 24);	
            
  /* Set PIO4 register value according to FSMC_IOSpaceTimingStructure parameters */
  FSMC_Bank4->PIO4 = (uint32_t)FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_SetupTime |
                     (FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_WaitSetupTime << 8) |
                     (FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_HoldSetupTime << 16)|
                     (FSMC_PCCARDInitStruct->FSMC_IOSpaceTimingStruct->FSMC_HiZSetupTime << 24);             
}


/// Enables or disables the specified NOR/SRAM Memory Bank.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank1_NORSRAM1: FSMC Bank1 NOR/SRAM1  
/// * FSMC_Bank1_NORSRAM2: FSMC Bank1 NOR/SRAM2 
/// * FSMC_Bank1_NORSRAM3: FSMC Bank1 NOR/SRAM3 
/// * FSMC_Bank1_NORSRAM4: FSMC Bank1 NOR/SRAM4 
/// `new_state : FunctionalState`: new state of the FSMC_Bank.
///   This parameter can be: ENABLE or DISABLE.
void FSMC_NORSRAMCmd(uint32_t FSMC_Bank, FunctionalState new_state : FunctionalState) {
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the selected NOR/SRAM Bank by setting the PBKEN bit in the BCRx register */
    FSMC_Bank1->BTCR[FSMC_Bank] |= BCR_MBKEN_Set;
  } else {
    /* Disable the selected NOR/SRAM Bank by clearing the PBKEN bit in the BCRx register */
    FSMC_Bank1->BTCR[FSMC_Bank] &= BCR_MBKEN_Reset;
  }
}

/// Enables or disables the specified NAND Memory Bank.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// `new_state : FunctionalState`: new state of the FSMC_Bank.
///   This parameter can be: ENABLE or DISABLE.
void FSMC_NANDCmd(uint32_t FSMC_Bank, FunctionalState new_state : FunctionalState) {
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the selected NAND Bank by setting the PBKEN bit in the PCRx register */
    if(FSMC_Bank == FSMC_Bank2_NAND) {
      FSMC_Bank2->PCR2 |= PCR_PBKEN_Set;
    } else {
      FSMC_Bank3->PCR3 |= PCR_PBKEN_Set;
    }
  } else {
    /* Disable the selected NAND Bank by clearing the PBKEN bit in the PCRx register */
    if(FSMC_Bank == FSMC_Bank2_NAND) {
      FSMC_Bank2->PCR2 &= PCR_PBKEN_Reset;
    } else {
      FSMC_Bank3->PCR3 &= PCR_PBKEN_Reset;
    }
  }
}

/// Enables or disables the PCCARD Memory Bank.
/// `new_state : FunctionalState`: new state of the PCCARD Memory Bank.  
///   This parameter can be: ENABLE or DISABLE.
void FSMC_PCCARDCmd(FunctionalState new_state : FunctionalState) {
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the PCCARD Bank by setting the PBKEN bit in the PCR4 register */
    FSMC_Bank4->PCR4 |= PCR_PBKEN_Set;
  } else {
    /* Disable the PCCARD Bank by clearing the PBKEN bit in the PCR4 register */
    FSMC_Bank4->PCR4 &= PCR_PBKEN_Reset;
  }
}

/// Enables or disables the FSMC NAND ECC feature.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// `new_state : FunctionalState`: new state of the FSMC NAND ECC feature.  
///   This parameter can be: ENABLE or DISABLE.
void FSMC_NANDECCCmd(uint32_t FSMC_Bank, FunctionalState new_state : FunctionalState) {
  
  if (new_state : FunctionalState != DISABLE) {
    /* Enable the selected NAND Bank ECC function by setting the ECCEN bit in the PCRx register */
    if(FSMC_Bank == FSMC_Bank2_NAND) {
      FSMC_Bank2->PCR2 |= PCR_ECCEN_Set;
    } else {
      FSMC_Bank3->PCR3 |= PCR_ECCEN_Set;
    }
  } else {
    /* Disable the selected NAND Bank ECC function by clearing the ECCEN bit in the PCRx register */
    if(FSMC_Bank == FSMC_Bank2_NAND) {
      FSMC_Bank2->PCR2 &= PCR_ECCEN_Reset;
    } else {
      FSMC_Bank3->PCR3 &= PCR_ECCEN_Reset;
    }
  }
}

/// Returns the error correction code register value.
/// `FSMC_Bank`: specifies the FSMC Bank to be used
///   This parameter can be one of the following values:
/// * FSMC_Bank2_NAND: FSMC Bank2 NAND 
/// * FSMC_Bank3_NAND: FSMC Bank3 NAND
/// @retval : The Error Correction Code (ECC) value.
uint32_t FSMC_GetECC(uint32_t FSMC_Bank) {
  uint32_t eccval = 0x00000000;
  
  if(FSMC_Bank == FSMC_Bank2_NAND) {
    /* Get the ECCR2 register value */
    eccval = FSMC_Bank2->ECCR2;
  } else {
    /* Get the ECCR3 register value */
    eccval = FSMC_Bank3->ECCR3;
  }
  /* Return the error correction code value */
  return(eccval);
}








