#[doc = "Register `M2SEAR` reader"]
pub type R = crate::R<M2SEARrs>;
#[doc = "Field `ESEA` reader - ESEA"]
pub type ESEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ESEA"]
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m2sear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2SEARrs;
impl crate::RegisterSpec for M2SEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2sear::R`](R) reader structure"]
impl crate::Readable for M2SEARrs {}
#[doc = "`reset()` method sets M2SEAR to value 0"]
impl crate::Resettable for M2SEARrs {
    const RESET_VALUE: u32 = 0;
}
