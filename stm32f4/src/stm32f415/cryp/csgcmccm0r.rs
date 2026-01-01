///Register `CSGCMCCM0R` reader
pub type R = crate::R<CSGCMCCM0Rrs>;
///Register `CSGCMCCM0R` writer
pub type W = crate::W<CSGCMCCM0Rrs>;
///Field `CSGCMCCM0R` reader - CSGCMCCM0R
pub type CSGCMCCM0R_R = crate::FieldReader<u32>;
///Field `CSGCMCCM0R` writer - CSGCMCCM0R
pub type CSGCMCCM0R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM0R
    #[inline(always)]
    pub fn csgcmccm0r(&self) -> CSGCMCCM0R_R {
        CSGCMCCM0R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM0R")
            .field("csgcmccm0r", &self.csgcmccm0r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM0R
    #[inline(always)]
    pub fn csgcmccm0r(&mut self) -> CSGCMCCM0R_W<'_, CSGCMCCM0Rrs> {
        CSGCMCCM0R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#CRYP:CSGCMCCM0R)*/
pub struct CSGCMCCM0Rrs;
impl crate::RegisterSpec for CSGCMCCM0Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm0r::R`](R) reader structure
impl crate::Readable for CSGCMCCM0Rrs {}
///`write(|w| ..)` method takes [`csgcmccm0r::W`](W) writer structure
impl crate::Writable for CSGCMCCM0Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM0R to value 0
impl crate::Resettable for CSGCMCCM0Rrs {}
