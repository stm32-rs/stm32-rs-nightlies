#[doc = "Register `MMCRGUFCR` reader"]
pub type R = crate::R<MMCRGUFCRrs>;
#[doc = "Field `RGUFC` reader - RGUFC"]
pub type RGUFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RGUFC"]
    #[inline(always)]
    pub fn rgufc(&self) -> RGUFC_R {
        RGUFC_R::new(self.bits)
    }
}
#[doc = "MMC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrgufcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRGUFCRrs;
impl crate::RegisterSpec for MMCRGUFCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrgufcr::R`](R) reader structure"]
impl crate::Readable for MMCRGUFCRrs {}
#[doc = "`reset()` method sets MMCRGUFCR to value 0"]
impl crate::Resettable for MMCRGUFCRrs {
    const RESET_VALUE: u32 = 0;
}
