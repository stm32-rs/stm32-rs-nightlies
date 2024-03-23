#[doc = "Register `M5FDRH` reader"]
pub type R = crate::R<M5FDRHrs>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fdrh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5FDRHrs;
impl crate::RegisterSpec for M5FDRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5fdrh::R`](R) reader structure"]
impl crate::Readable for M5FDRHrs {}
#[doc = "`reset()` method sets M5FDRH to value 0"]
impl crate::Resettable for M5FDRHrs {
    const RESET_VALUE: u32 = 0;
}
