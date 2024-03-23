#[doc = "Register `M4FDRH` reader"]
pub type R = crate::R<M4FDRHrs>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fdrh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4FDRHrs;
impl crate::RegisterSpec for M4FDRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4fdrh::R`](R) reader structure"]
impl crate::Readable for M4FDRHrs {}
#[doc = "`reset()` method sets M4FDRH to value 0"]
impl crate::Resettable for M4FDRHrs {
    const RESET_VALUE: u32 = 0;
}
