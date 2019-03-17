

#define IS_CAN_ALL_PERIPH(PERIPH) (((*(uint32_t*)&(PERIPH)) == CAN1_BASE))

/// CAN init structure definition

pub struct CanStruct {
    pub TTCM : bool,
    pub ABOM : bool,
    pub AWUM : bool,
    pub NART : bool,
    pub RFLM : bool,
    pub TXFP : bool,
    pub Mode : CanMode,
    pub SJW  : CanSJWidth,
    pub BS1  : CanBitSegment1,
    pub BS2  : CanBitSegment2,
    pub Prescaler : u16
}


pub impl CanStruct {
    /// Fills each CAN_InitStruct member with its default value.
    /// `CAN_InitStruct`: pointer to a CAN_InitTypeDef structure which
    ///     will be initialized.
    pub fn init() -> CanStruct {
        /// Reset CAN init structure parameters values
        CanStruct {
            /* Initialize the time triggered communication mode */
            TTCM : false,
            /* Initialize the automatic bus-off management */
            ABOM : false,
            /* Initialize the automatic wake-up mode */
            AWUM : false,
            /* Initialize the no automatic retransmission */
            NART : false,
            /* Initialize the receive FIFO locked mode */
            RFLM : false,
            /* Initialize the transmit FIFO priority */
            TXFP : false,
            /* Initialize the CAN_Mode member */
            Mode : CanMode::Normal,
            /* Initialize the CAN_SJW member */
            SJW  : CanSJWidth::TQ1,
            /* Initialize the CAN_BS1 member */
            BS1  : CanBitSegment1::TQ4,
            /* Initialize the CAN_BS2 member */
            BS2  : CanBitSegment2::TQ3,
            /* Initialize the CAN_Prescaler member */
            Prescaler : 1
        }
    }
}


/// CAN filter init structure definition
pub struct CanFilterStruct {
    Number         : u8,
    Mode           : CanFilterMode,
    Scale          : CanFilterScale,
    IdHigh         : u16,
    IdLow          : u16,
    MaskIdHigh     : u16,
    MaskIdLow      : u16,
    FIFOAssignment : CanFilterFIFOAssignment;
    Activation     : bool;
}

/// CAN Tx message structure definition
pub struct CanTxMessageStruct {
    StdId : u32,
    ExtId : u32,
    IDE   : CanID,
    RTR   : CanRTR,
    DLC   : u8,
    Data  : [u8;8],
}

/// CAN Rx message structure definition
pub struct CanRxMessageStruct {
    StdId : u32,
    ExtId : u32,
    IDE   : CanID,
    RTR   : CanRTR,
    DLC   : u8,
    Data  : [u8;8],
    FMI   : u8,
}


/// CAN_sleep_constants 
#define CANINITFAILED              ((uint8_t)0x00) /* CAN initialization failed */
#define CANINITOK                  ((uint8_t)0x01) /* CAN initialization failed */


/// CAN_operating_mode
pub mod CanMode {
    pub const Normal          : u8 = 0x00;  /* normal mode */
    pub const LoopBack        : u8 = 0x01;  /* loopback mode */
    pub const Silent          : u8 = 0x02;  /* silent mode */
    pub const Silent_LoopBack : u8 = 0x03;  /* loopback combined with silent mode */
}

/// CAN_synchronisation_jump_width
pub use crate::pac::can::can_btr::SJWW as CanSynchronisationJumpWidth;
//#define CAN_SJW_1tq                 ((uint8_t)0x00)  /* 1 time quantum */
//#define CAN_SJW_2tq                 ((uint8_t)0x01)  /* 2 time quantum */
//#define CAN_SJW_3tq                 ((uint8_t)0x02)  /* 3 time quantum */
//#define CAN_SJW_4tq                 ((uint8_t)0x03)  /* 4 time quantum */


