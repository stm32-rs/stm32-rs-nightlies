///Register `CSGCMCCM6R` reader
pub type R = crate::R<CSGCMCCM6Rrs>;
///Register `CSGCMCCM6R` writer
pub type W = crate::W<CSGCMCCM6Rrs>;
///Field `CSGCMCCM6R` reader - CSGCMCCM6R
pub type CSGCMCCM6R_R = crate::FieldReader<u32>;
///Field `CSGCMCCM6R` writer - CSGCMCCM6R
pub type CSGCMCCM6R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM6R
    #[inline(always)]
    pub fn csgcmccm6r(&self) -> CSGCMCCM6R_R {
        CSGCMCCM6R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM6R")
            .field("csgcmccm6r", &self.csgcmccm6r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM6R
    #[inline(always)]
    pub fn csgcmccm6r(&mut self) -> CSGCMCCM6R_W<'_, CSGCMCCM6Rrs> {
        CSGCMCCM6R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#CRYP:CSGCMCCM6R)*/
pub struct CSGCMCCM6Rrs;
impl crate::RegisterSpec for CSGCMCCM6Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm6r::R`](R) reader structure
impl crate::Readable for CSGCMCCM6Rrs {}
///`write(|w| ..)` method takes [`csgcmccm6r::W`](W) writer structure
impl crate::Writable for CSGCMCCM6Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM6R to value 0
impl crate::Resettable for CSGCMCCM6Rrs {}
