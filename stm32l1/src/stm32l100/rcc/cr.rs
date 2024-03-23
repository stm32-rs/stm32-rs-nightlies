#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Internal high-speed clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Disabled,
            true => HSION::Enabled,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION::Disabled
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION::Enabled
    }
}
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Enabled)
    }
}
#[doc = "Internal high-speed clock ready flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    #[doc = "0: Oscillator is not stable"]
    NotReady = 0,
    #[doc = "1: Oscillator is stable"]
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - Internal high-speed clock ready flag"]
pub type HSIRDY_R = crate::BitReader<HSIRDYR>;
impl HSIRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSIRDYR {
        match self.bits {
            false => HSIRDYR::NotReady,
            true => HSIRDYR::Ready,
        }
    }
    #[doc = "Oscillator is not stable"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    #[doc = "Oscillator is stable"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
#[doc = "Field `MSION` reader - MSI clock enable"]
pub use HSION_R as MSION_R;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub use HSION_R as HSEON_R;
#[doc = "Field `MSION` writer - MSI clock enable"]
pub use HSION_W as MSION_W;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub use HSION_W as HSEON_W;
#[doc = "Field `MSIRDY` reader - MSI clock ready flag"]
pub use HSIRDY_R as MSIRDY_R;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub use HSIRDY_R as HSERDY_R;
#[doc = "HSE clock bypass\n\nValue on reset: 0"]
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
#[doc = "Field `HSEBYP` reader - HSE clock bypass"]
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
#[doc = "Field `HSEBYP` writer - HSE clock bypass"]
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
#[doc = "Field `PLLON` reader - PLL enable"]
pub use HSION_R as PLLON_R;
#[doc = "Field `PLLON` writer - PLL enable"]
pub use HSION_W as PLLON_W;
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
#[doc = "Field `CSSON` reader - Clock security system enable"]
pub use HSION_R as CSSON_R;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub use HSION_W as CSSON_W;
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
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - MSI clock enable"]
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSI clock ready flag"]
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Clock security system enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - TC/LCD prescaler"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bit 8 - MSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<CRrs> {
        MSION_W::new(self, 8)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<CRrs> {
        PLLON_W::new(self, 24)
    }
    #[doc = "Bit 28 - Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 28)
    }
    #[doc = "Bits 29:30 - TC/LCD prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<CRrs> {
        RTCPRE_W::new(self, 29)
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
