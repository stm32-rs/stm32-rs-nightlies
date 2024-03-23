#[doc = "Register `RAM5SEAR` reader"]
pub type R = crate::R<RAM5SEARrs>;
#[doc = "Field `ESEA` reader - ESEA"]
pub type ESEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ESEA"]
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram5sear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM5SEARrs;
impl crate::RegisterSpec for RAM5SEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram5sear::R`](R) reader structure"]
impl crate::Readable for RAM5SEARrs {}
#[doc = "`reset()` method sets RAM5SEAR to value 0"]
impl crate::Resettable for RAM5SEARrs {
    const RESET_VALUE: u32 = 0;
}
