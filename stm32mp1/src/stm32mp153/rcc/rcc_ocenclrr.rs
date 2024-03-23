#[doc = "Register `RCC_OCENCLRR` reader"]
pub type R = crate::R<RCC_OCENCLRRrs>;
#[doc = "Register `RCC_OCENCLRR` writer"]
pub type W = crate::W<RCC_OCENCLRRrs>;
#[doc = "Field `HSION` reader - HSION"]
pub type HSION_R = crate::BitReader;
#[doc = "Field `HSION` writer - HSION"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERON` reader - HSIKERON"]
pub type HSIKERON_R = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSIKERON"]
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSION` reader - CSION"]
pub type CSION_R = crate::BitReader;
#[doc = "Field `CSION` writer - CSION"]
pub type CSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSIKERON` reader - CSIKERON"]
pub type CSIKERON_R = crate::BitReader;
#[doc = "Field `CSIKERON` writer - CSIKERON"]
pub type CSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIGBYP` reader - DIGBYP"]
pub type DIGBYP_R = crate::BitReader;
#[doc = "Field `DIGBYP` writer - DIGBYP"]
pub type DIGBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEON` reader - HSEON"]
pub type HSEON_R = crate::BitReader;
#[doc = "Field `HSEON` writer - HSEON"]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEKERON` reader - HSEKERON"]
pub type HSEKERON_R = crate::BitReader;
#[doc = "Field `HSEKERON` writer - HSEKERON"]
pub type HSEKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEBYP` reader - HSEBYP"]
pub type HSEBYP_R = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSEBYP"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    pub fn csion(&self) -> CSION_R {
        CSION_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    pub fn csikeron(&self) -> CSIKERON_R {
        CSIKERON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    pub fn hsekeron(&self) -> HSEKERON_R {
        HSEKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSION"]
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<RCC_OCENCLRRrs> {
        HSION_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSIKERON"]
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<RCC_OCENCLRRrs> {
        HSIKERON_W::new(self, 1)
    }
    #[doc = "Bit 4 - CSION"]
    #[inline(always)]
    #[must_use]
    pub fn csion(&mut self) -> CSION_W<RCC_OCENCLRRrs> {
        CSION_W::new(self, 4)
    }
    #[doc = "Bit 5 - CSIKERON"]
    #[inline(always)]
    #[must_use]
    pub fn csikeron(&mut self) -> CSIKERON_W<RCC_OCENCLRRrs> {
        CSIKERON_W::new(self, 5)
    }
    #[doc = "Bit 7 - DIGBYP"]
    #[inline(always)]
    #[must_use]
    pub fn digbyp(&mut self) -> DIGBYP_W<RCC_OCENCLRRrs> {
        DIGBYP_W::new(self, 7)
    }
    #[doc = "Bit 8 - HSEON"]
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<RCC_OCENCLRRrs> {
        HSEON_W::new(self, 8)
    }
    #[doc = "Bit 9 - HSEKERON"]
    #[inline(always)]
    #[must_use]
    pub fn hsekeron(&mut self) -> HSEKERON_W<RCC_OCENCLRRrs> {
        HSEKERON_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSEBYP"]
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<RCC_OCENCLRRrs> {
        HSEBYP_W::new(self, 10)
    }
}
#[doc = "This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ocenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ocenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_OCENCLRRrs;
impl crate::RegisterSpec for RCC_OCENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ocenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_OCENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ocenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_OCENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_OCENCLRR to value 0x01"]
impl crate::Resettable for RCC_OCENCLRRrs {
    const RESET_VALUE: u32 = 0x01;
}
