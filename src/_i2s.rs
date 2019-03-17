
/*
fn I2S_DeInit(&self);
fn Init(&self, SPI_InitTypeDef* SPI_InitStruct);
void I2S_Init(&self, I2S_InitTypeDef* I2S_InitStruct);
fn StructInit(SPI_InitTypeDef* SPI_InitStruct);
void I2S_StructInit(I2S_InitTypeDef* I2S_InitStruct);
fn Cmd(&self, new_state : bool);
void I2S_Cmd(&self, new_state : bool);
fn I2S_ITConfig(&self, uint8_t SPI_I2S_IT, new_state : bool);
fn I2S_DMACmd(&self, uint16_t SPI_I2S_DMAReq, new_state : bool);
fn I2S_SendData(&self, uint16_t Data);
uint16_t SPI_I2S_ReceiveData(&self);
fn NSSInternalSoftwareConfig(&self, uint16_t SPI_NSSInternalSoft);
fn SSOutputCmd(&self, new_state : bool);
fn DataSizeConfig(&self, uint16_t SPI_DataSize);
fn TransmitCRC(&self);
fn CalculateCRC(&self, new_state : bool);
uint16_t SPI_GetCRC(&self, uint8_t SPI_CRC);
uint16_t SPI_GetCRCPolynomial(&self);
fn BiDirectionalLineConfig(&self, uint16_t SPI_Direction);
FlagStatus SPI_I2S_GetFlagStatus(&self, uint16_t SPI_I2S_FLAG);
fn I2S_ClearFlag(&self, uint16_t SPI_I2S_FLAG);
ITStatus SPI_I2S_GetITStatus(&self, uint8_t SPI_I2S_IT);
fn I2S_ClearITPendingBit(&self, uint8_t SPI_I2S_IT);



/* SPI SPE mask */
#define CR1_SPE_Set          ((uint16_t)0x0040)
#define CR1_SPE_Reset        ((uint16_t)0xFFBF)

/* I2S I2SE mask */
#define I2SCFGR_I2SE_Set     ((uint16_t)0x0400)
#define I2SCFGR_I2SE_Reset   ((uint16_t)0xFBFF)

/* SPI CRCNext mask */
#define CR1_CRCNext_Set      ((uint16_t)0x1000)

/* SPI CRCEN mask */
#define CR1_CRCEN_Set        ((uint16_t)0x2000)
#define CR1_CRCEN_Reset      ((uint16_t)0xDFFF)

/* SPI SSOE mask */
#define CR2_SSOE_Set         ((uint16_t)0x0004)
#define CR2_SSOE_Reset       ((uint16_t)0xFFFB)

/* SPI registers Masks */
#define CR1_CLEAR_Mask       ((uint16_t)0x3040)
#define I2SCFGR_CLEAR_Mask   ((uint16_t)0xF040)

/* SPI or I2S mode selection masks */
#define SPI_Mode_Select      ((uint16_t)0xF7FF)
#define I2S_Mode_Select      ((uint16_t)0x0800) 
*/

/// I2S Init structure definition
pub struct I2sStruct {
  uint16_t I2S_Mode;
  uint16_t I2S_Standard;
  uint16_t I2S_DataFormat;
  uint16_t I2S_MCLKOutput;
  uint16_t I2S_AudioFreq;
  uint16_t I2S_CPOL;
}


pub impl I2sStruct {
	/// Fills each I2S_InitStruct member with its default value.
	/// `I2S_InitStruct` : pointer to a I2S_InitTypeDef structure
	///  which will be initialized.
	pub fn init() -> I2sStruct {
		/*--------------- Reset I2S init structure parameters values -----------------*/
		I2cStruct {
			/// Initialize the I2S_Mode member
			Mode       : I2S_Mode_SlaveTx,
			/// Initialize the I2S_Standard member
			Standard   : I2S_Standard_Phillips,
			/// Initialize the I2S_DataFormat member
			DataFormat : I2S_DataFormat_16b,
			/// Initialize the I2S_MCLKOutput member
			MCLKOutput : I2S_MCLKOutput_Disable,
			/// Initialize the I2S_AudioFreq member
			AudioFreq  : I2S_AudioFreq_Default,
			/// Initialize the I2S_CPOL member
			CPOL       : I2S_CPOL_Low
		}
	}
}


/// I2S_Mode
pub enum I2sMode {
	SlaveTx  = 0,
	SlaveRx  = 1,
	MasterTx = 2,
	MasterRx = 3
}