/// time_quantum_in_bit_segment_1
pub use crate::pac::can::can_btr::TS1W as CanBitSegment1;
//#define CAN_BS1_1tq                 ((uint8_t)0x00)  /* 1 time quantum */
//#define CAN_BS1_2tq                 ((uint8_t)0x01)  /* 2 time quantum */
//#define CAN_BS1_3tq                 ((uint8_t)0x02)  /* 3 time quantum */
//#define CAN_BS1_4tq                 ((uint8_t)0x03)  /* 4 time quantum */
//#define CAN_BS1_5tq                 ((uint8_t)0x04)  /* 5 time quantum */
//#define CAN_BS1_6tq                 ((uint8_t)0x05)  /* 6 time quantum */
//#define CAN_BS1_7tq                 ((uint8_t)0x06)  /* 7 time quantum */
//#define CAN_BS1_8tq                 ((uint8_t)0x07)  /* 8 time quantum */
//#define CAN_BS1_9tq                 ((uint8_t)0x08)  /* 9 time quantum */
//#define CAN_BS1_10tq                ((uint8_t)0x09)  /* 10 time quantum */
//#define CAN_BS1_11tq                ((uint8_t)0x0A)  /* 11 time quantum */
//#define CAN_BS1_12tq                ((uint8_t)0x0B)  /* 12 time quantum */
//#define CAN_BS1_13tq                ((uint8_t)0x0C)  /* 13 time quantum */
//#define CAN_BS1_14tq                ((uint8_t)0x0D)  /* 14 time quantum */
//#define CAN_BS1_15tq                ((uint8_t)0x0E)  /* 15 time quantum */
//#define CAN_BS1_16tq                ((uint8_t)0x0F)  /* 16 time quantum */

/// time_quantum_in_bit_segment_2 
pub use crate::pac::can::can_btr::TS1W as CanBitSegment2;
//#define CAN_BS2_1tq                 ((uint8_t)0x00)  /* 1 time quantum */
//#define CAN_BS2_2tq                 ((uint8_t)0x01)  /* 2 time quantum */
//#define CAN_BS2_3tq                 ((uint8_t)0x02)  /* 3 time quantum */
//#define CAN_BS2_4tq                 ((uint8_t)0x03)  /* 4 time quantum */
//#define CAN_BS2_5tq                 ((uint8_t)0x04)  /* 5 time quantum */
//#define CAN_BS2_6tq                 ((uint8_t)0x05)  /* 6 time quantum */
//#define CAN_BS2_7tq                 ((uint8_t)0x06)  /* 7 time quantum */
//#define CAN_BS2_8tq                 ((uint8_t)0x07)  /* 8 time quantum */


/// CAN_filter_mode
pub use crate::pac::can::can_fm1r::FBM0W as CanFilterMode;
//#define CAN_FilterMode_IdMask       ((uint8_t)0x00)  /* id/mask mode */
//#define CAN_FilterMode_IdList       ((uint8_t)0x01)  /* identifier list mode */


/// CAN_filter_scale
pub use crate::pac::can::can_fs1r::FSC0W as CanFilterScale;
//#define CAN_FilterScale_16bit       ((uint8_t)0x00) /* 16-bit filter scale */
//#define CAN_FilterScale_32bit       ((uint8_t)0x01) /* 2-bit filter scale */


/// CAN_filter_FIFO_assignation 
pub use crate::pac::can::can_ffa1r::FFA0W as CanFilterFIFOAssignment;
//#define CAN_FilterFIFO0             ((uint8_t)0x00)  /* Filter FIFO 0 assignment for filter x */
//#define CAN_FilterFIFO1             ((uint8_t)0x01)  /* Filter FIFO 1 assignment for filter x */


/// CAN_identifier_type
pub use crate::pac::can::can_ti0r::IDEW as CanID;
//#define CAN_ID_STD                 ((uint32_t)0x00000000)  /* Standard Id */
//#define CAN_ID_EXT                 ((uint32_t)0x00000004)  /* Extended Id */


/// CAN_remote_transmission_request 
pub use crate::pac::can::can_ti0r::RTRW as CanRTR;
//#define CAN_RTR_DATA                ((uint32_t)0x00000000)  /* Data frame */
//#define CAN_RTR_REMOTE              ((uint32_t)0x00000002)  /* Remote frame */


/// CAN_transmit_constants 

