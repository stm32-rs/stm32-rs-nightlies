#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCRrs>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCRrs>;
#[doc = "LSE oscillator enabled Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON {
    #[doc = "0: LSE oscillator Off"]
    Off = 0,
    #[doc = "1: LSE oscillator On"]
    On = 1,
}
impl From<LSEON> for bool {
    #[inline(always)]
    fn from(variant: LSEON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEON` reader - LSE oscillator enabled Set and reset by software."]
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
    #[doc = "LSE oscillator Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON::Off
    }
    #[doc = "LSE oscillator On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON::On
    }
}
#[doc = "Field `LSEON` writer - LSE oscillator enabled Set and reset by software."]
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG, LSEON>;
impl<'a, REG> LSEON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::Off)
    }
    #[doc = "LSE oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSEON::On)
    }
}
#[doc = "LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYR {
    #[doc = "0: LSE oscillator not ready"]
    NotReady = 0,
    #[doc = "1: LSE oscillator ready"]
    Ready = 1,
}
impl From<LSERDYR> for bool {
    #[inline(always)]
    fn from(variant: LSERDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSERDY` reader - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
pub type LSERDY_R = crate::BitReader<LSERDYR>;
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSERDYR {
        match self.bits {
            false => LSERDYR::NotReady,
            true => LSERDYR::Ready,
        }
    }
    #[doc = "LSE oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDYR::NotReady
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDYR::Ready
    }
}
#[doc = "Field `LSERDY` writer - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
pub type LSERDY_W<'a, REG> = crate::BitWriter<'a, REG, LSERDYR>;
impl<'a, REG> LSERDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYR::NotReady)
    }
    #[doc = "LSE oscillator ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSERDYR::Ready)
    }
}
#[doc = "LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP {
    #[doc = "0: LSE crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: LSE crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<LSEBYP> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
pub type LSEBYP_R = crate::BitReader<LSEBYP>;
impl LSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEBYP {
        match self.bits {
            false => LSEBYP::NotBypassed,
            true => LSEBYP::Bypassed,
        }
    }
    #[doc = "LSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP::NotBypassed
    }
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP::Bypassed
    }
}
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::NotBypassed)
    }
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Bypassed)
    }
}
#[doc = "LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV {
    #[doc = "0: Lowest LSE oscillator driving capability"]
    Lowest = 0,
    #[doc = "1: Medium low LSE oscillator driving capability"]
    MediumLow = 1,
    #[doc = "2: Medium high LSE oscillator driving capability"]
    MediumHigh = 2,
    #[doc = "3: Highest LSE oscillator driving capability"]
    Highest = 3,
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
#[doc = "Field `LSEDRV` reader - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
pub type LSEDRV_R = crate::FieldReader<LSEDRV>;
impl LSEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV {
        match self.bits {
            0 => LSEDRV::Lowest,
            1 => LSEDRV::MediumLow,
            2 => LSEDRV::MediumHigh,
            3 => LSEDRV::Highest,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest LSE oscillator driving capability"]
    #[inline(always)]
    pub fn is_lowest(&self) -> bool {
        *self == LSEDRV::Lowest
    }
    #[doc = "Medium low LSE oscillator driving capability"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV::MediumLow
    }
    #[doc = "Medium high LSE oscillator driving capability"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV::MediumHigh
    }
    #[doc = "Highest LSE oscillator driving capability"]
    #[inline(always)]
    pub fn is_highest(&self) -> bool {
        *self == LSEDRV::Highest
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
pub type LSEDRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LSEDRV>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest LSE oscillator driving capability"]
    #[inline(always)]
    pub fn lowest(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Lowest)
    }
    #[doc = "Medium low LSE oscillator driving capability"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumLow)
    }
    #[doc = "Medium high LSE oscillator driving capability"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumHigh)
    }
    #[doc = "Highest LSE oscillator driving capability"]
    #[inline(always)]
    pub fn highest(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Highest)
    }
}
#[doc = "LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON {
    #[doc = "0: Clock security system on 32 kHz oscillator off"]
    SecurityOff = 0,
    #[doc = "1: Clock security system on 32 kHz oscillator on"]
    SecurityOn = 1,
}
impl From<LSECSSON> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSON` reader - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
pub type LSECSSON_R = crate::BitReader<LSECSSON>;
impl LSECSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSON {
        match self.bits {
            false => LSECSSON::SecurityOff,
            true => LSECSSON::SecurityOn,
        }
    }
    #[doc = "Clock security system on 32 kHz oscillator off"]
    #[inline(always)]
    pub fn is_security_off(&self) -> bool {
        *self == LSECSSON::SecurityOff
    }
    #[doc = "Clock security system on 32 kHz oscillator on"]
    #[inline(always)]
    pub fn is_security_on(&self) -> bool {
        *self == LSECSSON::SecurityOn
    }
}
#[doc = "Field `LSECSSON` writer - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
pub type LSECSSON_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSON>;
impl<'a, REG> LSECSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock security system on 32 kHz oscillator off"]
    #[inline(always)]
    pub fn security_off(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::SecurityOff)
    }
    #[doc = "Clock security system on 32 kHz oscillator on"]
    #[inline(always)]
    pub fn security_on(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSON::SecurityOn)
    }
}
#[doc = "LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSDR {
    #[doc = "0: No failure detected on 32 kHz oscillator"]
    NoFailure = 0,
    #[doc = "1: Failure detected on 32 kHz oscillator"]
    Failure = 1,
}
impl From<LSECSSDR> for bool {
    #[inline(always)]
    fn from(variant: LSECSSDR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSECSSD` reader - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
