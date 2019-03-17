use crate::pac::CRC;

pub trait CrcStd {
    /// Resets the CRC Data register (DR).
    /// @param	None
    fn reset_dr(&self);

    /// Computes the 32-bit CRC of a given data word(32-bit).
    /// `Data`: data word(32-bit) to compute its CRC
    /// @retval : 32-bit CRC
    fn calc_crc(&self, data: u32) -> u32;

    /// Computes the 32-bit CRC of a given buffer of data word(32-bit).
    /// `pBuffer`: pointer to the buffer containing the data to be
    ///	 computed
    /// `BufferLength`: length of the buffer to be computed
    /// @retval : 32-bit CRC
    fn calc_block_crc(&self, pbuffer: &[u32], buffer_length: u32) -> u32;

    /// Returns the current CRC value.
    /// @param	None
    /// @retval : 32-bit CRC
    fn get_crc(&self) -> u32;

    /// Stores a 8-bit data in the Independent Data(ID) register.
    /// `IDValue`: 8-bit value to be stored in the ID register
    fn set_id_register(&self, id_value: u8);

    /// Returns the 8-bit data stored in the Independent Data(ID) register
    /// @param	None
    /// @retval : 8-bit value of the ID register
    fn get_id_register(&self) -> u8;
}

impl CrcStd for CRC {
    /// Resets the CRC Data register (DR).
    /// @param	None
    fn reset_dr(&self) {
        /* Reset CRC generator */
        self.cr.write(|w| w.reset().set_bit());
    }

    /// Computes the 32-bit CRC of a given data word(32-bit).
    /// `Data`: data word(32-bit) to compute its CRC
    /// @retval : 32-bit CRC
    fn calc_crc(&self, data: u32) -> u32 {
        self.dr.write(|w| unsafe { w.bits(data) });
        self.dr.read().bits()
    }

    /// Computes the 32-bit CRC of a given buffer of data word(32-bit).
    /// `pBuffer`: pointer to the buffer containing the data to be
    ///	 computed
    /// `BufferLength`: length of the buffer to be computed
    /// @retval : 32-bit CRC
    fn calc_block_crc(&self, pbuffer: &[u32], buffer_length: u32) -> u32 {
        for b in pbuffer.iter().take(buffer_length as usize) {
            self.dr.write(|w| unsafe { w.bits(*b) });
        }
        self.dr.read().bits()
    }

    /// Returns the current CRC value.
    /// @param	None
    /// @retval : 32-bit CRC
    fn get_crc(&self) -> u32 {
        self.dr.read().bits()
    }

    /// Stores a 8-bit data in the Independent Data(ID) register.
    /// `IDValue`: 8-bit value to be stored in the ID register
    fn set_id_register(&self, id_value: u8) {
        self.idr.write(|w| w.idr().bits(id_value));
    }

    /// Returns the 8-bit data stored in the Independent Data(ID) register
    /// @param	None
    /// @retval : 8-bit value of the ID register
    fn get_id_register(&self) -> u8 {
        self.idr.read().idr().bits()
    }
}
