
use crate::pac::DAC;

pub struct DacChannel1<D> { dac: D }
pub struct DacChannel2<D> { dac: D }
pub struct DacChannelDual<D> { dac: D }
/*
pub struct DacChannel1<'a>(pub &'a DAC);
pub struct DacChannel2<'a>(pub &'a DAC);
pub struct DacChannelDual<'a>(pub &'a DAC);
*/


/// DAC Init structure definition

pub struct DacChannelStruct {
	pub trigger_enabled               : bool,
	pub trigger                       : Trigger,
	pub wave_generation               : WaveGeneration,
	pub lfs_unmask_triangle_amplitude : Mamp,
	pub output_buffer                 : bool
}

impl DacChannelStruct {
	/// Fills each DAC_InitStruct member with its default value.
	/// `DAC_InitStruct`: pointer to a DAC_InitTypeDef structure
	///   which will be initialized.
	pub fn init() -> DacChannelStruct {
		/* Set the default configuration */
		DacChannelStruct {
			/*--------------- Reset DAC init structure parameters values -----------------*/
			/* Initialize the DAC_Trigger member */
			trigger_enabled               : false,
			trigger                       : Trigger::TIM6TRGO,
			/* Initialize the DAC_WaveGeneration member */
			wave_generation               : WaveGeneration::None,
			/* Initialize the DAC_LFSRUnmask_TriangleAmplitude member */
			lfs_unmask_triangle_amplitude : Mamp::Unmask(LFSRUnmask::Bit0),
			/* Initialize the DAC_OutputBuffer member */
			output_buffer                 : true
		}
	}
}

/// DAC_Exported_Constants

/// DAC_trigger_selection 
pub use crate::pac::dac::cr::WAVE1W as Trigger;
/*
#define DAC_Trigger_None                   ((uint32_t)0x00000000)
#define DAC_Trigger_T6_TRGO                ((uint32_t)0x00000004)
#define DAC_Trigger_T8_TRGO                ((uint32_t)0x0000000C)
#define DAC_Trigger_T7_TRGO                ((uint32_t)0x00000014)
#define DAC_Trigger_T5_TRGO                ((uint32_t)0x0000001C)
#define DAC_Trigger_T2_TRGO                ((uint32_t)0x00000024)
#define DAC_Trigger_T4_TRGO                ((uint32_t)0x0000002C)
#define DAC_Trigger_Ext_IT9                ((uint32_t)0x00000034)
#define DAC_Trigger_Software               ((uint32_t)0x0000003C)
*/

/// DAC_wave_generation 
pub enum WaveGeneration {
	None = 0,
	Noise = 1,
	Triangle = 2
}

/// DAC_wave_generation 
pub enum Wave {
	Noise = 1,
	Triangle = 2
}
/*
#define DAC_Wave_Noise                     ((uint32_t)0x00000040)
#define DAC_Wave_Triangle                  ((uint32_t)0x00000080)
*/

/// DAC_noise_wave_generation_mask_triangle_wave_generation_max_amplitude 
#[derive(Clone, Copy, PartialEq)]
pub enum LFSRUnmask {
	Bit0  = 0,
	Bits1 = 1,
	Bits2 = 2,
	Bits3 = 3,
	Bits4 = 4,
	Bits5 = 5,
	Bits6 = 6,
	Bits7 = 7,
	Bits8 = 8,
	Bits9 = 9,
	Bits10 = 10,
	Bits11 = 11
}
/*#define DAC_LFSRUnmask_Bits2_0             ((uint32_t)0x00000200)
#define DAC_LFSRUnmask_Bits3_0             ((uint32_t)0x00000300)
#define DAC_LFSRUnmask_Bits4_0             ((uint32_t)0x00000400)
#define DAC_LFSRUnmask_Bits5_0             ((uint32_t)0x00000500)
#define DAC_LFSRUnmask_Bits6_0             ((uint32_t)0x00000600)
#define DAC_LFSRUnmask_Bits7_0             ((uint32_t)0x00000700)
#define DAC_LFSRUnmask_Bits8_0             ((uint32_t)0x00000800)
#define DAC_LFSRUnmask_Bits9_0             ((uint32_t)0x00000900)
#define DAC_LFSRUnmask_Bits10_0            ((uint32_t)0x00000A00)
#define DAC_LFSRUnmask_Bits11_0            ((uint32_t)0x00000B00)
*/

