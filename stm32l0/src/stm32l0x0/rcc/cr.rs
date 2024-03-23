#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "16 MHz high-speed internal clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16ON {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<HSI16ON> for bool {
    #[inline(always)]
    fn from(variant: HSI16ON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI16ON` reader - 16 MHz high-speed internal clock enable"]
pub type HSI16ON_R = crate::BitReader<HSI16ON>;
impl HSI16ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI16ON {
        match self.bits {
            false => HSI16ON::Disabled,
            true => HSI16ON::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI16ON::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI16ON::Enabled
    }
}
#[doc = "Field `HSI16ON` writer - 16 MHz high-speed internal clock enable"]
pub type HSI16ON_W<'a, REG> = crate::BitWriter<'a, REG, HSI16ON>;
impl<'a, REG> HSI16ON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16ON::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16ON::Enabled)
    }
}
#[doc = "Field `HSI16KERON` reader - High-speed internal clock enable bit for some IP kernels"]
pub use HSI16ON_R as HSI16KERON_R;
#[doc = "Internal high-speed clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16RDYFR {
    #[doc = "0: HSI 16 MHz oscillator not ready"]
    NotReady = 0,
    #[doc = "1: HSI 16 MHz oscillator ready"]
    Ready = 1,
}
impl From<HSI16RDYFR> for bool {
    #[inline(always)]
    fn from(variant: HSI16RDYFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI16RDYF` reader - Internal high-speed clock ready flag"]
pub type HSI16RDYF_R = crate::BitReader<HSI16RDYFR>;
impl HSI16RDYF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI16RDYFR {
        match self.bits {
            false => HSI16RDYFR::NotReady,
            true => HSI16RDYFR::Ready,
        }
    }
    #[doc = "HSI 16 MHz oscillator not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI16RDYFR::NotReady
    }
    #[doc = "HSI 16 MHz oscillator ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI16RDYFR::Ready
    }
}
#[doc = "Field `HSI16RDYF` writer - Internal high-speed clock ready flag"]
pub type HSI16RDYF_W<'a, REG> = crate::BitWriter<'a, REG, HSI16RDYFR>;
impl<'a, REG> HSI16RDYF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI 16 MHz oscillator not ready"]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16RDYFR::NotReady)
    }
    #[doc = "HSI 16 MHz oscillator ready"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16RDYFR::Ready)
    }
}
#[doc = "HSI16DIVEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16DIVEN {
    #[doc = "0: no 16 MHz HSI division requested"]
    NotDivided = 0,
    #[doc = "1: 16 MHz HSI division by 4 requested"]
    Div4 = 1,
}
impl From<HSI16DIVEN> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI16DIVEN` reader - HSI16DIVEN"]
pub type HSI16DIVEN_R = crate::BitReader<HSI16DIVEN>;
impl HSI16DIVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI16DIVEN {
        match self.bits {
            false => HSI16DIVEN::NotDivided,
            true => HSI16DIVEN::Div4,
        }
    }
    #[doc = "no 16 MHz HSI division requested"]
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVEN::NotDivided
    }
    #[doc = "16 MHz HSI division by 4 requested"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVEN::Div4
    }
}
#[doc = "Field `HSI16DIVEN` writer - HSI16DIVEN"]
pub type HSI16DIVEN_W<'a, REG> = crate::BitWriter<'a, REG, HSI16DIVEN>;
impl<'a, REG> HSI16DIVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no 16 MHz HSI division requested"]
    #[inline(always)]
    pub fn not_divided(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16DIVEN::NotDivided)
    }
    #[doc = "16 MHz HSI division by 4 requested"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16DIVEN::Div4)
    }
}
#[doc = "HSI16DIVF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16DIVFR {
    #[doc = "0: 16 MHz HSI clock not divided"]
    NotDivided = 0,
    #[doc = "1: 16 MHz HSI clock divided by 4"]
    Div4 = 1,
}
impl From<HSI16DIVFR> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVFR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI16DIVF` reader - HSI16DIVF"]
pub type HSI16DIVF_R = crate::BitReader<HSI16DIVFR>;
impl HSI16DIVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI16DIVFR {
        match self.bits {
            false => HSI16DIVFR::NotDivided,
            true => HSI16DIVFR::Div4,
        }
    }
    #[doc = "16 MHz HSI clock not divided"]
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVFR::NotDivided
    }
    #[doc = "16 MHz HSI clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVFR::Div4
    }
}
#[doc = "16 MHz high-speed internal clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16OUTEN {
    #[doc = "0: HSI output clock disabled"]
    Disabled = 0,
    #[doc = "1: HSI output clock enabled"]
    Enabled = 1,
}
impl From<HSI16OUTEN> for bool {
    #[inline(always)]
    fn from(variant: HSI16OUTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI16OUTEN` reader - 16 MHz high-speed internal clock output enable"]
pub type HSI16OUTEN_R = crate::BitReader<HSI16OUTEN>;
impl HSI16OUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSI16OUTEN {
        match self.bits {
            false => HSI16OUTEN::Disabled,
            true => HSI16OUTEN::Enabled,
        }
    }
    #[doc = "HSI output clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI16OUTEN::Disabled
    }
    #[doc = "HSI output clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI16OUTEN::Enabled
    }
}
#[doc = "Field `HSI16OUTEN` writer - 16 MHz high-speed internal clock output enable"]
pub type HSI16OUTEN_W<'a, REG> = crate::BitWriter<'a, REG, HSI16OUTEN>;
impl<'a, REG> HSI16OUTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSI output clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16OUTEN::Disabled)
    }
    #[doc = "HSI output clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSI16OUTEN::Enabled)
    }
}
#[doc = "Field `MSION` reader - MSI clock enable bit"]
pub use HSI16ON_R as MSION_R;
#[doc = "Field `MSION` writer - MSI clock enable bit"]
pub use HSI16ON_W as MSION_W;
#[doc = "MSI clock ready flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYR {
    #[doc = "0: Oscillator is not stable"]
    NotReady = 0,
    #[doc = "1: Oscillator is stable"]
    Ready = 1,
}
impl From<MSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSIRDY` reader - MSI clock ready flag"]
pub type MSIRDY_R = crate::BitReader<MSIRDYR>;
impl MSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSIRDYR {
        match self.bits {
            false => MSIRDYR::NotReady,
            true => MSIRDYR::Ready,
        }
    }
    #[doc = "Oscillator is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDYR::NotReady
    }
    #[doc = "Oscillator is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDYR::Ready
    }
}
#[doc = "Field `HSEON` reader - HSE clock enable bit"]
pub use HSI16ON_R as HSEON_R;
#[doc = "Field `HSEON` writer - HSE clock enable bit"]
pub use HSI16ON_W as HSEON_W;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub use MSIRDY_R as HSERDY_R;
#[doc = "HSE clock bypass bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    #[doc = "0: HSE oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: HSE oscillator bypassed"]
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - HSE clock bypass bit"]
pub type HSEBYP_R = crate::BitReader<HSEBYP>;
impl HSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSEBYP {
        match self.bits {
            false => HSEBYP::NotBypassed,
            true => HSEBYP::Bypassed,
        }
    }
    #[doc = "HSE oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    #[doc = "HSE oscillator bypassed"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
