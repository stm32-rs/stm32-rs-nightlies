#[doc = "Register `M3FAR` reader"]
pub type R = crate::R<M3FARrs>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3far::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3FARrs;
impl crate::RegisterSpec for M3FARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3far::R`](R) reader structure"]
impl crate::Readable for M3FARrs {}
#[doc = "`reset()` method sets M3FAR to value 0"]
impl crate::Resettable for M3FARrs {
    const RESET_VALUE: u32 = 0;
}
