///Register `M1FECR` reader
pub type R = crate::R<M1FECRrs>;
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
        f.debug_struct("M1FECR").field("fec", &self.fec()).finish()
    }
}
/**RAMECC monitor 1 failing ECC error code register

You can [`read`](crate::Reg::read) this register and get [`m1fecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RAMCFG:M1FECR)*/
pub struct M1FECRrs;
impl crate::RegisterSpec for M1FECRrs {
    type Ux = u32;
}
///`read()` method returns [`m1fecr::R`](R) reader structure
impl crate::Readable for M1FECRrs {}
///`reset()` method sets M1FECR to value 0
impl crate::Resettable for M1FECRrs {}
