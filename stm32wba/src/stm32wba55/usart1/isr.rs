///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.

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
///Field `PE` reader - Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.
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
/**Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.

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
///Field `FE` reader - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.
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
/**Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Section 78.5.9: Tolerance of the USART receiver to clock deviation on page 4584). Note: This error is associated with the character in the USART_RDR.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NE {
    ///0: No noise is detected
    NoNoise = 0,
    ///1: Noise is detected
    Noise = 1,
}
impl From<NE> for bool {
    #[inline(always)]
    fn from(variant: NE) -> Self {
        variant as u8 != 0
    }
}
///Field `NE` reader - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Section 78.5.9: Tolerance of the USART receiver to clock deviation on page 4584). Note: This error is associated with the character in the USART_RDR.
pub type NE_R = crate::BitReader<NE>;
impl NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NE {
        match self.bits {
            false => NE::NoNoise,
            true => NE::Noise,
        }
    }
    ///No noise is detected
    #[inline(always)]
    pub fn is_no_noise(&self) -> bool {
        *self == NE::NoNoise
    }
    ///Noise is detected
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == NE::Noise
    }
}
/**Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.

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
///Field `ORE` reader - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.
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
/**Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set.

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
///Field `IDLE` reader - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set.
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
/**RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFNE {
    ///0: Data is not received
    NoData = 0,
    ///1: Received data is ready to be read
    DataReady = 1,
}
impl From<RXFNE> for bool {
    #[inline(always)]
    fn from(variant: RXFNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFNE` reader - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register.
pub type RXFNE_R = crate::BitReader<RXFNE>;
impl RXFNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFNE {
        match self.bits {
            false => RXFNE::NoData,
            true => RXFNE::DataReady,
        }
    }
    ///Data is not received
    #[inline(always)]
    pub fn is_no_data(&self) -> bool {
        *self == RXFNE::NoData
    }
    ///Received data is ready to be read
    #[inline(always)]
    pub fn is_data_ready(&self) -> bool {
        *self == RXFNE::DataReady
    }
}
/**Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. The TC flag behaves as follows: When TDN = 0, the TC flag is set when the transmission of a frame containing data is complete and when TXE/TXFE is set. When TDN is equal to the number of data in the TXFIFO, the TC flag is set when TXFIFO is empty and TDN is reached. When TDN is greater than the number of data in the TXFIFO, TC remains cleared until the TXFIFO is filled again to reach the programmed number of data to be transferred. When TDN is less than the number of data in the TXFIFO, TC is set when TDN is reached even if the TXFIFO is not empty. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is cleared by software by writing 1 to the TCCF in the USART_ICR register or by writing to the USART_TDR register.

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
///Field `TC` reader - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. The TC flag behaves as follows: When TDN = 0, the TC flag is set when the transmission of a frame containing data is complete and when TXE/TXFE is set. When TDN is equal to the number of data in the TXFIFO, the TC flag is set when TXFIFO is empty and TDN is reached. When TDN is greater than the number of data in the TXFIFO, TC remains cleared until the TXFIFO is filled again to reach the programmed number of data to be transferred. When TDN is less than the number of data in the TXFIFO, TC is set when TDN is reached even if the TXFIFO is not empty. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is cleared by software by writing 1 to the TCCF in the USART_ICR register or by writing to the USART_TDR register.
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
/**TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE is set at the same time). Note: This bit is used during single buffer transmission.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFNF {
    ///0: Transmit FIFO is full
    Full = 0,
    ///1: Transmit FIFO is not full
    NotFull = 1,
}
impl From<TXFNF> for bool {
    #[inline(always)]
    fn from(variant: TXFNF) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFNF` reader - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE is set at the same time). Note: This bit is used during single buffer transmission.
pub type TXFNF_R = crate::BitReader<TXFNF>;
impl TXFNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFNF {
        match self.bits {
            false => TXFNF::Full,
            true => TXFNF::NotFull,
        }
    }
    ///Transmit FIFO is full
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TXFNF::Full
    }
    ///Transmit FIFO is not full
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TXFNF::NotFull
    }
}
/**LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.

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
///Field `LBDF` reader - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
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
/**CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.

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
///Field `CTSIF` reader - CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
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
/**CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.

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
///Field `CTS` reader - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
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
/**Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. Note: The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.

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
///Field `RTOF` reader - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. Note: The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.
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
/**End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.

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
///Field `EOBF` reader - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
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
/**SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.

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
///Field `UDR` reader - SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
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
///Field `ABRE` reader - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
pub type ABRE_R = crate::BitReader;
///Field `ABRF` reader - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set, generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXFNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
pub type ABRF_R = crate::BitReader;
/**Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).

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
///Field `BUSY` reader - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).
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
/**Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register.

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
///Field `CMF` reader - Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register.
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
/**Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.

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
///Field `SBKF` reader - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.
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
///Field `RWU` reader - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
pub type RWU_R = crate::BitReader;
///Field `TEACK` reader - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period.
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
pub type REACK_R = crate::BitReader;
/**TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register.

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
///Field `TXFE` reader - TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register.
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
/**RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO size + 1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register.

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
///Field `RXFF` reader - RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO size + 1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register.
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
/**Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is 1. Refer to Section 78.4: USART implementation on page 4565.

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
///Field `TCBGT` reader - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is 1. Refer to Section 78.4: USART implementation on page 4565.
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
/**RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to 101, RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data.

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
///Field `RXFT` reader - RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to 101, RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data.
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
/**TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register.

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
///Field `TXFT` reader - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register.
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
    ///Bit 0 - Parity error This bit is set by hardware when a parity error occurs in Reception mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR1 register. Note: This error is associated with the character in the USART_RDR.
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NFCF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXFNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Section 78.5.9: Tolerance of the USART receiver to clock deviation on page 4584). Note: This error is associated with the character in the USART_RDR.
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXFF = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXFNEIE=1 or EIE = 1 in the USART_CR1 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE=1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXFNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME=1), IDLE is set if the USART is not mute (RWU=0), whatever the Mute mode selected by the WAKE bit. If RWU=1, IDLE is not set.
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFIFO not empty RXFNE bit is set by hardware when the RXFIFO is not empty, meaning that data can be read from the USART_RDR register. Every read operation from the USART_RDR frees a location in the RXFIFO. RXFNE is cleared when the RXFIFO is empty. The RXFNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXFNEIE=1 in the USART_CR1 register.
    #[inline(always)]
    pub fn rxfne(&self) -> RXFNE_R {
        RXFNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. The TC flag behaves as follows: When TDN = 0, the TC flag is set when the transmission of a frame containing data is complete and when TXE/TXFE is set. When TDN is equal to the number of data in the TXFIFO, the TC flag is set when TXFIFO is empty and TDN is reached. When TDN is greater than the number of data in the TXFIFO, TC remains cleared until the TXFIFO is filled again to reach the programmed number of data to be transferred. When TDN is less than the number of data in the TXFIFO, TC is set when TDN is reached even if the TXFIFO is not empty. An interrupt is generated if TCIE=1 in the USART_CR1 register. TC bit is cleared by software by writing 1 to the TCCF in the USART_ICR register or by writing to the USART_TDR register.
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFIFO not full TXFNF is set by hardware when TXFIFO is not full meaning that data can be written in the USART_TDR. Every write operation to the USART_TDR places the data in the TXFIFO. This flag remains set until the TXFIFO is full. When the TXFIFO is full, this flag is cleared indicating that data can not be written into the USART_TDR. An interrupt is generated if the TXFNFIE bit =1 in the USART_CR1 register. Note: The TXFNF is kept reset during the flush request until TXFIFO is empty. After sending the flush request (by setting TXFRQ bit), the flag TXFNF should be checked prior to writing in TXFIFO (TXFNF and TXFE is set at the same time). Note: This bit is used during single buffer transmission.
    #[inline(always)]
    pub fn txfnf(&self) -> TXFNF_R {
        TXFNF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE=1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE=1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. Note: The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - End of block flag This bit is set by hardware when a complete block has been received (for example T=1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI slave underrun error flag In Slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXFNE is also set, generating an interrupt if RXFNEIE = 1) or when the auto baud rate operation was completed without success (ABRE=1) (ABRE, RXFNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE=1in the USART_CR1 register.
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Receiver wakeup from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wakeup/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wakeup on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE=0, followed by TE=1 in the USART_CR1 register, in order to respect the TE=0 minimum period.
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wakeup from Stop feature, this bit is reserved and kept at reset value. Refer to Section 78.4: USART implementation on page 4565.
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFIFO Empty This bit is set by hardware when TXFIFO is Empty. When the TXFIFO contains at least one data, this flag is cleared. The TXFE flag can also be set by writing 1 to the bit TXFRQ (bit 4) in the USART_RQR register. An interrupt is generated if the TXFEIE bit =1 (bit 30) in the USART_CR1 register.
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - RXFIFO Full This bit is set by hardware when the number of received data corresponds to RXFIFO size + 1 (RXFIFO full + 1 data in the USART_RDR register. An interrupt is generated if the RXFFIE bit =1 in the USART_CR1 register.
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Transmission complete before guard time flag This bit is set when the last data written in the USART_TDR has been transmitted correctly out of the shift register. It is set by hardware in Smartcard mode, if the transmission of a frame containing data is complete and if the smartcard did not send back any NACK. An interrupt is generated if TCBGTIE=1 in the USART_CR3 register. This bit is cleared by software, by writing 1 to the TCBGTCF in the USART_ICR register or by a write to the USART_TDR register. Note: If the USART does not support the Smartcard mode, this bit is reserved and kept at reset value. If the USART supports the Smartcard mode and the Smartcard mode is enabled, the TCBGT reset value is 1. Refer to Section 78.4: USART implementation on page 4565.
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - RXFIFO threshold flag This bit is set by hardware when the threshold programmed in RXFTCFG in USART_CR3 register is reached. This means that there are (RXFTCFG - 1) data in the Receive FIFO and one data in the USART_RDR register. An interrupt is generated if the RXFTIE bit =1 (bit 27) in the USART_CR3 register. Note: When the RXFTCFG threshold is configured to 101, RXFT flag is set if 16 data are available i.e. 15 data in the RXFIFO and 1 data in the USART_RDR. Consequently, the 17th received data does not cause an overrun error. The overrun error occurs after receiving the 18th data.
    #[inline(always)]
    pub fn rxft(&self) -> RXFT_R {
        RXFT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXFIFO threshold flag This bit is set by hardware when the TXFIFO reaches the threshold programmed in TXFTCFG of USART_CR3 register i.e. the TXFIFO contains TXFTCFG empty locations. An interrupt is generated if the TXFTIE bit =1 (bit 31) in the USART_CR3 register.
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
            .field("ne", &self.ne())
            .field("ore", &self.ore())
            .field("idle", &self.idle())
            .field("rxfne", &self.rxfne())
            .field("tc", &self.tc())
            .field("txfnf", &self.txfnf())
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
/**USART interrupt and status register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#USART1:ISR)*/
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
