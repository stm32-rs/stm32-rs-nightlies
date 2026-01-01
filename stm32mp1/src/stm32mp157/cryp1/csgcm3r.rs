///Register `CSGCM3R` reader
pub type R = crate::R<CSGCM3Rrs>;
///Register `CSGCM3R` writer
pub type W = crate::W<CSGCM3Rrs>;
///Field `CSGCM3` reader - CSGCM3
pub type CSGCM3_R = crate::FieldReader<u32>;
///Field `CSGCM3` writer - CSGCM3
pub type CSGCM3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM3
    #[inline(always)]
    pub fn csgcm3(&self) -> CSGCM3_R {
        CSGCM3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCM3R")
            .field("csgcm3", &self.csgcm3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM3
    #[inline(always)]
    pub fn csgcm3(&mut self) -> CSGCM3_W<'_, CSGCM3Rrs> {
        CSGCM3_W::new(self, 0)
    }
}
/**Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM3R)*/
pub struct CSGCM3Rrs;
impl crate::RegisterSpec for CSGCM3Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm3r::R`](R) reader structure
impl crate::Readable for CSGCM3Rrs {}
///`write(|w| ..)` method takes [`csgcm3r::W`](W) writer structure
impl crate::Writable for CSGCM3Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM3R to value 0
impl crate::Resettable for CSGCM3Rrs {}
