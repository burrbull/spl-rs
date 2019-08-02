
/* DMA1 Channelx interrupt pending bit masks */
/*	pub const DMA1_Channel1_IT_Mask : u32 = 0xF << (4*0);
	pub const DMA1_Channel2_IT_Mask : u32 = 0xF << (4*1);
	pub const DMA1_Channel3_IT_Mask : u32 = 0xF << (4*2);
	pub const DMA1_Channel4_IT_Mask : u32 = 0xF << (4*3);
	pub const DMA1_Channel5_IT_Mask : u32 = 0xF << (4*4);
	pub const DMA1_Channel6_IT_Mask : u32 = 0xF << (4*5);
	pub const DMA1_Channel7_IT_Mask : u32 = 0xF << (4*6);
*/
/* DMA2 Channelx interrupt pending bit masks */
	pub const DMA2_Channel1_IT_Mask : u32 = 0xF << (4*0);
	pub const DMA2_Channel2_IT_Mask : u32 = 0xF << (4*1);
	pub const DMA2_Channel3_IT_Mask : u32 = 0xF << (4*2);
	pub const DMA2_Channel4_IT_Mask : u32 = 0xF << (4*3);
	pub const DMA2_Channel5_IT_Mask : u32 = 0xF << (4*4);

/* DMA2 FLAG mask */
	pub const FLAG_Mask : u32 = (1<<28)


/// DMA_interrupts_definition 
pub mod dmait {
	pub const TC : u32 = 1 << 1;
	pub const HT : u32 = 1 << 2;
	pub const TE : u32 = 1 << 3;

	/// For DMA1
	pub mod dma1 {
		pub const GL1 : u32 = 1 << 0;
		pub const TC1 : u32 = 1 << 1;
		pub const HT1 : u32 = 1 << 2;
		pub const TE1 : u32 = 1 << 3;
		pub const GL2 : u32 = 1 << 4;
		pub const TC2 : u32 = 1 << 5;
		pub const HT2 : u32 = 1 << 6;
		pub const TE2 : u32 = 1 << 7;
		pub const GL3 : u32 = 1 << 8;
		pub const TC3 : u32 = 1 << 9;
		pub const HT3 : u32 = 1 << 10;
		pub const TE3 : u32 = 1 << 11;
		pub const GL4 : u32 = 1 << 12;
		pub const TC4 : u32 = 1 << 13;
		pub const HT4 : u32 = 1 << 14;
		pub const TE4 : u32 = 1 << 15;
		pub const GL5 : u32 = 1 << 16;
		pub const TC5 : u32 = 1 << 17;
		pub const HT5 : u32 = 1 << 18;
		pub const TE5 : u32 = 1 << 19;
		pub const GL6 : u32 = 1 << 20;
		pub const TC6 : u32 = 1 << 21;
		pub const HT6 : u32 = 1 << 22;
		pub const TE6 : u32 = 1 << 23;
		pub const GL7 : u32 = 1 << 24;
		pub const TC7 : u32 = 1 << 25;
		pub const HT7 : u32 = 1 << 26;
		pub const TE7 : u32 = 1 << 27;
	}
	/// For DMA2
	pub mod dma2 {
		pub const GL1 : u32 = (1<<28) | (1 << 0);
		pub const TC1 : u32 = (1<<28) | (1 << 1);
		pub const HT1 : u32 = (1<<28) | (1 << 2);
		pub const TE1 : u32 = (1<<28) | (1 << 3);
		pub const GL2 : u32 = (1<<28) | (1 << 4);
		pub const TC2 : u32 = (1<<28) | (1 << 5);
		pub const HT2 : u32 = (1<<28) | (1 << 6);
		pub const TE2 : u32 = (1<<28) | (1 << 7);
		pub const GL3 : u32 = (1<<28) | (1 << 8);
		pub const TC3 : u32 = (1<<28) | (1 << 9);
		pub const HT3 : u32 = (1<<28) | (1 << 10);
		pub const TE3 : u32 = (1<<28) | (1 << 11);
		pub const GL4 : u32 = (1<<28) | (1 << 12);
		pub const TC4 : u32 = (1<<28) | (1 << 13);
		pub const HT4 : u32 = (1<<28) | (1 << 14);
		pub const TE4 : u32 = (1<<28) | (1 << 15);
		pub const GL5 : u32 = (1<<28) | (1 << 16);
		pub const TC5 : u32 = (1<<28) | (1 << 17);
		pub const HT5 : u32 = (1<<28) | (1 << 18);
		pub const TE5 : u32 = (1<<28) | (1 << 19);
	}
}

