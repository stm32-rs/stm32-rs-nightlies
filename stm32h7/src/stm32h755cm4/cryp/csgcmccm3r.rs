///Register `CSGCMCCM3R` reader
pub type R = crate::R<CSGCMCCM3Rrs>;
///Register `CSGCMCCM3R` writer
pub type W = crate::W<CSGCMCCM3Rrs>;
///Field `CSGCMCCM3R` reader - CSGCMCCM3R
pub type CSGCMCCM3R_R = crate::FieldReader<u32>;
///Field `CSGCMCCM3R` writer - CSGCMCCM3R
pub type CSGCMCCM3R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM3R
    #[inline(always)]
    pub fn csgcmccm3r(&self) -> CSGCMCCM3R_R {
        CSGCMCCM3R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM3R")
            .field("csgcmccm3r", &self.csgcmccm3r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM3R
    #[inline(always)]
    pub fn csgcmccm3r(&mut self) -> CSGCMCCM3R_W<CSGCMCCM3Rrs> {
        CSGCMCCM3R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm3r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm3r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#CRYP:CSGCMCCM3R)*/
pub struct CSGCMCCM3Rrs;
impl crate::RegisterSpec for CSGCMCCM3Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm3r::R`](R) reader structure
impl crate::Readable for CSGCMCCM3Rrs {}
///`write(|w| ..)` method takes [`csgcmccm3r::W`](W) writer structure
impl crate::Writable for CSGCMCCM3Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM3R to value 0
impl crate::Resettable for CSGCMCCM3Rrs {}
