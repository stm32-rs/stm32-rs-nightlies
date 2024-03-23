#[doc = "Register `LTDC_IER` reader"]
pub type R = crate::R<LTDC_IERrs>;
#[doc = "Register `LTDC_IER` writer"]
pub type W = crate::W<LTDC_IERrs>;
#[doc = "Field `LIE` reader - LIE"]
pub type LIE_R = crate::BitReader;
#[doc = "Field `LIE` writer - LIE"]
pub type LIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUIE` reader - FUIE"]
pub type FUIE_R = crate::BitReader;
#[doc = "Field `FUIE` writer - FUIE"]
pub type FUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - TERRIE"]
pub type TERRIE_R = crate::BitReader;
#[doc = "Field `TERRIE` writer - TERRIE"]
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRIE` reader - RRIE"]
pub type RRIE_R = crate::BitReader;
#[doc = "Field `RRIE` writer - RRIE"]
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LIE"]
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FUIE"]
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TERRIE"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RRIE"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LIE"]
    #[inline(always)]
    #[must_use]
    pub fn lie(&mut self) -> LIE_W<LTDC_IERrs> {
        LIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FUIE"]
    #[inline(always)]
    #[must_use]
    pub fn fuie(&mut self) -> FUIE_W<LTDC_IERrs> {
        FUIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - TERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn terrie(&mut self) -> TERRIE_W<LTDC_IERrs> {
        TERRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - RRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<LTDC_IERrs> {
        RRIE_W::new(self, 3)
    }
}
#[doc = "This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_IERrs;
impl crate::RegisterSpec for LTDC_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_ier::R`](R) reader structure"]
impl crate::Readable for LTDC_IERrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_ier::W`](W) writer structure"]
impl crate::Writable for LTDC_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_IER to value 0"]
impl crate::Resettable for LTDC_IERrs {
    const RESET_VALUE: u32 = 0;
}
