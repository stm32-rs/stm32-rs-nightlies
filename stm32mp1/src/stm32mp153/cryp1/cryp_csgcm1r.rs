///Register `CRYP_CSGCM1R` reader
pub type R = crate::R<CRYP_CSGCM1Rrs>;
///Register `CRYP_CSGCM1R` writer
pub type W = crate::W<CRYP_CSGCM1Rrs>;
///Field `CSGCM1` reader - CSGCM1
pub type CSGCM1_R = crate::FieldReader<u32>;
///Field `CSGCM1` writer - CSGCM1
pub type CSGCM1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM1
    #[inline(always)]
    pub fn csgcm1(&self) -> CSGCM1_R {
        CSGCM1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYP_CSGCM1R")
            .field("csgcm1", &self.csgcm1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM1
    #[inline(always)]
    #[must_use]
    pub fn csgcm1(&mut self) -> CSGCM1_W<CRYP_CSGCM1Rrs> {
        CSGCM1_W::new(self, 0)
    }
}
/**Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`cryp_csgcm1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryp_csgcm1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:CRYP_CSGCM1R)*/
pub struct CRYP_CSGCM1Rrs;
impl crate::RegisterSpec for CRYP_CSGCM1Rrs {
    type Ux = u32;
}
///`read()` method returns [`cryp_csgcm1r::R`](R) reader structure
impl crate::Readable for CRYP_CSGCM1Rrs {}
///`write(|w| ..)` method takes [`cryp_csgcm1r::W`](W) writer structure
impl crate::Writable for CRYP_CSGCM1Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRYP_CSGCM1R to value 0
impl crate::Resettable for CRYP_CSGCM1Rrs {
    const RESET_VALUE: u32 = 0;
}
