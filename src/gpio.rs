
/// Output Maximum frequency selection  
pub enum GpioSpeed { 
	Speed_10MHz = 1,
	Speed_2MHz, 
	Speed_50MHz
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Configuration Mode enumeration
pub enum GpioMode {
	AIN      = 0x0,
	FLOATING = 0x04,
	IPD      = 0x28,
	IPU      = 0x48,
	Out_OD   = 0x14,
	Out_PP   = 0x10,
	AF_OD    = 0x1C,
	AF_PP    = 0x18
}

/// GPIO Init structure definition  
pub struct GpioStruct {
	pub GPIO_Pin   : u16,
	pub GPIO_Speed : GpioSpeed,
	pub GPIO_Mode  : GpioMode
}

pub impl GpioStruct {
	/// Fills each GPIO_InitStruct member with its default value.
	/// `GPIO_InitStruct` : pointer to a GPIO_InitTypeDef structure
	///   which will be initialized.
	pub fn init() -> GpioStruct {
		/* Reset GPIO init structure parameters values */
		GpioStruct {
			GPIO_Pin   : GpioPin::All,
			GPIO_Mode  : GpioMode::IN_FLOATING,
			GPIO_Speed : GpioSpeed::Speed_2MHz
		}
	}
}


#[derive(Clone, Copy, Debug, PartialEq)]
/// Bit_SET and Bit_RESET enumeration
pub enum BitAction {
	RESET = 0,
	SET   = 1
}

impl From<BitAction> for bool {
	#[inline(always)]
    fn from(state: BitAction) -> Self {
        match state {
			BitAction::RESET => false,
			BitAction::SET   => true
		}
    }
}

impl From<bool> for BitAction {
	#[inline(always)]
    fn from(state: bool) -> Self {
        match state {
			false => BitAction::RESET,
			true  => BitAction::SET
		}
    }
}


/// GPIO_pins_define
pub mod GpioPin {
	/// Pin 0 selected
	pub const P0  : u16 = 0x0001;
	/// Pin 1 selected
	pub const P1  : u16 = 0x0002;
	/// Pin 2 selected
	pub const P2  : u16 = 0x0004;
	/// Pin 3 selected
	pub const P3  : u16 = 0x0008;
	/// Pin 4 selected
	pub const P4  : u16 = 0x0010;
	/// Pin 5 selected
	pub const P5  : u16 = 0x0020;
	/// Pin 6 selected
	pub const P6  : u16 = 0x0040;
	/// Pin 7 selected
	pub const P7  : u16 = 0x0080;
	/// Pin 8 selected
	pub const P8  : u16 = 0x0100;
	/// Pin 8 selected
	pub const P9  : u16 = 0x0200;
	/// Pin 8 selected
	pub const P10 : u16 = 0x0400;
	/// Pin 8 selected
	pub const P11 : u16 = 0x0800;
	/// Pin 8 selected
	pub const P12 : u16 = 0x1000;
	/// Pin 8 selected
	pub const P13 : u16 = 0x2000;
	/// Pin 8 selected
	pub const P14 : u16 = 0x4000;
	/// Pin 8 selected
	pub const P15 : u16 = 0x8000;
	/// All pins selected
	pub const All : u16 = 0xFFFF;
}

/// GPIO_Remap_define 
pub mod GPIORemap {
	/// SPI1 Alternate Function mapping
	pub const SPI1            : u32 = 0x00000001;
	/// I2C1 Alternate Function mapping
	pub const I2C1            : u32 = 0x00000002);
	/// USART1 Alternate Function mapping
	pub const USART1          : u32 = 0x00000004);
	/// USART2 Alternate Function mapping
	pub const USART2          : u32 = 0x00000008);
	/// USART3 Partial Alternate Function mapping
	pub const USART3_Partial  : u32 = 0x00140010);
	/// USART3 Full Alternate Function mapping
	pub const USART3_Full     : u32 = 0x00140030);
	/// TIM1 Partial Alternate Function mapping
	pub const TIM1_Partial    : u32 = 0x00160040);
	/// TIM1 Full Alternate Function mapping
	pub const TIM1_Full       : u32 = 0x001600C0);
	/// TIM2 Partial1 Alternate Function mapping
	pub const TIM2_Partial1   : u32 = 0x00180100);
	/// TIM2 Partial2 Alternate Function mapping
	pub const TIM2_Partial2   : u32 = 0x00180200);
	/// TIM2 Full Alternate Function mapping
	pub const TIM2_Full       : u32 = 0x00180300);
	/// TIM3 Partial Alternate Function mapping
	pub const TIM3_Partial    : u32 = 0x001A0800);
	/// TIM3 Full Alternate Function mapping
	pub const TIM3_Full       : u32 = 0x001A0C00);
	/// TIM4 Alternate Function mapping
	pub const TIM4            : u32 = 0x00001000);
	/// CAN Alternate Function mapping
	pub const CAN1_1          : u32 = 0x001D4000);
	/// CAN Alternate Function mapping
	pub const CAN1_2          : u32 = 0x001D6000);
	/// PD01 Alternate Function mapping
	pub const PD01            : u32 = 0x00008000);
	/// LSI connected to TIM5 Channel4 input capture for calibration
	pub const TIM5CH4_LSI     : u32 = 0x00200001);
	/// ADC1 External Trigger Injected Conversion remapping
	pub const ADC1_ETRGINJ    : u32 = 0x00200002);
	/// ADC1 External Trigger Regular Conversion remapping
	pub const ADC1_ETRGREG    : u32 = 0x00200004);
	/// ADC2 External Trigger Injected Conversion remapping
	pub const ADC2_ETRGINJ    : u32 = 0x00200008);
	/// ADC2 External Trigger Regular Conversion remapping
	pub const ADC2_ETRGREG    : u32 = 0x00200010);
	/// Full SWJ Enabled (JTAG-DP + SW-DP) but without JTRST
	pub const SWJ_NoJTRST     : u32 = 0x00300100);
	/// JTAG-DP Disabled and SW-DP Enabled
	pub const SWJ_JTAGDisable : u32 = 0x00300200);
	/// Full SWJ Disabled (JTAG-DP + SW-DP)
	pub const SWJ_Disable     : u32 = 0x00300400);
} 

