/// SPI Init structure definition  
#[derive(Clone, Copy, PartialEq)]
pub struct SpiStruct {
    pub direction: Direction,
    pub mode: Mode,
    pub data_size: DataSize,
    pub cpol: ClockPolarity,
    pub cpha: ClockPhase,
    pub nss: SlaveSelect,
    pub baudrate_prescaler: BaudRatePrescaler,
    pub first_bit: FirstBit,
    pub crc_polynomial: u16,
}

impl SpiStruct {
    /// Fills each SPI_InitStruct member with its default value.
    /// `SPI_InitStruct` : pointer to a SPI_InitTypeDef structure
    ///   which will be initialized.
    pub fn init() -> SpiStruct {
        /*--------------- Reset SPI init structure parameters values -----------------*/
        SpiStruct {
            direction: Direction::Lines2FullDuplex,
            mode: Mode::Slave,
            data_size: DataSize::Bit8,
            cpol: ClockPolarity::IdleLow,
            cpha: ClockPhase::CaptureOnFirstTransition,
            nss: SlaveSelect::Hard,
            baudrate_prescaler: BaudRatePrescaler::Div2,
            first_bit: FirstBit::MsbFirst,
            crc_polynomial: 7,
        }
    }
}

/// SPI_data_direction_mode
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Lines2FullDuplex,
    Lines2RxOnly,
    Line1Rx,
    Line1Tx,
}

impl Into<(bool, bool, bool)> for Direction {
    fn into(self) -> (bool, bool, bool) {
        // Bit 10, bit 14, 15
        match self {
            Direction::Lines2FullDuplex => (false, false, false),
            Direction::Lines2RxOnly => (true, false, false),
            Direction::Line1Rx => (false, false, true),
            Direction::Line1Tx => (false, true, true),
        }
    }
}

/// SPI_master_slave_mode
#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Master,
    Slave,
}

impl Into<(bool, bool)> for Mode {
    fn into(self) -> (bool, bool) {
        // Bit 2, bit 8
        match self {
            Mode::Slave => (false, false),
            Mode::Master => (true, true),
        }
    }
}

/// SPI_data_size
#[derive(Clone, Copy, PartialEq)]
pub enum DataSize {
    Bit8,  // Bit 11 0
    Bit16, // Bit 11 1
}

/// SPI_Clock_Polarity
use super::ehal::spi::Polarity as ClockPolarity;
/*pub enum ClockPolarity {
	Low,// IdleLow  = 0,
	High// IdleHigh = 1
}*/

/// SPI_Clock_Phase
use super::ehal::spi::Phase as ClockPhase;
/*pub enum ClockPhase {
	Edge1,//CaptureOnFirstTransition = 0,
	Edge2///CaptureOnSecondTransition = 1
}*/

/// SPI_Slave_Select_management
#[derive(Clone, Copy, PartialEq)]
pub enum SlaveSelect {
    Soft, // = 1,
    Hard, // = 0
}

/** @defgroup SPI_BaudRate_Prescaler_
/// @{
 */

#[derive(Clone, Copy, PartialEq)]
pub enum BaudRatePrescaler {
    Div2 = 0,
    Div4 = 1,
    Div8 = 2,
    Div16 = 3,
    Div32 = 4,
    Div64 = 5,
    Div128 = 6,
    Div256 = 7,
}

/** @defgroup SPI_MSB_LSB_transmission
/// @{
 */
#[derive(Clone, Copy, PartialEq)]
pub enum FirstBit {
    MsbFirst, // Bit 7 0
    LsbFirst, // Bit 7 1
}

/// SPI_CRC_Transmit_Receive
#[derive(Clone, Copy, PartialEq)]
pub enum Crc {
    Tx, // 0
    Rx, // 1
}

/// SPI_direction_transmit_receive
#[derive(Clone, Copy, PartialEq)]
pub enum BidiModeDirection {
    Rx,
    Tx,
}

pub trait SpiStdExt {
    /// Initializes the SPIx peripheral according to the specified
    ///   parameters in the SPI_InitStruct.
    /// `SPI_InitStruct`: pointer to a SPI_InitTypeDef structure that
    ///   contains the configuration information for the specified
    ///   SPI peripheral.
    fn init(&self, spi_initstruct: SpiStruct);

    /// Enables or disables the specified SPI peripheral.
    /// `new_state`: new state of the SPIx peripheral.
    ///   This parameter can be: ENABLE or DISABLE.
    fn cmd(&self, new_state: bool);

    /// Configures internally by software the NSS pin for the selected
    ///   SPI.
    /// `SPI_NSSInternalSoft`: specifies the SPI NSS internal state.
    ///   This parameter can be one of the following values:
    /// * SPI_NSSInternalSoft_Set: Set NSS pin internally
    /// * SPI_NSSInternalSoft_Reset: Reset NSS pin internally
    fn nss_internal_software_config(&self, nss_internal_soft: bool);

    /// Enables or disables the SS output for the selected SPI.
    /// `new_state`: new state of the SPIx SS output.
    ///   This parameter can be: ENABLE or DISABLE.
    fn ss_output_cmd(&self, new_state: bool);

    /// Configures the data size for the selected SPI.
    /// `SPI_DataSize`: specifies the SPI data size.
    ///   This parameter can be one of the following values:
    /// * SPI_DataSize_16b: Set data frame format to 16bit
    /// * SPI_DataSize_8b: Set data frame format to 8bit
    fn data_size_config(&self, data_size: DataSize);

    /// Transmit the SPIx CRC value.
    fn transmit_crc(&self);

