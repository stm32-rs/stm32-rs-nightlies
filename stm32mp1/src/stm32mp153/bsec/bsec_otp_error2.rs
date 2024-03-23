#[doc = "Register `BSEC_OTP_ERROR2` reader"]
pub type R = crate::R<BSEC_OTP_ERROR2rs>;
#[doc = "Field `ERR` reader - ERR"]
pub type ERR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ERR"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(self.bits)
    }
}
#[doc = "BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_error2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_ERROR2rs;
impl crate::RegisterSpec for BSEC_OTP_ERROR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_error2::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_ERROR2rs {}
#[doc = "`reset()` method sets BSEC_OTP_ERROR2 to value 0"]
impl crate::Resettable for BSEC_OTP_ERROR2rs {
    const RESET_VALUE: u32 = 0;
}
