#[doc = "Register `TIM5_SMCR` reader"]
pub struct R(crate::R<TIM5_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_SMCR` writer"]
pub struct W(crate::W<TIM5_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_SMCR_SPEC>;
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
impl From<crate::W<TIM5_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_SMCR_SPEC>) -> Self {
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
#[doc = "Field `ETF` reader - ETF"]
pub struct ETF_R(crate::FieldReader<u8, u8>);
impl ETF_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETF` writer - ETF"]
pub struct ETF_W<'a> {
    w: &'a mut W,
}
impl<'a> ETF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ETPS` reader - ETPS"]
pub struct ETPS_R(crate::FieldReader<u8, u8>);
impl ETPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETPS` writer - ETPS"]
pub struct ETPS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ECE` reader - ECE"]
pub struct ECE_R(crate::FieldReader<bool, bool>);
impl ECE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECE` writer - ECE"]
pub struct ECE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `ETP` reader - ETP"]
pub struct ETP_R(crate::FieldReader<bool, bool>);
impl ETP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETP` writer - ETP"]
pub struct ETP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `SMS3` reader - SMS3"]
pub struct SMS3_R(crate::FieldReader<bool, bool>);
impl SMS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMS3` writer - SMS3"]
pub struct SMS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMS3_W<'a> {
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
#[doc = "Field `TS3` reader - TS3"]
pub struct TS3_R(crate::FieldReader<bool, bool>);
impl TS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS3` writer - TS3"]
pub struct TS3_W<'a> {
    w: &'a mut W,
}
impl<'a> TS3_W<'a> {
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
#[doc = "Field `TS4` reader - TS4"]
pub struct TS4_R(crate::FieldReader<bool, bool>);
impl TS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS4` writer - TS4"]
pub struct TS4_W<'a> {
    w: &'a mut W,
}
impl<'a> TS4_W<'a> {
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
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - ETPS"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - ECE"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ETP"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SMS3"]
    #[inline(always)]
    pub fn sms3(&self) -> SMS3_R {
        SMS3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - TS3"]
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TS4"]
    #[inline(always)]
    pub fn ts4(&self) -> TS4_R {
        TS4_R::new(((self.bits >> 21) & 0x01) != 0)
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
    #[doc = "Bits 8:11 - ETF"]
    #[inline(always)]
    pub fn etf(&mut self) -> ETF_W {
        ETF_W { w: self }
    }
    #[doc = "Bits 12:13 - ETPS"]
    #[inline(always)]
    pub fn etps(&mut self) -> ETPS_W {
        ETPS_W { w: self }
    }
    #[doc = "Bit 14 - ECE"]
    #[inline(always)]
    pub fn ece(&mut self) -> ECE_W {
        ECE_W { w: self }
    }
    #[doc = "Bit 15 - ETP"]
    #[inline(always)]
    pub fn etp(&mut self) -> ETP_W {
        ETP_W { w: self }
    }
    #[doc = "Bit 16 - SMS3"]
    #[inline(always)]
    pub fn sms3(&mut self) -> SMS3_W {
        SMS3_W { w: self }
    }
    #[doc = "Bit 20 - TS3"]
    #[inline(always)]
    pub fn ts3(&mut self) -> TS3_W {
        TS3_W { w: self }
    }
    #[doc = "Bit 21 - TS4"]
    #[inline(always)]
    pub fn ts4(&mut self) -> TS4_W {
        TS4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_smcr](index.html) module"]
pub struct TIM5_SMCR_SPEC;
impl crate::RegisterSpec for TIM5_SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim5_smcr::R](R) reader structure"]
impl crate::Readable for TIM5_SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_smcr::W](W) writer structure"]
impl crate::Writable for TIM5_SMCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM5_SMCR to value 0"]
impl crate::Resettable for TIM5_SMCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
