#[doc = "Register `LTDC_L1CFBAR` reader"]
pub type R = crate::R<LTDC_L1CFBARrs>;
#[doc = "Register `LTDC_L1CFBAR` writer"]
pub type W = crate::W<LTDC_L1CFBARrs>;
#[doc = "Field `CFBADD` reader - CFBADD"]
pub type CFBADD_R = crate::FieldReader<u32>;
#[doc = "Field `CFBADD` writer - CFBADD"]
pub type CFBADD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CFBADD"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CFBADD"]
    #[inline(always)]
    #[must_use]
    pub fn cfbadd(&mut self) -> CFBADD_W<LTDC_L1CFBARrs> {
        CFBADD_W::new(self, 0)
    }
}
#[doc = "This register defines the color frame buffer start address which has to point to the address where the pixel data of the top left pixel of a layer is stored in the frame buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1cfbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1cfbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1CFBARrs;
impl crate::RegisterSpec for LTDC_L1CFBARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l1cfbar::R`](R) reader structure"]
impl crate::Readable for LTDC_L1CFBARrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1cfbar::W`](W) writer structure"]
impl crate::Writable for LTDC_L1CFBARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1CFBAR to value 0"]
impl crate::Resettable for LTDC_L1CFBARrs {
    const RESET_VALUE: u32 = 0;
}
