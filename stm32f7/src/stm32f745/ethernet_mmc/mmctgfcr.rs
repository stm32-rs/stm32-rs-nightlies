#[doc = "Register `MMCTGFCR` reader"]
pub type R = crate::R<MMCTGFCRrs>;
#[doc = "Field `TGFC` reader - HTL"]
pub type TGFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HTL"]
    #[inline(always)]
    pub fn tgfc(&self) -> TGFC_R {
        TGFC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctgfcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTGFCRrs;
impl crate::RegisterSpec for MMCTGFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfcr::R`](R) reader structure"]
impl crate::Readable for MMCTGFCRrs {}
#[doc = "`reset()` method sets MMCTGFCR to value 0"]
impl crate::Resettable for MMCTGFCRrs {
    const RESET_VALUE: u32 = 0;
}
