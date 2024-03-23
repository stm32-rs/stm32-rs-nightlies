#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Internal low-speed oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION {
    #[doc = "0: Oscillator OFF"]
    Off = 0,
    #[doc = "1: Oscillator ON"]
    On = 1,
}
impl From<LSION> for bool {
    #[inline(always)]
    fn from(variant: LSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LSION_R = crate::BitReader<LSION>;
impl LSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSION {
        match self.bits {
            false => LSION::Off,
            true => LSION::On,
        }
    }
    #[doc = "Oscillator OFF"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION::Off
    }
    #[doc = "Oscillator ON"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION::On
    }
}
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG, LSION>;
impl<'a, REG> LSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillator OFF"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::Off)
    }
    #[doc = "Oscillator ON"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(LSION::On)
    }
}
#[doc = "Internal low-speed oscillator ready bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY {
    #[doc = "0: Oscillator not ready"]
    NotReady = 0,
    #[doc = "1: Oscillator ready"]
    Ready = 1,
}
impl From<LSIRDY> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready bit"]
pub type LSIRDY_R = crate::BitReader<LSIRDY>;
impl LSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSIRDY {
        match self.bits {
            false => LSIRDY::NotReady,
            true => LSIRDY::Ready,
        }
    }
    #[doc = "Oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY::NotReady
    }
    #[doc = "Oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY::Ready
    }
}
#[doc = "Field `LSIRDY` writer - Internal low-speed oscillator ready bit"]
pub type LSIRDY_W<'a, REG> = crate::BitWriter<'a, REG, LSIRDY>;
impl<'a, REG> LSIRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Oscillator not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDY::NotReady)
    }
    #[doc = "Oscillator ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(LSIRDY::Ready)
    }
}
#[doc = "Field `LSEON` reader - External low-speed oscillator enable bit"]
pub use LSION_R as LSEON_R;
#[doc = "Field `LSEON` writer - External low-speed oscillator enable bit"]
pub use LSION_W as LSEON_W;
#[doc = "Field `LSERDY` reader - External low-speed oscillator ready bit"]
pub use LSIRDY_R as LSERDY_R;
#[doc = "External low-speed oscillator bypass bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP {
    #[doc = "0: LSE oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: LSE oscillator bypassed"]
    Bypassed = 1,
}
impl From<LSEBYP> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEBYP` reader - External low-speed oscillator bypass bit"]
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
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP::NotBypassed
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP::Bypassed
    }
}
#[doc = "Field `LSEBYP` writer - External low-speed oscillator bypass bit"]
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, LSEBYP>;
impl<'a, REG> LSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::NotBypassed)
    }
    #[doc = "LSE oscillator bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(LSEBYP::Bypassed)
    }
}
#[doc = "LSEDRV\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV {
    #[doc = "0: Lowest drive"]
    Low = 0,
    #[doc = "1: Medium low drive"]
    MediumLow = 1,
    #[doc = "2: Medium high drive"]
    MediumHigh = 2,
    #[doc = "3: Highest drive"]
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
#[doc = "Field `LSEDRV` reader - LSEDRV"]
pub type LSEDRV_R = crate::FieldReader<LSEDRV>;
impl LSEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEDRV {
        match self.bits {
            0 => LSEDRV::Low,
            1 => LSEDRV::MediumLow,
            2 => LSEDRV::MediumHigh,
            3 => LSEDRV::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV::Low
    }
    #[doc = "Medium low drive"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV::MediumLow
    }
    #[doc = "Medium high drive"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV::MediumHigh
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV::High
    }
}
#[doc = "Field `LSEDRV` writer - LSEDRV"]
pub type LSEDRV_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, LSEDRV>;
impl<'a, REG> LSEDRV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::Low)
    }
    #[doc = "Medium low drive"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumLow)
    }
    #[doc = "Medium high drive"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::MediumHigh)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEDRV::High)
    }
}
#[doc = "Field `CSSLSEON` reader - CSSLSEON"]
pub use LSION_R as CSSLSEON_R;
#[doc = "Field `CSSLSEON` writer - CSSLSEON"]
pub use LSION_W as CSSLSEON_W;
#[doc = "CSS on LSE failure detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSED {
    #[doc = "0: No failure detected on LSE (32 kHz oscillator)"]
    NoFailure = 0,
    #[doc = "1: Failure detected on LSE (32 kHz oscillator)"]
    Failure = 1,
}
impl From<CSSLSED> for bool {
    #[inline(always)]
    fn from(variant: CSSLSED) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSLSED` reader - CSS on LSE failure detection flag"]
