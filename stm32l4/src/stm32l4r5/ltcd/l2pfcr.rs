#[doc = "Register `L2PFCR` reader"]
pub type R = crate::R<L2PFCRrs>;
#[doc = "Register `L2PFCR` writer"]
pub type W = crate::W<L2PFCRrs>;
#[doc = "Field `PF` reader - Pixel Format"]
pub type PF_R = crate::FieldReader;
#[doc = "Field `PF` writer - Pixel Format"]
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<L2PFCRrs> {
        PF_W::new(self, 0)
    }
}
#[doc = "LTDC Layer Pixel Format Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2pfcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2pfcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2PFCRrs;
impl crate::RegisterSpec for L2PFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2pfcr::R`](R) reader structure"]
impl crate::Readable for L2PFCRrs {}
#[doc = "`write(|w| ..)` method takes [`l2pfcr::W`](W) writer structure"]
impl crate::Writable for L2PFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2PFCR to value 0"]
impl crate::Resettable for L2PFCRrs {
    const RESET_VALUE: u32 = 0;
}
