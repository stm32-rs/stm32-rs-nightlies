#[doc = "Register `COUNTR` reader"]
pub type R = crate::R<COUNTRrs>;
#[doc = "Field `COUNT` reader - COUNT"]
pub type COUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - COUNT"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "monotonic counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`countr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COUNTRrs;
impl crate::RegisterSpec for COUNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`countr::R`](R) reader structure"]
impl crate::Readable for COUNTRrs {}
#[doc = "`reset()` method sets COUNTR to value 0"]
impl crate::Resettable for COUNTRrs {
    const RESET_VALUE: u32 = 0;
}