/// GPIO_Port_Sources 
pub mod GpioPortSource {
	pub const GPIOA : u8 = 0x00;
	pub const GPIOB : u8 = 0x01;
	pub const GPIOC : u8 = 0x02;
	pub const GPIOD : u8 = 0x03;
	pub const GPIOE : u8 = 0x04;
	pub const GPIOF : u8 = 0x05;
	pub const GPIOG : u8 = 0x06;
}

/// GPIO_Pin_sources 
pub enum GpioPinSource {
	pub const Pin0  : u8 = 0x00;
	pub const Pin1  : u8 = 0x01;
	pub const Pin2  : u8 = 0x02;
	pub const Pin3  : u8 = 0x03;
	pub const Pin4  : u8 = 0x04;
	pub const Pin5  : u8 = 0x05;
	pub const Pin6  : u8 = 0x06;
	pub const Pin7  : u8 = 0x07;
	pub const Pin8  : u8 = 0x08;
	pub const Pin9  : u8 = 0x09;
	pub const Pin10 : u8 = 0x0A;
	pub const Pin11 : u8 = 0x0B;
	pub const Pin12 : u8 = 0x0C;
	pub const Pin13 : u8 = 0x0D;
	pub const Pin14 : u8 = 0x0E;
	pub const Pin15 : u8 = 0x0F;
}



/*
fn DeInit(&self);
fn AFIODeInit(void);
fn Init(&self, GPIO_InitTypeDef* GPIO_InitStruct);
fn StructInit(GPIO_InitTypeDef* GPIO_InitStruct);
uint8_t GPIO_ReadInputDataBit(&self, uint16_t GPIO_Pin);
uint16_t GPIO_ReadInputData(&self);
uint8_t GPIO_ReadOutputDataBit(&self, uint16_t GPIO_Pin);
uint16_t GPIO_ReadOutputData(&self);
fn SetBits(&self, uint16_t GPIO_Pin);
fn ResetBits(&self, uint16_t GPIO_Pin);
fn WriteBit(&self, uint16_t GPIO_Pin, BitAction BitVal);
fn Write(&self, uint16_t PortVal);
fn PinLockConfig(&self, uint16_t GPIO_Pin);
fn EventOutputConfig(uint8_t GPIO_PortSource, uint8_t GPIO_PinSource);
fn EventOutputCmd(new_state : FunctionalState);
fn PinRemapConfig(uint32_t GPIO_Remap, new_state : FunctionalState);
fn EXTILineConfig(uint8_t GPIO_PortSource, uint8_t GPIO_PinSource);


/* ------------ RCC registers bit address in the alias region ----------------*/
#define AFIO_OFFSET                 (AFIO_BASE - PERIPH_BASE)

