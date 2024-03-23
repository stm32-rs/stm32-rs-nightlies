#[doc = "Register `DCOUNT` reader"]
pub type R = crate::R<DCOUNTrs>;
#[doc = "Field `DATACOUNT` reader - Data count value"]
pub type DATACOUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "data counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcount::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCOUNTrs;
impl crate::RegisterSpec for DCOUNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcount::R`](R) reader structure"]
impl crate::Readable for DCOUNTrs {}
#[doc = "`reset()` method sets DCOUNT to value 0"]
impl crate::Resettable for DCOUNTrs {
    const RESET_VALUE: u32 = 0;
}
