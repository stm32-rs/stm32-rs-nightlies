#[doc = "Register `CRYP_CSGCM1R` reader"]
pub type R = crate::R<CRYP_CSGCM1Rrs>;
#[doc = "Register `CRYP_CSGCM1R` writer"]
pub type W = crate::W<CRYP_CSGCM1Rrs>;
#[doc = "Field `CSGCM1` reader - CSGCM1"]
pub type CSGCM1_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM1` writer - CSGCM1"]
pub type CSGCM1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM1"]
    #[inline(always)]
    pub fn csgcm1(&self) -> CSGCM1_R {
        CSGCM1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM1"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm1(&mut self) -> CSGCM1_W<CRYP_CSGCM1Rrs> {
        CSGCM1_W::new(self, 0)
    }
}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_CSGCM1Rrs;
impl crate::RegisterSpec for CRYP_CSGCM1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_csgcm1r::R`](R) reader structure"]
impl crate::Readable for CRYP_CSGCM1Rrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_csgcm1r::W`](W) writer structure"]
impl crate::Writable for CRYP_CSGCM1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_CSGCM1R to value 0"]
impl crate::Resettable for CRYP_CSGCM1Rrs {
    const RESET_VALUE: u32 = 0;
}