#[doc = "Field `HSEBYP` writer - HSE clock bypass bit"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    #[doc = "HSE oscillator bypassed"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
#[doc = "Field `CSSHSEON` reader - Clock security system on HSE enable bit"]
pub use HSI16ON_R as CSSHSEON_R;
#[doc = "Field `CSSHSEON` writer - Clock security system on HSE enable bit"]
pub use HSI16ON_W as CSSHSEON_W;
#[doc = "TC/LCD prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPRE {
    #[doc = "0: HSE divided by 2"]
    Div2 = 0,
    #[doc = "1: HSE divided by 4"]
    Div4 = 1,
    #[doc = "2: HSE divided by 8"]
    Div8 = 2,
    #[doc = "3: HSE divided by 16"]
    Div16 = 3,
}
impl From<RTCPRE> for u8 {
    #[inline(always)]
    fn from(variant: RTCPRE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCPRE {
    type Ux = u8;
}
#[doc = "Field `RTCPRE` reader - TC/LCD prescaler"]
pub type RTCPRE_R = crate::FieldReader<RTCPRE>;
impl RTCPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RTCPRE {
        match self.bits {
            0 => RTCPRE::Div2,
            1 => RTCPRE::Div4,
            2 => RTCPRE::Div8,
            3 => RTCPRE::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "HSE divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTCPRE::Div2
    }
    #[doc = "HSE divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTCPRE::Div4
    }
    #[doc = "HSE divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTCPRE::Div8
    }
    #[doc = "HSE divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTCPRE::Div16
    }
}
#[doc = "Field `RTCPRE` writer - TC/LCD prescaler"]
pub type RTCPRE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RTCPRE>;
impl<'a, REG> RTCPRE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSE divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div2)
    }
    #[doc = "HSE divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div4)
    }
    #[doc = "HSE divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div8)
    }
    #[doc = "HSE divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(RTCPRE::Div16)
    }
}
#[doc = "Field `PLLON` reader - PLL enable bit"]
pub use HSI16ON_R as PLLON_R;
#[doc = "Field `PLLON` writer - PLL enable bit"]
pub use HSI16ON_W as PLLON_W;
#[doc = "PLL clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYR {
    #[doc = "0: PLL unlocked"]
    Unlocked = 0,
    #[doc = "1: PLL locked"]
    Locked = 1,
}
impl From<PLLRDYR> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLRDY` reader - PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader<PLLRDYR>;
impl PLLRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLRDYR {
        match self.bits {
            false => PLLRDYR::Unlocked,
            true => PLLRDYR::Locked,
        }
    }
    #[doc = "PLL unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDYR::Unlocked
    }
    #[doc = "PLL locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDYR::Locked
    }
}
impl R {
    #[doc = "Bit 0 - 16 MHz high-speed internal clock enable"]
    #[inline(always)]
    pub fn hsi16on(&self) -> HSI16ON_R {
        HSI16ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High-speed internal clock enable bit for some IP kernels"]
    #[inline(always)]
    pub fn hsi16keron(&self) -> HSI16KERON_R {
        HSI16KERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSI16DIVEN"]
    #[inline(always)]
    pub fn hsi16diven(&self) -> HSI16DIVEN_R {
        HSI16DIVEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSI16DIVF"]
    #[inline(always)]
    pub fn hsi16divf(&self) -> HSI16DIVF_R {
        HSI16DIVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 16 MHz high-speed internal clock output enable"]
    #[inline(always)]
    pub fn hsi16outen(&self) -> HSI16OUTEN_R {
        HSI16OUTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - MSI clock enable bit"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable bit"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass bit"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock security system on HSE enable bit"]
    #[inline(always)]
    pub fn csshseon(&self) -> CSSHSEON_R {
        CSSHSEON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - TC/LCD prescaler"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - PLL enable bit"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 16 MHz high-speed internal clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16on(&mut self) -> HSI16ON_W<CRrs> {
        HSI16ON_W::new(self, 0)
    }
    #[doc = "Bit 2 - Internal high-speed clock ready flag"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16rdyf(&mut self) -> HSI16RDYF_W<CRrs> {
        HSI16RDYF_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSI16DIVEN"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16diven(&mut self) -> HSI16DIVEN_W<CRrs> {
        HSI16DIVEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - 16 MHz high-speed internal clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsi16outen(&mut self) -> HSI16OUTEN_W<CRrs> {
        HSI16OUTEN_W::new(self, 5)
    }
    #[doc = "Bit 8 - MSI clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<CRrs> {
        MSION_W::new(self, 8)
    }
    #[doc = "Bit 16 - HSE clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - HSE clock bypass bit"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system on HSE enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn csshseon(&mut self) -> CSSHSEON_W<CRrs> {
        CSSHSEON_W::new(self, 19)
    }
    #[doc = "Bits 20:21 - TC/LCD prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<CRrs> {
        RTCPRE_W::new(self, 20)
    }
    #[doc = "Bit 24 - PLL enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<CRrs> {
        PLLON_W::new(self, 24)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CR to value 0x0300"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x0300;
}
