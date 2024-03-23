#[doc = "Register `LTDC_L1CFBLR` reader"]
pub type R = crate::R<LTDC_L1CFBLRrs>;
#[doc = "Register `LTDC_L1CFBLR` writer"]
pub type W = crate::W<LTDC_L1CFBLRrs>;
#[doc = "Field `CFBLL` reader - CFBLL"]
pub type CFBLL_R = crate::FieldReader<u16>;
#[doc = "Field `CFBLL` writer - CFBLL"]
pub type CFBLL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `CFBP` reader - CFBP"]
pub type CFBP_R = crate::FieldReader<u16>;
#[doc = "Field `CFBP` writer - CFBP"]
pub type CFBP_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    #[must_use]
    pub fn cfbll(&mut self) -> CFBLL_W<LTDC_L1CFBLRrs> {
        CFBLL_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    #[must_use]
    pub fn cfbp(&mut self) -> CFBP_W<LTDC_L1CFBLRrs> {
        CFBP_W::new(self, 16)
    }
}
#[doc = "This register defines the color frame buffer line length and pitch.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1cfblr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1cfblr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1CFBLRrs;
impl crate::RegisterSpec for LTDC_L1CFBLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l1cfblr::R`](R) reader structure"]
impl crate::Readable for LTDC_L1CFBLRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1cfblr::W`](W) writer structure"]
impl crate::Writable for LTDC_L1CFBLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1CFBLR to value 0"]
impl crate::Resettable for LTDC_L1CFBLRrs {
    const RESET_VALUE: u32 = 0;
}
