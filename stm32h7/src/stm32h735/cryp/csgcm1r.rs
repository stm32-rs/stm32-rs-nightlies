///Register `CSGCM1R` reader
pub type R = crate::R<CSGCM1Rrs>;
///Register `CSGCM1R` writer
pub type W = crate::W<CSGCM1Rrs>;
///Field `CSGCM1R` reader - CSGCM1R
pub type CSGCM1R_R = crate::FieldReader<u32>;
///Field `CSGCM1R` writer - CSGCM1R
pub type CSGCM1R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM1R
    #[inline(always)]
    pub fn csgcm1r(&self) -> CSGCM1R_R {
        CSGCM1R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCM1R")
            .field("csgcm1r", &self.csgcm1r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM1R
    #[inline(always)]
    pub fn csgcm1r(&mut self) -> CSGCM1R_W<'_, CSGCM1Rrs> {
        CSGCM1R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#CRYP:CSGCM1R)*/
pub struct CSGCM1Rrs;
impl crate::RegisterSpec for CSGCM1Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcm1r::R`](R) reader structure
impl crate::Readable for CSGCM1Rrs {}
///`write(|w| ..)` method takes [`csgcm1r::W`](W) writer structure
impl crate::Writable for CSGCM1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCM1R to value 0
impl crate::Resettable for CSGCM1Rrs {}
