#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Wakeup clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUCKSEL {
    #[doc = "0: RTC/16 clock is selected"]
    Div16 = 0,
    #[doc = "1: RTC/8 clock is selected"]
    Div8 = 1,
    #[doc = "2: RTC/4 clock is selected"]
    Div4 = 2,
    #[doc = "3: RTC/2 clock is selected"]
    Div2 = 3,
    #[doc = "4: ck_spre (usually 1 Hz) clock is selected"]
    ClockSpare = 4,
    #[doc = "6: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    ClockSpareWithOffset = 6,
}
impl From<WUCKSEL> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUCKSEL {
    type Ux = u8;
}
#[doc = "Field `WUCKSEL` reader - Wakeup clock selection"]
pub type WUCKSEL_R = crate::FieldReader<WUCKSEL>;
impl WUCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WUCKSEL> {
        match self.bits {
            0 => Some(WUCKSEL::Div16),
            1 => Some(WUCKSEL::Div8),
            2 => Some(WUCKSEL::Div4),
            3 => Some(WUCKSEL::Div2),
            4 => Some(WUCKSEL::ClockSpare),
            6 => Some(WUCKSEL::ClockSpareWithOffset),
            _ => None,
        }
    }
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == WUCKSEL::Div16
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WUCKSEL::Div8
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WUCKSEL::Div4
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WUCKSEL::Div2
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected"]
    #[inline(always)]
    pub fn is_clock_spare(&self) -> bool {
        *self == WUCKSEL::ClockSpare
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    #[inline(always)]
    pub fn is_clock_spare_with_offset(&self) -> bool {
        *self == WUCKSEL::ClockSpareWithOffset
    }
}
#[doc = "Field `WUCKSEL` writer - Wakeup clock selection"]
pub type WUCKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, WUCKSEL>;
impl<'a, REG> WUCKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div16)
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div8)
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div4)
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::Div2)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected"]
    #[inline(always)]
    pub fn clock_spare(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::ClockSpare)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    #[inline(always)]
    pub fn clock_spare_with_offset(self) -> &'a mut crate::W<REG> {
        self.variant(WUCKSEL::ClockSpareWithOffset)
    }
}
#[doc = "Timestamp event active edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDGE {
    #[doc = "0: RTC_TS input rising edge generates a time-stamp event"]
    RisingEdge = 0,
    #[doc = "1: RTC_TS input falling edge generates a time-stamp event"]
    FallingEdge = 1,
}
impl From<TSEDGE> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSEDGE` reader - Timestamp event active edge"]
pub type TSEDGE_R = crate::BitReader<TSEDGE>;
impl TSEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSEDGE {
        match self.bits {
            false => TSEDGE::RisingEdge,
            true => TSEDGE::FallingEdge,
        }
    }
    #[doc = "RTC_TS input rising edge generates a time-stamp event"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TSEDGE::RisingEdge
    }
    #[doc = "RTC_TS input falling edge generates a time-stamp event"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TSEDGE::FallingEdge
    }
}
#[doc = "Field `TSEDGE` writer - Timestamp event active edge"]
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG, TSEDGE>;
impl<'a, REG> TSEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_TS input rising edge generates a time-stamp event"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::RisingEdge)
    }
    #[doc = "RTC_TS input falling edge generates a time-stamp event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::FallingEdge)
    }
}
#[doc = "RTC_REFIN reference clock detection enable (50 or 60 Hz)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCKON {
    #[doc = "0: RTC_REFIN detection disabled"]
    Disabled = 0,
    #[doc = "1: RTC_REFIN detection enabled"]
    Enabled = 1,
}
impl From<REFCKON> for bool {
    #[inline(always)]
    fn from(variant: REFCKON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFCKON_R = crate::BitReader<REFCKON>;
impl REFCKON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REFCKON {
        match self.bits {
            false => REFCKON::Disabled,
            true => REFCKON::Enabled,
        }
    }
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REFCKON::Disabled
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REFCKON::Enabled
    }
}
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG, REFCKON>;
impl<'a, REG> REFCKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::Disabled)
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::Enabled)
    }
}
#[doc = "Bypass the shadow registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPSHAD {
    #[doc = "0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    ShadowReg = 0,
    #[doc = "1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    BypassShadowReg = 1,
}
impl From<BYPSHAD> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers"]
pub type BYPSHAD_R = crate::BitReader<BYPSHAD>;
impl BYPSHAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BYPSHAD {
        match self.bits {
            false => BYPSHAD::ShadowReg,
            true => BYPSHAD::BypassShadowReg,
        }
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    #[inline(always)]
    pub fn is_shadow_reg(&self) -> bool {
        *self == BYPSHAD::ShadowReg
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    #[inline(always)]
    pub fn is_bypass_shadow_reg(&self) -> bool {
        *self == BYPSHAD::BypassShadowReg
    }
}
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers"]
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG, BYPSHAD>;
impl<'a, REG> BYPSHAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    #[inline(always)]
    pub fn shadow_reg(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD::ShadowReg)
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    #[inline(always)]
    pub fn bypass_shadow_reg(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD::BypassShadowReg)
    }
}
#[doc = "Hour format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMT {
    #[doc = "0: 24 hour/day format"]
    TwentyFourHour = 0,
    #[doc = "1: AM/PM hour format"]
    AmPm = 1,
}
impl From<FMT> for bool {
    #[inline(always)]
    fn from(variant: FMT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FMT` reader - Hour format"]
