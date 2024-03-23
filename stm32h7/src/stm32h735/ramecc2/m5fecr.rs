#[doc = "Register `M5FECR` reader"]
pub type R = crate::R<M5FECRrs>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5fecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5FECRrs;
impl crate::RegisterSpec for M5FECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5fecr::R`](R) reader structure"]
impl crate::Readable for M5FECRrs {}
#[doc = "`reset()` method sets M5FECR to value 0"]
impl crate::Resettable for M5FECRrs {
    const RESET_VALUE: u32 = 0;
}
