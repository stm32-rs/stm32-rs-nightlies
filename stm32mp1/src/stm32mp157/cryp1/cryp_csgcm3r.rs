#[doc = "Register `CRYP_CSGCM3R` reader"]
pub type R = crate::R<CRYP_CSGCM3Rrs>;
#[doc = "Register `CRYP_CSGCM3R` writer"]
pub type W = crate::W<CRYP_CSGCM3Rrs>;
#[doc = "Field `CSGCM3` reader - CSGCM3"]
pub type CSGCM3_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCM3` writer - CSGCM3"]
pub type CSGCM3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCM3"]
    #[inline(always)]
    pub fn csgcm3(&self) -> CSGCM3_R {
        CSGCM3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM3"]
    #[inline(always)]
    #[must_use]
    pub fn csgcm3(&mut self) -> CSGCM3_W<CRYP_CSGCM3Rrs> {
        CSGCM3_W::new(self, 0)
    }
}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcm3r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcm3r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_CSGCM3Rrs;
impl crate::RegisterSpec for CRYP_CSGCM3Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_csgcm3r::R`](R) reader structure"]
impl crate::Readable for CRYP_CSGCM3Rrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_csgcm3r::W`](W) writer structure"]
impl crate::Writable for CRYP_CSGCM3Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_CSGCM3R to value 0"]
impl crate::Resettable for CRYP_CSGCM3Rrs {
    const RESET_VALUE: u32 = 0;
}
