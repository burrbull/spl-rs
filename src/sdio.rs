

pub struct SdioStruct {
	pub ClockDiv            : u8,
	pub ClockEdge           : SdioClockEdge,
	pub ClockBypass         : false,
	pub ClockPowerSave      : false,
	pub BusWide             : SdioBusWide,
	pub HardwareFlowControl : false
}

pub impl SdioStruct {
	/// Fills each SDIO_InitStruct member with its default value.
	/// `SDIO_InitStruct`: pointer to an SDIO_InitTypeDef structure which 
	///   will be initialized.
	pub fn init() -> SdioStruct {
		/* SDIO_InitStruct members default value */
		SdioStruct {
			ClockDiv            : 0x00,
			ClockEdge           : SdioClockEdge::RISING,
			ClockBypass         : false,
			ClockPowerSave      : false,
			BusWide             : SdioBusWide::BIT1,
			HardwareFlowControl : false
		}
	}
}

pub struct SdioCmdStruct {
	Argument : u32,
	CmdIndex : u8,
	Response : SdioResponse,
	Wait     : SdioWait,
	CPSM     : bool
}


pub impl SdioCmdStruct {
	/// Fills each SDIO_CmdInitStruct member with its default value.
	/// `SDIO_CmdInitStruct`: pointer to an SDIO_CmdInitTypeDef 
	///   structure which will be initialized.
	pub fn init() -> SdioCmdStruct {
		/* SDIO_CmdInitStruct members default value */
		SdioCmdStruct {
			Argument : 0x00,
			CmdIndex : 0x00,
			Response : SdioResponse::NO,
			Wait     : SdioWait::No,
			CPSM     : false
		}
	}
}

pub struct SdioDataStruct {
	DataTimeOut   : u32,
	DataLength    : u32,
	DataBlockSize : SdioDataBlockSize,
	TransferDir   : SdioTransferDir,
	TransferMode  : SdioTransferMode,
	DPSM          : bool
}

pub impl SdioDataStruct {
	/// Fills each SDIO_DataInitStruct member with its default value.
	/// `SDIO_DataInitStruct`: pointer to an SDIO_DataInitTypeDef 
	///   structure which will be initialized.
	pub fn init() -> SdioDataStruct {
		/* SDIO_DataInitStruct members default value */
		SdioDataStruct {
			DataTimeOut   : 0xFFFFFFFF,
			DataLength    : 0x00,
			DataBlockSize : SdioDataBlockSize::BYTE1,
			TransferDir   : SdioTransferDir::TO_CARD,
			TransferMode  : SdioTransferMode::BLOCK,
			DPSM          : false
		}
	}
}

/// SDIO_Exported_Constants

/// SDIO_Clock_Edge 
pub use crate::device::sdio::clkcr::NEGEDGEW as SdioClockEdge;
//#define SDIO_ClockEdge_Rising               ((uint32_t)0x00000000)
//#define SDIO_ClockEdge_Falling              ((uint32_t)0x00002000)

/// SDIO_Bus_Wide
pub use crate::device::sdio::clkcr::WIDBUSW as SdioBusWide; 
/*
#define SDIO_BusWide_1b                     ((uint32_t)0x00000000)
#define SDIO_BusWide_4b                     ((uint32_t)0x00000800)
#define SDIO_BusWide_8b                     ((uint32_t)0x00001000)
*/

/// SDIO_Hardware_Flow_Control_ 
/*
#define SDIO_HardwareFlowControl_Disable    ((uint32_t)0x00000000)
#define SDIO_HardwareFlowControl_Enable     ((uint32_t)0x00004000)
*/
/// SDIO_Power_State 
pub use crate::device::sdio::power::PWRCTRLW as SdioPowerState; 
/*
#define SDIO_PowerState_OFF                 ((uint32_t)0x00000000)
#define SDIO_PowerState_ON                  ((uint32_t)0x00000003) 
*/


/// SDIO_Response_Type 
pub use crate::device::sdio::cmd::WAITRESPW as SdioResponse; 
/*
#define SDIO_Response_No                    ((uint32_t)0x00000000)
#define SDIO_Response_Short                 ((uint32_t)0x00000040)
#define SDIO_Response_Long                  ((uint32_t)0x000000C0)
*/
/// SDIO_Wait_Interrupt_State 
pub enum SdioWait {
	No             = 0x000, /* SDIO No Wait, TimeOut is enabled */
	IT             = 0x100, /* SDIO Wait Interrupt Request */
	Pend           = 0x200  /* SDIO Wait End of transfer */
}
impl SdioWait {
	pub enum to_int_pend(&self) -> (bool, bool) {
		match {
			SdioWait::No   => (false, false),
			SdioWait::IT   => (true,  false),
			SdioWait::Pend => (false, true)
		}
	}
}

