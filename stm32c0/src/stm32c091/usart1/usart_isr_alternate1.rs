///Register `USART_ISR_ALTERNATE1` reader
pub type R = crate::R<USART_ISR_ALTERNATE1rs>;
/**Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    ///0: No parity error
    B0x0 = 0,
    ///1: Parity error
    B0x1 = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register.
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::B0x0,
            true => PE::B0x1,
        }
    }
    ///No parity error
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PE::B0x0
    }
    ///Parity error
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PE::B0x1
    }
}
/**Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR3 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FE {
    ///0: No Framing error is detected
    B0x0 = 0,
    ///1: Framing error or break character is detected
    B0x1 = 1,
}
impl From<FE> for bool {
    #[inline(always)]
    fn from(variant: FE) -> Self {
        variant as u8 != 0
    }
}
///Field `FE` reader - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR3 register.
pub type FE_R = crate::BitReader<FE>;
impl FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FE {
        match self.bits {
            false => FE::B0x0,
            true => FE::B0x1,
        }
    }
    ///No Framing error is detected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FE::B0x0
    }
    ///Framing error or break character is detected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FE::B0x1
    }
}
/**Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Section 26.5.8: Tolerance of the USART receiver to clock deviation on page 709).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NE {
    ///0: No noise is detected
    B0x0 = 0,
    ///1: Noise is detected
    B0x1 = 1,
}
impl From<NE> for bool {
    #[inline(always)]
    fn from(variant: NE) -> Self {
        variant as u8 != 0
    }
}
///Field `NE` reader - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Section 26.5.8: Tolerance of the USART receiver to clock deviation on page 709).
pub type NE_R = crate::BitReader<NE>;
impl NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NE {
        match self.bits {
            false => NE::B0x0,
            true => NE::B0x1,
        }
    }
    ///No noise is detected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NE::B0x0
    }
    ///Noise is detected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NE::B0x1
    }
}
/**Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXNE = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXNEIE = 1 or EIE = 1 in the LPUART_CR1 register, or EIE = 1 in the USART_CR3 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORE {
    ///0: No overrun error
    B0x0 = 0,
    ///1: Overrun error is detected
    B0x1 = 1,
}
impl From<ORE> for bool {
    #[inline(always)]
    fn from(variant: ORE) -> Self {
        variant as u8 != 0
    }
}
///Field `ORE` reader - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXNE = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXNEIE = 1 or EIE = 1 in the LPUART_CR1 register, or EIE = 1 in the USART_CR3 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.
pub type ORE_R = crate::BitReader<ORE>;
impl ORE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ORE {
        match self.bits {
            false => ORE::B0x0,
            true => ORE::B0x1,
        }
    }
    ///No overrun error
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ORE::B0x0
    }
    ///Overrun error is detected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ORE::B0x1
    }
}
/**Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME = 1), IDLE is set if the USART is not mute (RWU = 0), whatever the Mute mode selected by the WAKE bit. If RWU = 1, IDLE is not set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE {
    ///0: No Idle line is detected
    B0x0 = 0,
    ///1: Idle line is detected
    B0x1 = 1,
}
impl From<IDLE> for bool {
    #[inline(always)]
    fn from(variant: IDLE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLE` reader - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME = 1), IDLE is set if the USART is not mute (RWU = 0), whatever the Mute mode selected by the WAKE bit. If RWU = 1, IDLE is not set.
pub type IDLE_R = crate::BitReader<IDLE>;
impl IDLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLE {
        match self.bits {
            false => IDLE::B0x0,
            true => IDLE::B0x1,
        }
    }
    ///No Idle line is detected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IDLE::B0x0
    }
    ///Idle line is detected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IDLE::B0x1
    }
}
/**Read data register not empty RXNE bit is set by hardware when the content of the USART_RDR shift register has been transferred to the USART_RDR register. It is cleared by reading from the USART_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE = 1 in the USART_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE {
    ///0: Data is not received
    B0x0 = 0,
    ///1: Received data is ready to be read.
    B0x1 = 1,
}
impl From<RXNE> for bool {
    #[inline(always)]
    fn from(variant: RXNE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNE` reader - Read data register not empty RXNE bit is set by hardware when the content of the USART_RDR shift register has been transferred to the USART_RDR register. It is cleared by reading from the USART_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE = 1 in the USART_CR1 register.
pub type RXNE_R = crate::BitReader<RXNE>;
impl RXNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNE {
        match self.bits {
            false => RXNE::B0x0,
            true => RXNE::B0x1,
        }
    }
    ///Data is not received
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXNE::B0x0
    }
    ///Received data is ready to be read.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXNE::B0x1
    }
}
/**Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXE is set. An interrupt is generated if TCIE = 1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC {
    ///0: Transmission is not complete
    B0x0 = 0,
    ///1: Transmission is complete
    B0x1 = 1,
}
impl From<TC> for bool {
    #[inline(always)]
    fn from(variant: TC) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXE is set. An interrupt is generated if TCIE = 1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately.
pub type TC_R = crate::BitReader<TC>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TC {
        match self.bits {
            false => TC::B0x0,
            true => TC::B0x1,
        }
    }
    ///Transmission is not complete
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TC::B0x0
    }
    ///Transmission is complete
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TC::B0x1
    }
}
/**Transmit data register empty TXE is set by hardware when the content of the USART_TDR register has been transferred into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T = 0 mode, in case of transmission failure). An interrupt is generated if the TXEIE bit = 1 in the USART_CR1 register.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE {
    ///0: Data register full
    B0x0 = 0,
    ///1: Data register not full
    B0x1 = 1,
}
impl From<TXE> for bool {
    #[inline(always)]
    fn from(variant: TXE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXE` reader - Transmit data register empty TXE is set by hardware when the content of the USART_TDR register has been transferred into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T = 0 mode, in case of transmission failure). An interrupt is generated if the TXEIE bit = 1 in the USART_CR1 register.
pub type TXE_R = crate::BitReader<TXE>;
impl TXE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXE {
        match self.bits {
            false => TXE::B0x0,
            true => TXE::B0x1,
        }
    }
    ///Data register full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXE::B0x0
    }
    ///Data register not full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXE::B0x1
    }
}
/**LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBDF {
    ///0: LIN Break not detected
    B0x0 = 0,
    ///1: LIN break detected
    B0x1 = 1,
}
impl From<LBDF> for bool {
    #[inline(always)]
    fn from(variant: LBDF) -> Self {
        variant as u8 != 0
    }
}
///Field `LBDF` reader - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type LBDF_R = crate::BitReader<LBDF>;
impl LBDF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LBDF {
        match self.bits {
            false => LBDF::B0x0,
            true => LBDF::B0x1,
        }
    }
    ///LIN Break not detected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LBDF::B0x0
    }
    ///LIN break detected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LBDF::B0x1
    }
}
/**CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE = 1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIF {
    ///0: No change occurred on the CTS status line
    B0x0 = 0,
    ///1: A change occurred on the CTS status line
    B0x1 = 1,
}
impl From<CTSIF> for bool {
    #[inline(always)]
    fn from(variant: CTSIF) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIF` reader - CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE = 1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
pub type CTSIF_R = crate::BitReader<CTSIF>;
impl CTSIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSIF {
        match self.bits {
            false => CTSIF::B0x0,
            true => CTSIF::B0x1,
        }
    }
    ///No change occurred on the CTS status line
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTSIF::B0x0
    }
    ///A change occurred on the CTS status line
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTSIF::B0x1
    }
}
/**CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTS {
    ///0: CTS line set
    B0x0 = 0,
    ///1: CTS line reset
    B0x1 = 1,
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
            false => CTS::B0x0,
            true => CTS::B0x1,
        }
    }
    ///CTS line set
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTS::B0x0
    }
    ///CTS line reset
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTS::B0x1
    }
}
/**Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE = 1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. Note: The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOF {
    ///0: Timeout value not reached
    B0x0 = 0,
    ///1: Timeout value reached without any data reception
    B0x1 = 1,
}
impl From<RTOF> for bool {
    #[inline(always)]
    fn from(variant: RTOF) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOF` reader - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE = 1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. Note: The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.
pub type RTOF_R = crate::BitReader<RTOF>;
impl RTOF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOF {
        match self.bits {
            false => RTOF::B0x0,
            true => RTOF::B0x1,
        }
    }
    ///Timeout value not reached
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTOF::B0x0
    }
    ///Timeout value reached without any data reception
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTOF::B0x1
    }
}
/**End of block flag This bit is set by hardware when a complete block has been received (for example T = 1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBF {
    ///0: End of Block not reached
    B0x0 = 0,
    ///1: End of Block (number of characters) reached
    B0x1 = 1,
}
impl From<EOBF> for bool {
    #[inline(always)]
    fn from(variant: EOBF) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBF` reader - End of block flag This bit is set by hardware when a complete block has been received (for example T = 1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type EOBF_R = crate::BitReader<EOBF>;
impl EOBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOBF {
        match self.bits {
            false => EOBF::B0x0,
            true => EOBF::B0x1,
        }
    }
    ///End of Block not reached
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOBF::B0x0
    }
    ///End of Block (number of characters) reached
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOBF::B0x1
    }
}
/**SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDR {
    ///0: No underrun error
    B0x0 = 0,
    ///1: underrun error
    B0x1 = 1,
}
impl From<UDR> for bool {
    #[inline(always)]
    fn from(variant: UDR) -> Self {
        variant as u8 != 0
    }
}
///Field `UDR` reader - SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type UDR_R = crate::BitReader<UDR>;
impl UDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDR {
        match self.bits {
            false => UDR::B0x0,
            true => UDR::B0x1,
        }
    }
    ///No underrun error
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UDR::B0x0
    }
    ///underrun error
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UDR::B0x1
    }
}
///Field `ABRE` reader - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
pub type ABRE_R = crate::BitReader;
///Field `ABRF` reader - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXNE is also set, generating an interrupt if RXNEIE = 1) or when the auto baud rate operation was completed without success (ABRE = 1) (ABRE, RXNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
pub type ABRF_R = crate::BitReader;
/**Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY {
    ///0: USART is idle (no reception)
    B0x0 = 0,
    ///1: Reception on going
    B0x1 = 1,
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
            false => BUSY::B0x0,
            true => BUSY::B0x1,
        }
    }
    ///USART is idle (no reception)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BUSY::B0x0
    }
    ///Reception on going
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BUSY::B0x1
    }
}
/**Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE = 1in the USART_CR1 register.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMF {
    ///0: No Character match detected
    B0x0 = 0,
    ///1: Character Match detected
    B0x1 = 1,
}
impl From<CMF> for bool {
    #[inline(always)]
    fn from(variant: CMF) -> Self {
        variant as u8 != 0
    }
}
///Field `CMF` reader - Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE = 1in the USART_CR1 register.
pub type CMF_R = crate::BitReader<CMF>;
impl CMF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMF {
        match self.bits {
            false => CMF::B0x0,
            true => CMF::B0x1,
        }
    }
    ///No Character match detected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CMF::B0x0
    }
    ///Character Match detected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CMF::B0x1
    }
}
/**Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBKF {
    ///0: Break character transmitted
    B0x0 = 0,
    ///1: Break character requested by setting SBKRQ bit in USART_RQR register
    B0x1 = 1,
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
            false => SBKF::B0x0,
            true => SBKF::B0x1,
        }
    }
    ///Break character transmitted
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SBKF::B0x0
    }
    ///Break character requested by setting SBKRQ bit in USART_RQR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SBKF::B0x1
    }
}
/**Receiver wake-up from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wake-up/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU {
    ///0: Receiver in active mode
    B0x0 = 0,
    ///1: Receiver in Mute mode
    B0x1 = 1,
}
impl From<RWU> for bool {
    #[inline(always)]
    fn from(variant: RWU) -> Self {
        variant as u8 != 0
    }
}
///Field `RWU` reader - Receiver wake-up from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wake-up/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type RWU_R = crate::BitReader<RWU>;
impl RWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWU {
        match self.bits {
            false => RWU::B0x0,
            true => RWU::B0x1,
        }
    }
    ///Receiver in active mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RWU::B0x0
    }
    ///Receiver in Mute mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RWU::B0x1
    }
}
///Field `WUF` reader - Wake-up from low-power mode flag This bit is set by hardware, when a wake-up event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE = 1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type WUF_R = crate::BitReader;
///Field `TEACK` reader - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE = 0, followed by TE = 1 in the USART_CR1 register, in order to respect the TE = 0 minimum period.
pub type TEACK_R = crate::BitReader;
///Field `REACK` reader - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type REACK_R = crate::BitReader;
/**Transmission complete before guard time flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGT {
    ///0: Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)
    B0x0 = 0,
    ///1: Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card).
    B0x1 = 1,
}
impl From<TCBGT> for bool {
    #[inline(always)]
    fn from(variant: TCBGT) -> Self {
        variant as u8 != 0
    }
}
///Field `TCBGT` reader - Transmission complete before guard time flag
pub type TCBGT_R = crate::BitReader<TCBGT>;
impl TCBGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCBGT {
        match self.bits {
            false => TCBGT::B0x0,
            true => TCBGT::B0x1,
        }
    }
    ///Transmission is not complete or transmission is complete unsuccessfully (i.e. a NACK is received from the card)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCBGT::B0x0
    }
    ///Transmission is complete successfully (before Guard time completion and there is no NACK from the smart card).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCBGT::B0x1
    }
}
impl R {
    ///Bit 0 - Parity error This bit is set by hardware when a parity error occurs in receiver mode. It is cleared by software, writing 1 to the PECF in the USART_ICR register. An interrupt is generated if PEIE = 1 in the USART_CR1 register.
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error This bit is set by hardware when a de-synchronization, excessive noise or a break character is detected. It is cleared by software, writing 1 to the FECF bit in the USART_ICR register. When transmitting data in Smartcard mode, this bit is set when the maximum number of transmit attempts is reached without success (the card NACKs the data frame). An interrupt is generated if EIE = 1 in the USART_CR3 register.
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detection flag This bit is set by hardware when noise is detected on a received frame. It is cleared by software, writing 1 to the NECF bit in the USART_ICR register. Note: This bit does not generate an interrupt as it appears at the same time as the RXNE bit which itself generates an interrupt. An interrupt is generated when the NE flag is set during multi buffer communication if the EIE bit is set. Note: When the line is noise-free, the NE flag can be disabled by programming the ONEBIT bit to 1 to increase the USART tolerance to deviations (Refer to Section 26.5.8: Tolerance of the USART receiver to clock deviation on page 709).
    #[inline(always)]
    pub fn ne(&self) -> NE_R {
        NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error This bit is set by hardware when the data currently being received in the shift register is ready to be transferred into the USART_RDR register while RXNE = 1. It is cleared by a software, writing 1 to the ORECF, in the USART_ICR register. An interrupt is generated if RXNEIE = 1 or EIE = 1 in the LPUART_CR1 register, or EIE = 1 in the USART_CR3 register. Note: When this bit is set, the USART_RDR register content is not lost but the shift register is overwritten. An interrupt is generated if the ORE flag is set during multi buffer communication if the EIE bit is set. Note: This bit is permanently forced to 0 (no overrun detection) when the bit OVRDIS is set in the USART_CR3 register.
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Idle line detected This bit is set by hardware when an Idle Line is detected. An interrupt is generated if IDLEIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the IDLECF in the USART_ICR register. Note: The IDLE bit is not set again until the RXNE bit has been set (i.e. a new idle line occurs). Note: If Mute mode is enabled (MME = 1), IDLE is set if the USART is not mute (RWU = 0), whatever the Mute mode selected by the WAKE bit. If RWU = 1, IDLE is not set.
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Read data register not empty RXNE bit is set by hardware when the content of the USART_RDR shift register has been transferred to the USART_RDR register. It is cleared by reading from the USART_RDR register. The RXNE flag can also be cleared by writing 1 to the RXFRQ in the USART_RQR register. An interrupt is generated if RXNEIE = 1 in the USART_CR1 register.
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete This bit indicates that the last data written in the USART_TDR has been transmitted out of the shift register. It is set by hardware when the transmission of a frame containing data is complete and when TXE is set. An interrupt is generated if TCIE = 1 in the USART_CR1 register. TC bit is is cleared by software, by writing 1 to the TCCF in the USART_ICR register or by a write to the USART_TDR register. Note: If TE bit is reset and no transmission is on going, the TC bit is set immediately.
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty TXE is set by hardware when the content of the USART_TDR register has been transferred into the shift register. It is cleared by writing to the USART_TDR register. The TXE flag can also be set by writing 1 to the TXFRQ in the USART_RQR register, in order to discard the data (only in Smartcard T = 0 mode, in case of transmission failure). An interrupt is generated if the TXEIE bit = 1 in the USART_CR1 register.
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LIN break detection flag This bit is set by hardware when the LIN break is detected. It is cleared by software, by writing 1 to the LBDCF in the USART_ICR. An interrupt is generated if LBDIE = 1 in the USART_CR2 register. Note: If the USART does not support LIN mode, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn lbdf(&self) -> LBDF_R {
        LBDF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS interrupt flag This bit is set by hardware when the CTS input toggles, if the CTSE bit is set. It is cleared by software, by writing 1 to the CTSCF bit in the USART_ICR register. An interrupt is generated if CTSIE = 1 in the USART_CR3 register. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn ctsif(&self) -> CTSIF_R {
        CTSIF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS flag This bit is set/reset by hardware. It is an inverted copy of the status of the CTS input pin. Note: If the hardware flow control feature is not supported, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver timeout This bit is set by hardware when the timeout value, programmed in the RTOR register has lapsed, without any communication. It is cleared by software, writing 1 to the RTOCF bit in the USART_ICR register. An interrupt is generated if RTOIE = 1 in the USART_CR2 register. In Smartcard mode, the timeout corresponds to the CWT or BWT timings. Note: If a time equal to the value programmed in RTOR register separates 2 characters, RTOF is not set. If this time exceeds this value + 2 sample times (2/16 or 2/8, depending on the oversampling method), RTOF flag is set. Note: The counter counts even if RE = 0 but RTOF is set only when RE = 1. If the timeout has already elapsed when RE is set, then RTOF is set. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn rtof(&self) -> RTOF_R {
        RTOF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - End of block flag This bit is set by hardware when a complete block has been received (for example T = 1 Smartcard mode). The detection is done when the number of received bytes (from the start of the block, including the prologue) is equal or greater than BLEN + 4. An interrupt is generated if the EOBIE = 1 in the USART_CR1 register. It is cleared by software, writing 1 to the EOBCF in the USART_ICR register. Note: If Smartcard mode is not supported, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn eobf(&self) -> EOBF_R {
        EOBF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI slave underrun error flag In slave transmission mode, this flag is set when the first clock pulse for data transmission appears while the software has not yet loaded any value into USART_TDR. This flag is reset by setting UDRCF bit in the USART_ICR register. Note: If the USART does not support the SPI slave mode, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Auto baud rate error This bit is set by hardware if the baud rate measurement failed (baud rate out of range or character comparison failed) It is cleared by software, by writing 1 to the ABRRQ bit in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn abre(&self) -> ABRE_R {
        ABRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Auto baud rate flag This bit is set by hardware when the automatic baud rate has been set (RXNE is also set, generating an interrupt if RXNEIE = 1) or when the auto baud rate operation was completed without success (ABRE = 1) (ABRE, RXNE and FE are also set in this case) It is cleared by software, in order to request a new auto baud rate detection, by writing 1 to the ABRRQ in the USART_RQR register. Note: If the USART does not support the auto baud rate feature, this bit is reserved and kept at reset value.
    #[inline(always)]
    pub fn abrf(&self) -> ABRF_R {
        ABRF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Busy flag This bit is set and reset by hardware. It is active when a communication is ongoing on the RX line (successful start bit detected). It is reset at the end of the reception (successful or not).
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Character match flag This bit is set by hardware, when a the character defined by ADD\[7:0\] is received. It is cleared by software, writing 1 to the CMCF in the USART_ICR register. An interrupt is generated if CMIE = 1in the USART_CR1 register.
    #[inline(always)]
    pub fn cmf(&self) -> CMF_R {
        CMF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Send break flag This bit indicates that a send break character was requested. It is set by software, by writing 1 to the SBKRQ bit in the USART_CR3 register. It is automatically reset by hardware during the stop bit of break transmission.
    #[inline(always)]
    pub fn sbkf(&self) -> SBKF_R {
        SBKF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Receiver wake-up from Mute mode This bit indicates if the USART is in Mute mode. It is cleared/set by hardware when a wake-up/mute sequence is recognized. The Mute mode control sequence (address or IDLE) is selected by the WAKE bit in the USART_CR1 register. When wake-up on IDLE mode is selected, this bit can only be set by software, writing 1 to the MMRQ bit in the USART_RQR register. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Wake-up from low-power mode flag This bit is set by hardware, when a wake-up event is detected. The event is defined by the WUS bitfield. It is cleared by software, writing a 1 to the WUCF in the USART_ICR register. An interrupt is generated if WUFIE = 1 in the USART_CR3 register. Note: When UESM is cleared, WUF flag is also cleared. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Transmit enable acknowledge flag This bit is set/reset by hardware, when the Transmit Enable value is taken into account by the USART. It can be used when an idle frame request is generated by writing TE = 0, followed by TE = 1 in the USART_CR1 register, in order to respect the TE = 0 minimum period.
    #[inline(always)]
    pub fn teack(&self) -> TEACK_R {
        TEACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Receive enable acknowledge flag This bit is set/reset by hardware, when the Receive Enable value is taken into account by the USART. It can be used to verify that the USART is ready for reception before entering low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn reack(&self) -> REACK_R {
        REACK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 25 - Transmission complete before guard time flag
    #[inline(always)]
    pub fn tcbgt(&self) -> TCBGT_R {
        TCBGT_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_ISR_ALTERNATE1")
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("ne", &self.ne())
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
            .field("wuf", &self.wuf())
            .field("teack", &self.teack())
            .field("reack", &self.reack())
            .field("tcbgt", &self.tcbgt())
            .finish()
    }
}
/**USART interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`usart_isr_alternate1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#USART1:USART_ISR_ALTERNATE1)*/
pub struct USART_ISR_ALTERNATE1rs;
impl crate::RegisterSpec for USART_ISR_ALTERNATE1rs {
    type Ux = u32;
}
///`read()` method returns [`usart_isr_alternate1::R`](R) reader structure
impl crate::Readable for USART_ISR_ALTERNATE1rs {}
///`reset()` method sets USART_ISR_ALTERNATE1 to value 0xc0
impl crate::Resettable for USART_ISR_ALTERNATE1rs {
    const RESET_VALUE: u32 = 0xc0;
}
