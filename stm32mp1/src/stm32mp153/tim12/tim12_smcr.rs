#[doc = "Register `TIM12_SMCR` reader"]
pub struct R(crate::R<TIM12_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM12_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM12_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM12_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM12_SMCR` writer"]
pub struct W(crate::W<TIM12_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM12_SMCR_SPEC>;
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
impl From<crate::W<TIM12_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM12_SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMS` reader - SMS"]
pub struct SMS_R(crate::FieldReader<u8, u8>);
impl SMS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMS` writer - SMS"]
pub struct SMS_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `TS` reader - TS"]
pub struct TS_R(crate::FieldReader<u8, u8>);
impl TS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS` writer - TS"]
pub struct TS_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `MSM` reader - MSM"]
pub struct MSM_R(crate::FieldReader<bool, bool>);
impl MSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSM` writer - MSM"]
pub struct MSM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SMS_3` reader - SMS_3"]
pub struct SMS_3_R(crate::FieldReader<bool, bool>);
impl SMS_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMS_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMS_3` writer - SMS_3"]
pub struct SMS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TS_3` reader - TS_3"]
pub struct TS_3_R(crate::FieldReader<bool, bool>);
impl TS_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_3` writer - TS_3"]
pub struct TS_3_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TS_4` reader - TS_4"]
pub struct TS_4_R(crate::FieldReader<bool, bool>);
impl TS_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_4` writer - TS_4"]
pub struct TS_4_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SMS_3"]
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TS_3"]
    #[inline(always)]
    pub fn ts_3(&self) -> TS_3_R {
        TS_3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TS_4"]
    #[inline(always)]
    pub fn ts_4(&self) -> TS_4_R {
        TS_4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMS"]
    #[inline(always)]
    pub fn sms(&mut self) -> SMS_W {
        SMS_W { w: self }
    }
    #[doc = "Bits 4:6 - TS"]
    #[inline(always)]
    pub fn ts(&mut self) -> TS_W {
        TS_W { w: self }
    }
    #[doc = "Bit 7 - MSM"]
    #[inline(always)]
    pub fn msm(&mut self) -> MSM_W {
        MSM_W { w: self }
    }
    #[doc = "Bit 16 - SMS_3"]
    #[inline(always)]
    pub fn sms_3(&mut self) -> SMS_3_W {
        SMS_3_W { w: self }
    }
    #[doc = "Bit 20 - TS_3"]
    #[inline(always)]
    pub fn ts_3(&mut self) -> TS_3_W {
        TS_3_W { w: self }
    }
    #[doc = "Bit 21 - TS_4"]
    #[inline(always)]
    pub fn ts_4(&mut self) -> TS_4_W {
        TS_4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM12 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim12_smcr](index.html) module"]
pub struct TIM12_SMCR_SPEC;
impl crate::RegisterSpec for TIM12_SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim12_smcr::R](R) reader structure"]
impl crate::Readable for TIM12_SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim12_smcr::W](W) writer structure"]
impl crate::Writable for TIM12_SMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM12_SMCR to value 0"]
impl crate::Resettable for TIM12_SMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
