#[doc = "Register `BSEC_OTP_DISTURBED0` reader"]
pub struct R(crate::R<BSEC_OTP_DISTURBED0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_DISTURBED0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_DISTURBED0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_DISTURBED0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIS` reader - DIS"]
pub struct DIS_R(crate::FieldReader<u32, u32>);
impl DIS_R {
    pub(crate) fn new(bits: u32) -> Self {
        DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - DIS"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_disturbed0](index.html) module"]
pub struct BSEC_OTP_DISTURBED0_SPEC;
impl crate::RegisterSpec for BSEC_OTP_DISTURBED0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_disturbed0::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_DISTURBED0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSEC_OTP_DISTURBED0 to value 0"]
impl crate::Resettable for BSEC_OTP_DISTURBED0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
