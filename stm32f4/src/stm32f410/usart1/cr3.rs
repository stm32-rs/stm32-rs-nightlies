#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3rs>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3rs>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE {
    #[doc = "0: Error interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Error interrupt enabled"]
    Enabled = 1,
}
impl From<EIE> for bool {
    #[inline(always)]
    fn from(variant: EIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<EIE>;
impl EIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EIE {
        match self.bits {
            false => EIE::Disabled,
            true => EIE::Enabled,
        }
    }
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE::Disabled
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE::Enabled
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG, EIE>;
impl<'a, REG> EIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Disabled)
    }
    #[doc = "Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EIE::Enabled)
    }
}
#[doc = "IrDA mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN {
    #[doc = "0: IrDA disabled"]
    Disabled = 0,
    #[doc = "1: IrDA enabled"]
    Enabled = 1,
}
impl From<IREN> for bool {
    #[inline(always)]
    fn from(variant: IREN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader<IREN>;
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IREN {
        match self.bits {
            false => IREN::Disabled,
            true => IREN::Enabled,
        }
    }
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN::Disabled
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN::Enabled
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG, IREN>;
impl<'a, REG> IREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Disabled)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IREN::Enabled)
    }
}
#[doc = "IrDA low-power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: Low-power mode"]
    LowPower = 1,
}
impl From<IRLP> for bool {
    #[inline(always)]
    fn from(variant: IRLP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader<IRLP>;
impl IRLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IRLP {
        match self.bits {
            false => IRLP::Normal,
            true => IRLP::LowPower,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP::Normal
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP::LowPower
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG, IRLP>;
impl<'a, REG> IRLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::Normal)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut crate::W<REG> {
        self.variant(IRLP::LowPower)
    }
}
#[doc = "Half-duplex selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL {
    #[doc = "0: Half duplex mode is not selected"]
    FullDuplex = 0,
    #[doc = "1: Half duplex mode is selected"]
    HalfDuplex = 1,
}
impl From<HDSEL> for bool {
    #[inline(always)]
    fn from(variant: HDSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader<HDSEL>;
impl HDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HDSEL {
        match self.bits {
            false => HDSEL::FullDuplex,
            true => HDSEL::HalfDuplex,
        }
    }
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == HDSEL::FullDuplex
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == HDSEL::HalfDuplex
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG, HDSEL>;
impl<'a, REG> HDSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::FullDuplex)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(HDSEL::HalfDuplex)
    }
}
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK {
    #[doc = "0: NACK transmission in case of parity error is disabled"]
    Disabled = 0,
    #[doc = "1: NACK transmission during parity error is enabled"]
    Enabled = 1,
}
impl From<NACK> for bool {
    #[inline(always)]
    fn from(variant: NACK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Smartcard NACK enable"]
pub type NACK_R = crate::BitReader<NACK>;
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NACK {
        match self.bits {
            false => NACK::Disabled,
            true => NACK::Enabled,
        }
    }
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACK::Disabled
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACK::Enabled
    }
}
#[doc = "Field `NACK` writer - Smartcard NACK enable"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG, NACK>;
impl<'a, REG> NACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::Disabled)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NACK::Enabled)
    }
}
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCEN {
    #[doc = "0: Smartcard mode disabled"]
    Disabled = 0,
    #[doc = "1: Smartcard mode enabled"]
    Enabled = 1,
}
impl From<SCEN> for bool {
    #[inline(always)]
    fn from(variant: SCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader<SCEN>;
impl SCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCEN {
        match self.bits {
            false => SCEN::Disabled,
            true => SCEN::Enabled,
        }
    }
    #[doc = "Smartcard mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCEN::Disabled
    }
    #[doc = "Smartcard mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCEN::Enabled
    }
}
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG, SCEN>;
impl<'a, REG> SCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smartcard mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN::Disabled)
    }
    #[doc = "Smartcard mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SCEN::Enabled)
    }
}
#[doc = "DMA enable receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR {
    #[doc = "0: DMA mode is disabled for reception"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for reception"]
    Enabled = 1,
}
impl From<DMAR> for bool {
    #[inline(always)]
    fn from(variant: DMAR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader<DMAR>;
impl DMAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAR {
        match self.bits {
            false => DMAR::Disabled,
            true => DMAR::Enabled,
        }
    }
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR::Disabled
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR::Enabled
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG, DMAR>;
impl<'a, REG> DMAR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Disabled)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAR::Enabled)
    }
}
#[doc = "DMA enable transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT {
    #[doc = "0: DMA mode is disabled for transmission"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for transmission"]
    Enabled = 1,
}
impl From<DMAT> for bool {
    #[inline(always)]
    fn from(variant: DMAT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader<DMAT>;
impl DMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAT {
        match self.bits {
            false => DMAT::Disabled,
            true => DMAT::Enabled,
        }
    }
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT::Disabled
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT::Enabled
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG, DMAT>;
impl<'a, REG> DMAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Disabled)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAT::Enabled)
    }
}
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE {
    #[doc = "0: RTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: RTS hardware flow control enabled"]
    Enabled = 1,
}
impl From<RTSE> for bool {
    #[inline(always)]
    fn from(variant: RTSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader<RTSE>;
impl RTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTSE {
        match self.bits {
            false => RTSE::Disabled,
            true => RTSE::Enabled,
        }
    }
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE::Disabled
    }
    #[doc = "RTS hardware flow control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE::Enabled
    }
}
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG, RTSE>;
impl<'a, REG> RTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Disabled)
    }
    #[doc = "RTS hardware flow control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTSE::Enabled)
    }
}
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE {
    #[doc = "0: CTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: CTS hardware flow control enabled"]
    Enabled = 1,
}
impl From<CTSE> for bool {
    #[inline(always)]
    fn from(variant: CTSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader<CTSE>;
impl CTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSE {
        match self.bits {
            false => CTSE::Disabled,
            true => CTSE::Enabled,
        }
    }
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE::Disabled
    }
    #[doc = "CTS hardware flow control enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE::Enabled
    }
}
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG, CTSE>;
impl<'a, REG> CTSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Disabled)
    }
    #[doc = "CTS hardware flow control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSE::Enabled)
    }
}
#[doc = "CTS interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE {
    #[doc = "0: CTS interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CTS interrupt enabled"]
    Enabled = 1,
}
impl From<CTSIE> for bool {
    #[inline(always)]
    fn from(variant: CTSIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<CTSIE>;
impl CTSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSIE {
        match self.bits {
            false => CTSIE::Disabled,
            true => CTSIE::Enabled,
        }
    }
    #[doc = "CTS interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE::Disabled
    }
    #[doc = "CTS interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE::Enabled
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG, CTSIE>;
impl<'a, REG> CTSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CTS interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Disabled)
    }
    #[doc = "CTS interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CTSIE::Enabled)
    }
}
#[doc = "One sample bit method enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEBIT {
    #[doc = "0: Three sample bit method"]
    Sample3 = 0,
    #[doc = "1: One sample bit method"]
    Sample1 = 1,
}
impl From<ONEBIT> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONEBIT` reader - One sample bit method enable"]
pub type ONEBIT_R = crate::BitReader<ONEBIT>;
impl ONEBIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ONEBIT {
        match self.bits {
            false => ONEBIT::Sample3,
            true => ONEBIT::Sample1,
        }
    }
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == ONEBIT::Sample3
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == ONEBIT::Sample1
    }
}
#[doc = "Field `ONEBIT` writer - One sample bit method enable"]
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG, ONEBIT>;
impl<'a, REG> ONEBIT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn sample3(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::Sample3)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn sample1(self) -> &'a mut crate::W<REG> {
        self.variant(ONEBIT::Sample1)
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<CR3rs> {
        EIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<CR3rs> {
        IREN_W::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<CR3rs> {
        IRLP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<CR3rs> {
        HDSEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<CR3rs> {
        NACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<CR3rs> {
        SCEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<CR3rs> {
        DMAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<CR3rs> {
        DMAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<CR3rs> {
        RTSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<CR3rs> {
        CTSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<CR3rs> {
        CTSIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<CR3rs> {
        ONEBIT_W::new(self, 11)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3rs;
impl crate::RegisterSpec for CR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3rs {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3rs {
    const RESET_VALUE: u32 = 0;
}
