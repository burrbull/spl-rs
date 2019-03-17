
/// SDIO_Interrupt_soucres 

#define SDIO_IT_CCRCFAIL                    ((uint32_t)0x00000001)
#define SDIO_IT_DCRCFAIL                    ((uint32_t)0x00000002)
#define SDIO_IT_CTIMEOUT                    ((uint32_t)0x00000004)
#define SDIO_IT_DTIMEOUT                    ((uint32_t)0x00000008)
#define SDIO_IT_TXUNDERR                    ((uint32_t)0x00000010)
#define SDIO_IT_RXOVERR                     ((uint32_t)0x00000020)
#define SDIO_IT_CMDREND                     ((uint32_t)0x00000040)
#define SDIO_IT_CMDSENT                     ((uint32_t)0x00000080)
#define SDIO_IT_DATAEND                     ((uint32_t)0x00000100)
#define SDIO_IT_STBITERR                    ((uint32_t)0x00000200)
#define SDIO_IT_DBCKEND                     ((uint32_t)0x00000400)
#define SDIO_IT_CMDACT                      ((uint32_t)0x00000800)
#define SDIO_IT_TXACT                       ((uint32_t)0x00001000)
#define SDIO_IT_RXACT                       ((uint32_t)0x00002000)
#define SDIO_IT_TXFIFOHE                    ((uint32_t)0x00004000)
#define SDIO_IT_RXFIFOHF                    ((uint32_t)0x00008000)
#define SDIO_IT_TXFIFOF                     ((uint32_t)0x00010000)
#define SDIO_IT_RXFIFOF                     ((uint32_t)0x00020000)
#define SDIO_IT_TXFIFOE                     ((uint32_t)0x00040000)
#define SDIO_IT_RXFIFOE                     ((uint32_t)0x00080000)
#define SDIO_IT_TXDAVL                      ((uint32_t)0x00100000)
#define SDIO_IT_RXDAVL                      ((uint32_t)0x00200000)
#define SDIO_IT_SDIOIT                      ((uint32_t)0x00400000)
#define SDIO_IT_CEATAEND                    ((uint32_t)0x00800000)

/// SDIO_Flags 

#define SDIO_FLAG_CCRCFAIL                  ((uint32_t)0x00000001)
#define SDIO_FLAG_DCRCFAIL                  ((uint32_t)0x00000002)
#define SDIO_FLAG_CTIMEOUT                  ((uint32_t)0x00000004)
#define SDIO_FLAG_DTIMEOUT                  ((uint32_t)0x00000008)
#define SDIO_FLAG_TXUNDERR                  ((uint32_t)0x00000010)
#define SDIO_FLAG_RXOVERR                   ((uint32_t)0x00000020)
#define SDIO_FLAG_CMDREND                   ((uint32_t)0x00000040)
#define SDIO_FLAG_CMDSENT                   ((uint32_t)0x00000080)
#define SDIO_FLAG_DATAEND                   ((uint32_t)0x00000100)
#define SDIO_FLAG_STBITERR                  ((uint32_t)0x00000200)
#define SDIO_FLAG_DBCKEND                   ((uint32_t)0x00000400)
#define SDIO_FLAG_CMDACT                    ((uint32_t)0x00000800)
#define SDIO_FLAG_TXACT                     ((uint32_t)0x00001000)
#define SDIO_FLAG_RXACT                     ((uint32_t)0x00002000)
#define SDIO_FLAG_TXFIFOHE                  ((uint32_t)0x00004000)
#define SDIO_FLAG_RXFIFOHF                  ((uint32_t)0x00008000)
#define SDIO_FLAG_TXFIFOF                   ((uint32_t)0x00010000)
#define SDIO_FLAG_RXFIFOF                   ((uint32_t)0x00020000)
#define SDIO_FLAG_TXFIFOE                   ((uint32_t)0x00040000)
#define SDIO_FLAG_RXFIFOE                   ((uint32_t)0x00080000)
#define SDIO_FLAG_TXDAVL                    ((uint32_t)0x00100000)
#define SDIO_FLAG_RXDAVL                    ((uint32_t)0x00200000)
#define SDIO_FLAG_SDIOIT                    ((uint32_t)0x00400000)
#define SDIO_FLAG_CEATAEND                  ((uint32_t)0x00800000)



