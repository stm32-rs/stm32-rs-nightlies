#[doc = "Register `M1FDRH` reader"]
pub type R = crate::R<M1FDRHrs>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m1fdrh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1FDRHrs;
impl crate::RegisterSpec for M1FDRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1fdrh::R`](R) reader structure"]
impl crate::Readable for M1FDRHrs {}
#[doc = "`reset()` method sets M1FDRH to value 0"]
impl crate::Resettable for M1FDRHrs {
    const RESET_VALUE: u32 = 0;
}