/// DMA_flags_definition
pub mod dmaflag {
	/// For DMA1
	pub mod dma1 {
		pub const GL1 : u32 = 1 << 0;
		pub const TC1 : u32 = 1 << 1;
		pub const HT1 : u32 = 1 << 2;
		pub const TE1 : u32 = 1 << 3;
		pub const GL2 : u32 = 1 << 4;
		pub const TC2 : u32 = 1 << 5;
		pub const HT2 : u32 = 1 << 6;
		pub const TE2 : u32 = 1 << 7;
		pub const GL3 : u32 = 1 << 8;
		pub const TC3 : u32 = 1 << 9;
		pub const HT3 : u32 = 1 << 10;
		pub const TE3 : u32 = 1 << 11;
		pub const GL4 : u32 = 1 << 12;
		pub const TC4 : u32 = 1 << 13;
		pub const HT4 : u32 = 1 << 14;
		pub const TE4 : u32 = 1 << 15;
		pub const GL5 : u32 = 1 << 16;
		pub const TC5 : u32 = 1 << 17;
		pub const HT5 : u32 = 1 << 18;
		pub const TE5 : u32 = 1 << 19;
		pub const GL6 : u32 = 1 << 20;
		pub const TC6 : u32 = 1 << 21;
		pub const HT6 : u32 = 1 << 22;
		pub const TE6 : u32 = 1 << 23;
		pub const GL7 : u32 = 1 << 24;
		pub const TC7 : u32 = 1 << 25;
		pub const HT7 : u32 = 1 << 26;
		pub const TE7 : u32 = 1 << 27;
	}
	/// For DMA2
	pub mod dma2 {
		pub const GL1 : u32 = (1<<28) | (1 << 0);
		pub const TC1 : u32 = (1<<28) | (1 << 1);
		pub const HT1 : u32 = (1<<28) | (1 << 2);
		pub const TE1 : u32 = (1<<28) | (1 << 3);
		pub const GL2 : u32 = (1<<28) | (1 << 4);
		pub const TC2 : u32 = (1<<28) | (1 << 5);
		pub const HT2 : u32 = (1<<28) | (1 << 6);
		pub const TE2 : u32 = (1<<28) | (1 << 7);
		pub const GL3 : u32 = (1<<28) | (1 << 8);
		pub const TC3 : u32 = (1<<28) | (1 << 9);
		pub const HT3 : u32 = (1<<28) | (1 << 10);
		pub const TE3 : u32 = (1<<28) | (1 << 11);
		pub const GL4 : u32 = (1<<28) | (1 << 12);
		pub const TC4 : u32 = (1<<28) | (1 << 13);
		pub const HT4 : u32 = (1<<28) | (1 << 14);
		pub const TE4 : u32 = (1<<28) | (1 << 15);
		pub const GL5 : u32 = (1<<28) | (1 << 16);
		pub const TC5 : u32 = (1<<28) | (1 << 17);
		pub const HT5 : u32 = (1<<28) | (1 << 18);
		pub const TE5 : u32 = (1<<28) | (1 << 19);
	}
}


void DMA_ITConfig(DMA_Channel_TypeDef* DMAy_Channelx, uint32_t DMA_IT, FunctionalState NewState);
FlagStatus DMA_GetFlagStatus(uint32_t DMA_FLAG);
void DMA_ClearFlag(uint32_t DMA_FLAG);
ITStatus DMA_GetITStatus(uint32_t DMA_IT);
void DMA_ClearITPendingBit(uint32_t DMA_IT);


