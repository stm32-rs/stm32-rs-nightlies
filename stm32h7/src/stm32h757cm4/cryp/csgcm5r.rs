///Register `CSGCM5R` reader
pub type R = crate::R<CSGCM5Rrs>;
///Register `CSGCM5R` writer
pub type W = crate::W<CSGCM5Rrs>;
///Field `CSGCM5R` reader - CSGCM5R
pub type CSGCM5R_R = crate::FieldReader<u32>;
///Field `CSGCM5R` writer - CSGCM5R
pub type CSGCM5R_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCM5R
    #[inline(always)]
    pub fn csgcm5r(&self) -> CSGCM5R_R {
        CSGCM5R_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCM5R")
            .field("csgcm5r", &self.csgcm5r())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCM5R
    #[inline(always)]
    pub fn csgcm5r(&mut self) -> CSGCM5R_W<'_, CSGCM5Rrs> {
        CSGCM5R_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcm5r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcm5r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#CRYP:CSGCM5R)*/
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