/// SDIO_Response_Registers 
pub enum SdioRESP {
	R1 = 0x0,
	R2 = 0x4,
	R3 = 0x8,
	R4 = 0xC
}


/// SDIO_Data_Block_Size 
pub use crate::device::sdio::dctrl::DBLOCKSIZEW as SdioDataBlockSize;
/*
#define SDIO_DataBlockSize_1b               ((uint32_t)0x00000000)
#define SDIO_DataBlockSize_2b               ((uint32_t)0x00000010)
#define SDIO_DataBlockSize_4b               ((uint32_t)0x00000020)
#define SDIO_DataBlockSize_8b               ((uint32_t)0x00000030)
#define SDIO_DataBlockSize_16b              ((uint32_t)0x00000040)
#define SDIO_DataBlockSize_32b              ((uint32_t)0x00000050)
#define SDIO_DataBlockSize_64b              ((uint32_t)0x00000060)
#define SDIO_DataBlockSize_128b             ((uint32_t)0x00000070)
#define SDIO_DataBlockSize_256b             ((uint32_t)0x00000080)
#define SDIO_DataBlockSize_512b             ((uint32_t)0x00000090)
#define SDIO_DataBlockSize_1024b            ((uint32_t)0x000000A0)
#define SDIO_DataBlockSize_2048b            ((uint32_t)0x000000B0)
#define SDIO_DataBlockSize_4096b            ((uint32_t)0x000000C0)
#define SDIO_DataBlockSize_8192b            ((uint32_t)0x000000D0)
#define SDIO_DataBlockSize_16384b           ((uint32_t)0x000000E0) 
*/

/// SDIO_Transfer_Direction 
pub use crate::device::sdio::dctrl::DTDIRW as SdioTransferDir;
/*
#define SDIO_TransferDir_ToCard             ((uint32_t)0x00000000)
#define SDIO_TransferDir_ToSDIO             ((uint32_t)0x00000002)
*/

/// SDIO_Transfer_Type 
pub use crate::device::sdio::dctrl::DTMODEW as SdioTransferMode;
/*
#define SDIO_TransferMode_Block             ((uint32_t)0x00000000)
#define SDIO_TransferMode_Stream            ((uint32_t)0x00000004)
*/

/// SDIO_Clock_Bypass 
/*
#define SDIO_ClockBypass_Disable             ((uint32_t)0x00000000)
#define SDIO_ClockBypass_Enable              ((uint32_t)0x00000400)
*/
/// SDIO_Clock_Power_Save_ 
/*
#define SDIO_ClockPowerSave_Disable         ((uint32_t)0x00000000)
#define SDIO_ClockPowerSave_Enable          ((uint32_t)0x00000200)
*/
/// SDIO_CPSM_State 
/*
#define SDIO_CPSM_Disable                    ((uint32_t)0x00000000)
#define SDIO_CPSM_Enable                     ((uint32_t)0x00000400)
*/
/// SDIO_DPSM_State 
/*
#define SDIO_DPSM_Disable                    ((uint32_t)0x00000000)
#define SDIO_DPSM_Enable                     ((uint32_t)0x00000001)
*/
/// SDIO_Read_Wait_Mode 

#define SDIO_ReadWaitMode_CLK               ((uint32_t)0x00000000)
#define SDIO_ReadWaitMode_DATA2             ((uint32_t)0x00000001)


/// SDIO_Exported_Macros





/// Deinitializes the SDIO peripheral registers to their default
///   reset values.
fn deinit(&self) {
	self.power .reset();
	self.clkcr .reset();
	self.arg   .reset();
	self.cmd   .reset();
	self.dtimer.reset();
	self.dlen  .reset();
	self.dctrl .reset();
	self.icr   .write(|w| w.bits(0x00C007FF) );
	self.mask  .reset();
}

/// Initializes the SDIO peripheral according to the specified 
///   parameters in the SDIO_InitStruct.
/// `SDIO_InitStruct`: pointer to a SDIO_InitTypeDef structure 
///   that contains the configuration information for the SDIO 
///   peripheral.
fn init(&self, SDIO_InitStruct : &SdioStruct) {
	/*---------------------------- SDIO CLKCR Configuration ------------------------*/ 
	/* Set CLKDIV bits according to SDIO_ClockDiv value */
	/* Set PWRSAV bit according to SDIO_ClockPowerSave value */
	/* Set BYPASS bit according to SDIO_ClockBypass value */
	/* Set WIDBUS bits according to SDIO_BusWide value */
	/* Set NEGEDGE bits according to SDIO_ClockEdge value */
	/* Set HWFC_EN bits according to SDIO_HardwareFlowControl value */
	self.clkcr     .modify(|w| unsafe { w
		.clkdiv()  .bits( SDIO_InitStruct.ClockDiv )
		.pwrsav()  .bit( SDIO_InitStruct.ClockPowerSave )
		.bypass()  .bit( SDIO_InitStruct.ClockBypass )
		.widbus()  .variant( SDIO_InitStruct.BusWide )
		.negedge() .variant( SDIO_InitStruct.ClockEdge )
		.hwfc_en() .bit( SDIO_InitStruct.HardwareFlowControl )
	});
}

