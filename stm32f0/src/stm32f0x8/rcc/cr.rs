#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Internal High Speed clock enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION {
    #[doc = "0: Clock Off"]
    Off = 0,
    #[doc = "1: Clock On"]
    On = 1,
}
impl From<HSION> for bool {
    #[inline(always)]
    fn from(variant: HSION) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSION` reader - Internal High Speed clock enable"]
pub type HSION_R = crate::BitReader<HSION>;
impl HSION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HSION {
        match self.bits {
            false => HSION::Off,
            true => HSION::On,
        }
    }
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION::Off
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION::On
    }
}
#[doc = "Field `HSION` writer - Internal High Speed clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG, HSION>;
impl<'a, REG> HSION_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::Off)
    }
    #[doc = "Clock On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(HSION::On)
    }
}
#[doc = "Internal High Speed clock ready flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR {
    #[doc = "0: Clock not ready"]
    NotReady = 0,
    #[doc = "1: Clock ready"]
    Ready = 1,
}
impl From<HSIRDYR> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSIRDY` reader - Internal High Speed clock ready flag"]
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
    #[doc = "Clock not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR::NotReady
    }
    #[doc = "Clock ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR::Ready
    }
}
#[doc = "Field `HSITRIM` reader - Internal High Speed clock trimming"]
pub type HSITRIM_R = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - Internal High Speed clock trimming"]
pub type HSITRIM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 5>;
#[doc = "Field `HSICAL` reader - Internal High Speed clock Calibration"]
pub type HSICAL_R = crate::FieldReader;
#[doc = "Field `HSEON` reader - External High Speed clock enable"]
pub use HSION_R as HSEON_R;
#[doc = "Field `HSEON` writer - External High Speed clock enable"]
pub use HSION_W as HSEON_W;
#[doc = "Field `HSERDY` reader - External High Speed clock ready flag"]
pub use HSIRDY_R as HSERDY_R;
#[doc = "External High Speed clock Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP {
    #[doc = "0: HSE crystal oscillator not bypassed"]
    NotBypassed = 0,
    #[doc = "1: HSE crystal oscillator bypassed with external clock"]
    Bypassed = 1,
}
impl From<HSEBYP> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSEBYP` reader - External High Speed clock Bypass"]
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
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP::NotBypassed
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP::Bypassed
    }
}
#[doc = "Field `HSEBYP` writer - External High Speed clock Bypass"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG, HSEBYP>;
impl<'a, REG> HSEBYP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::NotBypassed)
    }
    #[doc = "HSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut crate::W<REG> {
        self.variant(HSEBYP::Bypassed)
    }
}
#[doc = "Clock Security System enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON {
    #[doc = "0: Clock security system disabled (clock detector OFF)"]
    Off = 0,
    #[doc = "1: Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    On = 1,
}
impl From<CSSON> for bool {
    #[inline(always)]
    fn from(variant: CSSON) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSSON` reader - Clock Security System enable"]
pub type CSSON_R = crate::BitReader<CSSON>;
impl CSSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CSSON {
        match self.bits {
            false => CSSON::Off,
            true => CSSON::On,
        }
    }
    #[doc = "Clock security system disabled (clock detector OFF)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CSSON::Off
    }
    #[doc = "Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CSSON::On
    }
}
#[doc = "Field `CSSON` writer - Clock Security System enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG, CSSON>;
impl<'a, REG> CSSON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock security system disabled (clock detector OFF)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::Off)
    }
    #[doc = "Clock security system enable (clock detector ON if the HSE is ready, OFF if not)"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CSSON::On)
    }
}
#[doc = "Field `PLLON` reader - PLL enable"]
pub use HSION_R as PLLON_R;
#[doc = "Field `PLLON` writer - PLL enable"]
pub use HSION_W as PLLON_W;
#[doc = "Field `PLLRDY` reader - PLL clock ready flag"]
pub use HSIRDY_R as PLLRDY_R;
impl R {
    #[doc = "Bit 0 - Internal High Speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal High Speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Internal High Speed clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Internal High Speed clock Calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - External High Speed clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External High Speed clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - External High Speed clock Bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - Internal High Speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bits 3:7 - Internal High Speed clock trimming"]
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<CRrs> {
        HSITRIM_W::new(self, 3)
    }
    #[doc = "Bit 16 - External High Speed clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - External High Speed clock Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clock Security System enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
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
#[doc = "`reset()` method sets CR to value 0x83"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x83;
}