/// Enables or disables the specified DMAy Channelx interrupts.
/// `DMAy_Channelx`: where y can be 1 or 2 to select the DMA and 
///   x can be 1 to 7 for DMA1 and 1 to 5 for DMA2 to select the 
///   DMA Channel.
/// `DMA_IT`: specifies the DMA interrupts sources to be enabled
///   or disabled. 
///   This parameter can be any combination of the following values:
/// * DMA_IT_TC:  Transfer complete interrupt mask
/// * DMA_IT_HT:  Half transfer interrupt mask
/// * DMA_IT_TE:  Transfer error interrupt mask
/// `NewState`: new state of the specified DMA interrupts.
///   This parameter can be: ENABLE or DISABLE.
void DMA_ITConfig(DMA_Channel_TypeDef* DMAy_Channelx, uint32_t DMA_IT, FunctionalState NewState) {
  if (NewState != DISABLE) {
    /* Enable the selected DMA interrupts */
    DMAy_Channelx->CCR |= DMA_IT;
  } else {
    /* Disable the selected DMA interrupts */
    DMAy_Channelx->CCR &= ~DMA_IT;
  }
}

/// Checks whether the specified DMAy Channelx flag is set or not.
/// `DMA_FLAG`: specifies the flag to check.
///   This parameter can be one of the following values:
/// * DMA1_FLAG_GL1: DMA1 Channel1 global flag.
/// * DMA1_FLAG_TC1: DMA1 Channel1 transfer complete flag.
/// * DMA1_FLAG_HT1: DMA1 Channel1 half transfer flag.
/// * DMA1_FLAG_TE1: DMA1 Channel1 transfer error flag.
/// * DMA1_FLAG_GL2: DMA1 Channel2 global flag.
/// * DMA1_FLAG_TC2: DMA1 Channel2 transfer complete flag.
/// * DMA1_FLAG_HT2: DMA1 Channel2 half transfer flag.
/// * DMA1_FLAG_TE2: DMA1 Channel2 transfer error flag.
/// * DMA1_FLAG_GL3: DMA1 Channel3 global flag.
/// * DMA1_FLAG_TC3: DMA1 Channel3 transfer complete flag.
/// * DMA1_FLAG_HT3: DMA1 Channel3 half transfer flag.
/// * DMA1_FLAG_TE3: DMA1 Channel3 transfer error flag.
/// * DMA1_FLAG_GL4: DMA1 Channel4 global flag.
/// * DMA1_FLAG_TC4: DMA1 Channel4 transfer complete flag.
/// * DMA1_FLAG_HT4: DMA1 Channel4 half transfer flag.
/// * DMA1_FLAG_TE4: DMA1 Channel4 transfer error flag.
/// * DMA1_FLAG_GL5: DMA1 Channel5 global flag.
/// * DMA1_FLAG_TC5: DMA1 Channel5 transfer complete flag.
/// * DMA1_FLAG_HT5: DMA1 Channel5 half transfer flag.
/// * DMA1_FLAG_TE5: DMA1 Channel5 transfer error flag.
/// * DMA1_FLAG_GL6: DMA1 Channel6 global flag.
/// * DMA1_FLAG_TC6: DMA1 Channel6 transfer complete flag.
/// * DMA1_FLAG_HT6: DMA1 Channel6 half transfer flag.
/// * DMA1_FLAG_TE6: DMA1 Channel6 transfer error flag.
/// * DMA1_FLAG_GL7: DMA1 Channel7 global flag.
/// * DMA1_FLAG_TC7: DMA1 Channel7 transfer complete flag.
/// * DMA1_FLAG_HT7: DMA1 Channel7 half transfer flag.
/// * DMA1_FLAG_TE7: DMA1 Channel7 transfer error flag.
/// * DMA2_FLAG_GL1: DMA2 Channel1 global flag.
/// * DMA2_FLAG_TC1: DMA2 Channel1 transfer complete flag.
/// * DMA2_FLAG_HT1: DMA2 Channel1 half transfer flag.
/// * DMA2_FLAG_TE1: DMA2 Channel1 transfer error flag.
/// * DMA2_FLAG_GL2: DMA2 Channel2 global flag.
/// * DMA2_FLAG_TC2: DMA2 Channel2 transfer complete flag.
/// * DMA2_FLAG_HT2: DMA2 Channel2 half transfer flag.
/// * DMA2_FLAG_TE2: DMA2 Channel2 transfer error flag.
/// * DMA2_FLAG_GL3: DMA2 Channel3 global flag.
/// * DMA2_FLAG_TC3: DMA2 Channel3 transfer complete flag.
/// * DMA2_FLAG_HT3: DMA2 Channel3 half transfer flag.
/// * DMA2_FLAG_TE3: DMA2 Channel3 transfer error flag.
/// * DMA2_FLAG_GL4: DMA2 Channel4 global flag.
/// * DMA2_FLAG_TC4: DMA2 Channel4 transfer complete flag.
/// * DMA2_FLAG_HT4: DMA2 Channel4 half transfer flag.
/// * DMA2_FLAG_TE4: DMA2 Channel4 transfer error flag.
/// * DMA2_FLAG_GL5: DMA2 Channel5 global flag.
/// * DMA2_FLAG_TC5: DMA2 Channel5 transfer complete flag.
/// * DMA2_FLAG_HT5: DMA2 Channel5 half transfer flag.
/// * DMA2_FLAG_TE5: DMA2 Channel5 transfer error flag.
/// @retval : The new state of DMA_FLAG (SET or RESET).
FlagStatus DMA_GetFlagStatus(uint32_t DMA_FLAG) {
  FlagStatus bitstatus = RESET;
  uint32_t tmpreg = 0;
  /* Calculate the used DMA */
  if ((DMA_FLAG & FLAG_Mask) != (uint32_t)RESET) {
    /* Get DMA2 ISR register value */
    tmpreg = DMA2->ISR ;
  } else {
    /* Get DMA1 ISR register value */
    tmpreg = DMA1->ISR ;
  }
  /* Check the status of the specified DMA flag */
  if ((tmpreg & DMA_FLAG) != (uint32_t)RESET) {
    /* DMA_FLAG is set */
    bitstatus = SET;
  } else {
    /* DMA_FLAG is reset */
    bitstatus = RESET;
  }
  
  /* Return the DMA_FLAG status */
  return  bitstatus;
}

