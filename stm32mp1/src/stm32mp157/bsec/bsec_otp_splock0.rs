#[doc = "Register `BSEC_OTP_SPLOCK0` reader"]
pub struct R(crate::R<BSEC_OTP_SPLOCK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_SPLOCK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_SPLOCK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_SPLOCK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_SPLOCK0` writer"]
pub struct W(crate::W<BSEC_OTP_SPLOCK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_SPLOCK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BSEC_OTP_SPLOCK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_SPLOCK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPLOCK` reader - SPLOCK"]
pub struct SPLOCK_R(crate::FieldReader<u32, u32>);
impl SPLOCK_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPLOCK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLOCK` writer - SPLOCK"]
pub struct SPLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    pub fn splock(&self) -> SPLOCK_R {
        SPLOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    pub fn splock(&mut self) -> SPLOCK_W {
        SPLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_splock0](index.html) module"]
pub struct BSEC_OTP_SPLOCK0_SPEC;
impl crate::RegisterSpec for BSEC_OTP_SPLOCK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_splock0::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_SPLOCK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_splock0::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_SPLOCK0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSEC_OTP_SPLOCK0 to value 0"]
impl crate::Resettable for BSEC_OTP_SPLOCK0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