void SDIO_ITConfig(uint32_t SDIO_IT, FunctionalState NewState);
FlagStatus SDIO_GetFlagStatus(uint32_t SDIO_FLAG);
void SDIO_ClearFlag(uint32_t SDIO_FLAG);
ITStatus SDIO_GetITStatus(uint32_t SDIO_IT);
void SDIO_ClearITPendingBit(uint32_t SDIO_IT);

/// Enables or disables the SDIO interrupts.
/// `SDIO_IT`: specifies the SDIO interrupt sources to be 
///   enabled or disabled.
///   This parameter can be one or a combination of the following values:
/// * SDIO_IT_CCRCFAIL: Command response received (CRC check failed) interrupt
/// * SDIO_IT_DCRCFAIL: Data block sent/received (CRC check failed) interrupt
/// * SDIO_IT_CTIMEOUT: Command response timeout interrupt
/// * SDIO_IT_DTIMEOUT: Data timeout interrupt
/// * SDIO_IT_TXUNDERR: Transmit FIFO underrun error interrupt
/// * SDIO_IT_RXOVERR:  Received FIFO overrun error interrupt
/// * SDIO_IT_CMDREND:  Command response received (CRC check passed) interrupt
/// * SDIO_IT_CMDSENT:  Command sent (no response required) interrupt
/// * SDIO_IT_DATAEND:  Data end (data counter, SDIDCOUNT, is zero) interrupt
/// * SDIO_IT_STBITERR: Start bit not detected on all data signals in wide 
///                        bus mode interrupt
/// * SDIO_IT_DBCKEND:  Data block sent/received (CRC check passed) interrupt
/// * SDIO_IT_CMDACT:   Command transfer in progress interrupt
/// * SDIO_IT_TXACT:    Data transmit in progress interrupt
/// * SDIO_IT_RXACT:    Data receive in progress interrupt
/// * SDIO_IT_TXFIFOHE: Transmit FIFO Half Empty interrupt
/// * SDIO_IT_RXFIFOHF: Receive FIFO Half Full interrupt
/// * SDIO_IT_TXFIFOF:  Transmit FIFO full interrupt
/// * SDIO_IT_RXFIFOF:  Receive FIFO full interrupt
/// * SDIO_IT_TXFIFOE:  Transmit FIFO empty interrupt
/// * SDIO_IT_RXFIFOE:  Receive FIFO empty interrupt
/// * SDIO_IT_TXDAVL:   Data available in transmit FIFO interrupt
/// * SDIO_IT_RXDAVL:   Data available in receive FIFO interrupt
/// * SDIO_IT_SDIOIT:   SD I/O interrupt received interrupt
/// * SDIO_IT_CEATAEND: CE-ATA command completion signal received for CMD61 
///                        interrupt
/// `NewState`: new state of the specified SDIO interrupts.
///   This parameter can be: ENABLE or DISABLE. 
void SDIO_ITConfig(uint32_t SDIO_IT, FunctionalState NewState) {
  
  if (NewState != DISABLE) {
    /* Enable the SDIO interrupts */
    SDIO->MASK |= SDIO_IT;
  } else {
    /* Disable the SDIO interrupts */
    SDIO->MASK &= ~SDIO_IT;
  } 
}


