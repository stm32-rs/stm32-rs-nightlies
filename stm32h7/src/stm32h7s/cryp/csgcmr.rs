///Register `CSGCM%sR` reader
pub type R = crate::R<CSGCMRrs>;
///Register `CSGCM%sR` writer
pub type W = crate::W<CSGCMRrs>;
///Field `CSGCM` reader - Context swap for GCM/GMAC modes CRYP_CSGCMxR registers contain the complete internal register states of the CRYP when the GCM or GMAC processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMxR registers are not used in other chaining modes than GCM or GMAC.
pub type CSGCM_R = crate::FieldReader<u32>;
///Field `CSGCM` writer - Context swap for GCM/GMAC modes CRYP_CSGCMxR registers contain the complete internal register states of the CRYP when the GCM or GMAC processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMxR registers are not used in other chaining modes than GCM or GMAC.
pub type CSGCM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Context swap for GCM/GMAC modes CRYP_CSGCMxR registers contain the complete internal register states of the CRYP when the GCM or GMAC processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMxR registers are not used in other chaining modes than GCM or GMAC.
    #[inline(always)]
    pub fn csgcm(&self) -> CSGCM_R {
        CSGCM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMR")
            .field("csgcm", &self.csgcm())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Context swap for GCM/GMAC modes CRYP_CSGCMxR registers contain the complete internal register states of the CRYP when the GCM or GMAC processing of the current task is suspended to process a higher-priority task. Refer to Section 60.4.8: CRYP suspend and resume operations for more details. CRYP_CSGCMxR registers are not used in other chaining modes than GCM or GMAC.
    #[inline(always)]
    pub fn csgcm(&mut self) -> CSGCM_W<'_, CSGCMRrs> {
        CSGCM_W::new(self, 0)
    }
}
/**CRYP context swap GCM registers

You can [`read`](crate::Reg::read) this register and get [`csgcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#CRYP:CSGCM[0]R)*/
pub struct CSGCMRrs;
impl crate::RegisterSpec for CSGCMRrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmr::R`](R) reader structure
impl crate::Readable for CSGCMRrs {}
///`write(|w| ..)` method takes [`csgcmr::W`](W) writer structure
impl crate::Writable for CSGCMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM%sR to value 0
impl crate::Resettable for CSGCMRrs {}