#define CANTXFAILED                 ((uint8_t)0x00) /* CAN transmission failed */
#define CANTXOK                     ((uint8_t)0x01) /* CAN transmission succeeded */
#define CANTXPENDING                ((uint8_t)0x02) /* CAN transmission pending */
#define CAN_NO_MB                   ((uint8_t)0x04) /* CAN cell did not provide an empty mailbox */


/// CAN_receive_FIFO_number_constants 

#define CAN_FIFO0                 ((uint8_t)0x00) /* CAN FIFO0 used to receive */
#define CAN_FIFO1                 ((uint8_t)0x01) /* CAN FIFO1 used to receive */


/// CAN_sleep_constants 

#define CANSLEEPFAILED              ((uint8_t)0x00) /* CAN did not enter the sleep mode */
#define CANSLEEPOK                  ((uint8_t)0x01) /* CAN entered the sleep mode */


/// CAN_wake_up_constants 

#define CANWAKEUPFAILED             ((uint8_t)0x00) /* CAN did not leave the sleep mode */
#define CANWAKEUPOK                 ((uint8_t)0x01) /* CAN leaved the sleep mode */


void CAN_DeInit(CAN_TypeDef* CANx);
uint8_t CAN_Init(CAN_TypeDef* CANx, CAN_InitTypeDef* CAN_InitStruct);
void CAN_FilterInit(CAN_FilterInitTypeDef* CAN_FilterInitStruct);
uint8_t CAN_Transmit(CAN_TypeDef* CANx, CanTxMsg* TxMessage);
uint8_t CAN_TransmitStatus(CAN_TypeDef* CANx, uint8_t TransmitMailbox);
void CAN_CancelTransmit(CAN_TypeDef* CANx, uint8_t Mailbox);
void CAN_FIFORelease(CAN_TypeDef* CANx, uint8_t FIFONumber);
uint8_t CAN_MessagePending(CAN_TypeDef* CANx, uint8_t FIFONumber);
void CAN_Receive(CAN_TypeDef* CANx, uint8_t FIFONumber, CanRxMsg* RxMessage);
void CAN_DBGFreeze(CAN_TypeDef* CANx, new_state : bool);
uint8_t CAN_Sleep(CAN_TypeDef* CANx);
uint8_t CAN_WakeUp(CAN_TypeDef* CANx);



/// CAN Master Control Register bits
pub mod CanMasterControl {
    /// Initialization request
    pub const INRQ  : u32 = 0x00000001;
    /// Sleep mode request
    pub const SLEEP : u32 = 0x00000002;
    /// Transmit FIFO priority
    pub const TXFP  : u32 = 0x00000004;
    /// Receive FIFO locked mode
    pub const RFLM  : u32 = 0x00000008;
    /// No automatic retransmission
    pub const NART  : u32 = 0x00000010;
    /// Automatic wake up mode
    pub const AWUM  : u32 = 0x00000020;
    /// Automatic bus-off management
    pub const ABOM  : u32 = 0x00000040;
    /// time triggered communication
    pub const TTCM  : u32 = 0x00000080;
    /// time triggered communication
    pub const RESET : u32 = 0x00008000;
    /// software master reset
    pub const DBF   : u32 = 0x00010000;
}

/// CAN Master Status Register bits
pub mod CanMasterStatus {
    /// Initialization acknowledge
    pub const INAK  : u32 = 0x00000001;
    /// Wake-up interrupt
    pub const WKUI  : u32 = 0x00000008;
    /// Sleep acknowledge interrupt
    pub const SLAKI : u32 = 0x00000010;
}

/// CAN Transmit Status Register bits
pub mod CanTransmitStatus {
    /// Request completed mailbox0
    pub const RQCP0 : u32 = 0x00000001;
    /// Transmission OK of mailbox0
    pub const TXOK0 : u32 = 0x00000002;
    /// Abort request for mailbox0
    pub const ABRQ0 : u32 = 0x00000080;
    /// Request completed mailbox1
    pub const RQCP1 : u32 = 0x00000100;
    /// Transmission OK of mailbox1
    pub const TXOK1 : u32 = 0x00000200;
    /// Abort request for mailbox1
    pub const ABRQ1 : u32 = 0x00008000;
    /// Request completed mailbox2
    pub const RQCP2 : u32 = 0x00010000;
    /// Transmission OK of mailbox2
    pub const TXOK2 : u32 = 0x00020000;
    /// Abort request for mailbox2
    pub const ABRQ2 : u32 = 0x00800000;
    /// Transmit mailbox 0 empty
    pub const TME0  : u32 = 0x04000000;
    /// Transmit mailbox 1 empty
    pub const TME1  : u32 = 0x08000000;
    /// Transmit mailbox 2 empty
    pub const TME2  : u32 = 0x10000000;
}


