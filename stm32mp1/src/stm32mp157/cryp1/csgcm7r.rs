///Register `CSGCM7R` reader
pub type R = crate::R<CSGCM7Rrs>;
///Register `CSGCM7R` writer
pub type W = crate::W<CSGCM7Rrs>;
///Field `CSGCM7` reader - CSGCM7
pub type CSGCM7_R = crate::FieldReader<u32>;
///Field `CSGCM7` writer - CSGCM7
pub type CSGCM7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM7
    #[inline(always)]
    pub fn csgcm7(&self) -> CSGCM7_R {
        CSGCM7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCM7R")
            .field("csgcm7", &self.csgcm7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM7
    #[inline(always)]
    pub fn csgcm7(&mut self) -> CSGCM7_W<CSGCM7Rrs> {
        CSGCM7_W::new(self, 0)
    }
}
/**Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.

You can [`read`](crate::Reg::read) this register and get [`csgcm7r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm7r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#CRYP1:CSGCM7R)*/
pub struct CSGCM7Rrs;
impl crate::RegisterSpec for CSGCM7Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm7r::R`](R) reader structure
impl crate::Readable for CSGCM7Rrs {}
///`write(|w| ..)` method takes [`csgcm7r::W`](W) writer structure
impl crate::Writable for CSGCM7Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM7R to value 0
impl crate::Resettable for CSGCM7Rrs {}
