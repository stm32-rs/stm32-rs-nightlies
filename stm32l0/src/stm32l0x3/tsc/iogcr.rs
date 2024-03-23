#[doc = "Register `IOG%sCR` reader"]
pub type R = crate::R<IOGCRrs>;
#[doc = "Field `CNT` reader - Counter value"]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iogcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOGCRrs;
impl crate::RegisterSpec for IOGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iogcr::R`](R) reader structure"]
impl crate::Readable for IOGCRrs {}
#[doc = "`reset()` method sets IOG%sCR to value 0"]
impl crate::Resettable for IOGCRrs {
    const RESET_VALUE: u32 = 0;
}
