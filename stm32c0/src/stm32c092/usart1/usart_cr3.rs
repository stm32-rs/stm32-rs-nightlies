///Register `USART_CR3` reader
pub type R = crate::R<USART_CR3rs>;
///Register `USART_CR3` writer
pub type W = crate::W<USART_CR3rs>;
/**Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE = 1 or ORE = 1 or NE = 1 or UDR = 1 in the USART_ISR register).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
    B0x1 = 1,
}
impl From<EIE> for bool {
    #[inline(always)]
    fn from(variant: EIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE = 1 or ORE = 1 or NE = 1 or UDR = 1 in the USART_ISR register).
pub type EIE_R = crate::BitReader<EIE>;
impl EIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EIE {
        match self.bits {
            false => EIE::B0x0,
            true => EIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EIE::B0x0
    }
    ///interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EIE::B0x1
    }
}
///Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE = 1 or ORE = 1 or NE = 1 or UDR = 1 in the USART_ISR register).
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG, EIE>;
impl<'a, REG> EIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::B0x0)
    }
    ///interrupt generated when FE = 1 or ORE = 1 or NE = 1 or UDR = 1 (in SPI slave mode) in the USART_ISR register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::B0x1)
    }
}
/**IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN {
    ///0: IrDA disabled
    B0x0 = 0,
    ///1: IrDA enabled
    B0x1 = 1,
}
impl From<IREN> for bool {
    #[inline(always)]
    fn from(variant: IREN) -> Self {
        variant as u8 != 0
    }
}
///Field `IREN` reader - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type IREN_R = crate::BitReader<IREN>;
impl IREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IREN {
        match self.bits {
            false => IREN::B0x0,
            true => IREN::B0x1,
        }
    }
    ///IrDA disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IREN::B0x0
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IREN::B0x1
    }
}
///Field `IREN` writer - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG, IREN>;
impl<'a, REG> IREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IrDA disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::B0x0)
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::B0x1)
    }
}
/**IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP {
    ///0: Normal mode
    B0x0 = 0,
    ///1: Low-power mode
    B0x1 = 1,
}
impl From<IRLP> for bool {
    #[inline(always)]
    fn from(variant: IRLP) -> Self {
        variant as u8 != 0
    }
}
///Field `IRLP` reader - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type IRLP_R = crate::BitReader<IRLP>;
impl IRLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IRLP {
        match self.bits {
            false => IRLP::B0x0,
            true => IRLP::B0x1,
        }
    }
    ///Normal mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IRLP::B0x0
    }
    ///Low-power mode
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IRLP::B0x1
    }
}
///Field `IRLP` writer - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG, IRLP>;
impl<'a, REG> IRLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Normal mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::B0x0)
    }
    ///Low-power mode
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::B0x1)
    }
}
/**Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL {
    ///0: Half duplex mode is not selected
    B0x0 = 0,
    ///1: Half duplex mode is selected
    B0x1 = 1,
}
impl From<HDSEL> for bool {
    #[inline(always)]
    fn from(variant: HDSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE = 0).
pub type HDSEL_R = crate::BitReader<HDSEL>;
impl HDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HDSEL {
        match self.bits {
            false => HDSEL::B0x0,
            true => HDSEL::B0x1,
        }
    }
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HDSEL::B0x0
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HDSEL::B0x1
    }
}
///Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE = 0).
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG, HDSEL>;
impl<'a, REG> HDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::B0x0)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::B0x1)
    }
}
/**Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK {
    ///0: NACK transmission in case of parity error is disabled
    B0x0 = 0,
    ///1: NACK transmission during parity error is enabled
    B0x1 = 1,
}
impl From<NACK> for bool {
    #[inline(always)]
    fn from(variant: NACK) -> Self {
        variant as u8 != 0
    }
}
///Field `NACK` reader - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type NACK_R = crate::BitReader<NACK>;
impl NACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NACK {
        match self.bits {
            false => NACK::B0x0,
            true => NACK::B0x1,
        }
    }
    ///NACK transmission in case of parity error is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NACK::B0x0
    }
    ///NACK transmission during parity error is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NACK::B0x1
    }
}
///Field `NACK` writer - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG, NACK>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///NACK transmission in case of parity error is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::B0x0)
    }
    ///NACK transmission during parity error is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::B0x1)
    }
}
/**Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCEN {
    ///0: Smartcard Mode disabled
    B0x0 = 0,
    ///1: Smartcard Mode enabled
    B0x1 = 1,
}
impl From<SCEN> for bool {
    #[inline(always)]
    fn from(variant: SCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SCEN` reader - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type SCEN_R = crate::BitReader<SCEN>;
impl SCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SCEN {
        match self.bits {
            false => SCEN::B0x0,
            true => SCEN::B0x1,
        }
    }
    ///Smartcard Mode disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SCEN::B0x0
    }
    ///Smartcard Mode enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SCEN::B0x1
    }
}
///Field `SCEN` writer - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG, SCEN>;
impl<'a, REG> SCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Smartcard Mode disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN::B0x0)
    }
    ///Smartcard Mode enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN::B0x1)
    }
}
/**DMA enable receiver This bit is set/reset by software

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR {
    ///1: DMA mode is enabled for reception
    B0x1 = 1,
    ///0: DMA mode is disabled for reception
    B0x0 = 0,
}
impl From<DMAR> for bool {
    #[inline(always)]
    fn from(variant: DMAR) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAR` reader - DMA enable receiver This bit is set/reset by software
pub type DMAR_R = crate::BitReader<DMAR>;
impl DMAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAR {
        match self.bits {
            true => DMAR::B0x1,
            false => DMAR::B0x0,
        }
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAR::B0x1
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAR::B0x0
    }
}
///Field `DMAR` writer - DMA enable receiver This bit is set/reset by software
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG, DMAR>;
impl<'a, REG> DMAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::B0x1)
    }
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::B0x0)
    }
}
/**DMA enable transmitter This bit is set/reset by software

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT {
    ///1: DMA mode is enabled for transmission
    B0x1 = 1,
    ///0: DMA mode is disabled for transmission
    B0x0 = 0,
}
impl From<DMAT> for bool {
    #[inline(always)]
    fn from(variant: DMAT) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software
pub type DMAT_R = crate::BitReader<DMAT>;
impl DMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAT {
        match self.bits {
            true => DMAT::B0x1,
            false => DMAT::B0x0,
        }
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DMAT::B0x1
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DMAT::B0x0
    }
}
///Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG, DMAT>;
impl<'a, REG> DMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::B0x1)
    }
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::B0x0)
    }
}
/**RTS enable This bit can only be written when the USART is disabled (UE = 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE {
    ///0: RTS hardware flow control disabled
    B0x0 = 0,
    ///1: RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The RTS output is deasserted (pulled to 0) when data can be received.
    B0x1 = 1,
}
impl From<RTSE> for bool {
    #[inline(always)]
    fn from(variant: RTSE) -> Self {
        variant as u8 != 0
    }
}
///Field `RTSE` reader - RTS enable This bit can only be written when the USART is disabled (UE = 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type RTSE_R = crate::BitReader<RTSE>;
impl RTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTSE {
        match self.bits {
            false => RTSE::B0x0,
            true => RTSE::B0x1,
        }
    }
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTSE::B0x0
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The RTS output is deasserted (pulled to 0) when data can be received.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTSE::B0x1
    }
}
///Field `RTSE` writer - RTS enable This bit can only be written when the USART is disabled (UE = 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG, RTSE>;
impl<'a, REG> RTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTS hardware flow control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::B0x0)
    }
    ///RTS output enabled, data is only requested when there is space in the receive buffer. The transmission of data is expected to cease after the current character has been transmitted. The RTS output is deasserted (pulled to 0) when data can be received.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::B0x1)
    }
}
/**CTS enable This bit can only be written when the USART is disabled (UE = 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE {
    ///0: CTS hardware flow control disabled
    B0x0 = 0,
    ///1: CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0). If the CTS input is asserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while CTS is asserted, the transmission is postponed until CTS is deasserted.
    B0x1 = 1,
}
impl From<CTSE> for bool {
    #[inline(always)]
    fn from(variant: CTSE) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSE` reader - CTS enable This bit can only be written when the USART is disabled (UE = 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type CTSE_R = crate::BitReader<CTSE>;
impl CTSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSE {
        match self.bits {
            false => CTSE::B0x0,
            true => CTSE::B0x1,
        }
    }
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTSE::B0x0
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0). If the CTS input is asserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while CTS is asserted, the transmission is postponed until CTS is deasserted.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTSE::B0x1
    }
}
///Field `CTSE` writer - CTS enable This bit can only be written when the USART is disabled (UE = 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG, CTSE>;
impl<'a, REG> CTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CTS hardware flow control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::B0x0)
    }
    ///CTS mode enabled, data is only transmitted when the CTS input is deasserted (tied to 0). If the CTS input is asserted while data is being transmitted, then the transmission is completed before stopping. If data is written into the data register while CTS is asserted, the transmission is postponed until CTS is deasserted.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::B0x1)
    }
}
/**CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE {
    ///0: Interrupt is inhibited
    B0x0 = 0,
    ///1: An interrupt is generated whenever CTSIF = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<CTSIE> for bool {
    #[inline(always)]
    fn from(variant: CTSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CTSIE` reader - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type CTSIE_R = crate::BitReader<CTSIE>;
impl CTSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CTSIE {
        match self.bits {
            false => CTSIE::B0x0,
            true => CTSIE::B0x1,
        }
    }
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CTSIE::B0x0
    }
    ///An interrupt is generated whenever CTSIF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CTSIE::B0x1
    }
}
///Field `CTSIE` writer - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG, CTSIE>;
impl<'a, REG> CTSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt is inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::B0x0)
    }
    ///An interrupt is generated whenever CTSIF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::B0x1)
    }
}
/**One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEBIT {
    ///0: Three sample bit method
    B0x0 = 0,
    ///1: One sample bit method
    B0x1 = 1,
}
impl From<ONEBIT> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT) -> Self {
        variant as u8 != 0
    }
}
///Field `ONEBIT` reader - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE = 0).
pub type ONEBIT_R = crate::BitReader<ONEBIT>;
impl ONEBIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ONEBIT {
        match self.bits {
            false => ONEBIT::B0x0,
            true => ONEBIT::B0x1,
        }
    }
    ///Three sample bit method
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ONEBIT::B0x0
    }
    ///One sample bit method
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ONEBIT::B0x1
    }
}
///Field `ONEBIT` writer - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE = 0).
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG, ONEBIT>;
impl<'a, REG> ONEBIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Three sample bit method
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::B0x0)
    }
    ///One sample bit method
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::B0x1)
    }
}
/**Overrun disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE = 0). Note: This control bit enables checking the communication flow w/o reading the data

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDIS {
    ///0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data.
    B0x0 = 0,
    ///1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set
    B0x1 = 1,
}
impl From<OVRDIS> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `OVRDIS` reader - Overrun disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE = 0). Note: This control bit enables checking the communication flow w/o reading the data
pub type OVRDIS_R = crate::BitReader<OVRDIS>;
impl OVRDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRDIS {
        match self.bits {
            false => OVRDIS::B0x0,
            true => OVRDIS::B0x1,
        }
    }
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVRDIS::B0x0
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVRDIS::B0x1
    }
}
///Field `OVRDIS` writer - Overrun disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE = 0). Note: This control bit enables checking the communication flow w/o reading the data
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG, OVRDIS>;
impl<'a, REG> OVRDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Overrun Error Flag, ORE, is set when received data is not read before receiving new data.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::B0x0)
    }
    ///Overrun functionality is disabled. If new data is received while the RXNE flag is still set
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRDIS::B0x1)
    }
}
/**DMA Disable on reception error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRE {
    ///0: DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred (used for Smartcard mode).
    B0x0 = 0,
    ///1: DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE/RXFNE is case FIFO mode is enabled) before clearing the error flag.
    B0x1 = 1,
}
impl From<DDRE> for bool {
    #[inline(always)]
    fn from(variant: DDRE) -> Self {
        variant as u8 != 0
    }
}
///Field `DDRE` reader - DMA Disable on reception error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
pub type DDRE_R = crate::BitReader<DDRE>;
impl DDRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DDRE {
        match self.bits {
            false => DDRE::B0x0,
            true => DDRE::B0x1,
        }
    }
    ///DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred (used for Smartcard mode).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DDRE::B0x0
    }
    ///DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE/RXFNE is case FIFO mode is enabled) before clearing the error flag.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DDRE::B0x1
    }
}
///Field `DDRE` writer - DMA Disable on reception error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG, DDRE>;
impl<'a, REG> DDRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DMA is not disabled in case of reception error. The corresponding error flag is set but RXNE is kept 0 preventing from overrun. As a consequence, the DMA request is not asserted, so the erroneous data is not transferred (no DMA request), but next correct received data is transferred (used for Smartcard mode).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::B0x0)
    }
    ///DMA is disabled following a reception error. The corresponding error flag is set, as well as RXNE. The DMA request is masked until the error flag is cleared. This means that the software must first disable the DMA request (DMAR = 0) or clear RXNE/RXFNE is case FIFO mode is enabled) before clearing the error flag.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DDRE::B0x1)
    }
}
/**Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEM {
    ///0: DE function is disabled.
    B0x0 = 0,
    ///1: DE function is enabled. The DE signal is output on the RTS pin.
    B0x1 = 1,
}
impl From<DEM> for bool {
    #[inline(always)]
    fn from(variant: DEM) -> Self {
        variant as u8 != 0
    }
}
///Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
pub type DEM_R = crate::BitReader<DEM>;
impl DEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEM {
        match self.bits {
            false => DEM::B0x0,
            true => DEM::B0x1,
        }
    }
    ///DE function is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DEM::B0x0
    }
    ///DE function is enabled. The DE signal is output on the RTS pin.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DEM::B0x1
    }
}
///Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG, DEM>;
impl<'a, REG> DEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DE function is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::B0x0)
    }
    ///DE function is enabled. The DE signal is output on the RTS pin.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEM::B0x1)
    }
}
/**Driver enable polarity selection This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP {
    ///0: DE signal is active high.
    B0x0 = 0,
    ///1: DE signal is active low.
    B0x1 = 1,
}
impl From<DEP> for bool {
    #[inline(always)]
    fn from(variant: DEP) -> Self {
        variant as u8 != 0
    }
}
///Field `DEP` reader - Driver enable polarity selection This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEP_R = crate::BitReader<DEP>;
impl DEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DEP {
        match self.bits {
            false => DEP::B0x0,
            true => DEP::B0x1,
        }
    }
    ///DE signal is active high.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DEP::B0x0
    }
    ///DE signal is active low.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DEP::B0x1
    }
}
///Field `DEP` writer - Driver enable polarity selection This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG, DEP>;
impl<'a, REG> DEP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DE signal is active high.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::B0x0)
    }
    ///DE signal is active low.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DEP::B0x1)
    }
}
/**Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE = 0). When the USART is enabled (UE = 1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCARCNT {
    ///0: retransmission disabled - No automatic retransmission in transmit mode.
    B0x0 = 0,
}
impl From<SCARCNT> for u8 {
    #[inline(always)]
    fn from(variant: SCARCNT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCARCNT {
    type Ux = u8;
}
impl crate::IsEnum for SCARCNT {}
///Field `SCARCNT` reader - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE = 0). When the USART is enabled (UE = 1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type SCARCNT_R = crate::FieldReader<SCARCNT>;
impl SCARCNT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SCARCNT> {
        match self.bits {
            0 => Some(SCARCNT::B0x0),
            _ => None,
        }
    }
    ///retransmission disabled - No automatic retransmission in transmit mode.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SCARCNT::B0x0
    }
}
///Field `SCARCNT` writer - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE = 0). When the USART is enabled (UE = 1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type SCARCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SCARCNT>;
impl<'a, REG> SCARCNT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///retransmission disabled - No automatic retransmission in transmit mode.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SCARCNT::B0x0)
    }
}
/**Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUS {
    ///0: WUF active on address match (as defined by ADD\[7:0\] and ADDM7)
    B0x0 = 0,
    ///1: Reserved.
    B0x1 = 1,
    ///2: WUF active on start bit detection
    B0x2 = 2,
    ///3: WUF active on RXNE/RXFNE.
    B0x3 = 3,
}
impl From<WUS> for u8 {
    #[inline(always)]
    fn from(variant: WUS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUS {
    type Ux = u8;
}
impl crate::IsEnum for WUS {}
///Field `WUS` reader - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type WUS_R = crate::FieldReader<WUS>;
impl WUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUS {
        match self.bits {
            0 => WUS::B0x0,
            1 => WUS::B0x1,
            2 => WUS::B0x2,
            3 => WUS::B0x3,
            _ => unreachable!(),
        }
    }
    ///WUF active on address match (as defined by ADD\[7:0\] and ADDM7)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WUS::B0x0
    }
    ///Reserved.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WUS::B0x1
    }
    ///WUF active on start bit detection
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == WUS::B0x2
    }
    ///WUF active on RXNE/RXFNE.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == WUS::B0x3
    }
}
///Field `WUS` writer - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2, WUS, crate::Safe>;
impl<'a, REG> WUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///WUF active on address match (as defined by ADD\[7:0\] and ADDM7)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::B0x0)
    }
    ///Reserved.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::B0x1)
    }
    ///WUF active on start bit detection
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::B0x2)
    }
    ///WUF active on RXNE/RXFNE.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(WUS::B0x3)
    }
}
/**Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated whenever WUF = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<WUFIE> for bool {
    #[inline(always)]
    fn from(variant: WUFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `WUFIE` reader - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type WUFIE_R = crate::BitReader<WUFIE>;
impl WUFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WUFIE {
        match self.bits {
            false => WUFIE::B0x0,
            true => WUFIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WUFIE::B0x0
    }
    ///USART interrupt generated whenever WUF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WUFIE::B0x1
    }
}
///Field `WUFIE` writer - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG, WUFIE>;
impl<'a, REG> WUFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE::B0x0)
    }
    ///USART interrupt generated whenever WUF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WUFIE::B0x1)
    }
}
/**TXFIFO threshold interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFTIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG.
    B0x1 = 1,
}
impl From<TXFTIE> for bool {
    #[inline(always)]
    fn from(variant: TXFTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software.
pub type TXFTIE_R = crate::BitReader<TXFTIE>;
impl TXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFTIE {
        match self.bits {
            false => TXFTIE::B0x0,
            true => TXFTIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXFTIE::B0x0
    }
    ///USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXFTIE::B0x1
    }
}
///Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software.
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFTIE>;
impl<'a, REG> TXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE::B0x0)
    }
    ///USART interrupt generated when TXFIFO reaches the threshold programmed in TXFTCFG.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTIE::B0x1)
    }
}
/**Transmission complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCBGTIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    B0x1 = 1,
}
impl From<TCBGTIE> for bool {
    #[inline(always)]
    fn from(variant: TCBGTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCBGTIE` reader - Transmission complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type TCBGTIE_R = crate::BitReader<TCBGTIE>;
impl TCBGTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCBGTIE {
        match self.bits {
            false => TCBGTIE::B0x0,
            true => TCBGTIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCBGTIE::B0x0
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCBGTIE::B0x1
    }
}
///Field `TCBGTIE` writer - Transmission complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type TCBGTIE_W<'a, REG> = crate::BitWriter<'a, REG, TCBGTIE>;
impl<'a, REG> TCBGTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE::B0x0)
    }
    ///USART interrupt generated whenever TCBGT=1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCBGTIE::B0x1)
    }
}
/**Receive FIFO threshold configuration Remaining combinations: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXFTCFG {
    ///0: Receive FIFO reaches 1/8 of its depth
    B0x0 = 0,
    ///1: Receive FIFO reaches 1/4 of its depth
    B0x1 = 1,
    ///2: Receive FIFO reaches 1/2 of its depth
    B0x2 = 2,
    ///3: Receive FIFO reaches 3/4 of its depth
    B0x3 = 3,
    ///4: Receive FIFO reaches 7/8 of its depth
    B0x4 = 4,
    ///5: Receive FIFO becomes full
    B0x5 = 5,
}
impl From<RXFTCFG> for u8 {
    #[inline(always)]
    fn from(variant: RXFTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RXFTCFG {
    type Ux = u8;
}
impl crate::IsEnum for RXFTCFG {}
///Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved
pub type RXFTCFG_R = crate::FieldReader<RXFTCFG>;
impl RXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<RXFTCFG> {
        match self.bits {
            0 => Some(RXFTCFG::B0x0),
            1 => Some(RXFTCFG::B0x1),
            2 => Some(RXFTCFG::B0x2),
            3 => Some(RXFTCFG::B0x3),
            4 => Some(RXFTCFG::B0x4),
            5 => Some(RXFTCFG::B0x5),
            _ => None,
        }
    }
    ///Receive FIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXFTCFG::B0x0
    }
    ///Receive FIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXFTCFG::B0x1
    }
    ///Receive FIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == RXFTCFG::B0x2
    }
    ///Receive FIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == RXFTCFG::B0x3
    }
    ///Receive FIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == RXFTCFG::B0x4
    }
    ///Receive FIFO becomes full
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == RXFTCFG::B0x5
    }
}
///Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, RXFTCFG>;
impl<'a, REG> RXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Receive FIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::B0x0)
    }
    ///Receive FIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::B0x1)
    }
    ///Receive FIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::B0x2)
    }
    ///Receive FIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::B0x3)
    }
    ///Receive FIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::B0x4)
    }
    ///Receive FIFO becomes full
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTCFG::B0x5)
    }
}
/**RXFIFO threshold interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFTIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
    B0x1 = 1,
}
impl From<RXFTIE> for bool {
    #[inline(always)]
    fn from(variant: RXFTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software.
pub type RXFTIE_R = crate::BitReader<RXFTIE>;
impl RXFTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFTIE {
        match self.bits {
            false => RXFTIE::B0x0,
            true => RXFTIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXFTIE::B0x0
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXFTIE::B0x1
    }
}
///Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software.
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG, RXFTIE>;
impl<'a, REG> RXFTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE::B0x0)
    }
    ///USART interrupt generated when Receive FIFO reaches the threshold programmed in RXFTCFG.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXFTIE::B0x1)
    }
}
/**TXFIFO threshold configuration Remaining combinations: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXFTCFG {
    ///0: TXFIFO reaches 1/8 of its depth
    B0x0 = 0,
    ///1: TXFIFO reaches 1/4 of its depth
    B0x1 = 1,
    ///2: TXFIFO reaches 1/2 of its depth
    B0x2 = 2,
    ///3: TXFIFO reaches 3/4 of its depth
    B0x3 = 3,
    ///4: TXFIFO reaches 7/8 of its depth
    B0x4 = 4,
    ///5: TXFIFO becomes empty
    B0x5 = 5,
}
impl From<TXFTCFG> for u8 {
    #[inline(always)]
    fn from(variant: TXFTCFG) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TXFTCFG {
    type Ux = u8;
}
impl crate::IsEnum for TXFTCFG {}
///Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved
pub type TXFTCFG_R = crate::FieldReader<TXFTCFG>;
impl TXFTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TXFTCFG> {
        match self.bits {
            0 => Some(TXFTCFG::B0x0),
            1 => Some(TXFTCFG::B0x1),
            2 => Some(TXFTCFG::B0x2),
            3 => Some(TXFTCFG::B0x3),
            4 => Some(TXFTCFG::B0x4),
            5 => Some(TXFTCFG::B0x5),
            _ => None,
        }
    }
    ///TXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXFTCFG::B0x0
    }
    ///TXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXFTCFG::B0x1
    }
    ///TXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TXFTCFG::B0x2
    }
    ///TXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TXFTCFG::B0x3
    }
    ///TXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == TXFTCFG::B0x4
    }
    ///TXFIFO becomes empty
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == TXFTCFG::B0x5
    }
}
///Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TXFTCFG>;
impl<'a, REG> TXFTCFG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TXFIFO reaches 1/8 of its depth
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::B0x0)
    }
    ///TXFIFO reaches 1/4 of its depth
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::B0x1)
    }
    ///TXFIFO reaches 1/2 of its depth
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::B0x2)
    }
    ///TXFIFO reaches 3/4 of its depth
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::B0x3)
    }
    ///TXFIFO reaches 7/8 of its depth
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::B0x4)
    }
    ///TXFIFO becomes empty
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(TXFTCFG::B0x5)
    }
}
impl R {
    ///Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE = 1 or ORE = 1 or NE = 1 or UDR = 1 in the USART_ISR register).
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver This bit is set/reset by software
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter This bit is set/reset by software
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE = 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE = 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Overrun disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE = 0). Note: This control bit enables checking the communication flow w/o reading the data
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Disable on reception error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE = 0). When the USART is enabled (UE = 1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:21 - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Transmission complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_CR3")
            .field("eie", &self.eie())
            .field("iren", &self.iren())
            .field("irlp", &self.irlp())
            .field("hdsel", &self.hdsel())
            .field("nack", &self.nack())
            .field("scen", &self.scen())
            .field("dmar", &self.dmar())
            .field("dmat", &self.dmat())
            .field("rtse", &self.rtse())
            .field("ctse", &self.ctse())
            .field("ctsie", &self.ctsie())
            .field("onebit", &self.onebit())
            .field("ovrdis", &self.ovrdis())
            .field("ddre", &self.ddre())
            .field("dem", &self.dem())
            .field("dep", &self.dep())
            .field("scarcnt", &self.scarcnt())
            .field("wus", &self.wus())
            .field("wufie", &self.wufie())
            .field("txftie", &self.txftie())
            .field("tcbgtie", &self.tcbgtie())
            .field("rxftcfg", &self.rxftcfg())
            .field("rxftie", &self.rxftie())
            .field("txftcfg", &self.txftcfg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FE = 1 or ORE = 1 or NE = 1 or UDR = 1 in the USART_ISR register).
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<'_, USART_CR3rs> {
        EIE_W::new(self, 0)
    }
    ///Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<'_, USART_CR3rs> {
        IREN_W::new(self, 1)
    }
    ///Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UE = 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W<'_, USART_CR3rs> {
        IRLP_W::new(self, 2)
    }
    ///Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<'_, USART_CR3rs> {
        HDSEL_W::new(self, 3)
    }
    ///Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<'_, USART_CR3rs> {
        NACK_W::new(self, 4)
    }
    ///Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W<'_, USART_CR3rs> {
        SCEN_W::new(self, 5)
    }
    ///Bit 6 - DMA enable receiver This bit is set/reset by software
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<'_, USART_CR3rs> {
        DMAR_W::new(self, 6)
    }
    ///Bit 7 - DMA enable transmitter This bit is set/reset by software
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<'_, USART_CR3rs> {
        DMAT_W::new(self, 7)
    }
    ///Bit 8 - RTS enable This bit can only be written when the USART is disabled (UE = 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<'_, USART_CR3rs> {
        RTSE_W::new(self, 8)
    }
    ///Bit 9 - CTS enable This bit can only be written when the USART is disabled (UE = 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<'_, USART_CR3rs> {
        CTSE_W::new(self, 9)
    }
    ///Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<'_, USART_CR3rs> {
        CTSIE_W::new(self, 10)
    }
    ///Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W<'_, USART_CR3rs> {
        ONEBIT_W::new(self, 11)
    }
    ///Bit 12 - Overrun disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UE = 0). Note: This control bit enables checking the communication flow w/o reading the data
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<'_, USART_CR3rs> {
        OVRDIS_W::new(self, 12)
    }
    ///Bit 13 - DMA Disable on reception error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error.
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<'_, USART_CR3rs> {
        DDRE_W::new(self, 13)
    }
    ///Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<'_, USART_CR3rs> {
        DEM_W::new(self, 14)
    }
    ///Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<'_, USART_CR3rs> {
        DEP_W::new(self, 15)
    }
    ///Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UE = 0). When the USART is enabled (UE = 1), this bitfield may only be written to 0x0, in order to stop retransmission. 0x1 to 0x7: number of automatic retransmission attempts (before signaling error) Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W<'_, USART_CR3rs> {
        SCARCNT_W::new(self, 17)
    }
    ///Bits 20:21 - Wake-up from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (wake-up from low-power mode flag). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<'_, USART_CR3rs> {
        WUS_W::new(self, 20)
    }
    ///Bit 22 - Wake-up from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<'_, USART_CR3rs> {
        WUFIE_W::new(self, 22)
    }
    ///Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<'_, USART_CR3rs> {
        TXFTIE_W::new(self, 23)
    }
    ///Bit 24 - Transmission complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W<'_, USART_CR3rs> {
        TCBGTIE_W::new(self, 24)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<'_, USART_CR3rs> {
        RXFTCFG_W::new(self, 25)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<'_, USART_CR3rs> {
        RXFTIE_W::new(self, 28)
    }
    ///Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<'_, USART_CR3rs> {
        TXFTCFG_W::new(self, 29)
    }
}
/**USART control register 3

You can [`read`](crate::Reg::read) this register and get [`usart_cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#USART1:USART_CR3)*/
pub struct USART_CR3rs;
impl crate::RegisterSpec for USART_CR3rs {
    type Ux = u32;
}
///`read()` method returns [`usart_cr3::R`](R) reader structure
impl crate::Readable for USART_CR3rs {}
///`write(|w| ..)` method takes [`usart_cr3::W`](W) writer structure
impl crate::Writable for USART_CR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USART_CR3 to value 0
impl crate::Resettable for USART_CR3rs {}
