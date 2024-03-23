#[doc = "Register `LTDC_L1CLUTWR` writer"]
pub type W = crate::W<LTDC_L1CLUTWRrs>;
#[doc = "Field `BLUE` writer - BLUE"]
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` writer - GREEN"]
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED` writer - RED"]
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLUTADD` writer - CLUTADD"]
pub type CLUTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - BLUE"]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<LTDC_L1CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GREEN"]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<LTDC_L1CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - RED"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<LTDC_L1CLUTWRrs> {
        RED_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CLUTADD"]
    #[inline(always)]
    #[must_use]
    pub fn clutadd(&mut self) -> CLUTADD_W<LTDC_L1CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
#[doc = "This register defines the CLUT address and the RGB value.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1CLUTWRrs;
impl crate::RegisterSpec for LTDC_L1CLUTWRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1clutwr::W`](W) writer structure"]
impl crate::Writable for LTDC_L1CLUTWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1CLUTWR to value 0"]
impl crate::Resettable for LTDC_L1CLUTWRrs {
    const RESET_VALUE: u32 = 0;
}