/// Enables or disables the SDIO Clock.
/// `new_state : bool`: new state of the SDIO Clock.
///   This parameter can be: ENABLE or DISABLE.
fn clock_cmd(&self, new_state : bool) {
	self.clkcr       .modify(|_,w| w
		.clken()     .bit( new_state )
	);
}

/// Sets the power status of the controller.
/// `SDIO_PowerState`: new state of the Power state. 
///   This parameter can be one of the following values:
/// * SDIO_PowerState_OFF
/// * SDIO_PowerState_ON
fn set_power_state(&self, SDIO_PowerState : SdioPowerState) {
	self.power     .modify(|_,w| w
		.pwrctrl()  .variant( SDIO_PowerState )
	);
}

/// Gets the power status of the controller.
/// @retval : Power status of the controller. The returned value can
///   be one of the following:
/// - 0x00: Power OFF
/// - 0x02: Power UP
/// - 0x03: Power ON 
fn get_power_state(&self) -> SdioPowerState {
	self.power.read().pwrctrl()
}


/// Enables or disables the SDIO DMA request.
/// `new_state : bool`: new state of the selected SDIO DMA request.
///   This parameter can be: ENABLE or DISABLE.
fn dma_cmd(&self, new_state : bool) {
	self.dctrl       .modify(|_,w| w
		.dmaen()     .bit( new_state )
	);
}

/// Initializes the SDIO Command according to the specified 
///   parameters in the SDIO_CmdInitStruct and send the command.
/// `SDIO_CmdInitStruct`: pointer to a SDIO_CmdInitTypeDef 
///   structure that contains the configuration information 
///   for the SDIO command.
fn send_command(&self, SDIO_CmdInitStruct : &SdioCmdStruct) {
	/*---------------------------- SDIO ARG Configuration ------------------------*/
	self.arg       .write(|w| unsafe { w
		.bits( SDIO_CmdInitStruct.Argument )
	});
  
	/*---------------------------- SDIO CMD Configuration ------------------------*/
	/* Set CMDINDEX bits according to SDIO_CmdIndex value */
	/* Set WAITRESP bits according to SDIO_Response value */
	/* Set WAITINT and WAITPEND bits according to SDIO_Wait value */
	/* Set CPSMEN bits according to SDIO_CPSM value */
	self.cmd    .modify(|w| unsafe { w
		.cmdindex() .bits( SDIO_CmdInitStruct.CmdIndex )
		.waitresp() .variant( SDIO_CmdInitStruct.Response )
		.waitint()  .bit( SDIO_CmdInitStruct.Wait )
		.cpsmen()   .bit( SDIO_CmdInitStruct.CPSM )
	});
}

/// Returns command index of last command for which response 
///   received.
/// @retval : Returns the command index of the last command response received.
fn get_command_response(&self) -> u8 {
	self.respcmd.read().respcmd().bits()
}

/// Returns response received from the card for the last command.
/// `SDIO_RESP`: Specifies the SDIO response register. 
///   This parameter can be one of the following values:
/// * SDIO_RESP1: Response Register 1
/// * SDIO_RESP2: Response Register 2
/// * SDIO_RESP3: Response Register 3
/// * SDIO_RESP4: Response Register 4
/// @retval : The Corresponding response register value.
fn get_gesponse(&self, SDIO_RESP : SdioRESP) -> u32 {
	match SDIO_RESP {
		SdioRESP::R1 => self.resp1.read().bits(),
		SdioRESP::R2 => self.resp2.read().bits(),
		SdioRESP::R3 => self.resp3.read().bits(),
		SdioRESP::R4 => self.resp4.read().bits()
	}; 
}

