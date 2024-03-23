#[doc = "Register `NSEPOCHR_CUR` reader"]
pub type R = crate::R<NSEPOCHR_CURrs>;
#[doc = "Field `NS_EPOCH` reader - Non-volatile non-secure EPOCH counter"]
pub type NS_EPOCH_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Non-volatile non-secure EPOCH counter"]
    #[inline(always)]
    pub fn ns_epoch(&self) -> NS_EPOCH_R {
        NS_EPOCH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FLASH non-secure EPOCH register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nsepochr_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NSEPOCHR_CURrs;
impl crate::RegisterSpec for NSEPOCHR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nsepochr_cur::R`](R) reader structure"]
impl crate::Readable for NSEPOCHR_CURrs {}
#[doc = "`reset()` method sets NSEPOCHR_CUR to value 0"]
impl crate::Resettable for NSEPOCHR_CURrs {
    const RESET_VALUE: u32 = 0;
}
