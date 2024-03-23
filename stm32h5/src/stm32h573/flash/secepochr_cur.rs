#[doc = "Register `SECEPOCHR_CUR` reader"]
pub type R = crate::R<SECEPOCHR_CURrs>;
#[doc = "Field `SEC_EPOCH` reader - Non-volatile secure EPOCH counter"]
pub type SEC_EPOCH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Non-volatile secure EPOCH counter"]
    #[inline(always)]
    pub fn sec_epoch(&self) -> SEC_EPOCH_R {
        SEC_EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FLASH secure EPOCH register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secepochr_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECEPOCHR_CURrs;
impl crate::RegisterSpec for SECEPOCHR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secepochr_cur::R`](R) reader structure"]
impl crate::Readable for SECEPOCHR_CURrs {}
#[doc = "`reset()` method sets SECEPOCHR_CUR to value 0"]
impl crate::Resettable for SECEPOCHR_CURrs {
    const RESET_VALUE: u32 = 0;
}
