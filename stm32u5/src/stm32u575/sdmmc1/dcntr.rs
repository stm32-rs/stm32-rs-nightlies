#[doc = "Register `DCNTR` reader"]
pub type R = crate::R<DCNTRrs>;
#[doc = "Field `DATACOUNT` reader - Data count value"]
pub type DATACOUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCNTRrs;
impl crate::RegisterSpec for DCNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcntr::R`](R) reader structure"]
impl crate::Readable for DCNTRrs {}
#[doc = "`reset()` method sets DCNTR to value 0"]
impl crate::Resettable for DCNTRrs {
    const RESET_VALUE: u32 = 0;
}
