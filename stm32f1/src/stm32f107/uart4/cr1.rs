#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Send break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBK {
    #[doc = "0: No break character is transmitted"]
    NoBreak = 0,
    #[doc = "1: Break character transmitted"]
    Break = 1,
}
impl From<SBK> for bool {
    #[inline(always)]
    fn from(variant: SBK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBK` reader - Send break"]
pub type SBK_R = crate::BitReader<SBK>;
impl SBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SBK {
        match self.bits {
            false => SBK::NoBreak,
            true => SBK::Break,
        }
    }
    #[doc = "No break character is transmitted"]
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == SBK::NoBreak
    }
    #[doc = "Break character transmitted"]
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBK::Break
    }
}
#[doc = "Field `SBK` writer - Send break"]
pub type SBK_W<'a, REG> = crate::BitWriter<'a, REG, SBK>;
impl<'a, REG> SBK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No break character is transmitted"]
    #[inline(always)]
    pub fn no_break(self) -> &'a mut crate::W<REG> {
        self.variant(SBK::NoBreak)
    }
    #[doc = "Break character transmitted"]
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(SBK::Break)
    }
}
#[doc = "Receiver wakeup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU {
    #[doc = "0: Receiver in active mode"]
    Active = 0,
    #[doc = "1: Receiver in mute mode"]
    Mute = 1,
}
impl From<RWU> for bool {
    #[inline(always)]
    fn from(variant: RWU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWU` reader - Receiver wakeup"]
pub type RWU_R = crate::BitReader<RWU>;
impl RWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RWU {
        match self.bits {
            false => RWU::Active,
            true => RWU::Mute,
        }
    }
    #[doc = "Receiver in active mode"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RWU::Active
    }
    #[doc = "Receiver in mute mode"]
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == RWU::Mute
    }
}
#[doc = "Field `RWU` writer - Receiver wakeup"]
pub type RWU_W<'a, REG> = crate::BitWriter<'a, REG, RWU>;
impl<'a, REG> RWU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver in active mode"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RWU::Active)
    }
    #[doc = "Receiver in mute mode"]
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(RWU::Mute)
    }
}
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    #[doc = "0: Receiver disabled"]
    Disabled = 0,
    #[doc = "1: Receiver enabled"]
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
    #[doc = "Receiver disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE::Disabled
    }
    #[doc = "Receiver enabled"]
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
    #[doc = "Receiver disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Disabled)
    }
    #[doc = "Receiver enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Enabled)
    }
}
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE {
    #[doc = "0: Transmitter disabled"]
    Disabled = 0,
    #[doc = "1: Transmitter enabled"]
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
    #[doc = "Transmitter disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE::Disabled
    }
    #[doc = "Transmitter enabled"]
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
    #[doc = "Transmitter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Disabled)
    }
    #[doc = "Transmitter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Enabled)
    }
}
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE {
    #[doc = "0: IDLE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: IDLE interrupt enabled"]
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
    #[doc = "IDLE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE::Disabled
    }
    #[doc = "IDLE interrupt enabled"]
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
    #[doc = "IDLE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Disabled)
    }
    #[doc = "IDLE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Enabled)
    }
}
#[doc = "RXNE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    #[doc = "0: RXNE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: RXNE interrupt enabled"]
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
    #[doc = "RXNE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE::Disabled
    }
    #[doc = "RXNE interrupt enabled"]
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
    #[doc = "RXNE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Disabled)
    }
    #[doc = "RXNE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Enabled)
    }
}
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: TC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TC interrupt enabled"]
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
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "TC interrupt enabled"]
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
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "TXE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    #[doc = "0: TXE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TXE interrupt enabled"]
    Enabled = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - TXE interrupt enable"]
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
    #[doc = "TXE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE::Disabled
    }
    #[doc = "TXE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE::Enabled
    }
}
#[doc = "Field `TXEIE` writer - TXE interrupt enable"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Disabled)
    }
    #[doc = "TXE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Enabled)
    }
}
#[doc = "PE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE {
    #[doc = "0: PE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: PE interrupt enabled"]
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
    #[doc = "PE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE::Disabled
    }
    #[doc = "PE interrupt enabled"]
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
    #[doc = "PE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::Disabled)
    }
    #[doc = "PE interrupt enabled"]
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
#[doc = "Wakeup method\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE {
    #[doc = "0: USART wakeup on idle line"]
    IdleLine = 0,
    #[doc = "1: USART wakeup on address mark"]
    AddressMark = 1,
}
impl From<WAKE> for bool {
    #[inline(always)]
    fn from(variant: WAKE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` reader - Wakeup method"]
pub type WAKE_R = crate::BitReader<WAKE>;
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAKE {
        match self.bits {
            false => WAKE::IdleLine,
            true => WAKE::AddressMark,
        }
    }
    #[doc = "USART wakeup on idle line"]
    #[inline(always)]
    pub fn is_idle_line(&self) -> bool {
        *self == WAKE::IdleLine
    }
    #[doc = "USART wakeup on address mark"]
    #[inline(always)]
    pub fn is_address_mark(&self) -> bool {
        *self == WAKE::AddressMark
    }
}
#[doc = "Field `WAKE` writer - Wakeup method"]
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG, WAKE>;
impl<'a, REG> WAKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART wakeup on idle line"]
    #[inline(always)]
    pub fn idle_line(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::IdleLine)
    }
    #[doc = "USART wakeup on address mark"]
    #[inline(always)]
    pub fn address_mark(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::AddressMark)
    }
}
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M {
    #[doc = "0: 8 data bits"]
    M8 = 0,
    #[doc = "1: 9 data bits"]
    M9 = 1,
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
            false => M::M8,
            true => M::M9,
        }
    }
    #[doc = "8 data bits"]
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == M::M8
    }
    #[doc = "9 data bits"]
    #[inline(always)]
    pub fn is_m9(&self) -> bool {
        *self == M::M9
    }
}
#[doc = "Field `M` writer - Word length"]
pub type M_W<'a, REG> = crate::BitWriter<'a, REG, M>;
impl<'a, REG> M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8 data bits"]
    #[inline(always)]
    pub fn m8(self) -> &'a mut crate::W<REG> {
        self.variant(M::M8)
    }
    #[doc = "9 data bits"]
    #[inline(always)]
    pub fn m9(self) -> &'a mut crate::W<REG> {
        self.variant(M::M9)
    }
}
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE {
    #[doc = "0: USART prescaler and outputs disabled"]
    Disabled = 0,
    #[doc = "1: USART enabled"]
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
    #[doc = "USART prescaler and outputs disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE::Disabled
    }
    #[doc = "USART enabled"]
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
    #[doc = "USART prescaler and outputs disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Disabled)
    }
    #[doc = "USART enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 7 - TXE interrupt enable"]
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
    #[doc = "Bit 11 - Wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SBK_W<CR1rs> {
        SBK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<CR1rs> {
        RWU_W::new(self, 1)
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
    #[doc = "Bit 7 - TXE interrupt enable"]
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
    #[doc = "Bit 11 - Wakeup method"]
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
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<CR1rs> {
        UE_W::new(self, 13)
    }
}
#[doc = "UART4 CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