    /// Enables or disables the CRC value calculation of the
    ///   transfered bytes.
    /// @param new_state: new state of the SPIx CRC value calculation.
    ///   This parameter can be: ENABLE or DISABLE.
    fn calculate_crc(&self, new_state: bool);

    /// Returns the transmit or the receive CRC register value for
    ///   the specified SPI.
    /// `SPI_CRC`: specifies the CRC register to be read.
    ///   This parameter can be one of the following values:
    /// * SPI_CRC_Tx: Selects Tx CRC register
    /// * SPI_CRC_Rx: Selects Rx CRC register
    /// @retval : The selected CRC register value.
    fn get_crc(&self, crc: Crc) -> u16;

    /// Returns the CRC Polynomial register value for the specified SPI.
    /// @retval : The CRC Polynomial register value.
    fn get_crc_polynomial(&self) -> u16;

    /// Selects the data transfer direction in bi-directional mode
    ///   for the specified SPI.
    /// `SPI_Direction`: specifies the data transfer direction in
    ///   bi-directional mode.
    ///   This parameter can be one of the following values:
    /// * SPI_Direction_Tx: Selects Tx transmission direction
    /// * SPI_Direction_Rx: Selects Rx receive direction
    fn bidirectional_line_config(&self, direction: BidiModeDirection);
}

macro_rules! impl_spi {
    ($SPIx:ty) => {
        impl SpiStdExt for $SPIx {
            fn init(&self, spi_initstruct: SpiStruct) {
                /* Configure SPIx: direction, NSS management, first transmitted bit, BaudRate prescaler
                                			 master/salve mode, CPOL and CPHA */
                /* Set BIDImode, BIDIOE and RxONLY bits according to SPI_Direction value */
                /* Set SSM, SSI and MSTR bits according to SPI_Mode and SPI_NSS values */
                /* Set LSBFirst bit according to SPI_FirstBit value */
                /* Set BR bits according to SPI_BaudRatePrescaler value */
                /* Set CPOL bit according to SPI_CPOL value */
                /* Set CPHA bit according to SPI_CPHA value */
                let direction: (bool, bool, bool) = spi_initstruct.direction.into();
                let mode: (bool, bool) = spi_initstruct.mode.into();
                self.cr1.modify(|_, w| unsafe {
                    w.rxonly()
                        .bit(direction.0)
                        .bidioe()
                        .bit(direction.1)
                        .bidimode()
                        .bit(direction.2)
                        .mstr()
                        .bit(mode.0)
                        .ssi()
                        .bit(mode.1)
                        .dff()
                        .bit(spi_initstruct.data_size == DataSize::Bit16)
                        .cpol()
                        .bit(spi_initstruct.cpol == ClockPolarity::IdleHigh)
                        .cpha()
                        .bit(spi_initstruct.cpha == ClockPhase::CaptureOnSecondTransition)
                        .ssm()
                        .bit(spi_initstruct.nss == SlaveSelect::Hard)
                        .br()
                        .bits(spi_initstruct.baudrate_prescaler as u8)
                        .lsbfirst()
                        .bit(spi_initstruct.first_bit == FirstBit::LsbFirst)
                });
        
                /* Activate the SPI mode (Reset I2SMOD bit in I2SCFGR register) */
                /*self.i2scfgr    .modify(|_, w| w
                                				i2smod() .clear_bit()
                                			);*/
                /*---------------------------- SPIx CRCPOLY Configuration --------------------*/
                /* Write to SPIx CRCPOLY */
                self.crcpr
                    .write(|w| unsafe { w.crcpoly().bits(spi_initstruct.crc_polynomial) });
            }
        
            fn cmd(&self, new_state: bool) {
                self.cr1.modify(|_, w| w.spe().bit(new_state));
            }
        
            fn nss_internal_software_config(&self, nss_internal_soft: bool) {
                self.cr1.modify(|_, w| w.ssi().bit(nss_internal_soft));
            }
        
            fn ss_output_cmd(&self, new_state: bool) {
                self.cr2.modify(|_, w| w.ssoe().bit(new_state));
            }
        
            fn data_size_config(&self, data_size: DataSize) {
                self.cr1
                    .modify(|_, w| w.dff().bit(data_size == DataSize::Bit16));
            }
        
            fn transmit_crc(&self) {
                /* Enable the selected SPI CRC transmission */
                self.cr1.modify(|_, w| w.crcnext().set_bit());
            }
        
            fn calculate_crc(&self, new_state: bool) {
                self.cr1.modify(|_, w| w.crcen().bit(new_state));
            }
        
            fn get_crc(&self, crc: Crc) -> u16 {
                if crc != Crc::Rx {
                    /* Get the Tx CRC register */
                    self.txcrcr.read().tx_crc().bits()
                } else {
                    /* Get the Rx CRC register */
                    self.rxcrcr.read().rx_crc().bits()
                }
            }
        
            fn get_crc_polynomial(&self) -> u16 {
                /* Return the CRC polynomial register */
                self.crcpr.read().crcpoly().bits()
            }
        
            fn bidirectional_line_config(&self, direction: BidiModeDirection) {
                self.cr1
                    .modify(|_, w| w.bidioe().bit(direction == BidiModeDirection::Tx));
            }
        }
    };
}

#[cfg(feature="spi1")]
impl_spi!(crate::pac::SPI1);
#[cfg(feature="spi2")]
impl_spi!(crate::pac::SPI2);
#[cfg(feature="spi3")]
impl_spi!(crate::pac::SPI3);
