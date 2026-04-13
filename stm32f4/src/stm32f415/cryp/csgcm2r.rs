///Register `CSGCM2R` reader
pub type R = crate::R<CSGCM2Rrs>;
///Register `CSGCM2R` writer
pub type W = crate::W<CSGCM2Rrs>;
///Field `CSGCM2R` reader - CSGCM2R
pub type CSGCM2R_R = crate::FieldReader<u32>;
///Field `CSGCM2R` writer - CSGCM2R
pub type CSGCM2R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM2R
    #[inline(always)]
    pub fn csgcm2r(&self) -> CSGCM2R_R {
        CSGCM2R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCM2R")
            .field("csgcm2r", &self.csgcm2r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM2R
    #[inline(always)]
    pub fn csgcm2r(&mut self) -> CSGCM2R_W<'_, CSGCM2Rrs> {
        CSGCM2R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CRYP:CSGCM2R)*/
pub struct CSGCM2Rrs;
impl crate::RegisterSpec for CSGCM2Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm2r::R`](R) reader structure
impl crate::Readable for CSGCM2Rrs {}
///`write(|w| ..)` method takes [`csgcm2r::W`](W) writer structure
impl crate::Writable for CSGCM2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM2R to value 0
impl crate::Resettable for CSGCM2Rrs {}
