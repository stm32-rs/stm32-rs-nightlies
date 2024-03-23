#[doc = "Register `ADF_GCR` reader"]
pub type R = crate::R<ADF_GCRrs>;
#[doc = "Register `ADF_GCR` writer"]
pub type W = crate::W<ADF_GCRrs>;
#[doc = "Field `TRGO` reader - Trigger output control Set by software and reset by"]
pub type TRGO_R = crate::BitReader;
#[doc = "Field `TRGO` writer - Trigger output control Set by software and reset by"]
pub type TRGO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger output control Set by software and reset by"]
    #[inline(always)]
    pub fn trgo(&self) -> TRGO_R {
        TRGO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output control Set by software and reset by"]
    #[inline(always)]
    #[must_use]
    pub fn trgo(&mut self) -> TRGO_W<ADF_GCRrs> {
        TRGO_W::new(self, 0)
    }
}
#[doc = "ADF Global Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adf_gcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adf_gcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADF_GCRrs;
impl crate::RegisterSpec for ADF_GCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adf_gcr::R`](R) reader structure"]
impl crate::Readable for ADF_GCRrs {}
#[doc = "`write(|w| ..)` method takes [`adf_gcr::W`](W) writer structure"]
impl crate::Writable for ADF_GCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADF_GCR to value 0"]
impl crate::Resettable for ADF_GCRrs {
    const RESET_VALUE: u32 = 0;
}
