#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `HSION` reader - HSI clock enable"]
pub type HSION_R = crate::BitReader;
#[doc = "Field `HSION` writer - HSI clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERON` reader - HSI always enable for peripheral kernels"]
pub type HSIKERON_R = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSI always enable for peripheral kernels"]
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HSIRDY_R = crate::BitReader;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HSEON_R = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
pub type HSEBYP_R = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLON` reader - Main PLL enable"]
pub type PLLON_R = crate::BitReader;
#[doc = "Field `PLLON` writer - Main PLL enable"]
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - Main PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL enable"]
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL clock ready flag"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - HSI clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<CRrs> {
        HSION_W::new(self, 8)
    }
    #[doc = "Bit 9 - HSI always enable for peripheral kernels"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<CRrs> {
        HSIKERON_W::new(self, 9)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<CRrs> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<CRrs> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<CRrs> {
        CSSON_W::new(self, 19)
    }
    #[doc = "Bit 24 - Main PLL enable"]
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
#[doc = "`reset()` method sets CR to value 0x63"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0x63;
}