/// Clears the DMAy Channelx's pending flags.
/// `DMA_FLAG`: specifies the flag to clear.
///   This parameter can be any combination (for the same DMA) of 
///   the following values:
/// * DMA1_FLAG_GL1: DMA1 Channel1 global flag.
/// * DMA1_FLAG_TC1: DMA1 Channel1 transfer complete flag.
/// * DMA1_FLAG_HT1: DMA1 Channel1 half transfer flag.
/// * DMA1_FLAG_TE1: DMA1 Channel1 transfer error flag.
/// * DMA1_FLAG_GL2: DMA1 Channel2 global flag.
/// * DMA1_FLAG_TC2: DMA1 Channel2 transfer complete flag.
/// * DMA1_FLAG_HT2: DMA1 Channel2 half transfer flag.
/// * DMA1_FLAG_TE2: DMA1 Channel2 transfer error flag.
/// * DMA1_FLAG_GL3: DMA1 Channel3 global flag.
/// * DMA1_FLAG_TC3: DMA1 Channel3 transfer complete flag.
/// * DMA1_FLAG_HT3: DMA1 Channel3 half transfer flag.
/// * DMA1_FLAG_TE3: DMA1 Channel3 transfer error flag.
/// * DMA1_FLAG_GL4: DMA1 Channel4 global flag.
/// * DMA1_FLAG_TC4: DMA1 Channel4 transfer complete flag.
/// * DMA1_FLAG_HT4: DMA1 Channel4 half transfer flag.
/// * DMA1_FLAG_TE4: DMA1 Channel4 transfer error flag.
/// * DMA1_FLAG_GL5: DMA1 Channel5 global flag.
/// * DMA1_FLAG_TC5: DMA1 Channel5 transfer complete flag.
/// * DMA1_FLAG_HT5: DMA1 Channel5 half transfer flag.
/// * DMA1_FLAG_TE5: DMA1 Channel5 transfer error flag.
/// * DMA1_FLAG_GL6: DMA1 Channel6 global flag.
/// * DMA1_FLAG_TC6: DMA1 Channel6 transfer complete flag.
/// * DMA1_FLAG_HT6: DMA1 Channel6 half transfer flag.
/// * DMA1_FLAG_TE6: DMA1 Channel6 transfer error flag.
/// * DMA1_FLAG_GL7: DMA1 Channel7 global flag.
/// * DMA1_FLAG_TC7: DMA1 Channel7 transfer complete flag.
/// * DMA1_FLAG_HT7: DMA1 Channel7 half transfer flag.
/// * DMA1_FLAG_TE7: DMA1 Channel7 transfer error flag.
/// * DMA2_FLAG_GL1: DMA2 Channel1 global flag.
/// * DMA2_FLAG_TC1: DMA2 Channel1 transfer complete flag.
/// * DMA2_FLAG_HT1: DMA2 Channel1 half transfer flag.
/// * DMA2_FLAG_TE1: DMA2 Channel1 transfer error flag.
/// * DMA2_FLAG_GL2: DMA2 Channel2 global flag.
/// * DMA2_FLAG_TC2: DMA2 Channel2 transfer complete flag.
/// * DMA2_FLAG_HT2: DMA2 Channel2 half transfer flag.
/// * DMA2_FLAG_TE2: DMA2 Channel2 transfer error flag.
/// * DMA2_FLAG_GL3: DMA2 Channel3 global flag.
/// * DMA2_FLAG_TC3: DMA2 Channel3 transfer complete flag.
/// * DMA2_FLAG_HT3: DMA2 Channel3 half transfer flag.
/// * DMA2_FLAG_TE3: DMA2 Channel3 transfer error flag.
/// * DMA2_FLAG_GL4: DMA2 Channel4 global flag.
/// * DMA2_FLAG_TC4: DMA2 Channel4 transfer complete flag.
/// * DMA2_FLAG_HT4: DMA2 Channel4 half transfer flag.
/// * DMA2_FLAG_TE4: DMA2 Channel4 transfer error flag.
/// * DMA2_FLAG_GL5: DMA2 Channel5 global flag.
/// * DMA2_FLAG_TC5: DMA2 Channel5 transfer complete flag.
/// * DMA2_FLAG_HT5: DMA2 Channel5 half transfer flag.
/// * DMA2_FLAG_TE5: DMA2 Channel5 transfer error flag.
void DMA_ClearFlag(uint32_t DMA_FLAG) {
  /* Calculate the used DMA */
  if ((DMA_FLAG & FLAG_Mask) != (uint32_t)RESET) {
    /* Clear the selected DMA flags */
    DMA2->IFCR = DMA_FLAG;
  } else {
    /* Clear the selected DMA flags */
    DMA1->IFCR = DMA_FLAG;
  }
}

