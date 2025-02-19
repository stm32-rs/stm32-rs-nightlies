///Register `M2FECR` reader
pub type R = crate::R<M2FECRrs>;
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
        f.debug_struct("M2FECR").field("fec", &self.fec()).finish()
    }
}
/**RAMECC monitor x failing ECC error code register

You can [`read`](crate::Reg::read) this register and get [`m2fecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H742.html#RAMECC1:M2FECR)*/
pub struct M2FECRrs;
impl crate::RegisterSpec for M2FECRrs {
    type Ux = u32;
}
///`read()` method returns [`m2fecr::R`](R) reader structure
impl crate::Readable for M2FECRrs {}
///`reset()` method sets M2FECR to value 0
impl crate::Resettable for M2FECRrs {
    const RESET_VALUE: u32 = 0;
}
