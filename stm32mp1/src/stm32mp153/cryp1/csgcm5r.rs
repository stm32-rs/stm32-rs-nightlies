///Register `CSGCM5R` reader
pub type R = crate::R<CSGCM5Rrs>;
///Register `CSGCM5R` writer
pub type W = crate::W<CSGCM5Rrs>;
///Field `CSGCM5` reader - CSGCM5
pub type CSGCM5_R = crate::FieldReader<u32>;
///Field `CSGCM5` writer - CSGCM5
pub type CSGCM5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM5
    #[inline(always)]
    pub fn csgcm5(&self) -> CSGCM5_R {
        CSGCM5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCM5R")
            .field("csgcm5", &self.csgcm5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM5
    #[inline(always)]
    pub fn csgcm5(&mut self) -> CSGCM5_W<'_, CSGCM5Rrs> {
        CSGCM5_W::new(self, 0)
    }
}
/**Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm5r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm5r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:CSGCM5R)*/
pub struct CSGCM5Rrs;
impl crate::RegisterSpec for CSGCM5Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm5r::R`](R) reader structure
impl crate::Readable for CSGCM5Rrs {}
///`write(|w| ..)` method takes [`csgcm5r::W`](W) writer structure
impl crate::Writable for CSGCM5Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM5R to value 0
impl crate::Resettable for CSGCM5Rrs {}