/// Checks whether the specified DMAy Channelx interrupt has 
///   occurred or not.
/// `DMA_IT`: specifies the DMA interrupt source to check. 
///   This parameter can be one of the following values:
/// * DMA1_IT_GL1: DMA1 Channel1 global interrupt.
/// * DMA1_IT_TC1: DMA1 Channel1 transfer complete interrupt.
/// * DMA1_IT_HT1: DMA1 Channel1 half transfer interrupt.
/// * DMA1_IT_TE1: DMA1 Channel1 transfer error interrupt.
/// * DMA1_IT_GL2: DMA1 Channel2 global interrupt.
/// * DMA1_IT_TC2: DMA1 Channel2 transfer complete interrupt.
/// * DMA1_IT_HT2: DMA1 Channel2 half transfer interrupt.
/// * DMA1_IT_TE2: DMA1 Channel2 transfer error interrupt.
/// * DMA1_IT_GL3: DMA1 Channel3 global interrupt.
/// * DMA1_IT_TC3: DMA1 Channel3 transfer complete interrupt.
/// * DMA1_IT_HT3: DMA1 Channel3 half transfer interrupt.
/// * DMA1_IT_TE3: DMA1 Channel3 transfer error interrupt.
/// * DMA1_IT_GL4: DMA1 Channel4 global interrupt.
/// * DMA1_IT_TC4: DMA1 Channel4 transfer complete interrupt.
/// * DMA1_IT_HT4: DMA1 Channel4 half transfer interrupt.
/// * DMA1_IT_TE4: DMA1 Channel4 transfer error interrupt.
/// * DMA1_IT_GL5: DMA1 Channel5 global interrupt.
/// * DMA1_IT_TC5: DMA1 Channel5 transfer complete interrupt.
/// * DMA1_IT_HT5: DMA1 Channel5 half transfer interrupt.
/// * DMA1_IT_TE5: DMA1 Channel5 transfer error interrupt.
/// * DMA1_IT_GL6: DMA1 Channel6 global interrupt.
/// * DMA1_IT_TC6: DMA1 Channel6 transfer complete interrupt.
/// * DMA1_IT_HT6: DMA1 Channel6 half transfer interrupt.
/// * DMA1_IT_TE6: DMA1 Channel6 transfer error interrupt.
/// * DMA1_IT_GL7: DMA1 Channel7 global interrupt.
/// * DMA1_IT_TC7: DMA1 Channel7 transfer complete interrupt.
/// * DMA1_IT_HT7: DMA1 Channel7 half transfer interrupt.
/// * DMA1_IT_TE7: DMA1 Channel7 transfer error interrupt.
/// * DMA2_IT_GL1: DMA2 Channel1 global interrupt.
/// * DMA2_IT_TC1: DMA2 Channel1 transfer complete interrupt.
/// * DMA2_IT_HT1: DMA2 Channel1 half transfer interrupt.
/// * DMA2_IT_TE1: DMA2 Channel1 transfer error interrupt.
/// * DMA2_IT_GL2: DMA2 Channel2 global interrupt.
/// * DMA2_IT_TC2: DMA2 Channel2 transfer complete interrupt.
/// * DMA2_IT_HT2: DMA2 Channel2 half transfer interrupt.
/// * DMA2_IT_TE2: DMA2 Channel2 transfer error interrupt.
/// * DMA2_IT_GL3: DMA2 Channel3 global interrupt.
/// * DMA2_IT_TC3: DMA2 Channel3 transfer complete interrupt.
/// * DMA2_IT_HT3: DMA2 Channel3 half transfer interrupt.
/// * DMA2_IT_TE3: DMA2 Channel3 transfer error interrupt.
/// * DMA2_IT_GL4: DMA2 Channel4 global interrupt.
/// * DMA2_IT_TC4: DMA2 Channel4 transfer complete interrupt.
/// * DMA2_IT_HT4: DMA2 Channel4 half transfer interrupt.
/// * DMA2_IT_TE4: DMA2 Channel4 transfer error interrupt.
/// * DMA2_IT_GL5: DMA2 Channel5 global interrupt.
/// * DMA2_IT_TC5: DMA2 Channel5 transfer complete interrupt.
/// * DMA2_IT_HT5: DMA2 Channel5 half transfer interrupt.
/// * DMA2_IT_TE5: DMA2 Channel5 transfer error interrupt.
/// @retval : The new state of DMA_IT (SET or RESET).
ITStatus DMA_GetITStatus(uint32_t DMA_IT) {
  ITStatus bitstatus = RESET;
  uint32_t tmpreg = 0;
  /* Calculate the used DMA */
  if ((DMA_IT & FLAG_Mask) != (uint32_t)RESET) {
    /* Get DMA2 ISR register value */
    tmpreg = DMA2->ISR ;
  } else {
    /* Get DMA1 ISR register value */
    tmpreg = DMA1->ISR ;
  }
  /* Check the status of the specified DMA interrupt */
  if ((tmpreg & DMA_IT) != (uint32_t)RESET) {
    /* DMA_IT is set */
    bitstatus = SET;
  } else {
    /* DMA_IT is reset */
    bitstatus = RESET;
  }
  /* Return the DMA_IT status */
  return  bitstatus;
}

