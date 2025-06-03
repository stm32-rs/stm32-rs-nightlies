///Register `M1FDRH` reader
pub type R = crate::R<M1FDRHrs>;
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
        f.debug_struct("M1FDRH")
            .field("fdatah", &self.fdatah())
            .finish()
    }
}
/**RAMECC monitor 1 failing data high register

You can [`read`](crate::Reg::read) this register and get [`m1fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#RAMCFG:M1FDRH)*/
pub struct M1FDRHrs;
impl crate::RegisterSpec for M1FDRHrs {
    type Ux = u32;
}
///`read()` method returns [`m1fdrh::R`](R) reader structure
impl crate::Readable for M1FDRHrs {}
///`reset()` method sets M1FDRH to value 0
impl crate::Resettable for M1FDRHrs {}
