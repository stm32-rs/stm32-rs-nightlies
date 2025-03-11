///Register `M5FDRH` reader
pub type R = crate::R<M5FDRHrs>;
///Field `FDATAH` reader - Failing data high (64-bit memory) When an ECC error occurs the FDATAH bitfield contains the MSB part of the data that generated the error.
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing data high (64-bit memory) When an ECC error occurs the FDATAH bitfield contains the MSB part of the data that generated the error.
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5FDRH")
            .field("fdatah", &self.fdatah())
            .finish()
    }
}
/**RAMECC monitor 5 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m5fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RAMCFG:M5FDRH)*/
pub struct M5FDRHrs;
impl crate::RegisterSpec for M5FDRHrs {
    type Ux = u32;
}
///`read()` method returns [`m5fdrh::R`](R) reader structure
impl crate::Readable for M5FDRHrs {}
///`reset()` method sets M5FDRH to value 0
impl crate::Resettable for M5FDRHrs {}
