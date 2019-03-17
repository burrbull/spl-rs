

/// Tamper_Pin_active_level
pub use crate::device::bkp::cr::TPALW as BkpTamperPinLevel;
//#define BKP_TamperPinLevel_High           ((uint16_t)0x0000)
//#define BKP_TamperPinLevel_Low            ((uint16_t)0x0001)


/// RTC_output_source_to_output_on_the_Tamper_pin
pub mod BkpRTCOutputSource {
	pub const None       : u16 = 0x0000;
	pub const CalibClock : u16 = 0x0080;
	pub const Alarm      : u16 = 0x0100;
	pub const Second     : u16 = 0x0300;
}

impl BkpRTCOutputSource {
	pub enum to_cco_asoe_asos(&self) -> (bool, bool, bool) {
		match {
			BkpRTCOutputSource::None       => (false, false, false),
			BkpRTCOutputSource::CalibClock => (true,  false, false),
			BkpRTCOutputSource::Alarm      => (false, true,  false),
			BkpRTCOutputSource::Second     => (false, true,  true)
		}
	}
}

/// Data_Backup_Register
#define BKP_DR1                           ((uint16_t)0x0004)
#define BKP_DR2                           ((uint16_t)0x0008)
#define BKP_DR3                           ((uint16_t)0x000C)
#define BKP_DR4                           ((uint16_t)0x0010)
#define BKP_DR5                           ((uint16_t)0x0014)
#define BKP_DR6                           ((uint16_t)0x0018)
#define BKP_DR7                           ((uint16_t)0x001C)
#define BKP_DR8                           ((uint16_t)0x0020)
#define BKP_DR9                           ((uint16_t)0x0024)
#define BKP_DR10                          ((uint16_t)0x0028)
#define BKP_DR11                          ((uint16_t)0x0040)
#define BKP_DR12                          ((uint16_t)0x0044)
#define BKP_DR13                          ((uint16_t)0x0048)
#define BKP_DR14                          ((uint16_t)0x004C)
#define BKP_DR15                          ((uint16_t)0x0050)
#define BKP_DR16                          ((uint16_t)0x0054)
#define BKP_DR17                          ((uint16_t)0x0058)
#define BKP_DR18                          ((uint16_t)0x005C)
#define BKP_DR19                          ((uint16_t)0x0060)
#define BKP_DR20                          ((uint16_t)0x0064)
#define BKP_DR21                          ((uint16_t)0x0068)
#define BKP_DR22                          ((uint16_t)0x006C)
#define BKP_DR23                          ((uint16_t)0x0070)
#define BKP_DR24                          ((uint16_t)0x0074)
#define BKP_DR25                          ((uint16_t)0x0078)
#define BKP_DR26                          ((uint16_t)0x007C)
#define BKP_DR27                          ((uint16_t)0x0080)
#define BKP_DR28                          ((uint16_t)0x0084)
#define BKP_DR29                          ((uint16_t)0x0088)
#define BKP_DR30                          ((uint16_t)0x008C)
#define BKP_DR31                          ((uint16_t)0x0090)
#define BKP_DR32                          ((uint16_t)0x0094)
#define BKP_DR33                          ((uint16_t)0x0098)
#define BKP_DR34                          ((uint16_t)0x009C)
#define BKP_DR35                          ((uint16_t)0x00A0)
#define BKP_DR36                          ((uint16_t)0x00A4)
#define BKP_DR37                          ((uint16_t)0x00A8)
#define BKP_DR38                          ((uint16_t)0x00AC)
#define BKP_DR39                          ((uint16_t)0x00B0)
#define BKP_DR40                          ((uint16_t)0x00B4)
#define BKP_DR41                          ((uint16_t)0x00B8)
#define BKP_DR42                          ((uint16_t)0x00BC)


void BKP_DeInit(void);
void BKP_TamperPinLevelConfig(uint16_t BKP_TamperPinLevel);
void BKP_TamperPinCmd(FunctionalState new_state : FunctionalState);
void BKP_RTCOutputConfig(uint16_t BKP_RTCOutputSource);
void BKP_SetRTCCalibrationValue(uint8_t CalibrationValue);
void BKP_WriteBackupRegister(uint16_t BKP_DR, uint16_t Data);
uint16_t BKP_ReadBackupRegister(uint16_t BKP_DR);

#endif /* __STM32F10x_BKP_H */


/* ------------ BKP registers bit address in the alias region --------------- */
#define BKP_OFFSET        (BKP_BASE - PERIPH_BASE)

/* --- CR Register ----*/

/* Alias word address of TPAL bit */
#define CR_OFFSET         (BKP_OFFSET + 0x30)
#define TPAL_BitNumber    0x01
#define CR_TPAL_BB        (PERIPH_BB_BASE + (CR_OFFSET * 32) + (TPAL_BitNumber * 4))

