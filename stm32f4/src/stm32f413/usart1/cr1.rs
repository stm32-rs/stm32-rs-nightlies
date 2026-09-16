///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**Send break

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBK {
    ///0: No break character is transmitted
    NoBreak = 0,
    ///1: Break character transmitted
    Break = 1,
}
impl From<SBK> for bool {
    #[inline(always)]
    fn from(variant: SBK) -> Self {
        variant as u8 != 0
    }
}
///Field `SBK` reader - Send break
pub type SBK_R = crate::BitReader<SBK>;
impl SBK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SBK {
        match self.bits {
            false => SBK::NoBreak,
            true => SBK::Break,
        }
    }
    ///No break character is transmitted
    #[inline(always)]
    pub fn is_no_break(&self) -> bool {
        *self == SBK::NoBreak
    }
    ///Break character transmitted
    #[inline(always)]
    pub fn is_break(&self) -> bool {
        *self == SBK::Break
    }
}
///Field `SBK` writer - Send break
pub type SBK_W<'a, REG> = crate::BitWriter<'a, REG, SBK>;
impl<'a, REG> SBK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No break character is transmitted
    #[inline(always)]
    pub fn no_break(self) -> &'a mut crate::W<REG> {
        self.variant(SBK::NoBreak)
    }
    ///Break character transmitted
    #[inline(always)]
    pub fn break_(self) -> &'a mut crate::W<REG> {
        self.variant(SBK::Break)
    }
}
/**Receiver wakeup

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU {
    ///0: Receiver in active mode
    Active = 0,
    ///1: Receiver in mute mode
    Mute = 1,
}
impl From<RWU> for bool {
    #[inline(always)]
    fn from(variant: RWU) -> Self {
        variant as u8 != 0
    }
}
///Field `RWU` reader - Receiver wakeup
pub type RWU_R = crate::BitReader<RWU>;
impl RWU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RWU {
        match self.bits {
            false => RWU::Active,
            true => RWU::Mute,
        }
    }
    ///Receiver in active mode
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == RWU::Active
    }
    ///Receiver in mute mode
    #[inline(always)]
    pub fn is_mute(&self) -> bool {
        *self == RWU::Mute
    }
}
///Field `RWU` writer - Receiver wakeup
pub type RWU_W<'a, REG> = crate::BitWriter<'a, REG, RWU>;
impl<'a, REG> RWU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver in active mode
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(RWU::Active)
    }
    ///Receiver in mute mode
    #[inline(always)]
    pub fn mute(self) -> &'a mut crate::W<REG> {
        self.variant(RWU::Mute)
    }
}
/**Receiver enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE {
    ///0: Receiver disabled
    Disabled = 0,
    ///1: Receiver enabled
    Enabled = 1,
}
impl From<RE> for bool {
    #[inline(always)]
    fn from(variant: RE) -> Self {
        variant as u8 != 0
    }
}
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader<RE>;
impl RE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RE {
        match self.bits {
            false => RE::Disabled,
            true => RE::Enabled,
        }
    }
    ///Receiver disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE::Disabled
    }
    ///Receiver enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE::Enabled
    }
}
///Field `RE` writer - Receiver enable
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG, RE>;
impl<'a, REG> RE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Receiver disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Disabled)
    }
    ///Receiver enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RE::Enabled)
    }
}
/**Transmitter enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE {
    ///0: Transmitter disabled
    Disabled = 0,
    ///1: Transmitter enabled
    Enabled = 1,
}
impl From<TE> for bool {
    #[inline(always)]
    fn from(variant: TE) -> Self {
        variant as u8 != 0
    }
}
///Field `TE` reader - Transmitter enable
pub type TE_R = crate::BitReader<TE>;
impl TE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TE {
        match self.bits {
            false => TE::Disabled,
            true => TE::Enabled,
        }
    }
    ///Transmitter disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE::Disabled
    }
    ///Transmitter enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE::Enabled
    }
}
///Field `TE` writer - Transmitter enable
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG, TE>;
impl<'a, REG> TE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transmitter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Disabled)
    }
    ///Transmitter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TE::Enabled)
    }
}
/**IDLE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE {
    ///0: IDLE interrupt disabled
    Disabled = 0,
    ///1: IDLE interrupt enabled
    Enabled = 1,
}
impl From<IDLEIE> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDLEIE` reader - IDLE interrupt enable
pub type IDLEIE_R = crate::BitReader<IDLEIE>;
impl IDLEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDLEIE {
        match self.bits {
            false => IDLEIE::Disabled,
            true => IDLEIE::Enabled,
        }
    }
    ///IDLE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE::Disabled
    }
    ///IDLE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE::Enabled
    }
}
///Field `IDLEIE` writer - IDLE interrupt enable
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG, IDLEIE>;
impl<'a, REG> IDLEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///IDLE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Disabled)
    }
    ///IDLE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDLEIE::Enabled)
    }
}
/**RXNE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    ///0: RXNE interrupt disabled
    Disabled = 0,
    ///1: RXNE interrupt enabled
    Enabled = 1,
}
impl From<RXNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNEIE` reader - RXNE interrupt enable
pub type RXNEIE_R = crate::BitReader<RXNEIE>;
impl RXNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE {
        match self.bits {
            false => RXNEIE::Disabled,
            true => RXNEIE::Enabled,
        }
    }
    ///RXNE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE::Disabled
    }
    ///RXNE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIE::Enabled
    }
}
///Field `RXNEIE` writer - RXNE interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RXNE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Disabled)
    }
    ///RXNE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Enabled)
    }
}
/**Transmission complete interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: TC interrupt disabled
    Disabled = 0,
    ///1: TC interrupt enabled
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transmission complete interrupt enable
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    ///TC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
///Field `TCIE` writer - Transmission complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
/**TXE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    ///0: TXE interrupt disabled
    Disabled = 0,
    ///1: TXE interrupt enabled
    Enabled = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXEIE` reader - TXE interrupt enable
pub type TXEIE_R = crate::BitReader<TXEIE>;
impl TXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE {
        match self.bits {
            false => TXEIE::Disabled,
            true => TXEIE::Enabled,
        }
    }
    ///TXE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE::Disabled
    }
    ///TXE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE::Enabled
    }
}
///Field `TXEIE` writer - TXE interrupt enable
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TXE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Disabled)
    }
    ///TXE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Enabled)
    }
}
/**PE interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE {
    ///0: PE interrupt disabled
    Disabled = 0,
    ///1: PE interrupt enabled
    Enabled = 1,
}
impl From<PEIE> for bool {
    #[inline(always)]
    fn from(variant: PEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PEIE` reader - PE interrupt enable
pub type PEIE_R = crate::BitReader<PEIE>;
impl PEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PEIE {
        match self.bits {
            false => PEIE::Disabled,
            true => PEIE::Enabled,
        }
    }
    ///PE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE::Disabled
    }
    ///PE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE::Enabled
    }
}
///Field `PEIE` writer - PE interrupt enable
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG, PEIE>;
impl<'a, REG> PEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::Disabled)
    }
    ///PE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEIE::Enabled)
    }
}
/**Parity selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS {
    ///0: Even parity
    Even = 0,
    ///1: Odd parity
    Odd = 1,
}
impl From<PS> for bool {
    #[inline(always)]
    fn from(variant: PS) -> Self {
        variant as u8 != 0
    }
}
///Field `PS` reader - Parity selection
pub type PS_R = crate::BitReader<PS>;
impl PS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PS {
        match self.bits {
            false => PS::Even,
            true => PS::Odd,
        }
    }
    ///Even parity
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS::Even
    }
    ///Odd parity
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS::Odd
    }
}
///Field `PS` writer - Parity selection
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG, PS>;
impl<'a, REG> PS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Even parity
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(PS::Even)
    }
    ///Odd parity
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(PS::Odd)
    }
}
/**Parity control enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE {
    ///0: Parity control disabled
    Disabled = 0,
    ///1: Parity control enabled
    Enabled = 1,
}
impl From<PCE> for bool {
    #[inline(always)]
    fn from(variant: PCE) -> Self {
        variant as u8 != 0
    }
}
///Field `PCE` reader - Parity control enable
pub type PCE_R = crate::BitReader<PCE>;
impl PCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCE {
        match self.bits {
            false => PCE::Disabled,
            true => PCE::Enabled,
        }
    }
    ///Parity control disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCE::Disabled
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCE::Enabled
    }
}
///Field `PCE` writer - Parity control enable
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG, PCE>;
impl<'a, REG> PCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Parity control disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::Disabled)
    }
    ///Parity control enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PCE::Enabled)
    }
}
/**Wakeup method

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE {
    ///0: USART wakeup on idle line
    IdleLine = 0,
    ///1: USART wakeup on address mark
    AddressMark = 1,
}
impl From<WAKE> for bool {
    #[inline(always)]
    fn from(variant: WAKE) -> Self {
        variant as u8 != 0
    }
}
///Field `WAKE` reader - Wakeup method
pub type WAKE_R = crate::BitReader<WAKE>;
impl WAKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WAKE {
        match self.bits {
            false => WAKE::IdleLine,
            true => WAKE::AddressMark,
        }
    }
    ///USART wakeup on idle line
    #[inline(always)]
    pub fn is_idle_line(&self) -> bool {
        *self == WAKE::IdleLine
    }
    ///USART wakeup on address mark
    #[inline(always)]
    pub fn is_address_mark(&self) -> bool {
        *self == WAKE::AddressMark
    }
}
///Field `WAKE` writer - Wakeup method
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG, WAKE>;
impl<'a, REG> WAKE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART wakeup on idle line
    #[inline(always)]
    pub fn idle_line(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::IdleLine)
    }
    ///USART wakeup on address mark
    #[inline(always)]
    pub fn address_mark(self) -> &'a mut crate::W<REG> {
        self.variant(WAKE::AddressMark)
    }
}
/**Word length

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M {
    ///0: 8 data bits
    M8 = 0,
    ///1: 9 data bits
    M9 = 1,
}
impl From<M> for bool {
    #[inline(always)]
    fn from(variant: M) -> Self {
        variant as u8 != 0
    }
}
///Field `M` reader - Word length
pub type M_R = crate::BitReader<M>;
impl M_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> M {
        match self.bits {
            false => M::M8,
            true => M::M9,
        }
    }
    ///8 data bits
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == M::M8
    }
    ///9 data bits
    #[inline(always)]
    pub fn is_m9(&self) -> bool {
        *self == M::M9
    }
}
///Field `M` writer - Word length
pub type M_W<'a, REG> = crate::BitWriter<'a, REG, M>;
impl<'a, REG> M_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///8 data bits
    #[inline(always)]
    pub fn m8(self) -> &'a mut crate::W<REG> {
        self.variant(M::M8)
    }
    ///9 data bits
    #[inline(always)]
    pub fn m9(self) -> &'a mut crate::W<REG> {
        self.variant(M::M9)
    }
}
/**USART enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE {
    ///0: USART prescaler and outputs disabled
    Disabled = 0,
    ///1: USART enabled
    Enabled = 1,
}
impl From<UE> for bool {
    #[inline(always)]
    fn from(variant: UE) -> Self {
        variant as u8 != 0
    }
}
///Field `UE` reader - USART enable
pub type UE_R = crate::BitReader<UE>;
impl UE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UE {
        match self.bits {
            false => UE::Disabled,
            true => UE::Enabled,
        }
    }
    ///USART prescaler and outputs disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE::Disabled
    }
    ///USART enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UE::Enabled
    }
}
///Field `UE` writer - USART enable
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG, UE>;
impl<'a, REG> UE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///USART prescaler and outputs disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Disabled)
    }
    ///USART enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UE::Enabled)
    }
}
/**Oversampling mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER8 {
    ///0: Oversampling by 16
    Oversample16 = 0,
    ///1: Oversampling by 8
    Oversample8 = 1,
}
impl From<OVER8> for bool {
    #[inline(always)]
    fn from(variant: OVER8) -> Self {
        variant as u8 != 0
    }
}
///Field `OVER8` reader - Oversampling mode
pub type OVER8_R = crate::BitReader<OVER8>;
impl OVER8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVER8 {
        match self.bits {
            false => OVER8::Oversample16,
            true => OVER8::Oversample8,
        }
    }
    ///Oversampling by 16
    #[inline(always)]
    pub fn is_oversample16(&self) -> bool {
        *self == OVER8::Oversample16
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn is_oversample8(&self) -> bool {
        *self == OVER8::Oversample8
    }
}
///Field `OVER8` writer - Oversampling mode
pub type OVER8_W<'a, REG> = crate::BitWriter<'a, REG, OVER8>;
impl<'a, REG> OVER8_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Oversampling by 16
    #[inline(always)]
    pub fn oversample16(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::Oversample16)
    }
    ///Oversampling by 8
    #[inline(always)]
    pub fn oversample8(self) -> &'a mut crate::W<REG> {
        self.variant(OVER8::Oversample8)
    }
}
impl R {
    ///Bit 0 - Send break
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Receiver wakeup
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TXE interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("over8", &self.over8())
            .field("ue", &self.ue())
            .field("m", &self.m())
            .field("wake", &self.wake())
            .field("pce", &self.pce())
            .field("ps", &self.ps())
            .field("peie", &self.peie())
            .field("txeie", &self.txeie())
            .field("tcie", &self.tcie())
            .field("rxneie", &self.rxneie())
            .field("idleie", &self.idleie())
            .field("te", &self.te())
            .field("re", &self.re())
            .field("rwu", &self.rwu())
            .field("sbk", &self.sbk())
            .finish()
    }
}
impl W {
    ///Bit 0 - Send break
    #[inline(always)]
    pub fn sbk(&mut self) -> SBK_W<'_, CR1rs> {
        SBK_W::new(self, 0)
    }
    ///Bit 1 - Receiver wakeup
    #[inline(always)]
    pub fn rwu(&mut self) -> RWU_W<'_, CR1rs> {
        RWU_W::new(self, 1)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&mut self) -> RE_W<'_, CR1rs> {
        RE_W::new(self, 2)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&mut self) -> TE_W<'_, CR1rs> {
        TE_W::new(self, 3)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W<'_, CR1rs> {
        IDLEIE_W::new(self, 4)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, CR1rs> {
        RXNEIE_W::new(self, 5)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - TXE interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<'_, CR1rs> {
        TXEIE_W::new(self, 7)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&mut self) -> PEIE_W<'_, CR1rs> {
        PEIE_W::new(self, 8)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<'_, CR1rs> {
        PS_W::new(self, 9)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&mut self) -> PCE_W<'_, CR1rs> {
        PCE_W::new(self, 10)
    }
    ///Bit 11 - Wakeup method
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W<'_, CR1rs> {
        WAKE_W::new(self, 11)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m(&mut self) -> M_W<'_, CR1rs> {
        M_W::new(self, 12)
    }
    ///Bit 13 - USART enable
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W<'_, CR1rs> {
        UE_W::new(self, 13)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&mut self) -> OVER8_W<'_, CR1rs> {
        OVER8_W::new(self, 15)
    }
}
/**Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#USART1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u16;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
