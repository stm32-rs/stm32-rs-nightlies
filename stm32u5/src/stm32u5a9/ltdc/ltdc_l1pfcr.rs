#[doc = "Register `LTDC_L1PFCR` reader"]
pub type R = crate::R<LTDC_L1PFCRrs>;
#[doc = "Register `LTDC_L1PFCR` writer"]
pub type W = crate::W<LTDC_L1PFCRrs>;
#[doc = "Field `PF` reader - pixel format These bits configure the pixel format"]
pub type PF_R = crate::FieldReader;
#[doc = "Field `PF` writer - pixel format These bits configure the pixel format"]
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - pixel format These bits configure the pixel format"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - pixel format These bits configure the pixel format"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<LTDC_L1PFCRrs> {
        PF_W::new(self, 0)
    }
}
#[doc = "LTDC layer 1 pixel format configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1pfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1pfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1PFCRrs;
impl crate::RegisterSpec for LTDC_L1PFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l1pfcr::R`](R) reader structure"]
impl crate::Readable for LTDC_L1PFCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1pfcr::W`](W) writer structure"]
impl crate::Writable for LTDC_L1PFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1PFCR to value 0"]
impl crate::Resettable for LTDC_L1PFCRrs {
    const RESET_VALUE: u32 = 0;
}
