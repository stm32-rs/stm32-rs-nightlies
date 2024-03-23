#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCRrs>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCRrs>;
#[doc = "External low-speed oscillator enable\n\nValue on reset: 0"]
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
#[doc = "Field `LSEON` reader - External low-speed oscillator enable"]
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
#[doc = "Field `LSEON` writer - External low-speed oscillator enable"]
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
#[doc = "External low-speed oscillator ready\n\nValue on reset: 0"]
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
#[doc = "Field `LSERDY` reader - External low-speed oscillator ready"]
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
#[doc = "External low-speed oscillator bypass\n\nValue on reset: 0"]
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
#[doc = "Field `LSEBYP` reader - External low-speed oscillator bypass"]
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
#[doc = "Field `LSEBYP` writer - External low-speed oscillator bypass"]
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
#[doc = "External low-speed oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEMOD {
    #[doc = "0: LSE oscillator low power mode selection"]
    Low = 0,
    #[doc = "1: LSE oscillator high drive mode selection"]
    High = 1,
}
impl From<LSEMOD> for bool {
    #[inline(always)]
    fn from(variant: LSEMOD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSEMOD` reader - External low-speed oscillator bypass"]
pub type LSEMOD_R = crate::BitReader<LSEMOD>;
impl LSEMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSEMOD {
        match self.bits {
            false => LSEMOD::Low,
            true => LSEMOD::High,
        }
    }
    #[doc = "LSE oscillator low power mode selection"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEMOD::Low
    }
    #[doc = "LSE oscillator high drive mode selection"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEMOD::High
    }
}
#[doc = "Field `LSEMOD` writer - External low-speed oscillator bypass"]
pub type LSEMOD_W<'a, REG> = crate::BitWriter<'a, REG, LSEMOD>;
impl<'a, REG> LSEMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSE oscillator low power mode selection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LSEMOD::Low)
    }
    #[doc = "LSE oscillator high drive mode selection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LSEMOD::High)
    }
}
#[doc = "RTC clock source selection\n\nValue on reset: 0"]
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
#[doc = "RTC clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `RTCEN` writer - RTC clock enable"]
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
#[doc = "Backup domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST {
    #[doc = "0: Reset not activated"]
    Disabled = 0,
    #[doc = "1: Reset the entire RTC domain"]
    Enabled = 1,
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
            false => BDRST::Disabled,
            true => BDRST::Enabled,
        }
    }
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDRST::Disabled
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDRST::Enabled
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
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::Disabled)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BDRST::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsemod(&self) -> LSEMOD_R {
        LSEMOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
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
}
impl W {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<BDCRrs> {
        LSEON_W::new(self, 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    #[doc = "Bit 3 - External low-speed oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsemod(&mut self) -> LSEMOD_W<BDCRrs> {
        LSEMOD_W::new(self, 3)
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
