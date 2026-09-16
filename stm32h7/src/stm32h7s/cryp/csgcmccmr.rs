///Register `CSGCMCCM%sR` reader
pub type R = crate::R<CSGCMCCMRrs>;
///Register `CSGCMCCM%sR` writer
pub type W = crate::W<CSGCMCCMRrs>;
///Field `CSGCMCCM` reader - Context swap for GCM/GMAC and CCM modes CRYP_CSGCMCCMxR registers contain the complete internal register states of the CRYP when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMCCMxR registers are not used in other chaining modes than GCM, GMAC or CCM.
pub type CSGCMCCM_R = crate::FieldReader<u32>;
///Field `CSGCMCCM` writer - Context swap for GCM/GMAC and CCM modes CRYP_CSGCMCCMxR registers contain the complete internal register states of the CRYP when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMCCMxR registers are not used in other chaining modes than GCM, GMAC or CCM.
pub type CSGCMCCM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap for GCM/GMAC and CCM modes CRYP_CSGCMCCMxR registers contain the complete internal register states of the CRYP when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMCCMxR registers are not used in other chaining modes than GCM, GMAC or CCM.
    #[inline(always)]
    pub fn csgcmccm(&self) -> CSGCMCCM_R {
        CSGCMCCM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCMR")
            .field("csgcmccm", &self.csgcmccm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap for GCM/GMAC and CCM modes CRYP_CSGCMCCMxR registers contain the complete internal register states of the CRYP when the GCM, GMAC or CCM processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMCCMxR registers are not used in other chaining modes than GCM, GMAC or CCM.
    #[inline(always)]
    pub fn csgcmccm(&mut self) -> CSGCMCCM_W<'_, CSGCMCCMRrs> {
        CSGCMCCM_W::new(self, 0)
    }
}
/**CRYP context swap GCM-CCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmccmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:CSGCMCCM[0]R)*/
pub struct CSGCMCCMRrs;
impl crate::RegisterSpec for CSGCMCCMRrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccmr::R`](R) reader structure
impl crate::Readable for CSGCMCCMRrs {}
///`write(|w| ..)` method takes [`csgcmccmr::W`](W) writer structure
impl crate::Writable for CSGCMCCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM%sR to value 0
impl crate::Resettable for CSGCMCCMRrs {}
