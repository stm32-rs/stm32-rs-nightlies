#[doc = "Register `IOG2CR` reader"]
pub type R = crate::R<IOG2CRrs>;
#[doc = "Field `CNT` reader - Counter value"]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iog2cr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOG2CRrs;
impl crate::RegisterSpec for IOG2CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iog2cr::R`](R) reader structure"]
impl crate::Readable for IOG2CRrs {}
#[doc = "`reset()` method sets IOG2CR to value 0"]
impl crate::Resettable for IOG2CRrs {
    const RESET_VALUE: u32 = 0;
}
