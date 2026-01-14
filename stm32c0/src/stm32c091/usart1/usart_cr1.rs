///Register `USART_CR1` reader
pub type R = crate::R<USART_CR1rs>;
///Register `USART_CR1` writer
pub type W = crate::W<USART_CR1rs>;
/**USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE {
    ///0: USART prescaler and outputs disabled, low-power mode
    B0x0 = 0,
    ///1: USART enabled
    B0x1 = 1,
}
impl From<UE> for bool {
    #[inline(always)]
    fn from(variant: UE) -> Self {
        variant as u8 != 0
    }
}
///Field `UE` reader - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
pub type UE_R = crate::BitReader<UE>;
impl UE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UE {
        match self.bits {
            false => UE::B0x0,
            true => UE::B0x1,
        }
    }
    ///USART prescaler and outputs disabled, low-power mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UE::B0x0
    }
    ///USART enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UE::B0x1
    }
}
///Field `UE` writer - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG, UE>;
impl<'a, REG> UE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART prescaler and outputs disabled, low-power mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UE::B0x0)
    }
    ///USART enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UE::B0x1)
    }
}
/**USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UESM {
    ///0: USART not able to wake up the MCU from low-power mode.
    B0x0 = 0,
    ///1: USART able to wake up the MCU from low-power mode.
    B0x1 = 1,
}
impl From<UESM> for bool {
    #[inline(always)]
    fn from(variant: UESM) -> Self {
        variant as u8 != 0
    }
}
///Field `UESM` reader - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type UESM_R = crate::BitReader<UESM>;
impl UESM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UESM {
        match self.bits {
            false => UESM::B0x0,
            true => UESM::B0x1,
        }
    }
    ///USART not able to wake up the MCU from low-power mode.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UESM::B0x0
    }
    ///USART able to wake up the MCU from low-power mode.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UESM::B0x1
    }
}
///Field `UESM` writer - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG, UESM>;
impl<'a, REG> UESM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART not able to wake up the MCU from low-power mode.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UESM::B0x0)
    }
    ///USART able to wake up the MCU from low-power mode.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UESM::B0x1)
    }
}
/**Receiver enable This bit enables the receiver. It is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    ///0: Receiver is disabled
    B0x0 = 0,
    ///1: Receiver is enabled and begins searching for a start bit
    B0x1 = 1,
}
impl From<RE> for bool {
    #[inline(always)]
    fn from(variant: RE) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_R = crate::BitReader<RE>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RE {
        match self.bits {
            false => RE::B0x0,
            true => RE::B0x1,
        }
    }
    ///Receiver is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RE::B0x0
    }
    ///Receiver is enabled and begins searching for a start bit
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RE::B0x1
    }
}
///Field `RE` writer - Receiver enable This bit enables the receiver. It is set and cleared by software.
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RE::B0x0)
    }
    ///Receiver is enabled and begins searching for a start bit
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RE::B0x1)
    }
}
/**Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE {
    ///0: Transmitter is disabled
    B0x0 = 0,
    ///1: Transmitter is enabled
    B0x1 = 1,
}
impl From<TE> for bool {
    #[inline(always)]
    fn from(variant: TE) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
pub type TE_R = crate::BitReader<TE>;
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TE {
        match self.bits {
            false => TE::B0x0,
            true => TE::B0x1,
        }
    }
    ///Transmitter is disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TE::B0x0
    }
    ///Transmitter is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TE::B0x1
    }
}
///Field `TE` writer - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmitter is disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TE::B0x0)
    }
    ///Transmitter is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TE::B0x1)
    }
}
/**IDLE interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated whenever IDLE = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<IDLEIE> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLEIE` reader - IDLE interrupt enable This bit is set and cleared by software.
pub type IDLEIE_R = crate::BitReader<IDLEIE>;
impl IDLEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLEIE {
        match self.bits {
            false => IDLEIE::B0x0,
            true => IDLEIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IDLEIE::B0x0
    }
    ///USART interrupt generated whenever IDLE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IDLEIE::B0x1
    }
}
///Field `IDLEIE` writer - IDLE interrupt enable This bit is set and cleared by software.
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG, IDLEIE>;
impl<'a, REG> IDLEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::B0x0)
    }
    ///USART interrupt generated whenever IDLE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::B0x1)
    }
}
/**RXFIFO not empty interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFNEIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated whenever ORE = 1 or RXFNE = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<RXFNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXFNEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFNEIE` reader - RXFIFO not empty interrupt enable This bit is set and cleared by software.
pub type RXFNEIE_R = crate::BitReader<RXFNEIE>;
impl RXFNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFNEIE {
        match self.bits {
            false => RXFNEIE::B0x0,
            true => RXFNEIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXFNEIE::B0x0
    }
    ///USART interrupt generated whenever ORE = 1 or RXFNE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXFNEIE::B0x1
    }
}
///Field `RXFNEIE` writer - RXFIFO not empty interrupt enable This bit is set and cleared by software.
pub type RXFNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXFNEIE>;
impl<'a, REG> RXFNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXFNEIE::B0x0)
    }
    ///USART interrupt generated whenever ORE = 1 or RXFNE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXFNEIE::B0x1)
    }
}
/**Transmission complete interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated whenever TC = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transmission complete interrupt enable This bit is set and cleared by software.
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::B0x0,
            true => TCIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIE::B0x0
    }
    ///USART interrupt generated whenever TC = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIE::B0x1
    }
}
///Field `TCIE` writer - Transmission complete interrupt enable This bit is set and cleared by software.
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::B0x0)
    }
    ///USART interrupt generated whenever TC = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::B0x1)
    }
}
/**TXFIFO not-full interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFNFIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated whenever TXFNF =1 in the USART_ISR register
    B0x1 = 1,
}
impl From<TXFNFIE> for bool {
    #[inline(always)]
    fn from(variant: TXFNFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFNFIE` reader - TXFIFO not-full interrupt enable This bit is set and cleared by software.
pub type TXFNFIE_R = crate::BitReader<TXFNFIE>;
impl TXFNFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFNFIE {
        match self.bits {
            false => TXFNFIE::B0x0,
            true => TXFNFIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXFNFIE::B0x0
    }
    ///USART interrupt generated whenever TXFNF =1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXFNFIE::B0x1
    }
}
///Field `TXFNFIE` writer - TXFIFO not-full interrupt enable This bit is set and cleared by software.
pub type TXFNFIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFNFIE>;
impl<'a, REG> TXFNFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNFIE::B0x0)
    }
    ///USART interrupt generated whenever TXFNF =1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFNFIE::B0x1)
    }
}
/**PE interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated whenever PE = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<PEIE> for bool {
    #[inline(always)]
    fn from(variant: PEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PEIE` reader - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_R = crate::BitReader<PEIE>;
impl PEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PEIE {
        match self.bits {
            false => PEIE::B0x0,
            true => PEIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PEIE::B0x0
    }
    ///USART interrupt generated whenever PE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PEIE::B0x1
    }
}
///Field `PEIE` writer - PE interrupt enable This bit is set and cleared by software.
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG, PEIE>;
impl<'a, REG> PEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::B0x0)
    }
    ///USART interrupt generated whenever PE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::B0x1)
    }
}
/**Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS {
    ///0: Even parity
    B0x0 = 0,
    ///1: Odd parity
    B0x1 = 1,
}
impl From<PS> for bool {
    #[inline(always)]
    fn from(variant: PS) -> Self {
        variant as u8 != 0
    }
}
///Field `PS` reader - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
pub type PS_R = crate::BitReader<PS>;
impl PS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PS {
        match self.bits {
            false => PS::B0x0,
            true => PS::B0x1,
        }
    }
    ///Even parity
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PS::B0x0
    }
    ///Odd parity
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PS::B0x1
    }
}
///Field `PS` writer - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG, PS>;
impl<'a, REG> PS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Even parity
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PS::B0x0)
    }
    ///Odd parity
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PS::B0x1)
    }
}
/**Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE {
    ///0: Parity control disabled
    B0x0 = 0,
    ///1: Parity control enabled
    B0x1 = 1,
}
impl From<PCE> for bool {
    #[inline(always)]
    fn from(variant: PCE) -> Self {
        variant as u8 != 0
    }
}
///Field `PCE` reader - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
pub type PCE_R = crate::BitReader<PCE>;
impl PCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCE {
        match self.bits {
            false => PCE::B0x0,
            true => PCE::B0x1,
        }
    }
    ///Parity control disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PCE::B0x0
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PCE::B0x1
    }
}
///Field `PCE` writer - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG, PCE>;
impl<'a, REG> PCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Parity control disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::B0x0)
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::B0x1)
    }
}
/**Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE {
    ///0: Idle line
    B0x0 = 0,
    ///1: Address mark
    B0x1 = 1,
}
impl From<WAKE> for bool {
    #[inline(always)]
    fn from(variant: WAKE) -> Self {
        variant as u8 != 0
    }
}
///Field `WAKE` reader - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
pub type WAKE_R = crate::BitReader<WAKE>;
impl WAKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAKE {
        match self.bits {
            false => WAKE::B0x0,
            true => WAKE::B0x1,
        }
    }
    ///Idle line
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WAKE::B0x0
    }
    ///Address mark
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WAKE::B0x1
    }
}
///Field `WAKE` writer - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG, WAKE>;
impl<'a, REG> WAKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Idle line
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::B0x0)
    }
    ///Address mark
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::B0x1)
    }
}
///Field `M0` reader - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
pub type M0_R = crate::BitReader;
///Field `M0` writer - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
pub type M0_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MME {
    ///0: Receiver in active mode permanently
    B0x0 = 0,
    ///1: Receiver can switch between Mute mode and active mode.
    B0x1 = 1,
}
impl From<MME> for bool {
    #[inline(always)]
    fn from(variant: MME) -> Self {
        variant as u8 != 0
    }
}
///Field `MME` reader - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_R = crate::BitReader<MME>;
impl MME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MME {
        match self.bits {
            false => MME::B0x0,
            true => MME::B0x1,
        }
    }
    ///Receiver in active mode permanently
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MME::B0x0
    }
    ///Receiver can switch between Mute mode and active mode.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MME::B0x1
    }
}
///Field `MME` writer - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG, MME>;
impl<'a, REG> MME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver in active mode permanently
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MME::B0x0)
    }
    ///Receiver can switch between Mute mode and active mode.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MME::B0x1)
    }
}
/**Character match interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated when the CMF bit is set in the USART_ISR register.
    B0x1 = 1,
}
impl From<CMIE> for bool {
    #[inline(always)]
    fn from(variant: CMIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CMIE` reader - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_R = crate::BitReader<CMIE>;
impl CMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMIE {
        match self.bits {
            false => CMIE::B0x0,
            true => CMIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CMIE::B0x0
    }
    ///USART interrupt generated when the CMF bit is set in the USART_ISR register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CMIE::B0x1
    }
}
///Field `CMIE` writer - Character match interrupt enable This bit is set and cleared by software.
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG, CMIE>;
impl<'a, REG> CMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE::B0x0)
    }
    ///USART interrupt generated when the CMF bit is set in the USART_ISR register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE::B0x1)
    }
}
/**Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER8 {
    ///0: Oversampling by 16
    B0x0 = 0,
    ///1: Oversampling by 8
    B0x1 = 1,
}
impl From<OVER8> for bool {
    #[inline(always)]
    fn from(variant: OVER8) -> Self {
        variant as u8 != 0
    }
}
///Field `OVER8` reader - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
pub type OVER8_R = crate::BitReader<OVER8>;
impl OVER8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVER8 {
        match self.bits {
            false => OVER8::B0x0,
            true => OVER8::B0x1,
        }
    }
    ///Oversampling by 16
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OVER8::B0x0
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OVER8::B0x1
    }
}
///Field `OVER8` writer - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG, OVER8>;
impl<'a, REG> OVER8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampling by 16
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::B0x0)
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::B0x1)
    }
}
///Field `DEDT` reader - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEDT_R = crate::FieldReader;
///Field `DEDT` writer - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEDT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DEAT` reader - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEAT_R = crate::FieldReader;
///Field `DEAT` writer - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type DEAT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
/**Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated when the RTOF bit is set in the USART_ISR register.
    B0x1 = 1,
}
impl From<RTOIE> for bool {
    #[inline(always)]
    fn from(variant: RTOIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RTOIE` reader - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
pub type RTOIE_R = crate::BitReader<RTOIE>;
impl RTOIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RTOIE {
        match self.bits {
            false => RTOIE::B0x0,
            true => RTOIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RTOIE::B0x0
    }
    ///USART interrupt generated when the RTOF bit is set in the USART_ISR register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RTOIE::B0x1
    }
}
///Field `RTOIE` writer - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
pub type RTOIE_W<'a, REG> = crate::BitWriter<'a, REG, RTOIE>;
impl<'a, REG> RTOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE::B0x0)
    }
    ///USART interrupt generated when the RTOF bit is set in the USART_ISR register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE::B0x1)
    }
}
/**End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated when the EOBF flag is set in the USART_ISR register
    B0x1 = 1,
}
impl From<EOBIE> for bool {
    #[inline(always)]
    fn from(variant: EOBIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOBIE` reader - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type EOBIE_R = crate::BitReader<EOBIE>;
impl EOBIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOBIE {
        match self.bits {
            false => EOBIE::B0x0,
            true => EOBIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOBIE::B0x0
    }
    ///USART interrupt generated when the EOBF flag is set in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOBIE::B0x1
    }
}
///Field `EOBIE` writer - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
pub type EOBIE_W<'a, REG> = crate::BitWriter<'a, REG, EOBIE>;
impl<'a, REG> EOBIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE::B0x0)
    }
    ///USART interrupt generated when the EOBF flag is set in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE::B0x1)
    }
}
///Field `M1` reader - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\] = 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\] = 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\] = 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.
pub type M1_R = crate::BitReader;
///Field `M1` writer - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\] = 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\] = 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\] = 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.
pub type M1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIFOEN {
    ///0: FIFO mode is disabled.
    B0x0 = 0,
    ///1: FIFO mode is enabled.
    B0x1 = 1,
}
impl From<FIFOEN> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FIFOEN` reader - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
pub type FIFOEN_R = crate::BitReader<FIFOEN>;
impl FIFOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FIFOEN {
        match self.bits {
            false => FIFOEN::B0x0,
            true => FIFOEN::B0x1,
        }
    }
    ///FIFO mode is disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FIFOEN::B0x0
    }
    ///FIFO mode is enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FIFOEN::B0x1
    }
}
///Field `FIFOEN` writer - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
pub type FIFOEN_W<'a, REG> = crate::BitWriter<'a, REG, FIFOEN>;
impl<'a, REG> FIFOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///FIFO mode is disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN::B0x0)
    }
    ///FIFO mode is enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FIFOEN::B0x1)
    }
}
/**TXFIFO empty interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFEIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated when TXFE = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<TXFEIE> for bool {
    #[inline(always)]
    fn from(variant: TXFEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXFEIE` reader - TXFIFO empty interrupt enable This bit is set and cleared by software.
pub type TXFEIE_R = crate::BitReader<TXFEIE>;
impl TXFEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXFEIE {
        match self.bits {
            false => TXFEIE::B0x0,
            true => TXFEIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXFEIE::B0x0
    }
    ///USART interrupt generated when TXFE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXFEIE::B0x1
    }
}
///Field `TXFEIE` writer - TXFIFO empty interrupt enable This bit is set and cleared by software.
pub type TXFEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXFEIE>;
impl<'a, REG> TXFEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXFEIE::B0x0)
    }
    ///USART interrupt generated when TXFE = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXFEIE::B0x1)
    }
}
/**RXFIFO full interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFFIE {
    ///0: Interrupt inhibited
    B0x0 = 0,
    ///1: USART interrupt generated when RXFF = 1 in the USART_ISR register
    B0x1 = 1,
}
impl From<RXFFIE> for bool {
    #[inline(always)]
    fn from(variant: RXFFIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXFFIE` reader - RXFIFO full interrupt enable This bit is set and cleared by software.
pub type RXFFIE_R = crate::BitReader<RXFFIE>;
impl RXFFIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXFFIE {
        match self.bits {
            false => RXFFIE::B0x0,
            true => RXFFIE::B0x1,
        }
    }
    ///Interrupt inhibited
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXFFIE::B0x0
    }
    ///USART interrupt generated when RXFF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXFFIE::B0x1
    }
}
///Field `RXFFIE` writer - RXFIFO full interrupt enable This bit is set and cleared by software.
pub type RXFFIE_W<'a, REG> = crate::BitWriter<'a, REG, RXFFIE>;
impl<'a, REG> RXFFIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt inhibited
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXFFIE::B0x0)
    }
    ///USART interrupt generated when RXFF = 1 in the USART_ISR register
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXFFIE::B0x1)
    }
}
impl R {
    ///Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXFIFO not empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxfneie(&self) -> RXFNEIE_R {
        RXFNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXFIFO not-full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfnfie(&self) -> TXFNFIE_R {
        TXFNFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\] = 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\] = 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\] = 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFIFO full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USART_CR1")
            .field("ue", &self.ue())
            .field("uesm", &self.uesm())
            .field("re", &self.re())
            .field("te", &self.te())
            .field("idleie", &self.idleie())
            .field("rxfneie", &self.rxfneie())
            .field("tcie", &self.tcie())
            .field("txfnfie", &self.txfnfie())
            .field("peie", &self.peie())
            .field("ps", &self.ps())
            .field("pce", &self.pce())
            .field("wake", &self.wake())
            .field("m0", &self.m0())
            .field("mme", &self.mme())
            .field("cmie", &self.cmie())
            .field("over8", &self.over8())
            .field("dedt", &self.dedt())
            .field("deat", &self.deat())
            .field("rtoie", &self.rtoie())
            .field("eobie", &self.eobie())
            .field("m1", &self.m1())
            .field("fifoen", &self.fifoen())
            .field("txfeie", &self.txfeie())
            .field("rxffie", &self.rxffie())
            .finish()
    }
}
impl W {
    ///Bit 0 - USART enable When this bit is cleared, the USART prescalers and outputs are stopped immediately, and all current operations are discarded. The USART configuration is kept, but all the USART_ISR status flags are reset. This bit is set and cleared by software. Note: To enter low-power mode without generating errors on the line, the TE bit must be previously reset and the software must wait for the TC bit in the USART_ISR to be set before resetting the UE bit. Note: The DMA requests are also reset when UE = 0 so the DMA channel must be disabled before resetting the UE bit. Note: In Smartcard mode, (SCEN = 1), the CK is always available when CLKEN = 1, regardless of the UE bit value.
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<'_, USART_CR1rs> {
        UE_W::new(self, 0)
    }
    ///Bit 1 - USART enable in low-power mode When this bit is cleared, the USART cannot wake up the MCU from low-power mode. When this bit is set, the USART can wake up the MCU from low-power mode. This bit is set and cleared by software. Note: It is recommended to set the UESM bit just before entering low-power mode and clear it when exit from low-power mode. Note: If the USART does not support the wake-up from Stop feature, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn uesm(&mut self) -> UESM_W<'_, USART_CR1rs> {
        UESM_W::new(self, 1)
    }
    ///Bit 2 - Receiver enable This bit enables the receiver. It is set and cleared by software.
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, USART_CR1rs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - Transmitter enable This bit enables the transmitter. It is set and cleared by software. Note: During transmission, a low pulse on the TE bit ( 0 followed by 1 ) sends a preamble (idle line) after the current word, except in Smartcard mode. In order to generate an idle character, the TE must not be immediately written to 1 . To ensure the required duration, the software can poll the TEACK bit in the USART_ISR register. Note: In Smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission starts.
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, USART_CR1rs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<'_, USART_CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - RXFIFO not empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxfneie(&mut self) -> RXFNEIE_W<'_, USART_CR1rs> {
        RXFNEIE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, USART_CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - TXFIFO not-full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfnfie(&mut self) -> TXFNFIE_W<'_, USART_CR1rs> {
        TXFNFIE_W::new(self, 7)
    }
    ///Bit 8 - PE interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<'_, USART_CR1rs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - Parity selection This bit selects the odd or even parity when the parity generation/detection is enabled (PCE bit set). It is set and cleared by software. The parity is selected after the current byte. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, USART_CR1rs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - Parity control enable This bit selects the hardware parity control (generation and detection). When the parity control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit if M = 0) and the parity is checked on the received data. This bit is set and cleared by software. Once it is set, PCE is active after the current byte (in reception and in transmission). This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<'_, USART_CR1rs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - Receiver wake-up method This bit determines the USART wake-up method from Mute mode. It is set or cleared by software. This bitfield can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<'_, USART_CR1rs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or cleared by software (refer to bit 28 (M1)description). This bit can only be written when the USART is disabled (UE = 0).
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W<'_, USART_CR1rs> {
        M0_W::new(self, 12)
    }
    ///Bit 13 - Mute mode enable This bit enables the USART Mute mode function. When set, the USART can switch between active and Mute mode, as defined by the WAKE bit. It is set and cleared by software.
    #[inline(always)]
    pub fn mme(&mut self) -> MME_W<'_, USART_CR1rs> {
        MME_W::new(self, 13)
    }
    ///Bit 14 - Character match interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn cmie(&mut self) -> CMIE_W<'_, USART_CR1rs> {
        CMIE_W::new(self, 14)
    }
    ///Bit 15 - Oversampling mode This bit can only be written when the USART is disabled (UE = 0). Note: In LIN, IrDA and Smartcard modes, this bit must be kept cleared.
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W<'_, USART_CR1rs> {
        OVER8_W::new(self, 15)
    }
    ///Bits 16:20 - Driver enable deassertion time This 5-bit value defines the time between the end of the last stop bit, in a transmitted message, and the de-activation of the DE (Driver Enable) signal. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). If the USART_TDR register is written during the DEDT time, the new data is transmitted only when the DEDT and DEAT times have both elapsed. This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn dedt(&mut self) -> DEDT_W<'_, USART_CR1rs> {
        DEDT_W::new(self, 16)
    }
    ///Bits 21:25 - Driver enable assertion time This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and the beginning of the start bit. It is expressed in sample time units (1/8 or 1/16 bit time, depending on the oversampling rate). This bitfield can only be written when the USART is disabled (UE = 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn deat(&mut self) -> DEAT_W<'_, USART_CR1rs> {
        DEAT_W::new(self, 21)
    }
    ///Bit 26 - Receiver timeout interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn rtoie(&mut self) -> RTOIE_W<'_, USART_CR1rs> {
        RTOIE_W::new(self, 26)
    }
    ///Bit 27 - End-of-block interrupt enable This bit is set and cleared by software. Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to Section 26.4: USART implementation on page 691.
    #[inline(always)]
    pub fn eobie(&mut self) -> EOBIE_W<'_, USART_CR1rs> {
        EOBIE_W::new(self, 27)
    }
    ///Bit 28 - Word length This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or cleared by software. M\[1:0\] = 00 : 1 start bit, 8 Data bits, n Stop bit M\[1:0\] = 01 : 1 start bit, 9 Data bits, n Stop bit M\[1:0\] = 10 : 1 start bit, 7 Data bits, n Stop bit This bit can only be written when the USART is disabled (UE = 0). Note: In 7-bits data length mode, the Smartcard mode, LIN master mode and auto baud rate (0x7F and 0x55 frames detection) are not supported.
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W<'_, USART_CR1rs> {
        M1_W::new(self, 28)
    }
    ///Bit 29 - FIFO mode enable This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UE = 0). Note: FIFO mode can be used on standard UART communication, in SPI master/slave mode and in Smartcard modes only. It must not be enabled in IrDA and LIN modes.
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<'_, USART_CR1rs> {
        FIFOEN_W::new(self, 29)
    }
    ///Bit 30 - TXFIFO empty interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn txfeie(&mut self) -> TXFEIE_W<'_, USART_CR1rs> {
        TXFEIE_W::new(self, 30)
    }
    ///Bit 31 - RXFIFO full interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn rxffie(&mut self) -> RXFFIE_W<'_, USART_CR1rs> {
        RXFFIE_W::new(self, 31)
    }
}
/**USART control register 1

You can [`read`](crate::Reg::read) this register and get [`usart_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#USART1:USART_CR1)*/
pub struct USART_CR1rs;
impl crate::RegisterSpec for USART_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`usart_cr1::R`](R) reader structure
impl crate::Readable for USART_CR1rs {}
///`write(|w| ..)` method takes [`usart_cr1::W`](W) writer structure
impl crate::Writable for USART_CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets USART_CR1 to value 0
impl crate::Resettable for USART_CR1rs {}
