#[doc = "Register `BSEC_OTP_WRLOCK1` reader"]
pub struct R(crate::R<BSEC_OTP_WRLOCK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_WRLOCK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_WRLOCK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_WRLOCK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRLOCK` reader - WRLOCK"]
pub struct WRLOCK_R(crate::FieldReader<u32, u32>);
impl WRLOCK_R {
    pub(crate) fn new(bits: u32) -> Self {
        WRLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRLOCK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - WRLOCK"]
    #[inline(always)]
    pub fn wrlock(&self) -> WRLOCK_R {
        WRLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_wrlock1](index.html) module"]
pub struct BSEC_OTP_WRLOCK1_SPEC;
impl crate::RegisterSpec for BSEC_OTP_WRLOCK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_wrlock1::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_WRLOCK1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSEC_OTP_WRLOCK1 to value 0"]
impl crate::Resettable for BSEC_OTP_WRLOCK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