/// Checks whether the specified SDIO flag is set or not.
/// `SDIO_FLAG`: specifies the flag to check. 
///   This parameter can be one of the following values:
/// * SDIO_FLAG_CCRCFAIL: Command response received (CRC check failed)
/// * SDIO_FLAG_DCRCFAIL: Data block sent/received (CRC check failed)
/// * SDIO_FLAG_CTIMEOUT: Command response timeout
/// * SDIO_FLAG_DTIMEOUT: Data timeout
/// * SDIO_FLAG_TXUNDERR: Transmit FIFO underrun error
/// * SDIO_FLAG_RXOVERR:  Received FIFO overrun error
/// * SDIO_FLAG_CMDREND:  Command response received (CRC check passed)
/// * SDIO_FLAG_CMDSENT:  Command sent (no response required)
/// * SDIO_FLAG_DATAEND:  Data end (data counter, SDIDCOUNT, is zero)
/// * SDIO_FLAG_STBITERR: Start bit not detected on all data signals in wide 
///                          bus mode.
/// * SDIO_FLAG_DBCKEND:  Data block sent/received (CRC check passed)
/// * SDIO_FLAG_CMDACT:   Command transfer in progress
/// * SDIO_FLAG_TXACT:    Data transmit in progress
/// * SDIO_FLAG_RXACT:    Data receive in progress
/// * SDIO_FLAG_TXFIFOHE: Transmit FIFO Half Empty
/// * SDIO_FLAG_RXFIFOHF: Receive FIFO Half Full
/// * SDIO_FLAG_TXFIFOF:  Transmit FIFO full
/// * SDIO_FLAG_RXFIFOF:  Receive FIFO full
/// * SDIO_FLAG_TXFIFOE:  Transmit FIFO empty
/// * SDIO_FLAG_RXFIFOE:  Receive FIFO empty
/// * SDIO_FLAG_TXDAVL:   Data available in transmit FIFO
/// * SDIO_FLAG_RXDAVL:   Data available in receive FIFO
/// * SDIO_FLAG_SDIOIT:   SD I/O interrupt received
/// * SDIO_FLAG_CEATAEND: CE-ATA command completion signal received for CMD61
/// @retval : The new state of SDIO_FLAG (SET or RESET).
FlagStatus SDIO_GetFlagStatus(uint32_t SDIO_FLAG) { 
  FlagStatus bitstatus = RESET;
  
  
  if ((SDIO->STA & SDIO_FLAG) != (uint32_t)RESET) {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the SDIO's pending flags.
/// `SDIO_FLAG`: specifies the flag to clear.  
///   This parameter can be one or a combination of the following values:
/// * SDIO_FLAG_CCRCFAIL: Command response received (CRC check failed)
/// * SDIO_FLAG_DCRCFAIL: Data block sent/received (CRC check failed)
/// * SDIO_FLAG_CTIMEOUT: Command response timeout
/// * SDIO_FLAG_DTIMEOUT: Data timeout
/// * SDIO_FLAG_TXUNDERR: Transmit FIFO underrun error
/// * SDIO_FLAG_RXOVERR:  Received FIFO overrun error
/// * SDIO_FLAG_CMDREND:  Command response received (CRC check passed)
/// * SDIO_FLAG_CMDSENT:  Command sent (no response required)
/// * SDIO_FLAG_DATAEND:  Data end (data counter, SDIDCOUNT, is zero)
/// * SDIO_FLAG_STBITERR: Start bit not detected on all data signals in wide 
///                          bus mode
/// * SDIO_FLAG_DBCKEND:  Data block sent/received (CRC check passed)
/// * SDIO_FLAG_SDIOIT:   SD I/O interrupt received
/// * SDIO_FLAG_CEATAEND: CE-ATA command completion signal received for CMD61
void SDIO_ClearFlag(uint32_t SDIO_FLAG) {
   
  SDIO->ICR = SDIO_FLAG;
}

/// Checks whether the specified SDIO interrupt has occurred or not.
/// `SDIO_IT`: specifies the SDIO interrupt source to check. 
///   This parameter can be one of the following values:
/// * SDIO_IT_CCRCFAIL: Command response received (CRC check failed) interrupt
/// * SDIO_IT_DCRCFAIL: Data block sent/received (CRC check failed) interrupt
/// * SDIO_IT_CTIMEOUT: Command response timeout interrupt
/// * SDIO_IT_DTIMEOUT: Data timeout interrupt
/// * SDIO_IT_TXUNDERR: Transmit FIFO underrun error interrupt
/// * SDIO_IT_RXOVERR:  Received FIFO overrun error interrupt
/// * SDIO_IT_CMDREND:  Command response received (CRC check passed) interrupt
/// * SDIO_IT_CMDSENT:  Command sent (no response required) interrupt
/// * SDIO_IT_DATAEND:  Data end (data counter, SDIDCOUNT, is zero) interrupt
/// * SDIO_IT_STBITERR: Start bit not detected on all data signals in wide 
///                        bus mode interrupt
/// * SDIO_IT_DBCKEND:  Data block sent/received (CRC check passed) interrupt
/// * SDIO_IT_CMDACT:   Command transfer in progress interrupt
/// * SDIO_IT_TXACT:    Data transmit in progress interrupt
/// * SDIO_IT_RXACT:    Data receive in progress interrupt
/// * SDIO_IT_TXFIFOHE: Transmit FIFO Half Empty interrupt
/// * SDIO_IT_RXFIFOHF: Receive FIFO Half Full interrupt
/// * SDIO_IT_TXFIFOF:  Transmit FIFO full interrupt
/// * SDIO_IT_RXFIFOF:  Receive FIFO full interrupt
/// * SDIO_IT_TXFIFOE:  Transmit FIFO empty interrupt
/// * SDIO_IT_RXFIFOE:  Receive FIFO empty interrupt
/// * SDIO_IT_TXDAVL:   Data available in transmit FIFO interrupt
/// * SDIO_IT_RXDAVL:   Data available in receive FIFO interrupt
/// * SDIO_IT_SDIOIT:   SD I/O interrupt received interrupt
/// * SDIO_IT_CEATAEND: CE-ATA command completion signal received for CMD61 
///                        interrupt
/// @retval : The new state of SDIO_IT (SET or RESET).
ITStatus SDIO_GetITStatus(uint32_t SDIO_IT) { 
  ITStatus bitstatus = RESET;
  
  if ((SDIO->STA & SDIO_IT) != (uint32_t)RESET)   {
    bitstatus = SET;
  } else {
    bitstatus = RESET;
  }
  return bitstatus;
}

/// Clears the SDIO’s interrupt pending bits.
/// `SDIO_IT`: specifies the interrupt pending bit to clear. 
///   This parameter can be one or a combination of the following values:
/// * SDIO_IT_CCRCFAIL: Command response received (CRC check failed) interrupt
/// * SDIO_IT_DCRCFAIL: Data block sent/received (CRC check failed) interrupt
/// * SDIO_IT_CTIMEOUT: Command response timeout interrupt
/// * SDIO_IT_DTIMEOUT: Data timeout interrupt
/// * SDIO_IT_TXUNDERR: Transmit FIFO underrun error interrupt
/// * SDIO_IT_RXOVERR:  Received FIFO overrun error interrupt
/// * SDIO_IT_CMDREND:  Command response received (CRC check passed) interrupt
/// * SDIO_IT_CMDSENT:  Command sent (no response required) interrupt
/// * SDIO_IT_DATAEND:  Data end (data counter, SDIDCOUNT, is zero) interrupt
/// * SDIO_IT_STBITERR: Start bit not detected on all data signals in wide 
///                        bus mode interrupt
/// * SDIO_IT_SDIOIT:   SD I/O interrupt received interrupt
/// * SDIO_IT_CEATAEND: CE-ATA command completion signal received for CMD61
void SDIO_ClearITPendingBit(uint32_t SDIO_IT) {
   
  SDIO->ICR = SDIO_IT;
}