pub type CSSLSED_R = crate::BitReader<CSSLSED>;
impl CSSLSED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSLSED {
        match self.bits {
            false => CSSLSED::NoFailure,
            true => CSSLSED::Failure,
        }
    }
    #[doc = "No failure detected on LSE (32 kHz oscillator)"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CSSLSED::NoFailure
    }
    #[doc = "Failure detected on LSE (32 kHz oscillator)"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CSSLSED::Failure
    }
}
#[doc = "Field `CSSLSED` writer - CSS on LSE failure detection flag"]
pub type CSSLSED_W<'a, REG> = crate::BitWriter<'a, REG, CSSLSED>;
impl<'a, REG> CSSLSED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No failure detected on LSE (32 kHz oscillator)"]
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut crate::W<REG> {
        self.variant(CSSLSED::NoFailure)
    }
    #[doc = "Failure detected on LSE (32 kHz oscillator)"]
    #[inline(always)]
    pub fn failure(self) -> &'a mut crate::W<REG> {
        self.variant(CSSLSED::Failure)
    }
}
#[doc = "RTC and LCD clock source selection bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL {
    #[doc = "0: No clock"]
    NoClock = 0,
    #[doc = "1: LSE oscillator clock used as RTC clock"]
    Lse = 1,
    #[doc = "2: LSI oscillator clock used as RTC clock"]
    Lsi = 2,
    #[doc = "3: HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\\[1:0\\]
bits in the RCC clock control register (RCC_CR)) used as the RTC clock"]
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
#[doc = "Field `RTCSEL` reader - RTC and LCD clock source selection bits"]
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
    #[doc = "HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\\[1:0\\]
bits in the RCC clock control register (RCC_CR)) used as the RTC clock"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL::Hse
    }
}
#[doc = "Field `RTCSEL` writer - RTC and LCD clock source selection bits"]
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
    #[doc = "HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\\[1:0\\]
