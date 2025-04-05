///Register `M5FECR` reader
pub type R = crate::R<M5FECRrs>;
///Register `M5FECR` writer
pub type W = crate::W<M5FECRrs>;
///Field `FEC` reader - ECC failing code
pub type FEC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ECC failing code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5FECR").field("fec", &self.fec()).finish()
    }
}
impl W {}
/**RAMECC monitor 5 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m5fecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5fecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#RAMECC1:M5FECR)*/
pub struct M5FECRrs;
impl crate::RegisterSpec for M5FECRrs {
    type Ux = u32;
}
///`read()` method returns [`m5fecr::R`](R) reader structure
impl crate::Readable for M5FECRrs {}
///`write(|w| ..)` method takes [`m5fecr::W`](W) writer structure
impl crate::Writable for M5FECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M5FECR to value 0
impl crate::Resettable for M5FECRrs {}
