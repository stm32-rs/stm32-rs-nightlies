#[doc = "Register `MMCRFCECR` reader"]
pub type R = crate::R<MMCRFCECRrs>;
#[doc = "Field `RFCFC` reader - RFCFC"]
pub type RFCFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RFCFC"]
    #[inline(always)]
    pub fn rfcfc(&self) -> RFCFC_R {
        RFCFC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfcecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRFCECRrs;
impl crate::RegisterSpec for MMCRFCECRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfcecr::R`](R) reader structure"]
impl crate::Readable for MMCRFCECRrs {}
#[doc = "`reset()` method sets MMCRFCECR to value 0"]
impl crate::Resettable for MMCRFCECRrs {
    const RESET_VALUE: u32 = 0;
}
