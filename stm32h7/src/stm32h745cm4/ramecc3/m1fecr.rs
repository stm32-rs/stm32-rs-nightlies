///Register `M1FECR` reader
pub type R = crate::R<M1FECRrs>;
///Register `M1FECR` writer
pub type W = crate::W<M1FECRrs>;
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
        f.debug_struct("M1FECR").field("fec", &self.fec()).finish()
    }
}
impl W {}
/**RAMECC monitor 1 failing error code register

You can [`read`](crate::Reg::read) this register and get [`m1fecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#RAMECC3:M1FECR)*/
pub struct M1FECRrs;
impl crate::RegisterSpec for M1FECRrs {
    type Ux = u32;
}
///`read()` method returns [`m1fecr::R`](R) reader structure
impl crate::Readable for M1FECRrs {}
///`write(|w| ..)` method takes [`m1fecr::W`](W) writer structure
impl crate::Writable for M1FECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets M1FECR to value 0
impl crate::Resettable for M1FECRrs {
    const RESET_VALUE: u32 = 0;
}