/* --- EVENTCR Register -----*/

/* Alias word address of EVOE bit */
#define EVCR_OFFSET                 (AFIO_OFFSET + 0x00)
#define EVOE_BitNumber              ((uint8_t)0x07)
#define EVCR_EVOE_BB                (PERIPH_BB_BASE + (EVCR_OFFSET * 32) + (EVOE_BitNumber * 4))
#define EVCR_PORTPINCONFIG_MASK     ((uint16_t)0xFF80)
#define LSB_MASK                    ((uint16_t)0xFFFF)
#define DBGAFR_POSITION_MASK        ((uint32_t)0x000F0000)
#define DBGAFR_SWJCFG_MASK          ((uint32_t)0xF0FFFFFF)
#define DBGAFR_LOCATION_MASK        ((uint32_t)0x00200000)
#define DBGAFR_NUMBITS_MASK         ((uint32_t)0x00100000)

 */




/// Initializes the GPIOx peripheral according to the specified
///   parameters in the GPIO_InitStruct.
/// `GPIO_InitStruct`: pointer to a GPIO_InitTypeDef structure that
///   contains the configuration information for the specified GPIO
///   peripheral.
fn Init(&self, GPIO_InitStruct : &GpioStruct) {
	let currentpin : u32, pos : u32, tmpreg : u32, pinmask : u32;
	
	/*---------------------------- GPIO Mode Configuration -----------------------*/
	let mut currentmode = (GPIO_InitStruct.Mode as u32) & 0x0Fu32;
	if ((GPIO_InitStruct.Mode as u32) & 0x10u32) != 0x00 {
		/* Output mode */
		currentmode |= GPIO_InitStruct.Speed as u32;
	}
	/*---------------------------- GPIO CRL Configuration ------------------------*/
	/* Configure the eight low port pins */
	if (GPIO_InitStruct.Pin as u32) & 0x00FFu32)) != 0x00 {
		tmpreg = self.crl.read().bits();
		for pinpos in 0x00..0x08 {
			pos = 0x01u32 << pinpos;
			/* Get the port pins position */
			currentpin = (GPIO_InitStruct.Pin as u32) & pos;
			if currentpin == pos {
				pos = pinpos << 2;
				/* Clear the corresponding low control register bits */
				pinmask = 0x0Fu32 << pos;
				tmpreg &= ~pinmask;
				/* Write the mode configuration in the corresponding bits */
				tmpreg |= currentmode << pos;
				match GPIO_InitStruct.Mode {
					GpioMode::IPD => {
						/* Reset the corresponding ODR bit */
						self.brr         .write(|w| unsafe { w
							.bits( 0x01u32 << pinpos )
						});
					},
					GpioMode::IPU => {
						/* Set the corresponding ODR bit */
						self.bsrr         .write(|w| unsafe { w
							.bits( 0x01u32 << pinpos )
						});
					},
					_ => {}
				}
			}
		}
		self.crl   .write(|w| unsafe { w
			.bits( tmpreg )
		});
	}
	/*---------------------------- GPIO CRH Configuration ------------------------*/
	/* Configure the eight high port pins */
	if GPIO_InitStruct.Pin > 0x00FF {
		tmpreg = self.crh.read().bits();
		for pinpos in 0x00..0x08 {
			pos = 0x01u32 << (pinpos + 0x08);
			/* Get the port pins position */
			currentpin = (GPIO_InitStruct.Pin as u32) & pos;
			if currentpin == pos {
				pos = pinpos << 2;
				/* Clear the corresponding high control register bits */
				pinmask = 0x0Fu32 << pos;
				tmpreg &= ~pinmask;
				/* Write the mode configuration in the corresponding bits */
				tmpreg |= currentmode << pos;
				match GPIO_InitStruct.Mode {
					GpioMode::IPD => {
						/* Reset the corresponding ODR bit */
						self.brr         .write(|w| unsafe { w
							.bits( 0x01u32 << (pinpos + 0x08) )
						});
					},
					GpioMode::IPU => {
						/* Set the corresponding ODR bit */
						self.bsrr         .write(|w| unsafe { w
							.bits( 0x01u32 << (pinpos + 0x08) )
						});
					},
					_ => {}
				}
			}
		}
		self.crh   .write(|w| unsafe { w
			.bits( tmpreg )
		});
	}
}