/// I2S_Standard
pub mod I2sStandard {
	pub const Phillips : u16 = 0x0000;
	pub const MSB      : u16 = 0x0010;
	pub const LSB      : u16 = 0x0020;
	pub const PCMShort : u16 = 0x0030;
	pub const PCMLong  : u16 = 0x00B0;
}



/// I2S_Data_Format
pub mod I2sDataFormat {
	pub const Bit16         : u16 = 0x0000;
	pub const Bit16Extended : u16 = 0x0001;
	pub const Bit24         : u16 = 0x0003;
	pub const Bit32         : u16 = 0x0005;
}

/** @defgroup I2S_MCLK_Output 
/// @{
  */

#define I2S_MCLKOutput_Enable           ((uint16_t)0x0200)
#define I2S_MCLKOutput_Disable          ((uint16_t)0x0000)
/**
/// @}
  */

/** @defgroup I2S_Audio_Frequency 
/// @{
  */

#define I2S_AudioFreq_48k                ((uint16_t)48000)
#define I2S_AudioFreq_44k                ((uint16_t)44100)
#define I2S_AudioFreq_22k                ((uint16_t)22050)
#define I2S_AudioFreq_16k                ((uint16_t)16000)
#define I2S_AudioFreq_8k                 ((uint16_t)8000)
#define I2S_AudioFreq_Default            ((uint16_t)2)
/**
/// @}
  */ 

/** @defgroup I2S_Clock_Polarity 
/// @{
  */

#define I2S_CPOL_Low                    ((uint16_t)0x0000)
#define I2S_CPOL_High                   ((uint16_t)0x0008)
/**
/// @}
  */



/** @defgroup SPI_I2S_DMA_transfer_requests 
/// @{
  */

#define SPI_I2S_DMAReq_Tx               ((uint16_t)0x0002)
#define SPI_I2S_DMAReq_Rx               ((uint16_t)0x0001)
/**
/// @}
  */



/// Initializes the SPIx peripheral according to the specified 
///   parameters in the I2S_InitStruct.
/// `SPIx`: where x can be  2 or 3 to select the SPI peripheral
///   (configured in I2S mode).
/// `I2S_InitStruct`: pointer to an I2S_InitTypeDef structure that
///   contains the configuration information for the specified
///   SPI peripheral configured in I2S mode.
void I2sInit(&self, I2S_InitTypeDef* I2S_InitStruct) {
  uint16_t tmpreg = 0, i2sdiv = 2, i2sodd = 0, packetlength = 1;
  uint32_t tmp = 0;
  RCC_ClocksTypeDef RCC_Clocks;
   
  /* Check the I2S parameters */
  assert_param(IS_SPI_23_PERIPH(SPIx));
  assert_param(IS_I2S_MODE(I2S_InitStruct->I2S_Mode));
  assert_param(IS_I2S_STANDARD(I2S_InitStruct->I2S_Standard));
  assert_param(IS_I2S_DATA_FORMAT(I2S_InitStruct->I2S_DataFormat));
  assert_param(IS_I2S_MCLK_OUTPUT(I2S_InitStruct->I2S_MCLKOutput));
  assert_param(IS_I2S_AUDIO_FREQ(I2S_InitStruct->I2S_AudioFreq));
  assert_param(IS_I2S_CPOL(I2S_InitStruct->I2S_CPOL));  
/*----------------------- SPIx I2SCFGR & I2SPR Configuration -----------------*/
  /* Clear I2SMOD, I2SE, I2SCFG, PCMSYNC, I2SSTD, CKPOL, DATLEN and CHLEN bits */
  SPIx->I2SCFGR &= I2SCFGR_CLEAR_Mask; 
  SPIx->I2SPR = 0x0002;
  
  /* Get the I2SCFGR register value */
  tmpreg = SPIx->I2SCFGR;
  
  /* If the default value has to be written, reinitialize i2sdiv and i2sodd*/
  if(I2S_InitStruct->I2S_AudioFreq == I2S_AudioFreq_Default) {
    i2sodd = (uint16_t)0;
    i2sdiv = (uint16_t)2;   
  }
  /* If the requested audio frequency is not the default, compute the prescaler */
  else {
    /* Check the frame length (For the Prescaler computing) */
    if(I2S_InitStruct->I2S_DataFormat == I2S_DataFormat_16b) {
      /* Packet length is 16 bits */
      packetlength = 1;
    }
    else {
      /* Packet length is 32 bits */
      packetlength = 2;
    }
    /* Get System Clock frequency */
    RCC_GetClocksFreq(&RCC_Clocks);
    
    /* Compute the Real divider depending on the MCLK output state with a flaoting point */
    if(I2S_InitStruct->I2S_MCLKOutput == I2S_MCLKOutput_Enable) {
      /* MCLK output is enabled */
      tmp = (uint16_t)(((10 * RCC_Clocks.SYSCLK_Frequency) / (256 * I2S_InitStruct->I2S_AudioFreq)) + 5);
    }
    else {
      /* MCLK output is disabled */
      tmp = (uint16_t)(((10 * RCC_Clocks.SYSCLK_Frequency) / (32 * packetlength * I2S_InitStruct->I2S_AudioFreq)) + 5);
    }
    
    /* Remove the flaoting point */
    tmp = tmp/10;  
      
    /* Check the parity of the divider */
    i2sodd = (uint16_t)(tmp & (uint16_t)0x0001);
   
    /* Compute the i2sdiv prescaler */
    i2sdiv = (uint16_t)((tmp - i2sodd) / 2);
   
    /* Get the Mask for the Odd bit (SPI_I2SPR[8]) register */
    i2sodd = (uint16_t) (i2sodd << 8);
  }
  
  /* Test if the divider is 1 or 0 */
  if ((i2sdiv < 2) || (i2sdiv > 0xFF)) {
    /* Set the default values */
    i2sdiv = 2;
    i2sodd = 0;
  }
  /* Write to SPIx I2SPR register the computed value */
  SPIx->I2SPR = (uint16_t)(i2sdiv | i2sodd | I2S_InitStruct->I2S_MCLKOutput);  
 
  /* Configure the I2S with the SPI_InitStruct values */
  tmpreg |= (uint16_t)(I2S_Mode_Select | I2S_InitStruct->I2S_Mode | \
                  I2S_InitStruct->I2S_Standard | I2S_InitStruct->I2S_DataFormat | \
                  I2S_InitStruct->I2S_CPOL);
 
  /* Write to SPIx I2SCFGR */
  SPIx->I2SCFGR = tmpreg;
}



