#[doc = "Register `CRYP_CSGCMCCM1R` reader"]
pub type R = crate::R<CRYP_CSGCMCCM1Rrs>;
#[doc = "Register `CRYP_CSGCMCCM1R` writer"]
pub type W = crate::W<CRYP_CSGCMCCM1Rrs>;
#[doc = "Field `CSGCMCCM1` reader - CSGCMCCM1"]
pub type CSGCMCCM1_R = crate::FieldReader<u32>;
#[doc = "Field `CSGCMCCM1` writer - CSGCMCCM1"]
pub type CSGCMCCM1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM1"]
    #[inline(always)]
    pub fn csgcmccm1(&self) -> CSGCMCCM1_R {
        CSGCMCCM1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM1"]
    #[inline(always)]
    #[must_use]
    pub fn csgcmccm1(&mut self) -> CSGCMCCM1_W<CRYP_CSGCMCCM1Rrs> {
        CSGCMCCM1_W::new(self, 0)
    }
}
#[doc = "These registers contain the complete internal register states of the CRYP processor when the GCM/GMAC or CCM algorithm is selected. They are useful when a context swap has to be performed because a high-priority task needs the cryptographic processor while it is already in use by another task. When such an event occurs, the CRYP_CSGCMCCM0..7R and CRYP_CSGCM0..7R (in GCM/GMAC mode) or CRYP_CSGCMCCM0..7R (in CCM mode) registers have to be read and the values retrieved have to be saved in the system memory space. The cryptographic processor can then be used by the preemptive task. Then when the cryptographic computation is complete, the saved context can be read from memory and written back into the corresponding context swap registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cryp_csgcmccm1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cryp_csgcmccm1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYP_CSGCMCCM1Rrs;
impl crate::RegisterSpec for CRYP_CSGCMCCM1Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cryp_csgcmccm1r::R`](R) reader structure"]
impl crate::Readable for CRYP_CSGCMCCM1Rrs {}
#[doc = "`write(|w| ..)` method takes [`cryp_csgcmccm1r::W`](W) writer structure"]
impl crate::Writable for CRYP_CSGCMCCM1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYP_CSGCMCCM1R to value 0"]
impl crate::Resettable for CRYP_CSGCMCCM1Rrs {
    const RESET_VALUE: u32 = 0;
}
