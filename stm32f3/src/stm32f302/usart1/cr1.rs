#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE {
    #[doc = "0: UART is disabled"]
    Disabled = 0,
    #[doc = "1: UART is enabled"]
    Enabled = 1,
}
impl From<UE> for bool {
    #[inline(always)]
    fn from(variant: UE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UE` reader - USART enable"]
pub type UE_R = crate::BitReader<UE>;
impl UE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UE {
        match self.bits {
            false => UE::Disabled,
            true => UE::Enabled,
        }
    }
    #[doc = "UART is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE::Disabled
    }
    #[doc = "UART is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UE::Enabled
    }
}
#[doc = "Field `UE` writer - USART enable"]
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG, UE>;
impl<'a, REG> UE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UART is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Disabled)
    }
    #[doc = "UART is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Enabled)
    }
}
#[doc = "USART enable in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UESM {
    #[doc = "0: USART not able to wake up the MCU from Stop mode"]
    Disabled = 0,
    #[doc = "1: USART able to wake up the MCU from Stop mode"]
    Enabled = 1,
}
impl From<UESM> for bool {
    #[inline(always)]
    fn from(variant: UESM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UESM` reader - USART enable in Stop mode"]
pub type UESM_R = crate::BitReader<UESM>;
impl UESM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UESM {
        match self.bits {
            false => UESM::Disabled,
            true => UESM::Enabled,
        }
    }
    #[doc = "USART not able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UESM::Disabled
    }
    #[doc = "USART able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UESM::Enabled
    }
}
#[doc = "Field `UESM` writer - USART enable in Stop mode"]
pub type UESM_W<'a, REG> = crate::BitWriter<'a, REG, UESM>;
impl<'a, REG> UESM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART not able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UESM::Disabled)
    }
    #[doc = "USART able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UESM::Enabled)
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    #[doc = "0: Receiver is disabled"]
    Disabled = 0,
    #[doc = "1: Receiver is enabled"]
    Enabled = 1,
}
impl From<RE> for bool {
    #[inline(always)]
    fn from(variant: RE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<RE>;
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RE {
        match self.bits {
            false => RE::Disabled,
            true => RE::Enabled,
        }
    }
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE::Disabled
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE::Enabled
    }
}
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Disabled)
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Enabled)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE {
    #[doc = "0: Transmitter is disabled"]
    Disabled = 0,
    #[doc = "1: Transmitter is enabled"]
    Enabled = 1,
}
impl From<TE> for bool {
    #[inline(always)]
    fn from(variant: TE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<TE>;
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TE {
        match self.bits {
            false => TE::Disabled,
            true => TE::Enabled,
        }
    }
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE::Disabled
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE::Enabled
    }
}
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Disabled)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Enabled)
    }
}
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever IDLE=1 in the ISR register"]
    Enabled = 1,
}
impl From<IDLEIE> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IDLEIE_R = crate::BitReader<IDLEIE>;
impl IDLEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IDLEIE {
        match self.bits {
            false => IDLEIE::Disabled,
            true => IDLEIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE::Disabled
    }
    #[doc = "Interrupt is generated whenever IDLE=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE::Enabled
    }
}
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG, IDLEIE>;
impl<'a, REG> IDLEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Disabled)
    }
    #[doc = "Interrupt is generated whenever IDLE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Enabled)
    }
}
#[doc = "RXNE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register"]
    Enabled = 1,
}
impl From<RXNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE>;
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE {
        match self.bits {
            false => RXNEIE::Disabled,
            true => RXNEIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE::Disabled
    }
    #[doc = "Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIE::Enabled
    }
}
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Disabled)
    }
    #[doc = "Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Enabled)
    }
}
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever TC=1 in the ISR register"]
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "Interrupt is generated whenever TC=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "Interrupt is generated whenever TC=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever TXE=1 in the ISR register"]
    Enabled = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE>;
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE {
        match self.bits {
            false => TXEIE::Disabled,
            true => TXEIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE::Disabled
    }
    #[doc = "Interrupt is generated whenever TXE=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE::Enabled
    }
}
#[doc = "Field `TXEIE` writer - interrupt enable"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Disabled)
    }
    #[doc = "Interrupt is generated whenever TXE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Enabled)
    }
}
#[doc = "PE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated whenever PE=1 in the ISR register"]
    Enabled = 1,
}
impl From<PEIE> for bool {
    #[inline(always)]
    fn from(variant: PEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PEIE_R = crate::BitReader<PEIE>;
impl PEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEIE {
        match self.bits {
            false => PEIE::Disabled,
            true => PEIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE::Disabled
    }
    #[doc = "Interrupt is generated whenever PE=1 in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE::Enabled
    }
}
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG, PEIE>;
impl<'a, REG> PEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::Disabled)
    }
    #[doc = "Interrupt is generated whenever PE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::Enabled)
    }
}
#[doc = "Parity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<PS> for bool {
    #[inline(always)]
    fn from(variant: PS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Parity selection"]
pub type PS_R = crate::BitReader<PS>;
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PS {
        match self.bits {
            false => PS::Even,
            true => PS::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS::Odd
    }
}
#[doc = "Field `PS` writer - Parity selection"]
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG, PS>;
impl<'a, REG> PS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PS::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PS::Odd)
    }
}
#[doc = "Parity control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE {
    #[doc = "0: Parity control disabled"]
    Disabled = 0,
    #[doc = "1: Parity control enabled"]
    Enabled = 1,
}
impl From<PCE> for bool {
    #[inline(always)]
    fn from(variant: PCE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PCE_R = crate::BitReader<PCE>;
impl PCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PCE {
        match self.bits {
            false => PCE::Disabled,
            true => PCE::Enabled,
        }
    }
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCE::Disabled
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCE::Enabled
    }
}
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG, PCE>;
impl<'a, REG> PCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::Disabled)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::Enabled)
    }
}
#[doc = "Receiver wakeup method\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE {
    #[doc = "0: Idle line"]
    Idle = 0,
    #[doc = "1: Address mask"]
    Address = 1,
}
impl From<WAKE> for bool {
    #[inline(always)]
    fn from(variant: WAKE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` reader - Receiver wakeup method"]
pub type WAKE_R = crate::BitReader<WAKE>;
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAKE {
        match self.bits {
            false => WAKE::Idle,
            true => WAKE::Address,
        }
    }
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WAKE::Idle
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WAKE::Address
    }
}
#[doc = "Field `WAKE` writer - Receiver wakeup method"]
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG, WAKE>;
impl<'a, REG> WAKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::Idle)
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn address(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::Address)
    }
}
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M {
    #[doc = "0: 1 start bit, 8 data bits, n stop bits"]
    Bit8 = 0,
    #[doc = "1: 1 start bit, 9 data bits, n stop bits"]
    Bit9 = 1,
}
impl From<M> for bool {
    #[inline(always)]
    fn from(variant: M) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M` reader - Word length"]
pub type M_R = crate::BitReader<M>;
impl M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M {
        match self.bits {
            false => M::Bit8,
            true => M::Bit9,
        }
    }
    #[doc = "1 start bit, 8 data bits, n stop bits"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == M::Bit8
    }
    #[doc = "1 start bit, 9 data bits, n stop bits"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == M::Bit9
    }
}
#[doc = "Field `M` writer - Word length"]
pub type M_W<'a, REG> = crate::BitWriter<'a, REG, M>;
impl<'a, REG> M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 start bit, 8 data bits, n stop bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(M::Bit8)
    }
    #[doc = "1 start bit, 9 data bits, n stop bits"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut crate::W<REG> {
        self.variant(M::Bit9)
    }
}
#[doc = "Mute mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MME {
    #[doc = "0: Receiver in active mode permanently"]
    Disabled = 0,
    #[doc = "1: Receiver can switch between mute mode and active mode"]
    Enabled = 1,
}
impl From<MME> for bool {
    #[inline(always)]
    fn from(variant: MME) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MME` reader - Mute mode enable"]
pub type MME_R = crate::BitReader<MME>;
impl MME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MME {
        match self.bits {
            false => MME::Disabled,
            true => MME::Enabled,
        }
    }
    #[doc = "Receiver in active mode permanently"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MME::Disabled
    }
    #[doc = "Receiver can switch between mute mode and active mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MME::Enabled
    }
}
#[doc = "Field `MME` writer - Mute mode enable"]
pub type MME_W<'a, REG> = crate::BitWriter<'a, REG, MME>;
impl<'a, REG> MME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver in active mode permanently"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MME::Disabled)
    }
    #[doc = "Receiver can switch between mute mode and active mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MME::Enabled)
    }
}
#[doc = "Character match interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMIE {
    #[doc = "0: Interrupt is disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt is generated when the CMF bit is set in the ISR register"]
    Enabled = 1,
}
impl From<CMIE> for bool {
    #[inline(always)]
    fn from(variant: CMIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMIE` reader - Character match interrupt enable"]
pub type CMIE_R = crate::BitReader<CMIE>;
impl CMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CMIE {
        match self.bits {
            false => CMIE::Disabled,
            true => CMIE::Enabled,
        }
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMIE::Disabled
    }
    #[doc = "Interrupt is generated when the CMF bit is set in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMIE::Enabled
    }
}
#[doc = "Field `CMIE` writer - Character match interrupt enable"]
pub type CMIE_W<'a, REG> = crate::BitWriter<'a, REG, CMIE>;
impl<'a, REG> CMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE::Disabled)
    }
    #[doc = "Interrupt is generated when the CMF bit is set in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CMIE::Enabled)
    }
}
#[doc = "Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER8 {
    #[doc = "0: Oversampling by 16"]
    Oversampling16 = 0,
    #[doc = "1: Oversampling by 8"]
    Oversampling8 = 1,
}
impl From<OVER8> for bool {
    #[inline(always)]
    fn from(variant: OVER8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVER8` reader - Oversampling mode"]
pub type OVER8_R = crate::BitReader<OVER8>;
impl OVER8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVER8 {
        match self.bits {
            false => OVER8::Oversampling16,
            true => OVER8::Oversampling8,
        }
    }
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn is_oversampling16(&self) -> bool {
        *self == OVER8::Oversampling16
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn is_oversampling8(&self) -> bool {
        *self == OVER8::Oversampling8
    }
}
#[doc = "Field `OVER8` writer - Oversampling mode"]
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG, OVER8>;
impl<'a, REG> OVER8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn oversampling16(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::Oversampling16)
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn oversampling8(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::Oversampling8)
    }
}
#[doc = "Field `DEDT` reader - Driver Enable deassertion time"]
pub type DEDT_R = crate::FieldReader;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time"]
pub type DEDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time"]
pub type DEAT_R = crate::FieldReader;
#[doc = "Field `DEAT` writer - Driver Enable assertion time"]
pub type DEAT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Receiver timeout interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOIE {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An USART interrupt is generated when the RTOF bit is set in the ISR register"]
    Enabled = 1,
}
impl From<RTOIE> for bool {
    #[inline(always)]
    fn from(variant: RTOIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTOIE` reader - Receiver timeout interrupt enable"]
pub type RTOIE_R = crate::BitReader<RTOIE>;
impl RTOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTOIE {
        match self.bits {
            false => RTOIE::Disabled,
            true => RTOIE::Enabled,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOIE::Disabled
    }
    #[doc = "An USART interrupt is generated when the RTOF bit is set in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOIE::Enabled
    }
}
#[doc = "Field `RTOIE` writer - Receiver timeout interrupt enable"]
pub type RTOIE_W<'a, REG> = crate::BitWriter<'a, REG, RTOIE>;
impl<'a, REG> RTOIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE::Disabled)
    }
    #[doc = "An USART interrupt is generated when the RTOF bit is set in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTOIE::Enabled)
    }
}
#[doc = "End of Block interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBIE {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: A USART interrupt is generated when the EOBF flag is set in the ISR register"]
    Enabled = 1,
}
impl From<EOBIE> for bool {
    #[inline(always)]
    fn from(variant: EOBIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOBIE` reader - End of Block interrupt enable"]
pub type EOBIE_R = crate::BitReader<EOBIE>;
impl EOBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EOBIE {
        match self.bits {
            false => EOBIE::Disabled,
            true => EOBIE::Enabled,
        }
    }
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOBIE::Disabled
    }
    #[doc = "A USART interrupt is generated when the EOBF flag is set in the ISR register"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOBIE::Enabled
    }
}
#[doc = "Field `EOBIE` writer - End of Block interrupt enable"]
pub type EOBIE_W<'a, REG> = crate::BitWriter<'a, REG, EOBIE>;
impl<'a, REG> EOBIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE::Disabled)
    }
    #[doc = "A USART interrupt is generated when the EOBF flag is set in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EOBIE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<CR1rs> {
        UE_W::new(self, 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<CR1rs> {
        UESM_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<CR1rs> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<CR1rs> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR1rs> {
        RXNEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CR1rs> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<CR1rs> {
        TXEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<CR1rs> {
        PEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<CR1rs> {
        PS_W::new(self, 9)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<CR1rs> {
        PCE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<CR1rs> {
        WAKE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<CR1rs> {
        M_W::new(self, 12)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<CR1rs> {
        MME_W::new(self, 13)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<CR1rs> {
        CMIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> OVER8_W<CR1rs> {
        OVER8_W::new(self, 15)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DEDT_W<CR1rs> {
        DEDT_W::new(self, 16)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DEAT_W<CR1rs> {
        DEAT_W::new(self, 21)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<CR1rs> {
        RTOIE_W::new(self, 26)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EOBIE_W<CR1rs> {
        EOBIE_W::new(self, 27)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
