#[doc = "Register `BSEC_OTP_CONFIG` reader"]
pub struct R(crate::R<BSEC_OTP_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_CONFIG` writer"]
pub struct W(crate::W<BSEC_OTP_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_CONFIG_SPEC>;
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
impl From<crate::W<BSEC_OTP_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRUP` reader - PWRUP"]
pub struct PWRUP_R(crate::FieldReader<bool, bool>);
impl PWRUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRUP` writer - PWRUP"]
pub struct PWRUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FRC` reader - FRC"]
pub struct FRC_R(crate::FieldReader<u8, u8>);
impl FRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRC` writer - FRC"]
pub struct FRC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `PRGWIDTH` reader - PRGWIDTH"]
pub struct PRGWIDTH_R(crate::FieldReader<u8, u8>);
impl PRGWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRGWIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRGWIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRGWIDTH` writer - PRGWIDTH"]
pub struct PRGWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `TREAD` reader - TREAD"]
pub struct TREAD_R(crate::FieldReader<u8, u8>);
impl TREAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        TREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TREAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TREAD` writer - TREAD"]
pub struct TREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TREAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PWRUP"]
    #[inline(always)]
    pub fn pwrup(&self) -> PWRUP_R {
        PWRUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - FRC"]
    #[inline(always)]
    pub fn frc(&self) -> FRC_R {
        FRC_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:6 - PRGWIDTH"]
    #[inline(always)]
    pub fn prgwidth(&self) -> PRGWIDTH_R {
        PRGWIDTH_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8 - TREAD"]
    #[inline(always)]
    pub fn tread(&self) -> TREAD_R {
        TREAD_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PWRUP"]
    #[inline(always)]
    pub fn pwrup(&mut self) -> PWRUP_W {
        PWRUP_W { w: self }
    }
    #[doc = "Bits 1:2 - FRC"]
    #[inline(always)]
    pub fn frc(&mut self) -> FRC_W {
        FRC_W { w: self }
    }
    #[doc = "Bits 3:6 - PRGWIDTH"]
    #[inline(always)]
    pub fn prgwidth(&mut self) -> PRGWIDTH_W {
        PRGWIDTH_W { w: self }
    }
    #[doc = "Bits 7:8 - TREAD"]
    #[inline(always)]
    pub fn tread(&mut self) -> TREAD_W {
        TREAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC OTP configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_config](index.html) module"]
pub struct BSEC_OTP_CONFIG_SPEC;
impl crate::RegisterSpec for BSEC_OTP_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_config::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_config::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSEC_OTP_CONFIG to value 0x0e"]
impl crate::Resettable for BSEC_OTP_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e
    }
}
