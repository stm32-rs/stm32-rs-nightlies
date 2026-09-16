///Register `M5FECR` reader
pub type R = crate::R<M5FECRrs>;
///Field `FEC` reader - Failing error code When an ECC error occurs the FEC bitfield contains the ECC failing code that generated the error.
pub type FEC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing error code When an ECC error occurs the FEC bitfield contains the ECC failing code that generated the error.
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
/**RAMECC monitor 5 failing ECC error code register

You can [`read`](crate::Reg::read) this register and get [`m5fecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RAMCFG:M5FECR)*/
pub struct M5FECRrs;
impl crate::RegisterSpec for M5FECRrs {
    type Ux = u32;
}
///`read()` method returns [`m5fecr::R`](R) reader structure
impl crate::Readable for M5FECRrs {}
///`reset()` method sets M5FECR to value 0
impl crate::Resettable for M5FECRrs {}
