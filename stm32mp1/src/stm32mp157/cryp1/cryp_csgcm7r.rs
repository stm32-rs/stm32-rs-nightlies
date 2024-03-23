#[doc = "Register `CRYP_CSGCM7R` reader"]
pub type R = crate::R<CRYP_CSGCM7Rrs>;
#[doc = "Register `CRYP_CSGCM7R` writer"]
pub type W = crate::W<CRYP_CSGCM7Rrs>;
#[doc = "Field `CSGCM7` reader - CSGCM7"]
pub type CSGCM7_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM7` writer - CSGCM7"]
pub type CSGCM7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM7"]
    #[inline(always)]
    pub fn csgcm7(&self) -> CSGCM7_R {
        CSGCM7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM7"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm7(&mut self) -> CSGCM7_W<CRYP_CSGCM7Rrs> {
        CSGCM7_W::new(self, 0)
    }
}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm7r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm7r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_CSGCM7Rrs;
impl crate::RegisterSpec for CRYP_CSGCM7Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_csgcm7r::R`](R) reader structure"]
impl crate::Readable for CRYP_CSGCM7Rrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_csgcm7r::W`](W) writer structure"]
impl crate::Writable for CRYP_CSGCM7Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_CSGCM7R to value 0"]
impl crate::Resettable for CRYP_CSGCM7Rrs {
    const RESET_VALUE: u32 = 0;
}
