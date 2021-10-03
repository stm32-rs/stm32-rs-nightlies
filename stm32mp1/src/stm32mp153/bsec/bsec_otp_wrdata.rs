#[doc = "Register `BSEC_OTP_WRDATA` reader"]
pub struct R(crate::R<BSEC_OTP_WRDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_WRDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_WRDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_WRDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_WRDATA` writer"]
pub struct W(crate::W<BSEC_OTP_WRDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_WRDATA_SPEC>;
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
impl From<crate::W<BSEC_OTP_WRDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_WRDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRDATA` reader - WRDATA"]
pub struct WRDATA_R(crate::FieldReader<u32, u32>);
impl WRDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        WRDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRDATA` writer - WRDATA"]
pub struct WRDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    pub fn wrdata(&self) -> WRDATA_R {
        WRDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - WRDATA"]
    #[inline(always)]
    pub fn wrdata(&mut self) -> WRDATA_W {
        WRDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC OTP write data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_wrdata](index.html) module"]
pub struct BSEC_OTP_WRDATA_SPEC;
impl crate::RegisterSpec for BSEC_OTP_WRDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_wrdata::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_WRDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_wrdata::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_WRDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSEC_OTP_WRDATA to value 0"]
impl crate::Resettable for BSEC_OTP_WRDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
