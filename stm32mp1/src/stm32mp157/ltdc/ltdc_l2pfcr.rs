#[doc = "Register `LTDC_L2PFCR` reader"]
pub type R = crate::R<LTDC_L2PFCRrs>;
#[doc = "Register `LTDC_L2PFCR` writer"]
pub type W = crate::W<LTDC_L2PFCRrs>;
#[doc = "Field `PF` reader - PF"]
pub type PF_R = crate::FieldReader;
#[doc = "Field `PF` writer - PF"]
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - PF"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PF"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<LTDC_L2PFCRrs> {
        PF_W::new(self, 0)
    }
}
#[doc = "This register defines the pixel format that is used for the stored data in the frame buffer of a layer. The pixel data is read from the frame buffer and then transformed to the internal format 8888 (ARGB).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l2pfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l2pfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L2PFCRrs;
impl crate::RegisterSpec for LTDC_L2PFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l2pfcr::R`](R) reader structure"]
impl crate::Readable for LTDC_L2PFCRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l2pfcr::W`](W) writer structure"]
impl crate::Writable for LTDC_L2PFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L2PFCR to value 0"]
impl crate::Resettable for LTDC_L2PFCRrs {
    const RESET_VALUE: u32 = 0;
}
