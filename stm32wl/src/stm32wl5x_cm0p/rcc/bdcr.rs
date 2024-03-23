#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCRrs>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCRrs>;
#[doc = "LSE oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON {
    #[doc = "0: LSE oscillator off"]
    Off = 0,
    #[doc = "1: LSE oscillator on"]
    On = 1,
}
impl From<LSEON> for bool {
    #[inline(always)]
    fn from(variant: LSEON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enable"]
pub type LSEON_R = crate::BitReader<LSEON>;
impl LSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEON {
        match self.bits {
            false => LSEON::Off,
            true => LSEON::On,
        }
    }
    #[doc = "LSE oscillator off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON::Off
    }
    #[doc = "LSE oscillator on"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON::On
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enable"]
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG, LSEON>;
impl<'a, REG> LSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::Off)
    }
    #[doc = "LSE oscillator on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::On)
    }
}
#[doc = "LSE oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDY {
    #[doc = "0: LSE oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSE oscillator ready"]
    Ready = 1,
}
impl From<LSERDY> for bool {
    #[inline(always)]
    fn from(variant: LSERDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready"]
pub type LSERDY_R = crate::BitReader<LSERDY>;
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDY {
        match self.bits {
            false => LSERDY::NotReady,
            true => LSERDY::Ready,
        }
    }
    #[doc = "LSE oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDY::NotReady
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDY::Ready
    }
}
#[doc = "LSE oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP {
    #[doc = "0: LSE oscillator not bypassed"]
    Disabled = 0,
    #[doc = "1: LSE oscillator bypassed"]
    Enabled = 1,
}
impl From<LSEBYP> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<LSEBYP>;
impl LSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEBYP {
        match self.bits {
            false => LSEBYP::Disabled,
            true => LSEBYP::Enabled,
        }
    }
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEBYP::Disabled
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEBYP::Enabled
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass"]
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Disabled)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Enabled)
    }
}
#[doc = "LSE oscillator drive capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV {
    #[doc = "0: Xtal mode lower driving capability"]
    Low = 0,
    #[doc = "1: Xtal mode medium-low driving capability"]
    MedLow = 1,
    #[doc = "2: Xtal mode medium-high driving capability"]
    MedHigh = 2,
    #[doc = "3: Xtal mode higher driving capability"]
    High = 3,
}
impl From<LSEDRV> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSEDRV {
    type Ux = u8;
}
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability"]
pub type LSEDRV_R = crate::FieldReader<LSEDRV>;
impl LSEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV {
        match self.bits {
            0 => LSEDRV::Low,
            1 => LSEDRV::MedLow,
            2 => LSEDRV::MedHigh,
            3 => LSEDRV::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Xtal mode lower driving capability"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV::Low
    }
    #[doc = "Xtal mode medium-low driving capability"]
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == LSEDRV::MedLow
    }
    #[doc = "Xtal mode medium-high driving capability"]
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == LSEDRV::MedHigh
    }
    #[doc = "Xtal mode higher driving capability"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV::High
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability"]
pub type LSEDRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LSEDRV>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Xtal mode lower driving capability"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Low)
    }
    #[doc = "Xtal mode medium-low driving capability"]
    #[inline(always)]
    pub fn med_low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MedLow)
    }
    #[doc = "Xtal mode medium-high driving capability"]
    #[inline(always)]
    pub fn med_high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MedHigh)
    }
    #[doc = "Xtal mode higher driving capability"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::High)
    }
}
#[doc = "CSS on LSE enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON {
    #[doc = "0: CSS on LSE disabled"]
    Disabled = 0,
    #[doc = "1: CSS on LSE enabled"]
    Enabled = 1,
}
impl From<LSECSSON> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSON` reader - CSS on LSE enable"]
pub type LSECSSON_R = crate::BitReader<LSECSSON>;
impl LSECSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSON {
        match self.bits {
            false => LSECSSON::Disabled,
            true => LSECSSON::Enabled,
        }
    }
    #[doc = "CSS on LSE disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSON::Disabled
    }
    #[doc = "CSS on LSE enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSON::Enabled
    }
}
#[doc = "Field `LSECSSON` writer - CSS on LSE enable"]
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSON>;
impl<'a, REG> LSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSS on LSE disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::Disabled)
    }
    #[doc = "CSS on LSE enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::Enabled)
    }
}
#[doc = "CSS on LSE failure Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD {
    #[doc = "0: No failure detected on LSE"]
    NoFailure = 0,
    #[doc = "1: Failure detected on LSE"]
    Failure = 1,
}
impl From<LSECSSD> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSD` reader - CSS on LSE failure Detection"]
pub type LSECSSD_R = crate::BitReader<LSECSSD>;
impl LSECSSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSD {
        match self.bits {
            false => LSECSSD::NoFailure,
            true => LSECSSD::Failure,
        }
    }
    #[doc = "No failure detected on LSE"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSD::NoFailure
    }
    #[doc = "Failure detected on LSE"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSD::Failure
    }
}
#[doc = "LSE system clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSEN {
    #[doc = "0: LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    Disabled = 0,
    #[doc = "1: LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    Enabled = 1,
}
impl From<LSESYSEN> for bool {
    #[inline(always)]
    fn from(variant: LSESYSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSESYSEN` reader - LSE system clock enable"]
pub type LSESYSEN_R = crate::BitReader<LSESYSEN>;
impl LSESYSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSESYSEN {
        match self.bits {
            false => LSESYSEN::Disabled,
            true => LSESYSEN::Enabled,
        }
    }
    #[doc = "LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSESYSEN::Disabled
    }
    #[doc = "LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSESYSEN::Enabled
    }
}
#[doc = "Field `LSESYSEN` writer - LSE system clock enable"]
pub type LSESYSEN_W<'a, REG> = crate::BitWriter<'a, REG, LSESYSEN>;
impl<'a, REG> LSESYSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSESYSEN::Disabled)
    }
    #[doc = "LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSESYSEN::Enabled)
    }
}
#[doc = "RTC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: LSE oscillator clock selected"]
    Lse = 1,
    #[doc = "2: LSI oscillator clock selected"]
    Lsi = 2,
    #[doc = "3: HSE32 oscillator clock divided by 32 selected"]
    Hse32 = 3,
}
impl From<RTCSEL> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL {
    type Ux = u8;
}
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RTCSEL_R = crate::FieldReader<RTCSEL>;
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCSEL {
        match self.bits {
            0 => RTCSEL::NoClock,
            1 => RTCSEL::Lse,
            2 => RTCSEL::Lsi,
            3 => RTCSEL::Hse32,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL::NoClock
    }
    #[doc = "LSE oscillator clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL::Lse
    }
    #[doc = "LSI oscillator clock selected"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL::Lsi
    }
    #[doc = "HSE32 oscillator clock divided by 32 selected"]
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == RTCSEL::Hse32
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RTCSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RTCSEL>;
impl<'a, REG> RTCSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::NoClock)
    }
    #[doc = "LSE oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lse)
    }
    #[doc = "LSI oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lsi)
    }
    #[doc = "HSE32 oscillator clock divided by 32 selected"]
    #[inline(always)]
    pub fn hse32(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Hse32)
    }
}
#[doc = "LSE system clock ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSRDY {
    #[doc = "0: LSE system clock not ready"]
    NotReady = 0,
    #[doc = "1: LSE system clock ready"]
    Ready = 1,
}
impl From<LSESYSRDY> for bool {
    #[inline(always)]
    fn from(variant: LSESYSRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSESYSRDY` reader - LSE system clock ready"]
pub type LSESYSRDY_R = crate::BitReader<LSESYSRDY>;
impl LSESYSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSESYSRDY {
        match self.bits {
            false => LSESYSRDY::NotReady,
            true => LSESYSRDY::Ready,
        }
    }
    #[doc = "LSE system clock not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSESYSRDY::NotReady
    }
    #[doc = "LSE system clock ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSESYSRDY::Ready
    }
}
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN {
    #[doc = "0: RTC kernel clock disabled"]
    Disabled = 0,
    #[doc = "1: RTC kernel clock enabled"]
    Enabled = 1,
}
impl From<RTCEN> for bool {
    #[inline(always)]
    fn from(variant: RTCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<RTCEN>;
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCEN {
        match self.bits {
            false => RTCEN::Disabled,
            true => RTCEN::Enabled,
        }
    }
    #[doc = "RTC kernel clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN::Disabled
    }
    #[doc = "RTC kernel clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN::Enabled
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC kernel clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Disabled)
    }
    #[doc = "RTC kernel clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Enabled)
    }
}
#[doc = "Backup domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST {
    #[doc = "0: Reset not activated"]
    NotActive = 0,
    #[doc = "1: Entire Backup domain reset"]
    Reset = 1,
}
impl From<BDRST> for bool {
    #[inline(always)]
    fn from(variant: BDRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BDRST_R = crate::BitReader<BDRST>;
impl BDRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BDRST {
        match self.bits {
            false => BDRST::NotActive,
            true => BDRST::Reset,
        }
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == BDRST::NotActive
    }
    #[doc = "Entire Backup domain reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRST::Reset
    }
}
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG, BDRST>;
impl<'a, REG> BDRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::NotActive)
    }
    #[doc = "Entire Backup domain reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::Reset)
    }
}
#[doc = "Low speed clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN {
    #[doc = "0: LSCO disabled"]
    Disabled = 0,
    #[doc = "1: LSCO enabled"]
    Enabled = 1,
}
impl From<LSCOEN> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOEN` reader - Low speed clock output enable"]
pub type LSCOEN_R = crate::BitReader<LSCOEN>;
impl LSCOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSCOEN {
        match self.bits {
            false => LSCOEN::Disabled,
            true => LSCOEN::Enabled,
        }
    }
    #[doc = "LSCO disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSCOEN::Disabled
    }
    #[doc = "LSCO enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSCOEN::Enabled
    }
}
#[doc = "Field `LSCOEN` writer - Low speed clock output enable"]
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG, LSCOEN>;
impl<'a, REG> LSCOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSCO disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::Disabled)
    }
    #[doc = "LSCO enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOEN::Enabled)
    }
}
#[doc = "Low speed clock output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL {
    #[doc = "0: LSI clock selected"]
    Lsi = 0,
    #[doc = "1: LSE clock selected"]
    Lse = 1,
}
impl From<LSCOSEL> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSCOSEL` reader - Low speed clock output selection"]
pub type LSCOSEL_R = crate::BitReader<LSCOSEL>;
impl LSCOSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSCOSEL {
        match self.bits {
            false => LSCOSEL::Lsi,
            true => LSCOSEL::Lse,
        }
    }
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LSCOSEL::Lsi
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LSCOSEL::Lse
    }
}
#[doc = "Field `LSCOSEL` writer - Low speed clock output selection"]
pub type LSCOSEL_W<'a, REG> = crate::BitWriter<'a, REG, LSCOSEL>;
impl<'a, REG> LSCOSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSI clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::Lsi)
    }
    #[doc = "LSE clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(LSCOSEL::Lse)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CSS on LSE failure Detection"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LSE system clock enable"]
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - LSE system clock ready"]
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<BDCRrs> {
        LSEON_W::new(self, 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    #[doc = "Bit 5 - CSS on LSE enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    #[doc = "Bit 7 - LSE system clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<BDCRrs> {
        LSESYSEN_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<BDCRrs> {
        BDRST_W::new(self, 16)
    }
    #[doc = "Bit 24 - Low speed clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Low speed clock output selection"]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
}
#[doc = "Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BDCRrs {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCRrs {
    const RESET_VALUE: u32 = 0;
}