pub type FMT_R = crate::BitReader<FMT>;
impl FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FMT {
        match self.bits {
            false => FMT::TwentyFourHour,
            true => FMT::AmPm,
        }
    }
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        *self == FMT::TwentyFourHour
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        *self == FMT::AmPm
    }
}
#[doc = "Field `FMT` writer - Hour format"]
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG, FMT>;
impl<'a, REG> FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::TwentyFourHour)
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::AmPm)
    }
}
#[doc = "SSR underflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSRUIE {
    #[doc = "0: SSR underflow interrupt disabled"]
    Disabled = 0,
    #[doc = "1: SSR underflow interrupt enabled"]
    Enabled = 1,
}
impl From<SSRUIE> for bool {
    #[inline(always)]
    fn from(variant: SSRUIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSRUIE` reader - SSR underflow interrupt enable"]
pub type SSRUIE_R = crate::BitReader<SSRUIE>;
impl SSRUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSRUIE {
        match self.bits {
            false => SSRUIE::Disabled,
            true => SSRUIE::Enabled,
        }
    }
    #[doc = "SSR underflow interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSRUIE::Disabled
    }
    #[doc = "SSR underflow interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSRUIE::Enabled
    }
}
#[doc = "Field `SSRUIE` writer - SSR underflow interrupt enable"]
pub type SSRUIE_W<'a, REG> = crate::BitWriter<'a, REG, SSRUIE>;
impl<'a, REG> SSRUIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSR underflow interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSRUIE::Disabled)
    }
    #[doc = "SSR underflow interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSRUIE::Enabled)
    }
}
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAE {
    #[doc = "0: Alarm A disabled"]
    Disabled = 0,
    #[doc = "1: Alarm A enabled"]
    Enabled = 1,
}
impl From<ALRAE> for bool {
    #[inline(always)]
    fn from(variant: ALRAE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type ALRAE_R = crate::BitReader<ALRAE>;
impl ALRAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRAE {
        match self.bits {
            false => ALRAE::Disabled,
            true => ALRAE::Enabled,
        }
    }
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAE::Disabled
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAE::Enabled
    }
}
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAE>;
impl<'a, REG> ALRAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::Disabled)
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::Enabled)
    }
}
#[doc = "Alarm B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBE {
    #[doc = "0: Alarm B disabled"]
    Disabled = 0,
    #[doc = "1: Alarm B enabled"]
    Enabled = 1,
}
impl From<ALRBE> for bool {
    #[inline(always)]
    fn from(variant: ALRBE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type ALRBE_R = crate::BitReader<ALRBE>;
impl ALRBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRBE {
        match self.bits {
            false => ALRBE::Disabled,
            true => ALRBE::Enabled,
        }
    }
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBE::Disabled
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBE::Enabled
    }
}
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type ALRBE_W<'a, REG> = crate::BitWriter<'a, REG, ALRBE>;
impl<'a, REG> ALRBE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBE::Disabled)
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBE::Enabled)
    }
}
#[doc = "Wakeup timer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTE {
    #[doc = "0: Wakeup timer disabled"]
    Disabled = 0,
    #[doc = "1: Wakeup timer enabled"]
    Enabled = 1,
}
impl From<WUTE> for bool {
    #[inline(always)]
    fn from(variant: WUTE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTE` reader - Wakeup timer enable"]
pub type WUTE_R = crate::BitReader<WUTE>;
impl WUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUTE {
        match self.bits {
            false => WUTE::Disabled,
            true => WUTE::Enabled,
        }
    }
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTE::Disabled
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTE::Enabled
    }
}
#[doc = "Field `WUTE` writer - Wakeup timer enable"]
pub type WUTE_W<'a, REG> = crate::BitWriter<'a, REG, WUTE>;
impl<'a, REG> WUTE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTE::Disabled)
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTE::Enabled)
    }
}
#[doc = "timestamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSE {
    #[doc = "0: Timestamp disabled"]
    Disabled = 0,
    #[doc = "1: Timestamp enabled"]
    Enabled = 1,
}
impl From<TSE> for bool {
    #[inline(always)]
    fn from(variant: TSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSE` reader - timestamp enable"]
pub type TSE_R = crate::BitReader<TSE>;
impl TSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSE {
        match self.bits {
            false => TSE::Disabled,
            true => TSE::Enabled,
        }
    }
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSE::Disabled
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSE::Enabled
    }
}
#[doc = "Field `TSE` writer - timestamp enable"]
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG, TSE>;
impl<'a, REG> TSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::Disabled)
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::Enabled)
    }
}
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAIE {
    #[doc = "0: Alarm A interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Alarm A interrupt enabled"]
    Enabled = 1,
}
impl From<ALRAIE> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type ALRAIE_R = crate::BitReader<ALRAIE>;
impl ALRAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRAIE {
        match self.bits {
            false => ALRAIE::Disabled,
            true => ALRAIE::Enabled,
        }
    }
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAIE::Disabled
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAIE::Enabled
    }
}
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAIE>;
impl<'a, REG> ALRAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::Disabled)
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::Enabled)
    }
}
#[doc = "Alarm B interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBIE {
    #[doc = "0: Alarm B Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Alarm B Interrupt enabled"]
    Enabled = 1,
}
impl From<ALRBIE> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type ALRBIE_R = crate::BitReader<ALRBIE>;
impl ALRBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALRBIE {
        match self.bits {
            false => ALRBIE::Disabled,
            true => ALRBIE::Enabled,
        }
    }
    #[doc = "Alarm B Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBIE::Disabled
    }
    #[doc = "Alarm B Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBIE::Enabled
    }
}
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type ALRBIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRBIE>;
impl<'a, REG> ALRBIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Alarm B Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBIE::Disabled)
    }
    #[doc = "Alarm B Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ALRBIE::Enabled)
    }
}
#[doc = "Wakeup timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTIE {
    #[doc = "0: Wakeup timer interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Wakeup timer interrupt enabled"]
    Enabled = 1,
}
impl From<WUTIE> for bool {
    #[inline(always)]
    fn from(variant: WUTIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WUTIE_R = crate::BitReader<WUTIE>;
impl WUTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WUTIE {
        match self.bits {
            false => WUTIE::Disabled,
            true => WUTIE::Enabled,
        }
    }
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTIE::Disabled
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTIE::Enabled
    }
}
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WUTIE_W<'a, REG> = crate::BitWriter<'a, REG, WUTIE>;
impl<'a, REG> WUTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTIE::Disabled)
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WUTIE::Enabled)
    }
}
#[doc = "Timestamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE {
    #[doc = "0: Time-stamp Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Time-stamp Interrupt enabled"]
    Enabled = 1,
}
impl From<TSIE> for bool {
    #[inline(always)]
    fn from(variant: TSIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIE` reader - Timestamp interrupt enable"]
pub type TSIE_R = crate::BitReader<TSIE>;
impl TSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TSIE {
        match self.bits {
            false => TSIE::Disabled,
            true => TSIE::Enabled,
        }
    }
    #[doc = "Time-stamp Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSIE::Disabled
    }
    #[doc = "Time-stamp Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSIE::Enabled
    }
}
#[doc = "Field `TSIE` writer - Timestamp interrupt enable"]
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG, TSIE>;
impl<'a, REG> TSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time-stamp Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::Disabled)
    }
    #[doc = "Time-stamp Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::Enabled)
    }
}
#[doc = "Add 1 hour (summer time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1HW {
    #[doc = "1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    Add1 = 1,
}
impl From<ADD1HW> for bool {
    #[inline(always)]
    fn from(variant: ADD1HW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change)"]
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG, ADD1HW>;
impl<'a, REG> ADD1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    #[inline(always)]
    pub fn add1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1HW::Add1)
    }
}
#[doc = "Subtract 1 hour (winter time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB1HW {
    #[doc = "1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    Sub1 = 1,
}
impl From<SUB1HW> for bool {
    #[inline(always)]
    fn from(variant: SUB1HW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change)"]
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG, SUB1HW>;
impl<'a, REG> SUB1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    #[inline(always)]
    pub fn sub1(self) -> &'a mut crate::W<REG> {
        self.variant(SUB1HW::Sub1)
    }
}
#[doc = "Backup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP {
    #[doc = "0: Daylight Saving Time change has not been performed"]
    DstnotChanged = 0,
    #[doc = "1: Daylight Saving Time change has been performed"]
    Dstchanged = 1,
}
impl From<BKP> for bool {
    #[inline(always)]
    fn from(variant: BKP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BKP` reader - Backup"]
pub type BKP_R = crate::BitReader<BKP>;
impl BKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BKP {
        match self.bits {
            false => BKP::DstnotChanged,
            true => BKP::Dstchanged,
        }
    }
    #[doc = "Daylight Saving Time change has not been performed"]
    #[inline(always)]
    pub fn is_dstnot_changed(&self) -> bool {
        *self == BKP::DstnotChanged
    }
    #[doc = "Daylight Saving Time change has been performed"]
    #[inline(always)]
    pub fn is_dstchanged(&self) -> bool {
        *self == BKP::Dstchanged
    }
}
#[doc = "Field `BKP` writer - Backup"]
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG, BKP>;
impl<'a, REG> BKP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Daylight Saving Time change has not been performed"]
    #[inline(always)]
    pub fn dstnot_changed(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::DstnotChanged)
    }
    #[doc = "Daylight Saving Time change has been performed"]
    #[inline(always)]
    pub fn dstchanged(self) -> &'a mut crate::W<REG> {
        self.variant(BKP::Dstchanged)
    }
}
#[doc = "Calibration output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSEL {
    #[doc = "0: Calibration output is 512 Hz (with default prescaler setting)"]
    CalFreq512hz = 0,
    #[doc = "1: Calibration output is 1 Hz (with default prescaler setting)"]
    CalFreq1hz = 1,
}
impl From<COSEL> for bool {
    #[inline(always)]
    fn from(variant: COSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COSEL` reader - Calibration output selection"]
pub type COSEL_R = crate::BitReader<COSEL>;
impl COSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COSEL {
        match self.bits {
            false => COSEL::CalFreq512hz,
            true => COSEL::CalFreq1hz,
        }
    }
    #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn is_cal_freq_512hz(&self) -> bool {
        *self == COSEL::CalFreq512hz
    }
    #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn is_cal_freq_1hz(&self) -> bool {
        *self == COSEL::CalFreq1hz
    }
}
#[doc = "Field `COSEL` writer - Calibration output selection"]
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG, COSEL>;
impl<'a, REG> COSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_512hz(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL::CalFreq512hz)
    }
    #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_1hz(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL::CalFreq1hz)
    }
}
#[doc = "Output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL {
    #[doc = "0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    High = 0,
    #[doc = "1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    Low = 1,
}
impl From<POL> for bool {
    #[inline(always)]
    fn from(variant: POL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL` reader - Output polarity"]
pub type POL_R = crate::BitReader<POL>;
impl POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POL {
        match self.bits {
            false => POL::High,
            true => POL::Low,
        }
    }
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL::High
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL::Low
    }
}
#[doc = "Field `POL` writer - Output polarity"]
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG, POL>;
impl<'a, REG> POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(POL::High)
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(POL::Low)
    }
}
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEL {
    #[doc = "0: Output disabled"]
    Disabled = 0,
    #[doc = "1: Alarm A output enabled"]
    AlarmA = 1,
    #[doc = "2: Alarm B output enabled"]
    AlarmB = 2,
    #[doc = "3: Wakeup output enabled"]
    Wakeup = 3,
}
impl From<OSEL> for u8 {
    #[inline(always)]
    fn from(variant: OSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSEL {
    type Ux = u8;
}
#[doc = "Field `OSEL` reader - Output selection"]
pub type OSEL_R = crate::FieldReader<OSEL>;
impl OSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OSEL {
        match self.bits {
            0 => OSEL::Disabled,
            1 => OSEL::AlarmA,
            2 => OSEL::AlarmB,
            3 => OSEL::Wakeup,
            _ => unreachable!(),
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSEL::Disabled
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        *self == OSEL::AlarmA
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        *self == OSEL::AlarmB
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OSEL::Wakeup
    }
}
#[doc = "Field `OSEL` writer - Output selection"]
pub type OSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OSEL>;
impl<'a, REG> OSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::Disabled)
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::AlarmA)
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::AlarmB)
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::Wakeup)
    }
}
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE {
    #[doc = "0: Calibration output disabled"]
    Disabled = 0,
    #[doc = "1: Calibration output enabled"]
    Enabled = 1,
}
impl From<COE> for bool {
    #[inline(always)]
    fn from(variant: COE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COE` reader - Calibration output enable"]
pub type COE_R = crate::BitReader<COE>;
impl COE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COE {
        match self.bits {
            false => COE::Disabled,
            true => COE::Enabled,
        }
    }
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COE::Disabled
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COE::Enabled
    }
}
#[doc = "Field `COE` writer - Calibration output enable"]
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG, COE>;
impl<'a, REG> COE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COE::Disabled)
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COE::Enabled)
    }
}
#[doc = "timestamp on internal event enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSE {
    #[doc = "0: Internal event timestamp disabled"]
    Disabled = 0,
    #[doc = "1: Internal event timestamp enabled"]
    Enabled = 1,
}
impl From<ITSE> for bool {
    #[inline(always)]
    fn from(variant: ITSE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub type ITSE_R = crate::BitReader<ITSE>;
impl ITSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITSE {
        match self.bits {
            false => ITSE::Disabled,
            true => ITSE::Enabled,
        }
    }
    #[doc = "Internal event timestamp disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITSE::Disabled
    }
    #[doc = "Internal event timestamp enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITSE::Enabled
    }
}
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub type ITSE_W<'a, REG> = crate::BitWriter<'a, REG, ITSE>;
impl<'a, REG> ITSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal event timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITSE::Disabled)
    }
    #[doc = "Internal event timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITSE::Enabled)
    }
}
#[doc = "Activate timestamp on tamper detection event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPTS {
    #[doc = "0: Tamper detection event does not cause a RTC timestamp to be saved"]
    Disabled = 0,
    #[doc = "1: Save RTC timestamp on tamper detection event"]
    Enabled = 1,
}
impl From<TAMPTS> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TAMPTS_R = crate::BitReader<TAMPTS>;
impl TAMPTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPTS {
        match self.bits {
            false => TAMPTS::Disabled,
            true => TAMPTS::Enabled,
        }
    }
    #[doc = "Tamper detection event does not cause a RTC timestamp to be saved"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPTS::Disabled
    }
    #[doc = "Save RTC timestamp on tamper detection event"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPTS::Enabled
    }
}
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPTS>;
impl<'a, REG> TAMPTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection event does not cause a RTC timestamp to be saved"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::Disabled)
    }
    #[doc = "Save RTC timestamp on tamper detection event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::Enabled)
    }
}
#[doc = "Tamper detection output enable on TAMPALRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPOE {
    #[doc = "0: The tamper flag is not routed on TAMPALRM"]
    Disabled = 0,
    #[doc = "1: The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL"]
    Enabled = 1,
}
impl From<TAMPOE> for bool {
    #[inline(always)]
    fn from(variant: TAMPOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_R = crate::BitReader<TAMPOE>;
impl TAMPOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPOE {
        match self.bits {
            false => TAMPOE::Disabled,
            true => TAMPOE::Enabled,
        }
    }
    #[doc = "The tamper flag is not routed on TAMPALRM"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPOE::Disabled
    }
    #[doc = "The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPOE::Enabled
    }
}
#[doc = "Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPOE>;
impl<'a, REG> TAMPOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The tamper flag is not routed on TAMPALRM"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPOE::Disabled)
    }
    #[doc = "The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and with the polarity provided by POL"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPOE::Enabled)
    }
}
#[doc = "TAMPALRM pull-up enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_PU {
    #[doc = "0: No pull-up is applied on TAMPALRM output"]
    NoPullUp = 0,
    #[doc = "1: A pull-up is applied on TAMPALRM output"]
    PullUp = 1,
}
impl From<TAMPALRM_PU> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_PU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_R = crate::BitReader<TAMPALRM_PU>;
impl TAMPALRM_PU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_PU {
        match self.bits {
            false => TAMPALRM_PU::NoPullUp,
            true => TAMPALRM_PU::PullUp,
        }
    }
    #[doc = "No pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn is_no_pull_up(&self) -> bool {
        *self == TAMPALRM_PU::NoPullUp
    }
    #[doc = "A pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == TAMPALRM_PU::PullUp
    }
}
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_PU>;
impl<'a, REG> TAMPALRM_PU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn no_pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU::NoPullUp)
    }
    #[doc = "A pull-up is applied on TAMPALRM output"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU::PullUp)
    }
}
#[doc = "TAMPALRM output type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_TYPE {
    #[doc = "0: TAMPALRM is push-pull output"]
    PushPull = 0,
    #[doc = "1: TAMPALRM is open-drain output"]
    OpenDrain = 1,
}
impl From<TAMPALRM_TYPE> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_TYPE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM output type"]
pub type TAMPALRM_TYPE_R = crate::BitReader<TAMPALRM_TYPE>;
impl TAMPALRM_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_TYPE {
        match self.bits {
            false => TAMPALRM_TYPE::PushPull,
            true => TAMPALRM_TYPE::OpenDrain,
        }
    }
    #[doc = "TAMPALRM is push-pull output"]
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == TAMPALRM_TYPE::PushPull
    }
    #[doc = "TAMPALRM is open-drain output"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == TAMPALRM_TYPE::OpenDrain
    }
}
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM output type"]
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_TYPE>;
impl<'a, REG> TAMPALRM_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TAMPALRM is push-pull output"]
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE::PushPull)
    }
    #[doc = "TAMPALRM is open-drain output"]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE::OpenDrain)
    }
}
#[doc = "RTC_OUT2 output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUT2EN {
    #[doc = "0: RTC output 2 disable"]
    Disabled = 0,
    #[doc = "1: RTC output 2 enable"]
    Enabled = 1,
}
impl From<OUT2EN> for bool {
    #[inline(always)]
    fn from(variant: OUT2EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUT2EN` reader - RTC_OUT2 output enable"]
pub type OUT2EN_R = crate::BitReader<OUT2EN>;
impl OUT2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUT2EN {
        match self.bits {
            false => OUT2EN::Disabled,
            true => OUT2EN::Enabled,
        }
    }
    #[doc = "RTC output 2 disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUT2EN::Disabled
    }
    #[doc = "RTC output 2 enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OUT2EN::Enabled
    }
}
#[doc = "Field `OUT2EN` writer - RTC_OUT2 output enable"]
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG, OUT2EN>;
impl<'a, REG> OUT2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC output 2 disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OUT2EN::Disabled)
    }
    #[doc = "RTC output 2 enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OUT2EN::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSR underflow interrupt enable"]
    #[inline(always)]
    pub fn ssruie(&self) -> SSRUIE_R {
        SSRUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable"]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<CRrs> {
        WUCKSEL_W::new(self, 0)
    }
    #[doc = "Bit 3 - Timestamp event active edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<CRrs> {
        TSEDGE_W::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<CRrs> {
        REFCKON_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<CRrs> {
        FMT_W::new(self, 6)
    }
    #[doc = "Bit 7 - SSR underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssruie(&mut self) -> SSRUIE_W<CRrs> {
        SSRUIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<CRrs> {
        ALRAE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<CRrs> {
        ALRBE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<CRrs> {
        WUTE_W::new(self, 10)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<CRrs> {
        TSE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<CRrs> {
        ALRAIE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<CRrs> {
        ALRBIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<CRrs> {
        WUTIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<CRrs> {
        TSIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<CRrs> {
        ADD1H_W::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<CRrs> {
        SUB1H_W::new(self, 17)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<CRrs> {
        BKP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<CRrs> {
        COSEL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<CRrs> {
        POL_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<CRrs> {
        OSEL_W::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<CRrs> {
        COE_W::new(self, 23)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<CRrs> {
        ITSE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    #[must_use]
    pub fn tampts(&mut self) -> TAMPTS_W<CRrs> {
        TAMPTS_W::new(self, 25)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    #[must_use]
    pub fn tampoe(&mut self) -> TAMPOE_W<CRrs> {
        TAMPOE_W::new(self, 26)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<CRrs> {
        TAMPALRM_PU_W::new(self, 29)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    #[must_use]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<CRrs> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable"]
    #[inline(always)]
    #[must_use]
    pub fn out2en(&mut self) -> OUT2EN_W<CRrs> {
        OUT2EN_W::new(self, 31)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
