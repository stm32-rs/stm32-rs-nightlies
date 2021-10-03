#[doc = "Register `EEFER1` reader"]
pub struct R(crate::R<EEFER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFER1` writer"]
pub struct W(crate::W<EEFER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFER1_SPEC>;
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
impl From<crate::W<EEFER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE5FLTR` reader - External Event 5 filter"]
pub struct EE5FLTR_R(crate::FieldReader<u8, u8>);
impl EE5FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE5FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE5FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE5FLTR` writer - External Event 5 filter"]
pub struct EE5FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
#[doc = "Field `EE5LTCH` reader - External Event 5 latch"]
pub struct EE5LTCH_R(crate::FieldReader<bool, bool>);
impl EE5LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE5LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE5LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE5LTCH` writer - External Event 5 latch"]
pub struct EE5LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE5LTCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `EE4FLTR` reader - External Event 4 filter"]
pub struct EE4FLTR_R(crate::FieldReader<u8, u8>);
impl EE4FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE4FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE4FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE4FLTR` writer - External Event 4 filter"]
pub struct EE4FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "Field `EE4LTCH` reader - External Event 4 latch"]
pub struct EE4LTCH_R(crate::FieldReader<bool, bool>);
impl EE4LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE4LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE4LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE4LTCH` writer - External Event 4 latch"]
pub struct EE4LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE4LTCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `EE3FLTR` reader - External Event 3 filter"]
pub struct EE3FLTR_R(crate::FieldReader<u8, u8>);
impl EE3FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE3FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE3FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE3FLTR` writer - External Event 3 filter"]
pub struct EE3FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "Field `EE3LTCH` reader - External Event 3 latch"]
pub struct EE3LTCH_R(crate::FieldReader<bool, bool>);
impl EE3LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE3LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE3LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE3LTCH` writer - External Event 3 latch"]
pub struct EE3LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE3LTCH_W<'a> {
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
#[doc = "Field `EE2FLTR` reader - External Event 2 filter"]
pub struct EE2FLTR_R(crate::FieldReader<u8, u8>);
impl EE2FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE2FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE2FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE2FLTR` writer - External Event 2 filter"]
pub struct EE2FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `EE2LTCH` reader - External Event 2 latch"]
pub struct EE2LTCH_R(crate::FieldReader<bool, bool>);
impl EE2LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE2LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE2LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE2LTCH` writer - External Event 2 latch"]
pub struct EE2LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE2LTCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `EE1FLTR` reader - External Event 1 filter"]
pub struct EE1FLTR_R(crate::FieldReader<u8, u8>);
impl EE1FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE1FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE1FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1FLTR` writer - External Event 1 filter"]
pub struct EE1FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `EE1LTCH` reader - External Event 1 latch"]
pub struct EE1LTCH_R(crate::FieldReader<bool, bool>);
impl EE1LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE1LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE1LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE1LTCH` writer - External Event 1 latch"]
pub struct EE1LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE1LTCH_W<'a> {
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
impl R {
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W {
        EE5FLTR_W { w: self }
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W {
        EE5LTCH_W { w: self }
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W {
        EE4FLTR_W { w: self }
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W {
        EE4LTCH_W { w: self }
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W {
        EE3FLTR_W { w: self }
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W {
        EE3LTCH_W { w: self }
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W {
        EE2FLTR_W { w: self }
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W {
        EE2LTCH_W { w: self }
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W {
        EE1FLTR_W { w: self }
    }
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W {
        EE1LTCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefer1](index.html) module"]
pub struct EEFER1_SPEC;
impl crate::RegisterSpec for EEFER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefer1::R](R) reader structure"]
impl crate::Readable for EEFER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefer1::W](W) writer structure"]
impl crate::Writable for EEFER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEFER1 to value 0"]
impl crate::Resettable for EEFER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
