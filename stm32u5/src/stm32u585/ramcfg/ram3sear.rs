#[doc = "Register `RAM3SEAR` reader"]
pub type R = crate::R<RAM3SEARrs>;
#[doc = "Field `ESEA` reader - ESEA"]
pub type ESEA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ESEA"]
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
#[doc = "RAMCFG RAM x ECC single error address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram3sear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAM3SEARrs;
impl crate::RegisterSpec for RAM3SEARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram3sear::R`](R) reader structure"]
impl crate::Readable for RAM3SEARrs {}
#[doc = "`reset()` method sets RAM3SEAR to value 0"]
impl crate::Resettable for RAM3SEARrs {
    const RESET_VALUE: u32 = 0;
}
