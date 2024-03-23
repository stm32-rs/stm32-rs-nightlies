#[doc = "Register `BSEC_OTP_DISTURBED2` reader"]
pub type R = crate::R<BSEC_OTP_DISTURBED2rs>;
#[doc = "Field `DIS` reader - DIS"]
pub type DIS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DIS"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(self.bits)
    }
}
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_disturbed2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_DISTURBED2rs;
impl crate::RegisterSpec for BSEC_OTP_DISTURBED2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_disturbed2::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_DISTURBED2rs {}
#[doc = "`reset()` method sets BSEC_OTP_DISTURBED2 to value 0"]
impl crate::Resettable for BSEC_OTP_DISTURBED2rs {
    const RESET_VALUE: u32 = 0;
}
