#[doc = "Register `M2FDRH` reader"]
pub type R = crate::R<M2FDRHrs>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2fdrh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2FDRHrs;
impl crate::RegisterSpec for M2FDRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2fdrh::R`](R) reader structure"]
impl crate::Readable for M2FDRHrs {}
#[doc = "`reset()` method sets M2FDRH to value 0"]
impl crate::Resettable for M2FDRHrs {
    const RESET_VALUE: u32 = 0;
}
