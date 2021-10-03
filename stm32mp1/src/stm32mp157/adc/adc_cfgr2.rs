#[doc = "Register `ADC_CFGR2` reader"]
pub struct R(crate::R<ADC_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CFGR2` writer"]
pub struct W(crate::W<ADC_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CFGR2_SPEC>;
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
impl From<crate::W<ADC_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROVSE` reader - ROVSE"]
pub struct ROVSE_R(crate::FieldReader<bool, bool>);
impl ROVSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVSE` writer - ROVSE"]
pub struct ROVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSE_W<'a> {
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
#[doc = "Field `JOVSE` reader - JOVSE"]
pub struct JOVSE_R(crate::FieldReader<bool, bool>);
impl JOVSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JOVSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JOVSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JOVSE` writer - JOVSE"]
pub struct JOVSE_W<'a> {
    w: &'a mut W,
}
impl<'a> JOVSE_W<'a> {
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
#[doc = "Field `OVSS` reader - OVSS"]
pub struct OVSS_R(crate::FieldReader<u8, u8>);
impl OVSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OVSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVSS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVSS` writer - OVSS"]
pub struct OVSS_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `TROVS` reader - TROVS"]
pub struct TROVS_R(crate::FieldReader<bool, bool>);
impl TROVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TROVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TROVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TROVS` writer - TROVS"]
pub struct TROVS_W<'a> {
    w: &'a mut W,
}
impl<'a> TROVS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ROVSM` reader - ROVSM"]
pub struct ROVSM_R(crate::FieldReader<bool, bool>);
impl ROVSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROVSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROVSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROVSM` writer - ROVSM"]
pub struct ROVSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVSM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RSHIFT1` reader - RSHIFT1"]
pub struct RSHIFT1_R(crate::FieldReader<bool, bool>);
impl RSHIFT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSHIFT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSHIFT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSHIFT1` writer - RSHIFT1"]
pub struct RSHIFT1_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `RSHIFT2` reader - RSHIFT2"]
pub struct RSHIFT2_R(crate::FieldReader<bool, bool>);
impl RSHIFT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSHIFT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSHIFT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSHIFT2` writer - RSHIFT2"]
pub struct RSHIFT2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RSHIFT3` reader - RSHIFT3"]
pub struct RSHIFT3_R(crate::FieldReader<bool, bool>);
impl RSHIFT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSHIFT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSHIFT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSHIFT3` writer - RSHIFT3"]
pub struct RSHIFT3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RSHIFT4` reader - RSHIFT4"]
pub struct RSHIFT4_R(crate::FieldReader<bool, bool>);
impl RSHIFT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSHIFT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSHIFT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSHIFT4` writer - RSHIFT4"]
pub struct RSHIFT4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSHIFT4_W<'a> {
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
#[doc = "Field `OSVR` reader - OSVR"]
pub struct OSVR_R(crate::FieldReader<u16, u16>);
impl OSVR_R {
    pub(crate) fn new(bits: u16) -> Self {
        OSVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSVR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSVR` writer - OSVR"]
pub struct OSVR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `LSHIFT` reader - LSHIFT"]
pub struct LSHIFT_R(crate::FieldReader<u8, u8>);
impl LSHIFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSHIFT` writer - LSHIFT"]
pub struct LSHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ROVSE"]
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - JOVSE"]
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - OVSS"]
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - TROVS"]
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - ROVSM"]
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RSHIFT1"]
    #[inline(always)]
    pub fn rshift1(&self) -> RSHIFT1_R {
        RSHIFT1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RSHIFT2"]
    #[inline(always)]
    pub fn rshift2(&self) -> RSHIFT2_R {
        RSHIFT2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RSHIFT3"]
    #[inline(always)]
    pub fn rshift3(&self) -> RSHIFT3_R {
        RSHIFT3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RSHIFT4"]
    #[inline(always)]
    pub fn rshift4(&self) -> RSHIFT4_R {
        RSHIFT4_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - OSVR"]
    #[inline(always)]
    pub fn osvr(&self) -> OSVR_R {
        OSVR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - LSHIFT"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ROVSE"]
    #[inline(always)]
    pub fn rovse(&mut self) -> ROVSE_W {
        ROVSE_W { w: self }
    }
    #[doc = "Bit 1 - JOVSE"]
    #[inline(always)]
    pub fn jovse(&mut self) -> JOVSE_W {
        JOVSE_W { w: self }
    }
    #[doc = "Bits 5:8 - OVSS"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OVSS_W {
        OVSS_W { w: self }
    }
    #[doc = "Bit 9 - TROVS"]
    #[inline(always)]
    pub fn trovs(&mut self) -> TROVS_W {
        TROVS_W { w: self }
    }
    #[doc = "Bit 10 - ROVSM"]
    #[inline(always)]
    pub fn rovsm(&mut self) -> ROVSM_W {
        ROVSM_W { w: self }
    }
    #[doc = "Bit 11 - RSHIFT1"]
    #[inline(always)]
    pub fn rshift1(&mut self) -> RSHIFT1_W {
        RSHIFT1_W { w: self }
    }
    #[doc = "Bit 12 - RSHIFT2"]
    #[inline(always)]
    pub fn rshift2(&mut self) -> RSHIFT2_W {
        RSHIFT2_W { w: self }
    }
    #[doc = "Bit 13 - RSHIFT3"]
    #[inline(always)]
    pub fn rshift3(&mut self) -> RSHIFT3_W {
        RSHIFT3_W { w: self }
    }
    #[doc = "Bit 14 - RSHIFT4"]
    #[inline(always)]
    pub fn rshift4(&mut self) -> RSHIFT4_W {
        RSHIFT4_W { w: self }
    }
    #[doc = "Bits 16:25 - OSVR"]
    #[inline(always)]
    pub fn osvr(&mut self) -> OSVR_W {
        OSVR_W { w: self }
    }
    #[doc = "Bits 28:31 - LSHIFT"]
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W {
        LSHIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cfgr2](index.html) module"]
pub struct ADC_CFGR2_SPEC;
impl crate::RegisterSpec for ADC_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_cfgr2::R](R) reader structure"]
impl crate::Readable for ADC_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_cfgr2::W](W) writer structure"]
impl crate::Writable for ADC_CFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CFGR2 to value 0"]
impl crate::Resettable for ADC_CFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
