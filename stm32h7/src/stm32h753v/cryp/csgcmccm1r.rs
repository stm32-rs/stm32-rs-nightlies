///Register `CSGCMCCM1R` reader
pub type R = crate::R<CSGCMCCM1Rrs>;
///Register `CSGCMCCM1R` writer
pub type W = crate::W<CSGCMCCM1Rrs>;
///Field `CSGCMCCM1` reader - CSGCMCCM1
pub type CSGCMCCM1_R = crate::FieldReader<u32>;
///Field `CSGCMCCM1` writer - CSGCMCCM1
pub type CSGCMCCM1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - CSGCMCCM1
    #[inline(always)]
    pub fn csgcmccm1(&self) -> CSGCMCCM1_R {
        CSGCMCCM1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSGCMCCM1R")
            .field("csgcmccm1", &self.csgcmccm1())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - CSGCMCCM1
    #[inline(always)]
    pub fn csgcmccm1(&mut self) -> CSGCMCCM1_W<'_, CSGCMCCM1Rrs> {
        CSGCMCCM1_W::new(self, 0)
    }
}
/**context swap register

You can [`read`](crate::Reg::read) this register and get [`csgcmccm1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csgcmccm1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#CRYP:CSGCMCCM1R)*/
pub struct CSGCMCCM1Rrs;
impl crate::RegisterSpec for CSGCMCCM1Rrs {
    type Ux = u32;
}
///`read()` method returns [`csgcmccm1r::R`](R) reader structure
impl crate::Readable for CSGCMCCM1Rrs {}
///`write(|w| ..)` method takes [`csgcmccm1r::W`](W) writer structure
impl crate::Writable for CSGCMCCM1Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSGCMCCM1R to value 0
impl crate::Resettable for CSGCMCCM1Rrs {}