/// Initializes the SDIO data path according to the specified 
///   parameters in the SDIO_DataInitStruct.
/// `SDIO_DataInitStruct`: pointer to a SDIO_DataInitTypeDef 
///   structure that contains the configuration information 
///   for the SDIO command.
fn data_config(&self, SDIO_DataInitStruct : &SdioDataStruct) {
	/*---------------------------- SDIO DTIMER Configuration ---------------------*/
	/* Set the SDIO Data TimeOut value */
	self.dtimer     .write(|w| unsafe { w
		.bits( SDIO_DataInitStruct.DataTimeOut )
	});

	/*---------------------------- SDIO DLEN Configuration -----------------------*/
	/* Set the SDIO DataLength value */
	self.dlen       .write(|w| unsafe { w
		.bits( SDIO_DataInitStruct.DataLength )
	});

	/*---------------------------- SDIO DCTRL Configuration ----------------------*/
	/* Set DEN bit according to SDIO_DPSM value */
	/* Set DTMODE bit according to SDIO_TransferMode value */
	/* Set DTDIR bit according to SDIO_TransferDir value */
	/* Set DBCKSIZE bits according to SDIO_DataBlockSize value */
	self.dctrl    .modify(|w| w
		.dblocksize() .variant( SDIO_DataInitStruct.DataBlockSize )
		.dtdir()      .variant( SDIO_DataInitStruct.TransferDir )
		.dtmode()     .variant( SDIO_DataInitStruct.TransferMode )
		.dten()       .bit( SDIO_DataInitStruct.DPSM )
	);
}

/// Returns number of remaining data bytes to be transferred.
/// @retval : Number of remaining data bytes to be transferred
fn get_data_counter(&self) -> u32 {
	self.dcount.read().bits()
}

/// Read one data word from Rx FIFO.
/// @retval : Data received
fn read_data(&self) -> u32 { 
	self.fifo.read().bits()
}

/// Write one data word to Tx FIFO.
/// `Data`: 32-bit data word to write.
fn write_data(&self, Data : u32) {
	self.fifo      .write(|w| w
		.bits( Data )
	);
}

/// Returns the number of words left to be written to or read
///   from FIFO.
/// @retval : Remaining number of words.
fn get_fifo_count(&self) -> u32 {
	self.fifocnt.read().bits()
}

/// Starts the SD I/O Read Wait operation.
/// `new_state : bool`: new state of the Start SDIO Read Wait operation. 
///   This parameter can be: ENABLE or DISABLE.
fn start_sdio_read_wait(&self, new_state : bool) {
	self.dctrl        .modify(|_,w| w
		.rwstart()     .bit( new_state )
	);
}

/// Stops the SD I/O Read Wait operation.
/// `new_state : bool`: new state of the Stop SDIO Read Wait operation. 
///   This parameter can be: ENABLE or DISABLE.
fn stop_sdio_read_wait(&self, new_state : bool) {
	self.dctrl        .modify(|_,w| w
		.rwstop()     .bit( new_state )
	);
}

/// Sets one of the two options of inserting read wait interval.
/// `SDIO_ReadWaitMode`: SD I/O Read Wait operation mode.
///   This parametre can be:
/// * SDIO_ReadWaitMode_CLK: Read Wait control by stopping SDIOCLK
/// * SDIO_ReadWaitMode_DATA2: Read Wait control using SDIO_DATA2
fn set_sdio_read_wait_mode(&self, uint32_t SDIO_ReadWaitMode) {
  
  *(__IO uint32_t *) DCTRL_RWMOD_BB = SDIO_ReadWaitMode;
}

/// Enables or disables the SD I/O Mode Operation.
/// `new_state : bool`: new state of SDIO specific operation. 
///   This parameter can be: ENABLE or DISABLE.
fn set_sdio_operation(&self, new_state : bool) {
	self.dctrl        .modify(|_,w| w
		.sdioen()     .bit( new_state )
	);
}

/// Enables or disables the SD I/O Mode suspend command sending.
/// `new_state : bool`: new state of the SD I/O Mode suspend command.
///   This parameter can be: ENABLE or DISABLE.
fn send_sdio_suspend_cmd(&self, new_state : bool) {
	self.cmd        .modify(|_,w| w
		.sdiosuspend()     .bit( new_state )
	);
}

/// Enables or disables the command completion signal.
/// `new_state : bool`: new state of command completion signal. 
///   This parameter can be: ENABLE or DISABLE.
fn command_completion_cmd(&self, new_state : bool) {
	self.cmd        .modify(|_,w| w
		.encmdcompl().bit( new_state )
	);
}

/// Enables or disables the CE-ATA interrupt.
/// `new_state : bool`: new state of CE-ATA interrupt. 
///   This parameter can be: ENABLE or DISABLE.
fn ceata_it_cmd(&self, new_state : bool) {
	self.cmd        .modify(|_,w| w
		.nien()     .bit( !new_state )
	);
}

/// Sends CE-ATA command (CMD61).
/// `new_state : bool`: new state of CE-ATA command. 
///   This parameter can be: ENABLE or DISABLE.
fn send_ceata_cmd(&self, new_state : bool) {
	self.cmd        .modify(|_,w| w
		.ce_atacmd().bit( new_state )
	);
}