/// Enables or disables the specified SPI peripheral (in I2S mode).
/// `SPIx`: where x can be 2 or 3 to select the SPI peripheral.
/// `new_state`: new state of the SPIx peripheral. 
///   This parameter can be: ENABLE or DISABLE.
fn I2sCmd(&self, new_state : bool) {
  if (new_state != DISABLE) {
    /* Enable the selected SPI peripheral (in I2S mode) */
    SPIx->I2SCFGR |= I2SCFGR_I2SE_Set;
  }
  else {
    /* Disable the selected SPI peripheral (in I2S mode) */
    SPIx->I2SCFGR &= I2SCFGR_I2SE_Reset;
  }
}



/// Enables or disables the SPIx/I2Sx DMA interface.
///   2 or 3 in I2S mode
/// `SPI_I2S_DMAReq`: specifies the SPI/I2S DMA transfer request 
///   to be enabled or disabled. 
///   This parameter can be any combination of the following values:
/// * SPI_I2S_DMAReq_Tx: Tx buffer DMA transfer request
/// * SPI_I2S_DMAReq_Rx: Rx buffer DMA transfer request
/// `new_state`: new state of the selected SPI/I2S DMA transfer 
///   request.
///   This parameter can be: ENABLE or DISABLE.
fn I2sDMACmd(&self, uint16_t SPI_I2S_DMAReq, new_state : bool) {
  if (new_state != DISABLE) {
    /* Enable the selected SPI/I2S DMA requests */
    SPIx->CR2 |= SPI_I2S_DMAReq;
  }
  else {
    /* Disable the selected SPI/I2S DMA requests */
    SPIx->CR2 &= (uint16_t)~SPI_I2S_DMAReq;
  }
}

/// Transmits a Data through the SPIx/I2Sx peripheral.
///   2 or 3 in I2S mode
/// @param Data : Data to be transmitted.
fn I2sSendData(&self, Data : u16) {
  
  /* Write in the DR register the data to be sent */
  SPIx->DR = Data;
}

/// Returns the most recent received data by the SPIx/I2Sx peripheral. 
///   2 or 3 in I2S mode
/// @retval : The value of the received data.
  */
fn I2sReceiveData(&self) -> u16 {
  
  /* Return the data in the DR register */
  return SPIx->DR;
}