/* CAN Receive FIFO 0 Register bits */
#define RF0R_FULL0   ((uint32_t)0x00000008)    /* FIFO 0 full */
#define RF0R_FOVR0   ((uint32_t)0x00000010)    /* FIFO 0 overrun */
#define RF0R_RFOM0   ((uint32_t)0x00000020)    /* Release FIFO 0 output mailbox */

/* CAN Receive FIFO 1 Register bits */
#define RF1R_FULL1   ((uint32_t)0x00000008)    /* FIFO 1 full */
#define RF1R_FOVR1   ((uint32_t)0x00000010)    /* FIFO 1 overrun */
#define RF1R_RFOM1   ((uint32_t)0x00000020)    /* Release FIFO 1 output mailbox */

/* CAN Error Status Register bits */
#define ESR_EWGF     ((uint32_t)0x00000001)    /* Error warning flag */
#define ESR_EPVF     ((uint32_t)0x00000002)    /* Error passive flag */
#define ESR_BOFF     ((uint32_t)0x00000004)    /* Bus-off flag */

/* CAN Mailbox Transmit Request */
#define TMIDxR_TXRQ  ((uint32_t)0x00000001) /* Transmit mailbox request */

/* CAN Filter Master Register bits */
#define FMR_FINIT    ((uint32_t)0x00000001) /* Filter init mode */

/* Time out for INAK bit */
#define INAK_TimeOut        ((uint32_t)0x0000FFFF)

/* Time out for SLAK bit */
#define SLAK_TimeOut        ((uint32_t)0x0000FFFF)



static ITStatus CheckITStatus(uint32_t CAN_Reg, uint32_t It_Bit);



/// Initializes the CAN peripheral according to the specified
///     parameters in the CAN_InitStruct.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `CAN_InitStruct`: pointer to a CAN_InitTypeDef structure that
///     contains the configuration information for the CAN peripheral.
/// @retval : Constant indicates initialization succeed which will be 
///     CANINITFAILED or CANINITOK.
uint8_t CAN_Init(CAN_TypeDef* CANx, CAN_InitTypeDef* CAN_InitStruct) {
    uint8_t InitStatus = CANINITFAILED;
    uint32_t wait_ack = 0x00000000;
    /* exit from sleep mode */
    CANx->MCR &= ~MCR_SLEEP;
    /* Request initialisation */
    CANx->MCR |= MCR_INRQ ;
    /* Wait the acknowledge */
    while ((wait_ack != INAK_TimeOut)&&((CANx->MSR & MSR_INAK) != MSR_INAK)) {
        wait_ack++;
    }
    /* ...and check acknowledged */
    if ((CANx->MSR & MSR_INAK) != MSR_INAK) {
        InitStatus = CANINITFAILED;
    } else    {
        /* Set the time triggered communication mode */
        if (CAN_InitStruct->CAN_TTCM == ENABLE) {
            CANx->MCR |= MCR_TTCM;
        } else {
            CANx->MCR &= ~MCR_TTCM;
        }
        /* Set the automatic bus-off management */
        if (CAN_InitStruct->CAN_ABOM == ENABLE) {
            CANx->MCR |= MCR_ABOM;
        } else {
            CANx->MCR &= ~MCR_ABOM;
        }
        /* Set the automatic wake-up mode */
        if (CAN_InitStruct->CAN_AWUM == ENABLE) {
            CANx->MCR |= MCR_AWUM;
        } else {
            CANx->MCR &= ~MCR_AWUM;
        }
        /* Set the no automatic retransmission */
        if (CAN_InitStruct->CAN_NART == ENABLE) {
            CANx->MCR |= MCR_NART;
        } else {
            CANx->MCR &= ~MCR_NART;
        }
        /* Set the receive FIFO locked mode */
        if (CAN_InitStruct->CAN_RFLM == ENABLE) {
            CANx->MCR |= MCR_RFLM;
        } else {
            CANx->MCR &= ~MCR_RFLM;
        }
        /* Set the transmit FIFO priority */
        if (CAN_InitStruct->CAN_TXFP == ENABLE) {
            CANx->MCR |= MCR_TXFP;
        } else {
            CANx->MCR &= ~MCR_TXFP;
        }
        /* Set the bit timing register */
        CANx->BTR = (uint32_t)((uint32_t)CAN_InitStruct->CAN_Mode << 30) | ((uint32_t)CAN_InitStruct->CAN_SJW << 24) |
                             ((uint32_t)CAN_InitStruct->CAN_BS1 << 16) | ((uint32_t)CAN_InitStruct->CAN_BS2 << 20) |
                             ((uint32_t)CAN_InitStruct->CAN_Prescaler - 1);
        /* Request leave initialisation */
        CANx->MCR &= ~MCR_INRQ;
     /* Wait the acknowledge */
     wait_ack = 0x00;
     while ((wait_ack != INAK_TimeOut)&&((CANx->MSR & MSR_INAK) == MSR_INAK)) {
         wait_ack++;
     }
        /* ...and check acknowledged */
        if ((CANx->MSR & MSR_INAK) == MSR_INAK) {
            InitStatus = CANINITFAILED;
        } else {
            InitStatus = CANINITOK ;
        }
    }
    /* At this step, return the status of initialization */
    return InitStatus;
}

