#[doc = "Register `M4FECR` reader"]
pub type R = crate::R<M4FECRrs>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FDATAH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m4fecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4FECRrs;
impl crate::RegisterSpec for M4FECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4fecr::R`](R) reader structure"]
impl crate::Readable for M4FECRrs {}
#[doc = "`reset()` method sets M4FECR to value 0"]
impl crate::Resettable for M4FECRrs {
    const RESET_VALUE: u32 = 0;
}
