#[doc = "Register `BSEC_OTP_LOCK` reader"]
pub struct R(crate::R<BSEC_OTP_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BSEC_OTP_LOCK` writer"]
pub struct W(crate::W<BSEC_OTP_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_LOCK_SPEC>;
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
impl From<crate::W<BSEC_OTP_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTP` reader - OTP"]
pub struct OTP_R(crate::FieldReader<bool, bool>);
impl OTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTP` writer - OTP"]
pub struct OTP_W<'a> {
    w: &'a mut W,
}
impl<'a> OTP_W<'a> {
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
#[doc = "Field `ROMLOCK` reader - ROMLOCK"]
pub struct ROMLOCK_R(crate::FieldReader<bool, bool>);
impl ROMLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROMLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROMLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROMLOCK` writer - ROMLOCK"]
pub struct ROMLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ROMLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DENREG` reader - DENREG"]
pub struct DENREG_R(crate::FieldReader<bool, bool>);
impl DENREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DENREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DENREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DENREG` writer - DENREG"]
pub struct DENREG_W<'a> {
    w: &'a mut W,
}
impl<'a> DENREG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `GPLOCK` reader - GPLOCK"]
pub struct GPLOCK_R(crate::FieldReader<bool, bool>);
impl GPLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPLOCK` writer - GPLOCK"]
pub struct GPLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    pub fn romlock(&self) -> ROMLOCK_R {
        ROMLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    pub fn denreg(&self) -> DENREG_R {
        DENREG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    pub fn gplock(&self) -> GPLOCK_R {
        GPLOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    pub fn otp(&mut self) -> OTP_W {
        OTP_W { w: self }
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    pub fn romlock(&mut self) -> ROMLOCK_W {
        ROMLOCK_W { w: self }
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    pub fn denreg(&mut self) -> DENREG_W {
        DENREG_W { w: self }
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    pub fn gplock(&mut self) -> GPLOCK_W {
        GPLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BSEC OTP lock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsec_otp_lock](index.html) module"]
pub struct BSEC_OTP_LOCK_SPEC;
impl crate::RegisterSpec for BSEC_OTP_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsec_otp_lock::R](R) reader structure"]
impl crate::Readable for BSEC_OTP_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bsec_otp_lock::W](W) writer structure"]
impl crate::Writable for BSEC_OTP_LOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BSEC_OTP_LOCK to value 0"]
impl crate::Resettable for BSEC_OTP_LOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