#[derive(Clone, Copy, PartialEq)]
pub enum TriangleAmplitude {
	Level1    = 0,
	Level3    = 1,
	Level7    = 2,
	Level15   = 3,
	Level31   = 4,
	Level63   = 5,
	Level127  = 6,
	Level255  = 7,
	Level511  = 8,
	Level1023 = 9,
	Level2047 = 10,
	Level4095 = 11
}
/*#define DAC_TriangleAmplitude_3            ((uint32_t)0x00000100)
#define DAC_TriangleAmplitude_7            ((uint32_t)0x00000200)
#define DAC_TriangleAmplitude_15           ((uint32_t)0x00000300)
#define DAC_TriangleAmplitude_31           ((uint32_t)0x00000400)
#define DAC_TriangleAmplitude_63           ((uint32_t)0x00000500)
#define DAC_TriangleAmplitude_127          ((uint32_t)0x00000600)
#define DAC_TriangleAmplitude_255          ((uint32_t)0x00000700)
#define DAC_TriangleAmplitude_511          ((uint32_t)0x00000800)
#define DAC_TriangleAmplitude_1023         ((uint32_t)0x00000900)
#define DAC_TriangleAmplitude_2047         ((uint32_t)0x00000A00)
#define DAC_TriangleAmplitude_4095         ((uint32_t)0x00000B00)*/

#[derive(Clone, Copy, PartialEq)]
pub enum Mamp {
	Unmask(LFSRUnmask),
	Triangle(TriangleAmplitude)
}

impl From<Mamp> for u8 {
	fn from( mode : Mamp ) -> u8 {
		match mode {
			Mamp::Unmask(bits) => bits as u8,
			Mamp::Triangle(bits) => bits as u8,
		}
	}
}


/// DAC_output_buffer 
//pub use crate::pac::dac::cr::BOFF1W as OutputBuffer;
/*
#define DAC_OutputBuffer_Enable            ((uint32_t)0x00000000)
#define DAC_OutputBuffer_Disable           ((uint32_t)0x00000002)
*/
/// DAC_Channel_selection 
/*
#define DAC_Channel_1                      ((uint32_t)0x00000000)
#define DAC_Channel_2                      ((uint32_t)0x00000010)
*/

/// DAC_data_alignement
pub enum DataAlign {
	Right12, // 0
	Left12,  // 4
	Right8   // 8
}
/*
#define DAC_Align_12b_R                    ((uint32_t)0x00000000)
#define DAC_Align_12b_L                    ((uint32_t)0x00000004)
#define DAC_Align_8b_R                     ((uint32_t)0x00000008)
*/


pub trait DacChannelSplExt {
	/// Initializes the DAC peripheral according to the specified 
	///   parameters in the DAC_InitStruct.
	/// `DAC_InitStruct`: pointer to a DAC_InitTypeDef structure that
	///   contains the configuration information for the specified
	///   DAC channel.
	fn init(&self, DAC_InitStruct : &DacChannelStruct);

	/// Enables or disables the specified DAC channel.
	/// `new_state : FunctionalState`: new state of the DAC channel. 
	///   This parameter can be: ENABLE or DISABLE.
	fn cmd(&self, new_state : bool);

	/// Enables or disables the specified DAC channel DMA request.
	/// `new_state : FunctionalState`: new state of the selected DAC channel DMA request.
	///   This parameter can be: ENABLE or DISABLE.
	fn dma_cmd(&self, new_state : bool);

	/// Enables or disables the selected DAC channel software trigger.
	/// `new_state : FunctionalState`: new state of the selected DAC channel software trigger.
	///   This parameter can be: ENABLE or DISABLE.
	fn software_trigger_cmd(&self, new_state : bool);

