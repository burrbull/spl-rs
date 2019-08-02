pub use crate::pac::dma1::ch::cr::PLW;
pub use crate::pac::dma1::ch::cr::PSIZEW as PSize;

/// DMA Init structure definition
#[derive(Clone, Copy, PartialEq)]
pub struct DmaStruct {
    pub peripheral_base_addr: u32,
    pub memory_base_addr: u32,
    pub dir: Direction,
    pub buffer_size: u16,
    pub peripheral_inc: bool,
    pub memory_inc: bool,
    pub peripheral_data_size: DataSize,
    pub memory_data_size: DataSize,
    pub mode: Mode,
    pub priority: Priority,
    pub m2m: bool,
}

impl DmaStruct {
    /// Fills each init_struct member with its default value.
    /// `init_struct`: pointer to a DMA_InitTypeDef structure
    ///   which will be initialized.
    pub fn init() -> DmaStruct {
        DmaStruct {
            /* Reset DMA init structure parameters values */
            peripheral_base_addr: 0,
            memory_base_addr: 0,
            dir: Direction::PeripheralSRC,
            buffer_size: 0,
            peripheral_inc: false,
            memory_inc: false,
            peripheral_data_size: DataSize::Byte,
            memory_data_size: DataSize::Byte,
            mode: Mode::Normal,
            priority: Priority::Low,
            m2m: false,
        }
    }
}

/// DMA_data_transfer_direction
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    PeripheralSRC,
    PeripheralDST,
}

impl From<Direction> for bool {
    fn from(mode: Direction) -> bool {
        match mode {
            Direction::PeripheralSRC => false,
            Direction::PeripheralDST => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DataSize {
    Byte,
    HalfWord,
    Word,
}

impl From<DataSize> for PSize {
    fn from(mode: DataSize) -> PSize {
        match mode {
            DataSize::Byte => PSize::BITS8,
            DataSize::HalfWord => PSize::BITS16,
            DataSize::Word => PSize::BITS32,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl From<Priority> for PLW {
    fn from(mode: Priority) -> PLW {
        match mode {
            Priority::Low => PLW::LOW,
            Priority::Medium => PLW::MEDIUM,
            Priority::High => PLW::HIGH,
            Priority::VeryHigh => PLW::VERYHIGH,
        }
    }
}

/// DMA_circular_normal_mode
#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Normal,
    Circular,
}

impl From<Mode> for bool {
    fn from(mode: Mode) -> bool {
        match mode {
            Mode::Normal => false,
            Mode::Circular => true,
        }
    }
}

pub trait DmaChannel {
    /// Deinitializes the DMAy Channelx registers to their default reset
    ///   values.
    fn deinit(&mut self);
    /// Initializes the DMAy Channelx according to the specified
    ///   parameters in the init_struct.
    /// `init_struct`: pointer to a DMA_InitTypeDef structure that
    ///   contains the configuration information for the specified
    ///   DMA Channel.
    fn init(&mut self, init_struct: &DmaStruct);
    /// Enables or disables the specified DMAy Channelx.
    /// `new_state : FunctionalState`: new state of the DMAy Channelx.
    ///   This parameter can be: ENABLE or DISABLE.
    fn cmd(&mut self, new_state: bool);
    /// Returns the number of remaining data units in the current
    ///   DMAy Channelx transfer.
    /// @retval : The number of remaining data units in the current DMAy Channelx
    ///   transfer.
    fn get_curr_data_counter(&mut self) -> u16;
}

macro_rules! dmachannel {
    ($($DMAX:ident: ($dmaX:ident, $dmaXen:ident, $dmaXrst:ident, {
        $($CX:ident: (
            $cteifX:ident,
            $chtifX:ident,
            $ctcifX:ident,
            $cgifX:ident
        ),)+
    }),)+) => {
        $(
            use crate::hal::dma::$dmaX;

            $(
                impl DmaChannel for $dmaX::$CX {

                    fn deinit(&mut self) {
                        let ch = self.ch();
                        /* Disable the selected DMAy Channelx */
                        ch.cr    .modify(|_,w| w
                            .en() .clear_bit()
                        );
                        /* Reset DMAy Channelx control register */
                        ch.cr   .reset();

                        /* Reset DMAy Channelx remaining bytes register */
                        ch.ndtr .reset();

                        /* Reset DMAy Channelx peripheral address register */
                        ch.par  .reset();

                        /* Reset DMAy Channelx memory address register */
                        ch.mar  .reset();

                        /* Reset interrupt pending bits for DMA Channel */
                        self.ifcr()    .write(|w| w
                            .$cgifX()  .set_bit()
                            .$ctcifX() .set_bit()
                            .$chtifX() .set_bit()
                            .$cteifX() .set_bit()
                        );
                    }


                    fn init(&mut self, init_struct : &DmaStruct) {
                        /*--------------------------- DMAy Channelx CCR Configuration -----------------*/
                        let ch = self.ch();
                        ch.cr    .modify(|_,w| w
                            .dir()   .bit(init_struct.dir.into())
                            .circ()  .bit(init_struct.mode.into())
                            .pinc()  .bit(init_struct.peripheral_inc)
                            .minc()  .bit(init_struct.memory_inc)
                            .psize() .variant(init_struct.peripheral_data_size.into())
                            .msize() .variant(init_struct.memory_data_size.into())
                            .pl()    .variant(init_struct.priority.into())
                            .mem2mem() .bit(init_struct.m2m)
                        );


                        /*--------------------------- DMAy Channelx CNDTR Configuration ---------------*/
                        ch.ndtr    .modify(|_,w| w
                            .ndt()   .bits(init_struct.buffer_size)
                        );
                        /*--------------------------- DMAy Channelx CPAR Configuration ----------------*/
                        ch.par    .modify(|_,w| w
                            .pa() .bits(init_struct.peripheral_base_addr)
                        );
                        /*--------------------------- DMAy Channelx CMAR Configuration ----------------*/
                        ch.mar    .modify(|_,w| w
                            .ma() .bits(init_struct.memory_base_addr)
                        );
                    }

                    fn cmd(&mut self, new_state : bool) {
                        self.ch().cr    .modify(|_,w| w
                            .en() .bit(new_state)
                        );
                    }

                    fn get_curr_data_counter(&mut self) -> u16 {
                        /* Return the number of remaining data units for DMAy Channelx */
                        self.ch().ndtr  .read().ndt().bits()
                    }
                }

            )+
        )+
    }
}

#[cfg(feature="dma1")]
dmachannel! {
    DMA1: (dma1, dma1en, dma1rst, {
        C1: (
            cteif1, chtif1, ctcif1, cgif1
        ),
        C2: (
            cteif2, chtif2, ctcif2, cgif2
        ),
        C3: (
            cteif3, chtif3, ctcif3, cgif3
        ),
        C4: (
            cteif4, chtif4, ctcif4, cgif4
        ),
        C5: (
            cteif5, chtif5, ctcif5, cgif5
        ),
        C6: (
            cteif6, chtif6, ctcif6, cgif6
        ),
        C7: (
            cteif7, chtif7, ctcif7, cgif7
        ),
    }),
}

#[cfg(feature="dma2")]
dmachannel! {
    DMA2: (dma2, dma2en, dma2rst, {
        C1: (
            cteif1, chtif1, ctcif1, cgif1
        ),
        C2: (
            cteif2, chtif2, ctcif2, cgif2
        ),
        C3: (
            cteif3, chtif3, ctcif3, cgif3
        ),
        C4: (
            cteif4, chtif4, ctcif4, cgif4
        ),
        C5: (
            cteif5, chtif5, ctcif5, cgif5
        ),
    }),
}
