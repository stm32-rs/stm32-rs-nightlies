///Register `CSGCMCCM5R` reader
pub type R = crate::R<CSGCMCCM5Rrs>;
///Register `CSGCMCCM5R` writer
pub type W = crate::W<CSGCMCCM5Rrs>;
///Field `CSGCMCCM5` reader - CSGCMCCM5
pub type CSGCMCCM5_R = crate::FieldReader<u32>;
///Field `CSGCMCCM5` writer - CSGCMCCM5
pub type CSGCMCCM5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM5
    #[inline(always)]
    pub fn csgcmccm5(&self) -> CSGCMCCM5_R {
        CSGCMCCM5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM5R")
            .field("csgcmccm5", &self.csgcmccm5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM5
    #[inline(always)]
    pub fn csgcmccm5(&mut self) -> CSGCMCCM5_W<'_, CSGCMCCM5Rrs> {
        CSGCMCCM5_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm5r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm5r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#CRYP:CSGCMCCM5R)*/
pub struct CSGCMCCM5Rrs;
impl crate::RegisterSpec for CSGCMCCM5Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm5r::R`](R) reader structure
impl crate::Readable for CSGCMCCM5Rrs {}
///`write(|w| ..)` method takes [`csgcmccm5r::W`](W) writer structure
impl crate::Writable for CSGCMCCM5Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM5R to value 0
impl crate::Resettable for CSGCMCCM5Rrs {}