bits in the RCC clock control register (RCC_CR)) used as the RTC clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(RTCSEL::Hse)
    }
}
#[doc = "RTC clock enable bit\n\nValue on reset: 0"]
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
#[doc = "Field `RTCEN` reader - RTC clock enable bit"]
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
#[doc = "Field `RTCEN` writer - RTC clock enable bit"]
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
#[doc = "RTC software reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCRSTW {
    #[doc = "1: Resets the RTC peripheral"]
    Reset = 1,
}
impl From<RTCRSTW> for bool {
    #[inline(always)]
    fn from(variant: RTCRSTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRST` reader - RTC software reset bit"]
pub type RTCRST_R = crate::BitReader<RTCRSTW>;
impl RTCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTCRSTW> {
        match self.bits {
            true => Some(RTCRSTW::Reset),
            _ => None,
        }
    }
    #[doc = "Resets the RTC peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RTCRSTW::Reset
    }
}
#[doc = "Field `RTCRST` writer - RTC software reset bit"]
pub type RTCRST_W<'a, REG> = crate::BitWriter<'a, REG, RTCRSTW>;
impl<'a, REG> RTCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resets the RTC peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RTCRSTW::Reset)
    }
}
#[doc = "Remove reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVFW {
    #[doc = "1: Clears the reset flag"]
    Clear = 1,
}
impl From<RMVFW> for bool {
    #[inline(always)]
    fn from(variant: RMVFW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<RMVFW>;
impl RMVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RMVFW> {
        match self.bits {
            true => Some(RMVFW::Clear),
            _ => None,
        }
    }
    #[doc = "Clears the reset flag"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVFW::Clear
    }
}
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG, RMVFW>;
impl<'a, REG> RMVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the reset flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(RMVFW::Clear)
    }
}
#[doc = "Firewall reset flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWRSTFR {
    #[doc = "0: No reset has occured"]
    NoReset = 0,
    #[doc = "1: A reset has occured"]
    Reset = 1,
}
impl From<FWRSTFR> for bool {
    #[inline(always)]
    fn from(variant: FWRSTFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWRSTF` reader - Firewall reset flag"]
pub type FWRSTF_R = crate::BitReader<FWRSTFR>;
impl FWRSTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FWRSTFR {
        match self.bits {
            false => FWRSTFR::NoReset,
            true => FWRSTFR::Reset,
        }
    }
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == FWRSTFR::NoReset
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FWRSTFR::Reset
    }
}
#[doc = "Field `FWRSTF` writer - Firewall reset flag"]
pub type FWRSTF_W<'a, REG> = crate::BitWriter<'a, REG, FWRSTFR>;
impl<'a, REG> FWRSTF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset has occured"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(FWRSTFR::NoReset)
    }
    #[doc = "A reset has occured"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(FWRSTFR::Reset)
    }
}
#[doc = "Field `OBLRSTF` reader - OBLRSTF"]
pub use FWRSTF_R as OBLRSTF_R;
#[doc = "Field `PINRSTF` reader - PIN reset flag"]
pub use FWRSTF_R as PINRSTF_R;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub use FWRSTF_R as PORRSTF_R;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub use FWRSTF_R as SFTRSTF_R;
#[doc = "Field `IWDGRSTF` reader - Independent watchdog reset flag"]
pub use FWRSTF_R as IWDGRSTF_R;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub use FWRSTF_R as WWDGRSTF_R;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub use FWRSTF_R as LPWRRSTF_R;
#[doc = "Field `OBLRSTF` writer - OBLRSTF"]
pub use FWRSTF_W as OBLRSTF_W;
#[doc = "Field `PINRSTF` writer - PIN reset flag"]
pub use FWRSTF_W as PINRSTF_W;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub use FWRSTF_W as PORRSTF_W;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub use FWRSTF_W as SFTRSTF_W;
#[doc = "Field `IWDGRSTF` writer - Independent watchdog reset flag"]
pub use FWRSTF_W as IWDGRSTF_W;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub use FWRSTF_W as WWDGRSTF_W;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag"]
pub use FWRSTF_W as LPWRRSTF_W;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready bit"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - External low-speed oscillator enable bit"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External low-speed oscillator ready bit"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External low-speed oscillator bypass bit"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - LSEDRV"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - CSSLSEON"]
    #[inline(always)]
    pub fn csslseon(&self) -> CSSLSEON_R {
        CSSLSEON_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CSS on LSE failure detection flag"]
    #[inline(always)]
    pub fn csslsed(&self) -> CSSLSED_R {
        CSSLSED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RTC and LCD clock source selection bits"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - RTC clock enable bit"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RTC software reset bit"]
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Firewall reset flag"]
    #[inline(always)]
    pub fn fwrstf(&self) -> FWRSTF_R {
        FWRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - OBLRSTF"]
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready bit"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<CSRrs> {
        LSIRDY_W::new(self, 1)
    }
    #[doc = "Bit 8 - External low-speed oscillator enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<CSRrs> {
        LSEON_W::new(self, 8)
    }
    #[doc = "Bit 10 - External low-speed oscillator bypass bit"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<CSRrs> {
        LSEBYP_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - LSEDRV"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<CSRrs> {
        LSEDRV_W::new(self, 11)
    }
    #[doc = "Bit 13 - CSSLSEON"]
    #[inline(always)]
    #[must_use]
    pub fn csslseon(&mut self) -> CSSLSEON_W<CSRrs> {
        CSSLSEON_W::new(self, 13)
    }
    #[doc = "Bit 14 - CSS on LSE failure detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn csslsed(&mut self) -> CSSLSED_W<CSRrs> {
        CSSLSED_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - RTC and LCD clock source selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<CSRrs> {
        RTCSEL_W::new(self, 16)
    }
    #[doc = "Bit 18 - RTC clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<CSRrs> {
        RTCEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - RTC software reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn rtcrst(&mut self) -> RTCRST_W<CSRrs> {
        RTCRST_W::new(self, 19)
    }
    #[doc = "Bit 23 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 23)
    }
    #[doc = "Bit 24 - Firewall reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn fwrstf(&mut self) -> FWRSTF_W<CSRrs> {
        FWRSTF_W::new(self, 24)
    }
    #[doc = "Bit 25 - OBLRSTF"]
    #[inline(always)]
    #[must_use]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<CSRrs> {
        OBLRSTF_W::new(self, 25)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<CSRrs> {
        PINRSTF_W::new(self, 26)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<CSRrs> {
        PORRSTF_W::new(self, 27)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<CSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<CSRrs> {
        IWDGRSTF_W::new(self, 29)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<CSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
#[doc = "Control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x0c00_0000"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