/// Initializes the CAN peripheral according to the specified
///     parameters in the CAN_FilterInitStruct.
/// `CAN_FilterInitStruct`: pointer to a CAN_FilterInitTypeDef
///     structure that contains the configuration information..
void CAN_FilterInit(CAN_FilterInitTypeDef* CAN_FilterInitStruct) {
    uint32_t filter_number_bit_pos = 0;
    filter_number_bit_pos = 
    (uint32_t)(((uint32_t)0x00000001) << ((uint32_t)CAN_FilterInitStruct->CAN_FilterNumber));
    /* Initialisation mode for the filter */
    CAN1->FMR |= FMR_FINIT;
    /* Filter Deactivation */
    CAN1->FA1R &= ~(uint32_t)filter_number_bit_pos;
    /* Filter Scale */
    if (CAN_FilterInitStruct->CAN_FilterScale == CAN_FilterScale_16bit) {
        /* 16-bit scale for the filter */
        CAN1->FS1R &= ~(uint32_t)filter_number_bit_pos;
        /* First 16-bit identifier and First 16-bit mask */
        /* Or First 16-bit identifier and Second 16-bit identifier */
        CAN1->sFilterRegister[CAN_FilterInitStruct->CAN_FilterNumber].FR1 = 
        ((uint32_t)((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterMaskIdLow) << 16) |
                ((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterIdLow);
        /* Second 16-bit identifier and Second 16-bit mask */
        /* Or Third 16-bit identifier and Fourth 16-bit identifier */
        CAN1->sFilterRegister[CAN_FilterInitStruct->CAN_FilterNumber].FR2 = 
        ((uint32_t)((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterMaskIdHigh) << 16) |
                ((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterIdHigh);
    }
    if (CAN_FilterInitStruct->CAN_FilterScale == CAN_FilterScale_32bit) {
        /* 32-bit scale for the filter */
        CAN1->FS1R |= filter_number_bit_pos;
        /* 32-bit identifier or First 32-bit identifier */
        CAN1->sFilterRegister[CAN_FilterInitStruct->CAN_FilterNumber].FR1 = 
        ((uint32_t)((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterIdHigh) << 16) |
                ((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterIdLow);
        /* 32-bit mask or Second 32-bit identifier */
        CAN1->sFilterRegister[CAN_FilterInitStruct->CAN_FilterNumber].FR2 = 
        ((uint32_t)((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterMaskIdHigh) << 16) |
                ((uint32_t)0x0000FFFF & CAN_FilterInitStruct->CAN_FilterMaskIdLow);
    }
    /* Filter Mode */
    if (CAN_FilterInitStruct->CAN_FilterMode == CAN_FilterMode_IdMask) {
        /*Id/Mask mode for the filter*/
        CAN1->FM1R &= ~(uint32_t)filter_number_bit_pos;
    } else /* CAN_FilterInitStruct->CAN_FilterMode == CAN_FilterMode_IdList */ {
        /*Identifier list mode for the filter*/
        CAN1->FM1R |= (uint32_t)filter_number_bit_pos;
    }
    /* Filter FIFO assignment */
    if (CAN_FilterInitStruct->CAN_FilterFIFOAssignment == CAN_FilterFIFO0) {
        /* FIFO 0 assignation for the filter */
        CAN1->FFA1R &= ~(uint32_t)filter_number_bit_pos;
    }
    if (CAN_FilterInitStruct->CAN_FilterFIFOAssignment == CAN_FilterFIFO1) {
        /* FIFO 1 assignation for the filter */
        CAN1->FFA1R |= (uint32_t)filter_number_bit_pos;
    }
    
    /* Filter activation */
    if (CAN_FilterInitStruct->CAN_FilterActivation == ENABLE) {
        CAN1->FA1R |= filter_number_bit_pos;
    }
    /* Leave the initialisation mode for the filter */
    CAN1->FMR &= ~FMR_FINIT;
}

/// Initiates the transmission of a message.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `TxMessage`: pointer to a structure which contains CAN Id, CAN
///     DLC and CAN datas.
/// @retval : The number of the mailbox that is used for transmission
///     or CAN_NO_MB if there is no empty mailbox.
uint8_t CAN_Transmit(CAN_TypeDef* CANx, CanTxMsg* TxMessage) {
    uint8_t transmit_mailbox = 0;
    /* Select one empty transmit mailbox */
    if ((CANx->TSR&TSR_TME0) == TSR_TME0) {
        transmit_mailbox = 0;
    } else if ((CANx->TSR&TSR_TME1) == TSR_TME1) {
        transmit_mailbox = 1;
    } else if ((CANx->TSR&TSR_TME2) == TSR_TME2) {
        transmit_mailbox = 2;
    } else {
        transmit_mailbox = CAN_NO_MB;
    }
    if (transmit_mailbox != CAN_NO_MB) {
        /* Set up the Id */
        CANx->sTxMailBox[transmit_mailbox].TIR &= TMIDxR_TXRQ;
        if (TxMessage->IDE == CAN_ID_STD) {
            CANx->sTxMailBox[transmit_mailbox].TIR |= ((TxMessage->StdId << 21) | TxMessage->RTR);
        } else {
            CANx->sTxMailBox[transmit_mailbox].TIR |= ((TxMessage->ExtId<<3) | TxMessage->IDE | 
                            TxMessage->RTR);
        }
        
        /* Set up the DLC */
        TxMessage->DLC &= (uint8_t)0x0000000F;
        CANx->sTxMailBox[transmit_mailbox].TDTR &= (uint32_t)0xFFFFFFF0;
        CANx->sTxMailBox[transmit_mailbox].TDTR |= TxMessage->DLC;
        /* Set up the data field */
        CANx->sTxMailBox[transmit_mailbox].TDLR = (((uint32_t)TxMessage->Data[3] << 24) | 
                            ((uint32_t)TxMessage->Data[2] << 16) |
                            ((uint32_t)TxMessage->Data[1] << 8) | 
                            ((uint32_t)TxMessage->Data[0]));
        CANx->sTxMailBox[transmit_mailbox].TDHR = (((uint32_t)TxMessage->Data[7] << 24) | 
                            ((uint32_t)TxMessage->Data[6] << 16) |
                            ((uint32_t)TxMessage->Data[5] << 8) |
                            ((uint32_t)TxMessage->Data[4]));
        /* Request transmission */
        CANx->sTxMailBox[transmit_mailbox].TIR |= TMIDxR_TXRQ;
    }
    return transmit_mailbox;
}

/// Checks the transmission of a message.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `transmit_mailbox`: the number of the mailbox that is used for
///     transmission.
/// @retval : CANTXOK if the CAN driver transmits the message, CANTXFAILED
///     in an other case.
uint8_t CAN_TransmitStatus(CAN_TypeDef* CANx, uint8_t transmit_mailbox) {
    /* RQCP, TXOK and TME bits */
    uint8_t state = 0;
    switch (transmit_mailbox) {
        case (0): state |= (uint8_t)((CANx->TSR & TSR_RQCP0) << 2);
            state |= (uint8_t)((CANx->TSR & TSR_TXOK0) >> 0);
            state |= (uint8_t)((CANx->TSR & TSR_TME0) >> 26);
            break;
        case (1): state |= (uint8_t)((CANx->TSR & TSR_RQCP1) >> 6);
            state |= (uint8_t)((CANx->TSR & TSR_TXOK1) >> 8);
            state |= (uint8_t)((CANx->TSR & TSR_TME1) >> 27);
            break;
        case (2): state |= (uint8_t)((CANx->TSR & TSR_RQCP2) >> 14);
            state |= (uint8_t)((CANx->TSR & TSR_TXOK2) >> 16);
            state |= (uint8_t)((CANx->TSR & TSR_TME2) >> 28);
            break;
        default:
            state = CANTXFAILED;
            break;
    }
    switch (state) {
            /* transmit pending    */
        case (0x0): state = CANTXPENDING;
            break;
            /* transmit failed    */
        case (0x5): state = CANTXFAILED;
            break;
            /* transmit succedeed    */
        case (0x7): state = CANTXOK;
            break;
        default:
            state = CANTXFAILED;
            break;
    }
    return state;
}

/// Cancels a transmit request.
/// `CANx`: where x can be 1 to select the CAN peripheral. 
/// `Mailbox`: Mailbox number..
void CAN_CancelTransmit(CAN_TypeDef* CANx, uint8_t Mailbox) {
    /* abort transmission */
    switch (Mailbox) {
        case (0): CANx->TSR |= TSR_ABRQ0;
            break;
        case (1): CANx->TSR |= TSR_ABRQ1;
            break;
        case (2): CANx->TSR |= TSR_ABRQ2;
            break;
        default:
            break;
    }
}

/// Releases a FIFO.
/// `CANx`: where x can be 1 to select the CAN peripheral. 
/// `FIFONumber`: FIFO to release, CAN_FIFO0 or CAN_FIFO1..
void CAN_FIFORelease(CAN_TypeDef* CANx, uint8_t FIFONumber) {
    /* Release FIFO0 */
    if (FIFONumber == CAN_FIFO0) {
        CANx->RF0R = RF0R_RFOM0;
    }
    /* Release FIFO1 */
    else /* FIFONumber == CAN_FIFO1 */ {
        CANx->RF1R = RF1R_RFOM1;
    }
}

/// Returns the number of pending messages.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `FIFONumber`: Receive FIFO number, CAN_FIFO0 or CAN_FIFO1.
/// @retval : NbMessage which is the number of pending message.
uint8_t CAN_MessagePending(CAN_TypeDef* CANx, uint8_t FIFONumber) {
    uint8_t message_pending=0;
    if (FIFONumber == CAN_FIFO0) {
        message_pending = (uint8_t)(CANx->RF0R&(uint32_t)0x03);
    } else if (FIFONumber == CAN_FIFO1) {
        message_pending = (uint8_t)(CANx->RF1R&(uint32_t)0x03);
    } else {
        message_pending = 0;
    }
    return message_pending;
}

/// Receives a message.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// `FIFONumber`: Receive FIFO number, CAN_FIFO0 or CAN_FIFO1.
/// `RxMessage`: pointer to a structure receive message which 
///     contains CAN Id, CAN DLC, CAN datas and FMI number..
void CAN_Receive(CAN_TypeDef* CANx, uint8_t FIFONumber, CanRxMsg* RxMessage) {
    /* Get the Id */
    RxMessage->IDE = (uint8_t)0x04 & CANx->sFIFOMailBox[FIFONumber].RIR;
    if (RxMessage->IDE == CAN_ID_STD) {
        RxMessage->StdId = (uint32_t)0x000007FF & (CANx->sFIFOMailBox[FIFONumber].RIR >> 21);
    } else {
        RxMessage->ExtId = (uint32_t)0x1FFFFFFF & (CANx->sFIFOMailBox[FIFONumber].RIR >> 3);
    }
    
    RxMessage->RTR = (uint8_t)0x02 & CANx->sFIFOMailBox[FIFONumber].RIR;
    /* Get the DLC */
    RxMessage->DLC = (uint8_t)0x0F & CANx->sFIFOMailBox[FIFONumber].RDTR;
    /* Get the FMI */
    RxMessage->FMI = (uint8_t)0xFF & (CANx->sFIFOMailBox[FIFONumber].RDTR >> 8);
    /* Get the data field */
    RxMessage->Data[0] = (uint8_t)0xFF & CANx->sFIFOMailBox[FIFONumber].RDLR;
    RxMessage->Data[1] = (uint8_t)0xFF & (CANx->sFIFOMailBox[FIFONumber].RDLR >> 8);
    RxMessage->Data[2] = (uint8_t)0xFF & (CANx->sFIFOMailBox[FIFONumber].RDLR >> 16);
    RxMessage->Data[3] = (uint8_t)0xFF & (CANx->sFIFOMailBox[FIFONumber].RDLR >> 24);
    RxMessage->Data[4] = (uint8_t)0xFF & CANx->sFIFOMailBox[FIFONumber].RDHR;
    RxMessage->Data[5] = (uint8_t)0xFF & (CANx->sFIFOMailBox[FIFONumber].RDHR >> 8);
    RxMessage->Data[6] = (uint8_t)0xFF & (CANx->sFIFOMailBox[FIFONumber].RDHR >> 16);
    RxMessage->Data[7] = (uint8_t)0xFF & (CANx->sFIFOMailBox[FIFONumber].RDHR >> 24);
    /* Release the FIFO */
    CAN_FIFORelease(CANx, FIFONumber);
}

/// Enables or disables the DBG Freeze for CAN.
/// @param    CANx: where x can be 1 to select the CAN peripheral.
/// `Newstate`: new state of the CAN peripheral.
///     This parameter can be: ENABLE or DISABLE..
void CAN_DBGFreeze(CAN_TypeDef* CANx, new_state : bool) {
     if (Newstate != DISABLE) {
        /* Enable Debug Freeze    */
        CANx->MCR |= MCR_DBF;
    } else {
        /* Disable Debug Freeze */
        CANx->MCR &= ~MCR_DBF;
    }
}

/// Enters the low power mode.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// @retval : CANSLEEPOK if sleep entered, CANSLEEPFAILED in an other case.
uint8_t CAN_Sleep(CAN_TypeDef* CANx) {
    uint8_t sleepstatus = CANSLEEPFAILED;
    
        
    /* Request Sleep mode */
     CANx->MCR = (((CANx->MCR) & (uint32_t)(~MCR_INRQ)) | MCR_SLEEP);
     
    /* Sleep mode status */
    if ((CANx->MSR & (CAN_MSR_SLAK|CAN_MSR_INAK)) == CAN_MSR_SLAK) {
        /* Sleep mode not entered */
        sleepstatus = CANSLEEPOK;
    }
    /* At this step, sleep mode status */
     return (uint8_t)sleepstatus;
}

/// Wakes the CAN up.
/// `CANx`: where x can be 1 to select the CAN peripheral.
/// @retval : CANWAKEUPOK if sleep mode left, CANWAKEUPFAILED in an other
///     case.
uint8_t CAN_WakeUp(CAN_TypeDef* CANx) {
    uint32_t wait_slak = SLAK_TimeOut    ;
    uint8_t wakeupstatus = CANWAKEUPFAILED;
    
        
    /* Wake up request */
    CANx->MCR &= ~MCR_SLEEP;
        
    /* Sleep mode status */
    while(((CANx->MSR & CAN_MSR_SLAK) == CAN_MSR_SLAK)&&(wait_slak!=0x00)) {
     wait_slak--;
    }
    if((CANx->MSR & CAN_MSR_SLAK) != CAN_MSR_SLAK) {
     /* Sleep mode exited */
        wakeupstatus = CANWAKEUPOK;
    }
    /* At this step, sleep mode status */
    return (uint8_t)wakeupstatus;
}