/// Reads the specified input port pin.
/// `GPIO_Pin`:	specifies the port bit to read.
///	 This parameter can be GPIO_Pin_x where x can be (0..15).
/// @retval : The input port pin value.
fn ReadInputDataBit(&self, GPIO_Pin : u16) -> bool {
	((self.idr.read().bits() as u16) & GPIO_Pin) != 0
}

/// Reads the specified GPIO input data port.
/// @retval : GPIO input data port value.
fn ReadInputData(&self) -> u16 {
	self.idr	.read().bits() as u16
}

/// Reads the specified output data port bit.
/// `GPIO_Pin`:	specifies the port bit to read.
///	 This parameter can be GPIO_Pin_x where x can be (0..15).
/// @retval : The output port pin value.
fn ReadOutputDataBit(&self, GPIO_Pin : u16) -> bool {
	((self.odr.read().bits() as u16) & GPIO_Pin) != 0
}

/// Reads the specified GPIO output data port.
/// @retval : GPIO output data port value.
fn ReadOutputData(&self) -> u16 {
	self.odr	.read().bits() as u16
}

/// Sets the selected data port bits.
/// `GPIO_Pin`: specifies the port bits to be written.
///	 This parameter can be any combination of GPIO_Pin_x where 
///	 x can be (0..15).
fn SetBits(&self, uint16_t GPIO_Pin) {
	self.bsrr	 .write(|w| w
		.bits( GPIO_Pin as u32 )
	);
}

/// Clears the selected data port bits.
/// `GPIO_Pin`: specifies the port bits to be written.
///	 This parameter can be any combination of GPIO_Pin_x where 
///	 x can be (0..15).
fn ResetBits(&self, GPIO_Pin : u16) {
	self.brr	 .write(|w| w
		.bits( GPIO_Pin as u32 )
	);
}

/// Sets or clears the selected data port bit.
/// `GPIO_Pin`: specifies the port bit to be written.
///	 This parameter can be one of GPIO_Pin_x where x can be (0..15).
/// `BitVal`: specifies the value to be written to the selected bit.
///	 This parameter can be one of the BitAction enum values:
/// * Bit_RESET: to clear the port pin
/// * Bit_SET: to set the port pin
fn WriteBit(&self, GPIO_Pin : u16, BitVal : BitAction) { 
	if BitVal.into() {
		self.bsrr	 .write(|w| w
			.bits( GPIO_Pin as u32 )
		);
	} else {
		self.brr		.write(|w| w
			.bits( GPIO_Pin as u32 )
		);
	}
}

/// Writes data to the specified GPIO data port.
/// `PortVal`: specifies the value to be written to the port output
///	 data register.
fn Write(&self, PortVal : u16) {
	self.odr      .write(|w| w
		.bits( PortVal as u32 )
	);
}

/// Locks GPIO Pins configuration registers.
/// `GPIO_Pin`: specifies the port bit to be written.
///	 This parameter can be any combination of GPIO_Pin_x where 
///	 x can be (0..15).
fn PinLockConfig(&self, GPIO_Pin : u16) {
	/* Set LCKK bit */
	self.lckr     .write(|w| w
		.lckk()   .set_bit()
		.lck()    .bits( GPIO_Pin )
	);
	/* Reset LCKK bit */
	self.lckr     .write(|w| w
		.lck()    .bits( GPIO_Pin )
	);
	/* Set LCKK bit */
	self.lckr     .write(|w| w
		.lckk()   .set_bit()
		.lck()    .bits( GPIO_Pin )
	);
	/* Read LCKK bit*/
	let _ = self.lckr.read().bits();
	/* Read LCKK bit*/
	let _ = self.lckr.read().bits();
}

/// Selects the GPIO pin used as Event output.
/// `GPIO_PortSource`: selects the GPIO port to be used as source
///	 for Event output.
///	 This parameter can be GPIO_PortSourceGPIOx where x can be
///	 (A..E).
/// `GPIO_PinSource`: specifies the pin for the Event output.
///	 This parameter can be GPIO_PinSourcex where x can be (0..15).
fn EventOutputConfig(GPIO_PortSource : u8, GPIO_PinSource : u8) {
	self.evcr       .modify(|_,r| unsafe { w
		.port()    .bits( GPIO_PortSource )
		.pin()     .bits( GPIO_PinSource )
	});
}