	/// Enables or disables the selected DAC channel wave generation.
	/// `DAC_Wave`: Specifies the wave type to enable or disable.
	///   This parameter can be one of the following values:
	/// * DAC_Wave_Noise: noise wave generation
	/// * DAC_Wave_Triangle: triangle wave generation
	/// `new_state : FunctionalState`: new state of the selected DAC channel wave generation.
	///   This parameter can be: ENABLE or DISABLE.
	fn wave_generation_cmd(&self, wave : Wave, new_state : bool);

	/// Set the specified data holding register value for DAC channel1.
	/// `DAC_Align`: Specifies the data alignement for DAC channel1.
	///   This parameter can be one of the following values:
	/// * DAC_Align_8b_R: 8bit right data alignement selected
	/// * DAC_Align_12b_L: 12bit left data alignement selected
	/// * DAC_Align_12b_R: 12bit right data alignement selected
	/// `Data`: Data to be loaded in the selected data holding 
	///   register.
	fn set_data(&self, align : DataAlign, data : u16);

	/// Returns the last data output value of the selected DAC cahnnel.
	/// @retval : The selected DAC channel data output value.
	fn get_data_output_value(&self) -> u16;
}

impl DacChannelSplExt for DacChannel1<DAC> {
	fn init(&self, dac_initstruct : &DacChannelStruct) {
		/* Configure for the selected DAC channel: buffer output, trigger, wave genration,
			mask/amplitude for wave genration */
		/* Set TSELx and TENx bits according to DAC_Trigger value */
		/* Set WAVEx bits according to DAC_WaveGeneration value */
		/* Set MAMPx bits according to DAC_LFSRUnmask_TriangleAmplitude value */ 
		/* Set BOFFx bit according to DAC_OutputBuffer value */   
		self.dac.cr      .modify(|_,w| unsafe { w
			.ten1()    .bit( dac_initstruct.trigger_enabled )
			.tsel1()   .bits( dac_initstruct.trigger )
			.wave1()   .bits( dac_initstruct.wave_generation as u8 )
			.mamp1()   .bits( dac_initstruct.lfs_unmask_triangle_amplitude.into() )
			.boff1()   .bit( !dac_initstruct.output_buffer )
		});
	}

	fn cmd(&self, new_state : bool) {
		self.dac.cr       .modify(|_,w| w
			.en1()      .bit( new_state )
		);
	}

	fn dma_cmd(&self, new_state : bool) {
		self.dac.cr      .modify(|_,w| w
			.dmaen1()  .bit( new_state )
		);
	}

	fn software_trigger_cmd(&self, new_state : bool) {
		self.dac.swtrigr  .write(|w| w
			.swtrig1() .bit( new_state )
		);
	}

	fn wave_generation_cmd(&self, wave : Wave, new_state : bool) {
		let wave = (wave as u8 as u32) << 2;
		self.dac.cr        .modify(|r,w| unsafe {
			w.bits(
				if new_state { r.bits() | wave
				} else {  r.bits() & !wave
				}
			)
		});
	}

	fn set_data(&self, align : DataAlign, dac_data : u16) {
		match align {
			DataAlign::Right8 => {
				self.dac.dhr8r1      .write(|w| unsafe { w
					.bits ( dac_data as u32 )
				});
			},
			DataAlign::Right12 => {
				self.dac.dhr12r1     .write(|w| unsafe { w
					.bits ( dac_data as u32 )
				});
			},
			DataAlign::Left12 => {
				self.dac.dhr12l1     .write(|w| unsafe { w
					.bits ( dac_data as u32 )
				});
			}
		}
	}

	fn get_data_output_value(&self) -> u16 {
		/* Returns the DAC channel data output register value */
		self.dac.dor1.read().dacc1dor().bits()
	}
}

