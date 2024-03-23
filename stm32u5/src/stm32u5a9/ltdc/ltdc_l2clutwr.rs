#[doc = "Register `LTDC_L2CLUTWR` writer"]
pub type W = crate::W<LTDC_L2CLUTWRrs>;
#[doc = "Field `BLUE` writer - blue value These bits configure the blue value."]
pub type BLUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` writer - green value These bits configure the green value."]
pub type GREEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED` writer - red value These bits configure the red value."]
pub type RED_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLUTADD` writer - CLUT address These bits configure the CLUT address (color position within the CLUT) of each RGB value."]
pub type CLUTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - blue value These bits configure the blue value."]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<LTDC_L2CLUTWRrs> {
        BLUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - green value These bits configure the green value."]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<LTDC_L2CLUTWRrs> {
        GREEN_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - red value These bits configure the red value."]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<LTDC_L2CLUTWRrs> {
        RED_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - CLUT address These bits configure the CLUT address (color position within the CLUT) of each RGB value."]
    #[inline(always)]
    #[must_use]
    pub fn clutadd(&mut self) -> CLUTADD_W<LTDC_L2CLUTWRrs> {
        CLUTADD_W::new(self, 24)
    }
}
#[doc = "LTDC layer 2 CLUT write register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l2clutwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L2CLUTWRrs;
impl crate::RegisterSpec for LTDC_L2CLUTWRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ltdc_l2clutwr::W`](W) writer structure"]
impl crate::Writable for LTDC_L2CLUTWRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L2CLUTWR to value 0"]
impl crate::Resettable for LTDC_L2CLUTWRrs {
    const RESET_VALUE: u32 = 0;
}
