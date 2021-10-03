#[doc = "Register `EEFDR2` reader"]
pub struct R(crate::R<EEFDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFDR2` writer"]
pub struct W(crate::W<EEFDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFDR2_SPEC>;
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
impl From<crate::W<EEFDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE10FLTR` reader - External Event 10 filter"]
pub struct EE10FLTR_R(crate::FieldReader<u8, u8>);
impl EE10FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE10FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE10FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE10FLTR` writer - External Event 10 filter"]
pub struct EE10FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
        self.w
    }
}
#[doc = "Field `EE10LTCH` reader - External Event 10 latch"]
pub struct EE10LTCH_R(crate::FieldReader<bool, bool>);
impl EE10LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE10LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE10LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE10LTCH` writer - External Event 10 latch"]
pub struct EE10LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10LTCH_W<'a> {
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
#[doc = "Field `EE9FLTR` reader - External Event 9 filter"]
pub struct EE9FLTR_R(crate::FieldReader<u8, u8>);
impl EE9FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE9FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE9FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE9FLTR` writer - External Event 9 filter"]
pub struct EE9FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "Field `EE9LTCH` reader - External Event 9 latch"]
pub struct EE9LTCH_R(crate::FieldReader<bool, bool>);
impl EE9LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE9LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE9LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE9LTCH` writer - External Event 9 latch"]
pub struct EE9LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9LTCH_W<'a> {
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
#[doc = "Field `EE8FLTR` reader - External Event 8 filter"]
pub struct EE8FLTR_R(crate::FieldReader<u8, u8>);
impl EE8FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE8FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE8FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE8FLTR` writer - External Event 8 filter"]
pub struct EE8FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
#[doc = "Field `EE8LTCH` reader - External Event 8 latch"]
pub struct EE8LTCH_R(crate::FieldReader<bool, bool>);
impl EE8LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE8LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE8LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE8LTCH` writer - External Event 8 latch"]
pub struct EE8LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8LTCH_W<'a> {
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
#[doc = "Field `EE7FLTR` reader - External Event 7 filter"]
pub struct EE7FLTR_R(crate::FieldReader<u8, u8>);
impl EE7FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE7FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE7FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE7FLTR` writer - External Event 7 filter"]
pub struct EE7FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `EE7LTCH` reader - External Event 7 latch"]
pub struct EE7LTCH_R(crate::FieldReader<bool, bool>);
impl EE7LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE7LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE7LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE7LTCH` writer - External Event 7 latch"]
pub struct EE7LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7LTCH_W<'a> {
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
#[doc = "Field `EE6FLTR` reader - External Event 6 filter"]
pub struct EE6FLTR_R(crate::FieldReader<u8, u8>);
impl EE6FLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE6FLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE6FLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6FLTR` writer - External Event 6 filter"]
pub struct EE6FLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6FLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `EE6LTCH` reader - External Event 6 latch"]
pub struct EE6LTCH_R(crate::FieldReader<bool, bool>);
impl EE6LTCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE6LTCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE6LTCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6LTCH` writer - External Event 6 latch"]
pub struct EE6LTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6LTCH_W<'a> {
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
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W {
        EE10FLTR_W { w: self }
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W {
        EE10LTCH_W { w: self }
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W {
        EE9FLTR_W { w: self }
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W {
        EE9LTCH_W { w: self }
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W {
        EE8FLTR_W { w: self }
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W {
        EE8LTCH_W { w: self }
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W {
        EE7FLTR_W { w: self }
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W {
        EE7LTCH_W { w: self }
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W {
        EE6FLTR_W { w: self }
    }
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W {
        EE6LTCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefdr2](index.html) module"]
pub struct EEFDR2_SPEC;
impl crate::RegisterSpec for EEFDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefdr2::R](R) reader structure"]
impl crate::Readable for EEFDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefdr2::W](W) writer structure"]
impl crate::Writable for EEFDR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEFDR2 to value 0"]
impl crate::Resettable for EEFDR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