pub type LSECSSD_R = crate::BitReader<LSECSSDR>;
impl LSECSSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSECSSDR {
        match self.bits {
            false => LSECSSDR::NoFailure,
            true => LSECSSDR::Failure,
        }
    }
    #[doc = "No failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSDR::NoFailure
    }
    #[doc = "Failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSDR::Failure
    }
}
#[doc = "Field `LSECSSD` writer - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
pub type LSECSSD_W<'a, REG> = crate::BitWriter<'a, REG, LSECSSDR>;
impl<'a, REG> LSECSSD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSDR::NoFailure)
    }
    #[doc = "Failure detected on 32 kHz oscillator"]
    #[inline(always)]
    pub fn failure(self) -> &'a mut crate::W<REG> {
        self.variant(LSECSSDR::Failure)
    }
}
#[doc = "low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEEXT {
    #[doc = "0: HSE in analog mode"]
    Analog = 0,
    #[doc = "1: HSE in digital mode"]
    Digital = 1,
}
impl From<LSEEXT> for bool {
    #[inline(always)]
    fn from(variant: LSEEXT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEEXT` reader - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
pub type LSEEXT_R = crate::BitReader<LSEEXT>;
impl LSEEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEEXT {
        match self.bits {
            false => LSEEXT::Analog,
            true => LSEEXT::Digital,
        }
    }
    #[doc = "HSE in analog mode"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == LSEEXT::Analog
    }
    #[doc = "HSE in digital mode"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == LSEEXT::Digital
    }
}
#[doc = "Field `LSEEXT` writer - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
pub type LSEEXT_W<'a, REG> = crate::BitWriter<'a, REG, LSEEXT>;
impl<'a, REG> LSEEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE in analog mode"]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(LSEEXT::Analog)
    }
    #[doc = "HSE in digital mode"]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(LSEEXT::Digital)
    }
}
#[doc = "RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: LSE oscillator clock used as RTC clock"]
    Lse = 1,
    #[doc = "2: LSI oscillator clock used as RTC clock"]
    Lsi = 2,
    #[doc = "3: HSE oscillator clock divided by a prescaler used as RTC clock"]
    Hse = 3,
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
#[doc = "Field `RTCSEL` reader - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
pub type RTCSEL_R = crate::FieldReader<RTCSEL>;
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCSEL {
        match self.bits {
            0 => RTCSEL::NoClock,
            1 => RTCSEL::Lse,
            2 => RTCSEL::Lsi,
            3 => RTCSEL::Hse,
            _ => unreachable!(),
        }
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL::NoClock
    }
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL::Lse
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL::Lsi
    }
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL::Hse
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
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
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lse)
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Lsi)
    }
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Hse)
    }
}
#[doc = "RTC clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN {
    #[doc = "0: RTC clock disabled"]
    Disabled = 0,
    #[doc = "1: RTC clock enabled"]
    Enabled = 1,
}
impl From<RTCEN> for bool {
    #[inline(always)]
    fn from(variant: RTCEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable Set and reset by software."]
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
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN::Disabled
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN::Enabled
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable Set and reset by software."]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG, RTCEN>;
impl<'a, REG> RTCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Disabled)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RTCEN::Enabled)
    }
}
#[doc = "VSwitch domain software reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSWRST {
    #[doc = "0: Reset not activated"]
    NotActivated = 0,
    #[doc = "1: Resets the entire VSW domain"]
    Reset = 1,
}
impl From<VSWRST> for bool {
    #[inline(always)]
    fn from(variant: VSWRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSWRST` reader - VSwitch domain software reset Set and reset by software."]
pub type VSWRST_R = crate::BitReader<VSWRST>;
impl VSWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VSWRST {
        match self.bits {
            false => VSWRST::NotActivated,
            true => VSWRST::Reset,
        }
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn is_not_activated(&self) -> bool {
        *self == VSWRST::NotActivated
    }
    #[doc = "Resets the entire VSW domain"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == VSWRST::Reset
    }
}
#[doc = "Field `VSWRST` writer - VSwitch domain software reset Set and reset by software."]
pub type VSWRST_W<'a, REG> = crate::BitWriter<'a, REG, VSWRST>;
impl<'a, REG> VSWRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn not_activated(self) -> &'a mut crate::W<REG> {
        self.variant(VSWRST::NotActivated)
    }
    #[doc = "Resets the entire VSW domain"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(VSWRST::Reset)
    }
}
#[doc = "Field `LSCOEN` reader - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_R = crate::BitReader;
#[doc = "Field `LSCOEN` writer - Low-speed clock output (LSCO) enable Set and cleared by software."]
pub type LSCOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low-speed clock output selection Set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `LSCOSEL` reader - Low-speed clock output selection Set and cleared by software."]
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
#[doc = "Field `LSCOSEL` writer - Low-speed clock output selection Set and cleared by software."]
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
#[doc = "LSI oscillator enable Set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    #[doc = "0: Oscillator disabled"]
    Disabled = 0,
    #[doc = "1: Oscillator enabled"]
    Enabled = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - LSI oscillator enable Set and cleared by software."]
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Disabled,
            true => LSION::Enabled,
        }
    }
    #[doc = "Oscillator disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSION::Disabled
    }
    #[doc = "Oscillator enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSION::Enabled
    }
}
#[doc = "Field `LSION` writer - LSI oscillator enable Set and cleared by software."]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Disabled)
    }
    #[doc = "Oscillator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Enabled)
    }
}
#[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDYR {
    #[doc = "0: Clock not ready"]
    NotReady = 0,
    #[doc = "1: Clock ready"]
    Ready = 1,
}
impl From<LSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: LSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
pub type LSIRDY_R = crate::BitReader<LSIRDYR>;
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDYR {
        match self.bits {
            false => LSIRDYR::NotReady,
            true => LSIRDYR::Ready,
        }
    }
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDYR::NotReady
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDYR::Ready
    }
}
#[doc = "Field `LSIRDY` writer - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDYR>;
impl<'a, REG> LSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYR::NotReady)
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDYR::Ready)
    }
}
impl R {
    #[doc = "Bit 0 - LSE oscillator enabled Set and reset by software."]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
    #[inline(always)]
    pub fn lseext(&self) -> LSEEXT_R {
        LSEEXT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and reset by software."]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VSwitch domain software reset Set and reset by software."]
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software."]
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enabled Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<BDCRrs> {
        LSEON_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn lserdy(&mut self) -> LSERDY_W<BDCRrs> {
        LSERDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<BDCRrs> {
        LSEDRV_W::new(self, 3)
    }
    #[doc = "Bit 5 - LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<BDCRrs> {
        LSECSSON_W::new(self, 5)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn lsecssd(&mut self) -> LSECSSD_W<BDCRrs> {
        LSECSSD_W::new(self, 6)
    }
    #[doc = "Bit 7 - low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn lseext(&mut self) -> LSEEXT_W<BDCRrs> {
        LSEEXT_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<BDCRrs> {
        RTCSEL_W::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - VSwitch domain software reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn vswrst(&mut self) -> VSWRST_W<BDCRrs> {
        VSWRST_W::new(self, 16)
    }
    #[doc = "Bit 24 - Low-speed clock output (LSCO) enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<BDCRrs> {
        LSCOEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - Low-speed clock output selection Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<BDCRrs> {
        LSCOSEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - LSI oscillator enable Set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<BDCRrs> {
        LSION_W::new(self, 26)
    }
    #[doc = "Bit 27 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<BDCRrs> {
        LSIRDY_W::new(self, 27)
    }
}
#[doc = "RCC Backup domain control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
