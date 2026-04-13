///Register `CSGCMCCM7R` reader
pub type R = crate::R<CSGCMCCM7Rrs>;
///Register `CSGCMCCM7R` writer
pub type W = crate::W<CSGCMCCM7Rrs>;
///Field `CSGCMCCM7` reader - CSGCMCCM7
pub type CSGCMCCM7_R = crate::FieldReader<u32>;
///Field `CSGCMCCM7` writer - CSGCMCCM7
pub type CSGCMCCM7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM7
    #[inline(always)]
    pub fn csgcmccm7(&self) -> CSGCMCCM7_R {
        CSGCMCCM7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM7R")
            .field("csgcmccm7", &self.csgcmccm7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM7
    #[inline(always)]
    pub fn csgcmccm7(&mut self) -> CSGCMCCM7_W<'_, CSGCMCCM7Rrs> {
        CSGCMCCM7_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm7r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm7r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753.html#CRYP:CSGCMCCM7R)*/
pub struct CSGCMCCM7Rrs;
impl crate::RegisterSpec for CSGCMCCM7Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm7r::R`](R) reader structure
impl crate::Readable for CSGCMCCM7Rrs {}
///`write(|w| ..)` method takes [`csgcmccm7r::W`](W) writer structure
impl crate::Writable for CSGCMCCM7Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM7R to value 0
impl crate::Resettable for CSGCMCCM7Rrs {}