/// Enables or disables the Event Output.
/// `new_state`: new state of the Event output.
///	 This parameter can be: ENABLE or DISABLE.
fn EventOutputCmd(new_state : FunctionalState) {
	
	*(__IO uint32_t *) EVCR_EVOE_BB = (uint32_t)new_state;
}

/// Changes the mapping of the specified pin.
/// `GPIO_Remap`: selects the pin to remap.
///	 This parameter can be one of the following values:
/// * GPIO_Remap_SPI1
/// * GPIO_Remap_I2C1
/// * GPIO_Remap_USART1
/// * GPIO_Remap_USART2
/// * GPIO_PartialRemap_USART3
/// * GPIO_FullRemap_USART3
/// * GPIO_PartialRemap_TIM1
/// * GPIO_FullRemap_TIM1
/// * GPIO_PartialRemap1_TIM2
/// * GPIO_PartialRemap2_TIM2
/// * GPIO_FullRemap_TIM2
/// * GPIO_PartialRemap_TIM3
/// * GPIO_FullRemap_TIM3
/// * GPIO_Remap_TIM4
/// * GPIO_Remap1_CAN1
/// * GPIO_Remap2_CAN1
/// * GPIO_Remap_PD01
/// * GPIO_Remap_TIM5CH4_LSI
/// * GPIO_Remap_ADC1_ETRGINJ
/// * GPIO_Remap_ADC1_ETRGREG
/// * GPIO_Remap_ADC2_ETRGINJ
/// * GPIO_Remap_ADC2_ETRGREG
/// * GPIO_Remap_SWJ_NoJTRST
/// * GPIO_Remap_SWJ_JTAGDisable
/// * GPIO_Remap_SWJ_Disable
/// `new_state`: new state of the port pin remapping.
///	 This parameter can be: ENABLE or DISABLE.
fn PinRemapConfig(GPIO_Remap : u32, new_state : FunctionalState) {
	uint32_t tmp = 0x00, tmp1 = 0x00, tmpreg = 0x00, tmpmask = 0x00; 
	
	tmpreg = AFIO->MAPR;
	tmpmask = (GPIO_Remap & DBGAFR_POSITION_MASK) >> 0x10;
	tmp = GPIO_Remap & LSB_MASK;
	if ((GPIO_Remap & (DBGAFR_LOCATION_MASK | DBGAFR_NUMBITS_MASK)) == (DBGAFR_LOCATION_MASK | DBGAFR_NUMBITS_MASK))
	{
		tmpreg &= DBGAFR_SWJCFG_MASK;
		AFIO->MAPR &= DBGAFR_SWJCFG_MASK;
	}
	else if ((GPIO_Remap & DBGAFR_NUMBITS_MASK) == DBGAFR_NUMBITS_MASK)
	{
		tmp1 = ((uint32_t)0x03) << tmpmask;
		tmpreg &= ~tmp1;
		tmpreg |= ~DBGAFR_SWJCFG_MASK;
	}
	else
	{
		tmpreg &= ~(tmp << ((GPIO_Remap >> 0x15)*0x10));
		tmpreg |= ~DBGAFR_SWJCFG_MASK;
	}
	if (new_state != DISABLE)
	{
		tmpreg |= (tmp << ((GPIO_Remap >> 0x15)*0x10));
	}
	AFIO->MAPR = tmpreg;
}

/// Selects the GPIO pin used as EXTI Line.
/// `GPIO_PortSource`: selects the GPIO port to be used as
///	 source for EXTI lines.
///	 This parameter can be GPIO_PortSourceGPIOx where x can be
///	 (A..G).
/// `GPIO_PinSource`: specifies the EXTI line to be configured.
///	 This parameter can be GPIO_PinSourcex where x can be (0..15).
fn EXTILineConfig(GPIO_PortSource : u8, uint8_t GPIO_PinSource : u8) {
	uint32_t tmp = 0x00;
	
	tmp = ((uint32_t)0x0F) << (0x04 * (GPIO_PinSource & (uint8_t)0x03));
	AFIO->EXTICR[GPIO_PinSource >> 0x02] &= ~tmp;
	AFIO->EXTICR[GPIO_PinSource >> 0x02] |= (((uint32_t)GPIO_PortSource) << (0x04 * (GPIO_PinSource & (uint8_t)0x03)));
}