impl DacChannelSplExt for DacChannel2<DAC> {
	fn init(&self, dac_initstruct : &DacChannelStruct) {
		/* Configure for the selected DAC channel: buffer output, trigger, wave genration,
			mask/amplitude for wave genration */
		/* Set TSELx and TENx bits according to DAC_Trigger value */
		/* Set WAVEx bits according to DAC_WaveGeneration value */
		/* Set MAMPx bits according to DAC_LFSRUnmask_TriangleAmplitude value */ 
		/* Set BOFFx bit according to DAC_OutputBuffer value */   
		self.dac.cr      .modify(|_,w| unsafe { w
			.ten2()    .bit( dac_initstruct.trigger_enabled )
			.tsel2()   .bits( dac_initstruct.trigger )
			.wave2()   .bits( dac_initstruct.wave_generation as u8 )
			.mamp2()   .bits( dac_initstruct.lfs_unmask_triangle_amplitude.into() )
			.boff2()   .bit( !dac_initstruct.output_buffer )
		});
	}

	fn cmd(&self, new_state : bool) {
		self.dac.cr       .modify(|_,w| w
			.en2()      .bit( new_state )
		);
	}

	fn dma_cmd(&self, new_state : bool) {
		self.dac.cr      .modify(|_,w| w
			.dmaen2()  .bit( new_state )
		);
	}

	fn software_trigger_cmd(&self, new_state : bool) {
		self.dac.swtrigr  .write(|w| w
			.swtrig2() .bit( new_state )
		);
	}

	fn wave_generation_cmd(&self, wave : Wave, new_state : bool) {
		let wave = (wave as u8 as u32) << 18;
		self.dac.cr       .modify(|r,w| unsafe {
			w.bits(
				if new_state { r.bits() | wave
				} else {  r.bits() & !wave
				}
			)
		});
	}

	fn set_data(&self, align : DataAlign, dac_data : u16) {
		match align {
			DataAlign::Right8 => {
				self.dac.dhr8r2      .write(|w| unsafe { w
					.bits ( dac_data as u32 )
				});
			},
			DataAlign::Right12 => {
				self.dac.dhr12r2     .write(|w| unsafe { w
					.bits ( dac_data as u32 )
				});
			},
			DataAlign::Left12 => {
				self.dac.dhr12l2     .write(|w| unsafe { w
					.bits ( dac_data as u32 )
				});
			}
		}
	}

	fn get_data_output_value(&self) -> u16 {
		/* Returns the DAC channel data output register value */
		self.dac.dor2.read().dacc2dor().bits()
	}
}


impl DacChannelDual<DAC> {
	/// Set the specified data holding register value for dual channel
	///   DAC.
	/// `DAC_Align`: Specifies the data alignement for dual channel DAC.
	///   This parameter can be one of the following values:
	/// * DAC_Align_8b_R: 8bit right data alignement selected
	/// * DAC_Align_12b_L: 12bit left data alignement selected
	/// * DAC_Align_12b_R: 12bit right data alignement selected
	/// `Data2`: Data for DAC Channel2 to be loaded in the selected data 
	///   holding register.
	/// `Data1`: Data for DAC Channel1 to be loaded in the selected data 
	///   holding register.
	fn set_dual_channel_data(&self, align : DataAlign, data2 : u16, data1 : u16) {
		match align {
			DataAlign::Right8 => {
				self.dac.dhr8rd      .write(|w| unsafe { w
					.dacc1dhr()   .bits ( data1 as u8 )
					.dacc2dhr()   .bits ( data2 as u8 )
				});
			},
			DataAlign::Right12 => {
				self.dac.dhr12rd      .write(|w| unsafe { w
					.dacc1dhr()   .bits ( data1 )
					.dacc2dhr()   .bits ( data2 )
				});
			},
			DataAlign::Left12 => {
				self.dac.dhr12ld      .write(|w| unsafe { w
					.bits ( (data1 as u32) | ((data2 as u32) << 16) )
				});
			}
		}
	}

	/// Enables or disables simultaneously the two DAC channels software
	///   triggers.
	/// `new_state : FunctionalState`: new state of the DAC channels software triggers.
	///   This parameter can be: ENABLE or DISABLE.
	fn dual_software_trigger_cmd(&self, new_state : bool) {
		self.dac.swtrigr    .write(|w| w
			.swtrig1() .bit( new_state )
			.swtrig2() .bit( new_state )
		);
	}
}
