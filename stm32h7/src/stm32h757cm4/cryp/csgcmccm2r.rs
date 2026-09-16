///Register `CSGCMCCM2R` reader
pub type R = crate::R<CSGCMCCM2Rrs>;
///Register `CSGCMCCM2R` writer
pub type W = crate::W<CSGCMCCM2Rrs>;
///Field `CSGCMCCM2R` reader - CSGCMCCM2R
pub type CSGCMCCM2R_R = crate::FieldReader<u32>;
///Field `CSGCMCCM2R` writer - CSGCMCCM2R
pub type CSGCMCCM2R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM2R
    #[inline(always)]
    pub fn csgcmccm2r(&self) -> CSGCMCCM2R_R {
        CSGCMCCM2R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM2R")
            .field("csgcmccm2r", &self.csgcmccm2r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM2R
    #[inline(always)]
    pub fn csgcmccm2r(&mut self) -> CSGCMCCM2R_W<'_, CSGCMCCM2Rrs> {
        CSGCMCCM2R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#CRYP:CSGCMCCM2R)*/
pub struct CSGCMCCM2Rrs;
impl crate::RegisterSpec for CSGCMCCM2Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm2r::R`](R) reader structure
impl crate::Readable for CSGCMCCM2Rrs {}
///`write(|w| ..)` method takes [`csgcmccm2r::W`](W) writer structure
impl crate::Writable for CSGCMCCM2Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM2R to value 0
impl crate::Resettable for CSGCMCCM2Rrs {}
