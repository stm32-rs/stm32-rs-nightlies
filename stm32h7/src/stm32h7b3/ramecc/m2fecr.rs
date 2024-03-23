#[doc = "Register `M2FECR` reader"]
pub type R = crate::R<M2FECRrs>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2FECRrs;
impl crate::RegisterSpec for M2FECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2fecr::R`](R) reader structure"]
impl crate::Readable for M2FECRrs {}
#[doc = "`reset()` method sets M2FECR to value 0"]
impl crate::Resettable for M2FECRrs {
    const RESET_VALUE: u32 = 0;
}
