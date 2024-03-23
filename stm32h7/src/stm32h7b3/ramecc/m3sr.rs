#[doc = "Register `M3SR` reader"]
pub type R = crate::R<M3SRrs>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FADD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3SRrs;
impl crate::RegisterSpec for M3SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3sr::R`](R) reader structure"]
impl crate::Readable for M3SRrs {}
#[doc = "`reset()` method sets M3SR to value 0"]
impl crate::Resettable for M3SRrs {
    const RESET_VALUE: u32 = 0;
}
