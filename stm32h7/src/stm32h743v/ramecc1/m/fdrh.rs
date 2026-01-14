///Register `FDRH` reader
pub type R = crate::R<FDRHrs>;
///Field `FDATAH` reader - Failing data high (64-bit memory)
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Failing data high (64-bit memory)
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDRH")
            .field("fdatah", &self.fdatah())
            .finish()
    }
}
/**RAMECC monitor x failing data high register

You can [`read`](crate::Reg::read) this register and get [`fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FDRHrs;
impl crate::RegisterSpec for FDRHrs {
    type Ux = u32;
}
///`read()` method returns [`fdrh::R`](R) reader structure
impl crate::Readable for FDRHrs {}
///`reset()` method sets FDRH to value 0
impl crate::Resettable for FDRHrs {}
