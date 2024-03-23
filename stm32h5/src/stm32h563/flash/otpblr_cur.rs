#[doc = "Register `OTPBLR_CUR` reader"]
pub type R = crate::R<OTPBLR_CURrs>;
#[doc = "Field `LOCKBL` reader - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it’s not possible to remove the write protection. Also if not locked, it is not possible to erase OTP words."]
pub type LOCKBL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it’s not possible to remove the write protection. Also if not locked, it is not possible to erase OTP words."]
    #[inline(always)]
    pub fn lockbl(&self) -> LOCKBL_R {
        LOCKBL_R::new(self.bits)
    }
}
#[doc = "FLASH non-secure OTP block lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otpblr_cur::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTPBLR_CURrs;
impl crate::RegisterSpec for OTPBLR_CURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpblr_cur::R`](R) reader structure"]
impl crate::Readable for OTPBLR_CURrs {}
#[doc = "`reset()` method sets OTPBLR_CUR to value 0"]
impl crate::Resettable for OTPBLR_CURrs {
    const RESET_VALUE: u32 = 0;
}
