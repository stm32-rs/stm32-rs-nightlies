#[doc = "Register `BSEC_OTP_DATA79` reader"]
pub struct R(crate::R<BSEC_OTP_DATA79_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_DATA79_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_DATA79_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_DATA79_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_DATA79` writer"]
pub struct W(crate::W<BSEC_OTP_DATA79_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_DATA79_SPEC>;
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
impl From<crate::W<BSEC_OTP_DATA79_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_DATA79_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - DATA"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\\[6:0\\]
(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_data79](index.html) module"]
pub struct BSEC_OTP_DATA79_SPEC;
impl crate::RegisterSpec for BSEC_OTP_DATA79_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_data79::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_DATA79_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_data79::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_DATA79_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSEC_OTP_DATA79 to value 0"]
impl crate::Resettable for BSEC_OTP_DATA79_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
