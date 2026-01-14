///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**PE: Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. -0: No parity error -1: Parity error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    ///0: No parity error
    NoError = 0,
    ///1: Parity error
    Error = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - PE: Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. -0: No parity error -1: Parity error
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::NoError,
            true => PE::Error,
        }
    }
    ///No parity error
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PE::NoError
    }
    ///Parity error
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PE::Error
    }
}
/**FE: Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. In Smartcard mode, in transmission, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. -0: No Framing error is detected -1: Framing error or break character is detected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE {
    ///0: No Framing error is detected
    NoError = 0,
    ///1: Framing error or break character is detected
    Error = 1,
}
impl From<FE> for bool {
    #[inline(always)]
    fn from(variant: FE) -> Self {
        variant as u8 != 0
    }
}
///Field `FE` reader - FE: Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. In Smartcard mode, in transmission, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. -0: No Framing error is detected -1: Framing error or break character is detected
pub type FE_R = crate::BitReader<FE>;
impl FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FE {
        match self.bits {
            false => FE::NoError,
            true => FE::Error,
        }
    }
    ///No Framing error is detected
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FE::NoError
    }
    ///Framing error or break character is detected
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FE::Error
    }
}
/**NF: START bit Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. -0: No noise is detected -1: Noise is detected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NF {
    ///0: No noise is detected
    NoNoise = 0,
    ///1: Noise is detected
    Noise = 1,
}
impl From<NF> for bool {
    #[inline(always)]
    fn from(variant: NF) -> Self {
        variant as u8 != 0
    }
}
///Field `NF` reader - NF: START bit Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. -0: No noise is detected -1: Noise is detected
pub type NF_R = crate::BitReader<NF>;
impl NF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NF {
        match self.bits {
            false => NF::NoNoise,
            true => NF::Noise,
        }
    }
    ///No noise is detected
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == NF::NoNoise
    }
    ///Noise is detected
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NF::Noise
    }
}
/**ORE: Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USARTx_RDR register while RXNE=1 (RXFF = 1 in case FIFO mode is enabled) . It is cleared by a software, writing 1 to the ORECF, in the USARTx_ICR register. An interrupt is generated if RXNEIE/ RXFNEIE=1 or EIE = 1 in the USARTx_CR1 register. -0: No overrun error -1: Overrun error is detected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORE {
    ///0: No Overrun error
    NoOverrun = 0,
    ///1: Overrun error is detected
    Overrun = 1,
}
impl From<ORE> for bool {
    #[inline(always)]
    fn from(variant: ORE) -> Self {
        variant as u8 != 0
    }
}
///Field `ORE` reader - ORE: Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USARTx_RDR register while RXNE=1 (RXFF = 1 in case FIFO mode is enabled) . It is cleared by a software, writing 1 to the ORECF, in the USARTx_ICR register. An interrupt is generated if RXNEIE/ RXFNEIE=1 or EIE = 1 in the USARTx_CR1 register. -0: No overrun error -1: Overrun error is detected
pub type ORE_R = crate::BitReader<ORE>;
impl ORE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORE {
        match self.bits {
            false => ORE::NoOverrun,
            true => ORE::Overrun,
        }
    }
    ///No Overrun error
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == ORE::NoOverrun
    }
    ///Overrun error is detected
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == ORE::Overrun
    }
}
/**IDLE: Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. -0: No Idle line is detected -1: Idle line is detected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE {
    ///0: No Idle Line is detected
    NoIdle = 0,
    ///1: Idle Line is detected
    Idle = 1,
}
impl From<IDLE> for bool {
    #[inline(always)]
    fn from(variant: IDLE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLE` reader - IDLE: Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. -0: No Idle line is detected -1: Idle line is detected
pub type IDLE_R = crate::BitReader<IDLE>;
impl IDLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLE {
        match self.bits {
            false => IDLE::NoIdle,
            true => IDLE::Idle,
        }
    }
    ///No Idle Line is detected
    #[inline(always)]
    pub fn is_no_idle(&self) -> bool {
        *self == IDLE::NoIdle
    }
    ///Idle Line is detected
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == IDLE::Idle
    }
}
/**RXNE/RXFNE:Read data register not empty/RXFIFO not empty RXNE bit is set by hardware when the content of the USARTx_RDR shift register has been transferred to the USARTx_RDR register. It is cleared by a read to the USARTx_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USARTx_RQR register. RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the USART_RDR register. Every read of the USART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXNE/RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE/RXFNEIE=1 in the USART_CR1 register. -0: Data is not received -1: Received data is ready to be read.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    ///0: Data is not received
    NoData = 0,
    ///1: Received data is ready to be read
    DataReady = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - RXNE/RXFNE:Read data register not empty/RXFIFO not empty RXNE bit is set by hardware when the content of the USARTx_RDR shift register has been transferred to the USARTx_RDR register. It is cleared by a read to the USARTx_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USARTx_RQR register. RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the USART_RDR register. Every read of the USART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXNE/RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE/RXFNEIE=1 in the USART_CR1 register. -0: Data is not received -1: Received data is ready to be read.
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::NoData,
            true => RXNE::DataReady,
        }
    }
    ///Data is not received
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == RXNE::NoData
    }
    ///Received data is ready to be read
    #[inline(always)]
    pub fn is_data_ready(&self) -> bool {
        *self == RXNE::DataReady
    }
}
/**TC: Transmission complete This bit indicates when the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware if the transmission of a frame containing data is complete and if TXE/TXFE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. An interrupt is generated if TCIE=1 in the USART_CR1 register. -0: Transmission is not complete -1: Transmission is complete

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC {
    ///0: Transmission is not complete
    TxNotComplete = 0,
    ///1: Transmission is complete
    TxComplete = 1,
}
impl From<TC> for bool {
    #[inline(always)]
    fn from(variant: TC) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - TC: Transmission complete This bit indicates when the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware if the transmission of a frame containing data is complete and if TXE/TXFE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. An interrupt is generated if TCIE=1 in the USART_CR1 register. -0: Transmission is not complete -1: Transmission is complete
pub type TC_R = crate::BitReader<TC>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TC {
        match self.bits {
            false => TC::TxNotComplete,
            true => TC::TxComplete,
        }
    }
    ///Transmission is not complete
    #[inline(always)]
    pub fn is_tx_not_complete(&self) -> bool {
        *self == TC::TxNotComplete
    }
    ///Transmission is complete
    #[inline(always)]
    pub fn is_tx_complete(&self) -> bool {
        *self == TC::TxComplete
    }
}
/**TXE/TXFNF: Transmit data register empty/TXFIFO not full When FIFO mode is disabled, TXE is set by hardware when the content of the USARTx_TDR register has been transferred into the shift register. It is cleared by a write to the USARTx_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T=0 mode, in case of transmission failure). When FIFO mode is enabled, TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the USART_TDR. Every write in the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty . After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO. (TXFNF and TXFE will be set at the same time). An interrupt is generated if the TXEIE/TXFNFIE bit =1 in the USART_CR1 register. -0: Data register is full/Transmit FIFO is full. -1: Data register/Transmit FIFO is not full

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE {
    ///0: Transmit FIFO is full
    Full = 0,
    ///1: Transmit FIFO is not full
    NotFull = 1,
}
impl From<TXE> for bool {
    #[inline(always)]
    fn from(variant: TXE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - TXE/TXFNF: Transmit data register empty/TXFIFO not full When FIFO mode is disabled, TXE is set by hardware when the content of the USARTx_TDR register has been transferred into the shift register. It is cleared by a write to the USARTx_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T=0 mode, in case of transmission failure). When FIFO mode is enabled, TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the USART_TDR. Every write in the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty . After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO. (TXFNF and TXFE will be set at the same time). An interrupt is generated if the TXEIE/TXFNFIE bit =1 in the USART_CR1 register. -0: Data register is full/Transmit FIFO is full. -1: Data register/Transmit FIFO is not full
pub type TXE_R = crate::BitReader<TXE>;
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXE {
        match self.bits {
            false => TXE::Full,
            true => TXE::NotFull,
        }
    }
    ///Transmit FIFO is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXE::Full
    }
    ///Transmit FIFO is not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXE::NotFull
    }
}
/**LBDF: LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. -0: LIN Break not detected -1: LIN break detected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDF {
    ///0: LIN break not detected
    NotDetected = 0,
    ///1: LIN break detected
    Detected = 1,
}
impl From<LBDF> for bool {
    #[inline(always)]
    fn from(variant: LBDF) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDF` reader - LBDF: LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. -0: LIN Break not detected -1: LIN break detected
pub type LBDF_R = crate::BitReader<LBDF>;
impl LBDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDF {
        match self.bits {
            false => LBDF::NotDetected,
            true => LBDF::Detected,
        }
    }
    ///LIN break not detected
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LBDF::NotDetected
    }
    ///LIN break detected
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LBDF::Detected
    }
}
/**CTSIF: CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. -0: No change occurred on the nCTS status line -1: A change occurred on the nCTS status line

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIF {
    ///0: No change occurred on the CTS status line
    NotChanged = 0,
    ///1: A change occurred on the CTS status line
    Changed = 1,
}
impl From<CTSIF> for bool {
    #[inline(always)]
    fn from(variant: CTSIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIF` reader - CTSIF: CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. -0: No change occurred on the nCTS status line -1: A change occurred on the nCTS status line
pub type CTSIF_R = crate::BitReader<CTSIF>;
impl CTSIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSIF {
        match self.bits {
            false => CTSIF::NotChanged,
            true => CTSIF::Changed,
        }
    }
    ///No change occurred on the CTS status line
    #[inline(always)]
    pub fn is_not_changed(&self) -> bool {
        *self == CTSIF::NotChanged
    }
    ///A change occurred on the CTS status line
    #[inline(always)]
    pub fn is_changed(&self) -> bool {
        *self == CTSIF::Changed
    }
}
/**CTS: CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. -0: nCTS line set -1: nCTS line reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTS {
    ///0: CTS line set
    Set = 0,
    ///1: CTS line reset
    Reset = 1,
}
impl From<CTS> for bool {
    #[inline(always)]
    fn from(variant: CTS) -> Self {
        variant as u8 != 0
    }
}
///Field `CTS` reader - CTS: CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. -0: nCTS line set -1: nCTS line reset
pub type CTS_R = crate::BitReader<CTS>;
impl CTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTS {
        match self.bits {
            false => CTS::Set,
            true => CTS::Reset,
        }
    }
    ///CTS line set
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CTS::Set
    }
    ///CTS line reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CTS::Reset
    }
}
/**RTOF: Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. -0: Timeout value not reached -1: Timeout value reached without any data reception

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOF {
    ///0: Timeout value not reached
    NotReached = 0,
    ///1: Timeout value reached without any data reception
    Reached = 1,
}
impl From<RTOF> for bool {
    #[inline(always)]
    fn from(variant: RTOF) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOF` reader - RTOF: Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. -0: Timeout value not reached -1: Timeout value reached without any data reception
pub type RTOF_R = crate::BitReader<RTOF>;
impl RTOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOF {
        match self.bits {
            false => RTOF::NotReached,
            true => RTOF::Reached,
        }
    }
    ///Timeout value not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == RTOF::NotReached
    }
    ///Timeout value reached without any data reception
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == RTOF::Reached
    }
}
/**EOBF: End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. -0: End of Block not reached -1: End of Block (number of characters) reached

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBF {
    ///0: End of Block not reached
    NotReached = 0,
    ///1: End of Block (number of characters) reached
    Reached = 1,
}
impl From<EOBF> for bool {
    #[inline(always)]
    fn from(variant: EOBF) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBF` reader - EOBF: End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. -0: End of Block not reached -1: End of Block (number of characters) reached
pub type EOBF_R = crate::BitReader<EOBF>;
impl EOBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOBF {
        match self.bits {
            false => EOBF::NotReached,
            true => EOBF::Reached,
        }
    }
    ///End of Block not reached
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == EOBF::NotReached
    }
    ///End of Block (number of characters) reached
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == EOBF::Reached
    }
}
/**UDR: SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock for data transmission appears while the software has not yet loaded any value into USARTx_DR. -0: No underrun error -1: underrun error

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR {
    ///0: No underrun error
    NoUnderrun = 0,
    ///1: underrun error
    Underrun = 1,
}
impl From<UDR> for bool {
    #[inline(always)]
    fn from(variant: UDR) -> Self {
        variant as u8 != 0
    }
}
///Field `UDR` reader - UDR: SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock for data transmission appears while the software has not yet loaded any value into USARTx_DR. -0: No underrun error -1: underrun error
pub type UDR_R = crate::BitReader<UDR>;
impl UDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDR {
        match self.bits {
            false => UDR::NoUnderrun,
            true => UDR::Underrun,
        }
    }
    ///No underrun error
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDR::NoUnderrun
    }
    ///underrun error
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDR::Underrun
    }
}
///Field `ABRE` reader - ABRE: Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_CR3 register
pub type ABRE_R = crate::BitReader;
///Field `ABRF` reader - ABRF: Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXNE will also be set, generating an interrupt if RXNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register.
pub type ABRF_R = crate::BitReader;
/**BUSY: Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not). -0: USART is idle (no reception) -1: Reception on going

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: USART is idle (no reception)
    Idle = 0,
    ///1: Reception on going
    Busy = 1,
}
impl From<BUSY> for bool {
    #[inline(always)]
    fn from(variant: BUSY) -> Self {
        variant as u8 != 0
    }
}
///Field `BUSY` reader - BUSY: Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not). -0: USART is idle (no reception) -1: Reception on going
pub type BUSY_R = crate::BitReader<BUSY>;
impl BUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUSY {
        match self.bits {
            false => BUSY::Idle,
            true => BUSY::Busy,
        }
    }
    ///USART is idle (no reception)
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY::Idle
    }
    ///Reception on going
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY::Busy
    }
}
/**CMF: Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register. -0: No Character match detected -1: Character Match detected

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMF {
    ///0: No Character match detected
    NoMatch = 0,
    ///1: Character match detected
    Match = 1,
}
impl From<CMF> for bool {
    #[inline(always)]
    fn from(variant: CMF) -> Self {
        variant as u8 != 0
    }
}
///Field `CMF` reader - CMF: Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register. -0: No Character match detected -1: Character Match detected
pub type CMF_R = crate::BitReader<CMF>;
impl CMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMF {
        match self.bits {
            false => CMF::NoMatch,
            true => CMF::Match,
        }
    }
    ///No Character match detected
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CMF::NoMatch
    }
    ///Character match detected
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CMF::Match
    }
}
/**SBKF: Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission. -0: No break character is transmitted -1: Break character will be transmitted

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKF {
    ///0: No break character transmitted
    NoBreak = 0,
    ///1: Break character transmitted
    Break = 1,
}
impl From<SBKF> for bool {
    #[inline(always)]
    fn from(variant: SBKF) -> Self {
        variant as u8 != 0
    }
}
///Field `SBKF` reader - SBKF: Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission. -0: No break character is transmitted -1: Break character will be transmitted
pub type SBKF_R = crate::BitReader<SBKF>;
impl SBKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBKF {
        match self.bits {
            false => SBKF::NoBreak,
            true => SBKF::Break,
        }
    }
    ///No break character transmitted
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == SBKF::NoBreak
    }
    ///Break character transmitted
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBKF::Break
    }
}
///Field `RWU` reader - RWU: Receiver wakeup from Mute mode This bit indicates if the USART is in mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. -0: Receiver in active mode -1: Receiver in mute mode
pub type RWU_R = crate::BitReader;
///Field `TEACK` reader - TEACK: Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period.
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - REACK: Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering Stop mode.
pub type REACK_R = crate::BitReader;
/**TXFE: TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register. -0: TXFIFO is not empty. -1: TXFIFO is empty.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFE {
    ///0: TXFIFO not empty.
    NotEmpty = 0,
    ///1: TXFIFO empty.
    Empty = 1,
}
impl From<TXFE> for bool {
    #[inline(always)]
    fn from(variant: TXFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFE` reader - TXFE: TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register. -0: TXFIFO is not empty. -1: TXFIFO is empty.
pub type TXFE_R = crate::BitReader<TXFE>;
impl TXFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFE {
        match self.bits {
            false => TXFE::NotEmpty,
            true => TXFE::Empty,
        }
    }
    ///TXFIFO not empty.
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXFE::NotEmpty
    }
    ///TXFIFO empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXFE::Empty
    }
}
/**RXFF: RXFIFO Full This bit is set by hardware when RXFIFO is Full. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register. -0: RXFIFO is not Full. -1: RXFIFO is Full.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFF {
    ///0: RXFIFO not full.
    NotFull = 0,
    ///1: RXFIFO Full.
    Full = 1,
}
impl From<RXFF> for bool {
    #[inline(always)]
    fn from(variant: RXFF) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFF` reader - RXFF: RXFIFO Full This bit is set by hardware when RXFIFO is Full. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register. -0: RXFIFO is not Full. -1: RXFIFO is Full.
pub type RXFF_R = crate::BitReader<RXFF>;
impl RXFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFF {
        match self.bits {
            false => RXFF::NotFull,
            true => RXFF::Full,
        }
    }
    ///RXFIFO not full.
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RXFF::NotFull
    }
    ///RXFIFO Full.
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXFF::Full
    }
}
/**TCBGT: Transmission complete before guard time flagl This bit indicates when the last data written in the USART_TDR has been transmitted correctly out of the shift register . It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if there is no NACK from the smartcard. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. It is cleared by software, writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. -0: Transmission is not complete or transmission is complete unsuccessfuly (i.e. a NACK is received from the card) -1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGT {
    ///0: Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)
    NotCompleted = 0,
    ///1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)
    Completed = 1,
}
impl From<TCBGT> for bool {
    #[inline(always)]
    fn from(variant: TCBGT) -> Self {
        variant as u8 != 0
    }
}
///Field `TCBGT` reader - TCBGT: Transmission complete before guard time flagl This bit indicates when the last data written in the USART_TDR has been transmitted correctly out of the shift register . It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if there is no NACK from the smartcard. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. It is cleared by software, writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. -0: Transmission is not complete or transmission is complete unsuccessfuly (i.e. a NACK is received from the card) -1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card).
pub type TCBGT_R = crate::BitReader<TCBGT>;
impl TCBGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCBGT {
        match self.bits {
            false => TCBGT::NotCompleted,
            true => TCBGT::Completed,
        }
    }
    ///Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == TCBGT::NotCompleted
    }
    ///Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card)
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == TCBGT::Completed
    }
}
/**RXFT: RXFIFO threshold flag This bit is set by hardware when the programmed threshold in RXFTCFG in USARTx_CR3 register is reached. This means that there are (RXFTCFG 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. -0: Receive FIFO doesn't reach the programmed threshold. -1: Receive FIFO reached the programmed threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFT {
    ///0: Receive FIFO does not reach the programmed threshold.
    NotReached = 0,
    ///1: Receive FIFO reached the programmed threshold.
    Reached = 1,
}
impl From<RXFT> for bool {
    #[inline(always)]
    fn from(variant: RXFT) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFT` reader - RXFT: RXFIFO threshold flag This bit is set by hardware when the programmed threshold in RXFTCFG in USARTx_CR3 register is reached. This means that there are (RXFTCFG 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. -0: Receive FIFO doesn't reach the programmed threshold. -1: Receive FIFO reached the programmed threshold
pub type RXFT_R = crate::BitReader<RXFT>;
impl RXFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFT {
        match self.bits {
            false => RXFT::NotReached,
            true => RXFT::Reached,
        }
    }
    ///Receive FIFO does not reach the programmed threshold.
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == RXFT::NotReached
    }
    ///Receive FIFO reached the programmed threshold.
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == RXFT::Reached
    }
}
/**TXFT: TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the programmed threshold in TXFTCFG in USARTx_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register. -0: TXFIFO doesn't reach the programmed threshold. -1: TXFIFO reached the programmed threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFT {
    ///0: TXFIFO does not reach the programmed threshold.
    NotReached = 0,
    ///1: TXFIFO reached the programmed threshold.
    Reached = 1,
}
impl From<TXFT> for bool {
    #[inline(always)]
    fn from(variant: TXFT) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFT` reader - TXFT: TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the programmed threshold in TXFTCFG in USARTx_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register. -0: TXFIFO doesn't reach the programmed threshold. -1: TXFIFO reached the programmed threshold
pub type TXFT_R = crate::BitReader<TXFT>;
impl TXFT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFT {
        match self.bits {
            false => TXFT::NotReached,
            true => TXFT::Reached,
        }
    }
    ///TXFIFO does not reach the programmed threshold.
    #[inline(always)]
    pub fn is_not_reached(&self) -> bool {
        *self == TXFT::NotReached
    }
    ///TXFIFO reached the programmed threshold.
    #[inline(always)]
    pub fn is_reached(&self) -> bool {
        *self == TXFT::Reached
    }
}
impl R {
    ///Bit 0 - PE: Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. -0: No parity error -1: Parity error
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FE: Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. In Smartcard mode, in transmission, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. -0: No Framing error is detected -1: Framing error or break character is detected
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NF: START bit Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. -0: No noise is detected -1: Noise is detected
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ORE: Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USARTx_RDR register while RXNE=1 (RXFF = 1 in case FIFO mode is enabled) . It is cleared by a software, writing 1 to the ORECF, in the USARTx_ICR register. An interrupt is generated if RXNEIE/ RXFNEIE=1 or EIE = 1 in the USARTx_CR1 register. -0: No overrun error -1: Overrun error is detected
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE: Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. -0: No Idle line is detected -1: Idle line is detected
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNE/RXFNE:Read data register not empty/RXFIFO not empty RXNE bit is set by hardware when the content of the USARTx_RDR shift register has been transferred to the USARTx_RDR register. It is cleared by a read to the USARTx_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USARTx_RQR register. RXFNE bit is set by hardware when the RXFIFO is not empty, and so data can be read from the USART_RDR register. Every read of the USART_RDR frees a location in the RXFIFO. It is cleared when the RXFIFO is empty. The RXNE/RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE/RXFNEIE=1 in the USART_CR1 register. -0: Data is not received -1: Received data is ready to be read.
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TC: Transmission complete This bit indicates when the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware if the transmission of a frame containing data is complete and if TXE/TXFE is set. An interrupt is generated if TCIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. An interrupt is generated if TCIE=1 in the USART_CR1 register. -0: Transmission is not complete -1: Transmission is complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXE/TXFNF: Transmit data register empty/TXFIFO not full When FIFO mode is disabled, TXE is set by hardware when the content of the USARTx_TDR register has been transferred into the shift register. It is cleared by a write to the USARTx_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T=0 mode, in case of transmission failure). When FIFO mode is enabled, TXFNF is set by hardware when TXFIFO is not full, and so data can be written in the USART_TDR. Every write in the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty . After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO. (TXFNF and TXFE will be set at the same time). An interrupt is generated if the TXEIE/TXFNFIE bit =1 in the USART_CR1 register. -0: Data register is full/Transmit FIFO is full. -1: Data register/Transmit FIFO is not full
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LBDF: LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. -0: LIN Break not detected -1: LIN break detected
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTSIF: CTS interrupt flag This bit is set by hardware when the nCTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. -0: No change occurred on the nCTS status line -1: A change occurred on the nCTS status line
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS: CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the nCTS input pin. -0: nCTS line set -1: nCTS line reset
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RTOF: Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. -0: Timeout value not reached -1: Timeout value reached without any data reception
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - EOBF: End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE=1 in the USART_CR2 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. -0: End of Block not reached -1: End of Block (number of characters) reached
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - UDR: SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock for data transmission appears while the software has not yet loaded any value into USARTx_DR. -0: No underrun error -1: underrun error
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ABRE: Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_CR3 register
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ABRF: Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXNE will also be set, generating an interrupt if RXNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register.
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - BUSY: Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not). -0: USART is idle (no reception) -1: Reception on going
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMF: Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register. -0: No Character match detected -1: Character Match detected
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SBKF: Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission. -0: No break character is transmitted -1: Break character will be transmitted
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RWU: Receiver wakeup from Mute mode This bit indicates if the USART is in mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. -0: Receiver in active mode -1: Receiver in mute mode
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - TEACK: Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period.
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - REACK: Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering Stop mode.
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFE: TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register. -0: TXFIFO is not empty. -1: TXFIFO is empty.
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - RXFF: RXFIFO Full This bit is set by hardware when RXFIFO is Full. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register. -0: RXFIFO is not Full. -1: RXFIFO is Full.
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - TCBGT: Transmission complete before guard time flagl This bit indicates when the last data written in the USART_TDR has been transmitted correctly out of the shift register . It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if there is no NACK from the smartcard. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. It is cleared by software, writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. -0: Transmission is not complete or transmission is complete unsuccessfuly (i.e. a NACK is received from the card) -1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card).
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - RXFT: RXFIFO threshold flag This bit is set by hardware when the programmed threshold in RXFTCFG in USARTx_CR3 register is reached. This means that there are (RXFTCFG 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. -0: Receive FIFO doesn't reach the programmed threshold. -1: Receive FIFO reached the programmed threshold
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFT: TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the programmed threshold in TXFTCFG in USARTx_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register. -0: TXFIFO doesn't reach the programmed threshold. -1: TXFIFO reached the programmed threshold
    #[inline(always)]
    pub fn txft(&self) -> TXFT_R {
        TXFT_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("nf", &self.nf())
            .field("ore", &self.ore())
            .field("idle", &self.idle())
            .field("rxne", &self.rxne())
            .field("tc", &self.tc())
            .field("txe", &self.txe())
            .field("lbdf", &self.lbdf())
            .field("ctsif", &self.ctsif())
            .field("cts", &self.cts())
            .field("rtof", &self.rtof())
            .field("eobf", &self.eobf())
            .field("udr", &self.udr())
            .field("abre", &self.abre())
            .field("abrf", &self.abrf())
            .field("busy", &self.busy())
            .field("cmf", &self.cmf())
            .field("sbkf", &self.sbkf())
            .field("rwu", &self.rwu())
            .field("teack", &self.teack())
            .field("reack", &self.reack())
            .field("txfe", &self.txfe())
            .field("rxff", &self.rxff())
            .field("tcbgt", &self.tcbgt())
            .field("rxft", &self.rxft())
            .field("txft", &self.txft())
            .finish()
    }
}
/**ISR register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#USART:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0xc0
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0xc0;
}