/// Clears the DMAy Channelx’s interrupt pending bits.
/// `DMA_IT`: specifies the DMA interrupt pending bit to clear.
///   This parameter can be any combination (for the same DMA) of
///   the following values:
/// * DMA1_IT_GL1: DMA1 Channel1 global interrupt.
/// * DMA1_IT_TC1: DMA1 Channel1 transfer complete interrupt.
/// * DMA1_IT_HT1: DMA1 Channel1 half transfer interrupt.
/// * DMA1_IT_TE1: DMA1 Channel1 transfer error interrupt.
/// * DMA1_IT_GL2: DMA1 Channel2 global interrupt.
/// * DMA1_IT_TC2: DMA1 Channel2 transfer complete interrupt.
/// * DMA1_IT_HT2: DMA1 Channel2 half transfer interrupt.
/// * DMA1_IT_TE2: DMA1 Channel2 transfer error interrupt.
/// * DMA1_IT_GL3: DMA1 Channel3 global interrupt.
/// * DMA1_IT_TC3: DMA1 Channel3 transfer complete interrupt.
/// * DMA1_IT_HT3: DMA1 Channel3 half transfer interrupt.
/// * DMA1_IT_TE3: DMA1 Channel3 transfer error interrupt.
/// * DMA1_IT_GL4: DMA1 Channel4 global interrupt.
/// * DMA1_IT_TC4: DMA1 Channel4 transfer complete interrupt.
/// * DMA1_IT_HT4: DMA1 Channel4 half transfer interrupt.
/// * DMA1_IT_TE4: DMA1 Channel4 transfer error interrupt.
/// * DMA1_IT_GL5: DMA1 Channel5 global interrupt.
/// * DMA1_IT_TC5: DMA1 Channel5 transfer complete interrupt.
/// * DMA1_IT_HT5: DMA1 Channel5 half transfer interrupt.
/// * DMA1_IT_TE5: DMA1 Channel5 transfer error interrupt.
/// * DMA1_IT_GL6: DMA1 Channel6 global interrupt.
/// * DMA1_IT_TC6: DMA1 Channel6 transfer complete interrupt.
/// * DMA1_IT_HT6: DMA1 Channel6 half transfer interrupt.
/// * DMA1_IT_TE6: DMA1 Channel6 transfer error interrupt.
/// * DMA1_IT_GL7: DMA1 Channel7 global interrupt.
/// * DMA1_IT_TC7: DMA1 Channel7 transfer complete interrupt.
/// * DMA1_IT_HT7: DMA1 Channel7 half transfer interrupt.
/// * DMA1_IT_TE7: DMA1 Channel7 transfer error interrupt.
/// * DMA2_IT_GL1: DMA2 Channel1 global interrupt.
/// * DMA2_IT_TC1: DMA2 Channel1 transfer complete interrupt.
/// * DMA2_IT_HT1: DMA2 Channel1 half transfer interrupt.
/// * DMA2_IT_TE1: DMA2 Channel1 transfer error interrupt.
/// * DMA2_IT_GL2: DMA2 Channel2 global interrupt.
/// * DMA2_IT_TC2: DMA2 Channel2 transfer complete interrupt.
/// * DMA2_IT_HT2: DMA2 Channel2 half transfer interrupt.
/// * DMA2_IT_TE2: DMA2 Channel2 transfer error interrupt.
/// * DMA2_IT_GL3: DMA2 Channel3 global interrupt.
/// * DMA2_IT_TC3: DMA2 Channel3 transfer complete interrupt.
/// * DMA2_IT_HT3: DMA2 Channel3 half transfer interrupt.
/// * DMA2_IT_TE3: DMA2 Channel3 transfer error interrupt.
/// * DMA2_IT_GL4: DMA2 Channel4 global interrupt.
/// * DMA2_IT_TC4: DMA2 Channel4 transfer complete interrupt.
/// * DMA2_IT_HT4: DMA2 Channel4 half transfer interrupt.
/// * DMA2_IT_TE4: DMA2 Channel4 transfer error interrupt.
/// * DMA2_IT_GL5: DMA2 Channel5 global interrupt.
/// * DMA2_IT_TC5: DMA2 Channel5 transfer complete interrupt.
/// * DMA2_IT_HT5: DMA2 Channel5 half transfer interrupt.
/// * DMA2_IT_TE5: DMA2 Channel5 transfer error interrupt.
void DMA_ClearITPendingBit(uint32_t DMA_IT) {
  /* Calculate the used DMA */
  if ((DMA_IT & FLAG_Mask) != (uint32_t)RESET) {
    /* Clear the selected DMA interrupt pending bits */
    DMA2->IFCR = DMA_IT;
  } else {
    /* Clear the selected DMA interrupt pending bits */
    DMA1->IFCR = DMA_IT;
  }
}