/* Alias word address of TPE bit */
#define TPE_BitNumber     0x00
#define CR_TPE_BB         (PERIPH_BB_BASE + (CR_OFFSET * 32) + (TPE_BitNumber * 4))

/* --- CSR Register ---*/

/* Alias word address of TPIE bit */
#define CSR_OFFSET        (BKP_OFFSET + 0x34)
#define TPIE_BitNumber    0x02
#define CSR_TPIE_BB       (PERIPH_BB_BASE + (CSR_OFFSET * 32) + (TPIE_BitNumber * 4))

/* Alias word address of TIF bit */
#define TIF_BitNumber     0x09
#define CSR_TIF_BB        (PERIPH_BB_BASE + (CSR_OFFSET * 32) + (TIF_BitNumber * 4))

/* Alias word address of TEF bit */
#define TEF_BitNumber     0x08
#define CSR_TEF_BB        (PERIPH_BB_BASE + (CSR_OFFSET * 32) + (TEF_BitNumber * 4))

/* ---------------------- BKP registers bit mask ------------------------ */

/* RTCCR register bit mask */
#define RTCCR_CAL_Mask    ((uint16_t)0xFF80)
#define RTCCR_Mask        ((uint16_t)0xFC7F)



/// Deinitializes the BKP peripheral registers to their default
///   reset values.
/// @param  None
void BKP_DeInit(void) {
  RCC_BackupResetCmd(ENABLE);
  RCC_BackupResetCmd(DISABLE);
}

/// Configures the Tamper Pin active level.
/// `BKP_TamperPinLevel`: specifies the Tamper Pin active level.
///   This parameter can be one of the following values:
/// * BKP_TamperPinLevel_High: Tamper pin active on high level
/// * BKP_TamperPinLevel_Low: Tamper pin active on low level
fn TamperPinLevelConfig(&self, BKP_TamperPinLevel : BkpTamperPinLevel) {
  *(__IO uint32_t *) CR_TPAL_BB = BKP_TamperPinLevel;
}

/// Enables or disables the Tamper Pin activation.
/// `new_state : FunctionalState`: new state of the Tamper Pin activation.
///   This parameter can be: ENABLE or DISABLE.
fn TamperPinCmd(&self, new_state : bool) {
  *(__IO uint32_t *) CR_TPE_BB = (uint32_t)new_state : FunctionalState;
}

/// Select the RTC output source to output on the Tamper pin.
/// `BKP_RTCOutputSource`: specifies the RTC output source.
///   This parameter can be one of the following values:
/// * BKP_RTCOutputSource_None: no RTC output on the Tamper pin.
/// * BKP_RTCOutputSource_CalibClock: output the RTC clock
///   with frequency divided by 64 on the Tamper pin.
/// * BKP_RTCOutputSource_Alarm: output the RTC Alarm pulse 
///   signal on the Tamper pin.
/// * BKP_RTCOutputSource_Second: output the RTC Second pulse 
///   signal on the Tamper pin.  
fn RTCOutputConfig(BKP_RTCOutputSource : BkpRTCOutputSource) {
	let (cco, asoe, asos) = BKP_RTCOutputSource.to_cco_asoe_asos();
	/* Set CCO, ASOE and ASOS bits according to BKP_RTCOutputSource value */
	self.rtccr   .modify(|_,w| w
		.cco()   .bit( cco )
		.asoe()  .bit( asoe )
		.asos()  .bit( asos )
	);
}

/// Sets RTC Clock Calibration value.
/// `CalibrationValue`: specifies the RTC Clock Calibration value.
///   This parameter must be a number between 0 and 0x7F.
fn SetRTCCalibrationValue(&self, CalibrationValue : u8) {
  uint16_t tmpreg = 0;
  tmpreg = BKP->RTCCR;
  /* Clear CAL[6:0] bits */
  tmpreg &= RTCCR_CAL_Mask;
  /* Set CAL[6:0] bits according to CalibrationValue value */
  tmpreg |= CalibrationValue;
  /* Store the new value */
  BKP->RTCCR = tmpreg;
}

/// Writes user data to the specified Data Backup Register.
/// `BKP_DR`: specifies the Data Backup Register.
///   This parameter can be BKP_DRx where x:[1, 42]
/// `Data`: data to write
fn WriteBackupRegister(&self, uint16_t BKP_DR, Data : u16) {
  *(__IO uint16_t *) (BKP_BASE + BKP_DR) = Data;
}

/// Reads data from the specified Data Backup Register.
/// `BKP_DR`: specifies the Data Backup Register.
///   This parameter can be BKP_DRx where x:[1, 42]
/// @retval : The content of the specified Data Backup Register
fn ReadBackupRegister(&self, uint16_t BKP_DR) -> u16{
  return (*(__IO uint16_t *) (BKP_BASE + BKP_DR));
}
