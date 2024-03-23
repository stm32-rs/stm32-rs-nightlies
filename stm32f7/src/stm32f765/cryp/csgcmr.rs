#[doc = "Register `CSGCM%sR` reader"]
pub type R = crate::R<CSGCMRrs>;
#[doc = "Register `CSGCM%sR` writer"]
pub type W = crate::W<CSGCMRrs>;
#[doc = "Field `CSGCMR` reader - CSGCM0R"]
pub type CSGCMR_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMR` writer - CSGCM0R"]
pub type CSGCMR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    pub fn csgcmr(&self) -> CSGCMR_R {
        CSGCMR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM0R"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmr(&mut self) -> CSGCMR_W<CSGCMRrs> {
        CSGCMR_W::new(self, 0)
    }
}
#[doc = "context swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csgcmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csgcmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSGCMRrs;
impl crate::RegisterSpec for CSGCMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csgcmr::R`](R) reader structure"]
impl crate::Readable for CSGCMRrs {}
#[doc = "`write(|w| ..)` method takes [`csgcmr::W`](W) writer structure"]
impl crate::Writable for CSGCMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSGCM%sR to value 0"]
impl crate::Resettable for CSGCMRrs {
    const RESET_VALUE: u32 = 0;
}
