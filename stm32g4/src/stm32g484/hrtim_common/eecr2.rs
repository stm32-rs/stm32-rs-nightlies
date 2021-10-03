#[doc = "Register `EECR2` reader"]
pub struct R(crate::R<EECR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR2` writer"]
pub struct W(crate::W<EECR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR2_SPEC>;
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
impl From<crate::W<EECR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE6SRC` reader - EE6SRC"]
pub struct EE6SRC_R(crate::FieldReader<u8, u8>);
impl EE6SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE6SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE6SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6SRC` writer - EE6SRC"]
pub struct EE6SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `EE6POL` reader - EE6POL"]
pub struct EE6POL_R(crate::FieldReader<bool, bool>);
impl EE6POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE6POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE6POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6POL` writer - EE6POL"]
pub struct EE6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6POL_W<'a> {
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
#[doc = "Field `EE6SNS` reader - EE6SNS"]
pub struct EE6SNS_R(crate::FieldReader<u8, u8>);
impl EE6SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE6SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE6SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE6SNS` writer - EE6SNS"]
pub struct EE6SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE6SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `EE7SRC` reader - EE7SRC"]
pub struct EE7SRC_R(crate::FieldReader<u8, u8>);
impl EE7SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE7SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE7SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE7SRC` writer - EE7SRC"]
pub struct EE7SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `EE7POL` reader - EE7POL"]
pub struct EE7POL_R(crate::FieldReader<bool, bool>);
impl EE7POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE7POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE7POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE7POL` writer - EE7POL"]
pub struct EE7POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EE7SNS` reader - EE7SNS"]
pub struct EE7SNS_R(crate::FieldReader<u8, u8>);
impl EE7SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE7SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE7SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE7SNS` writer - EE7SNS"]
pub struct EE7SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE7SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `EE8SRC` reader - EE8SRC"]
pub struct EE8SRC_R(crate::FieldReader<u8, u8>);
impl EE8SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE8SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE8SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE8SRC` writer - EE8SRC"]
pub struct EE8SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `EE8POL` reader - EE8POL"]
pub struct EE8POL_R(crate::FieldReader<bool, bool>);
impl EE8POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE8POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE8POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE8POL` writer - EE8POL"]
pub struct EE8POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8POL_W<'a> {
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
#[doc = "Field `EE8SNS` reader - EE8SNS"]
pub struct EE8SNS_R(crate::FieldReader<u8, u8>);
impl EE8SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE8SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE8SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE8SNS` writer - EE8SNS"]
pub struct EE8SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE8SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `EE9SRC` reader - EE9SRC"]
pub struct EE9SRC_R(crate::FieldReader<u8, u8>);
impl EE9SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE9SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE9SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE9SRC` writer - EE9SRC"]
pub struct EE9SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `EE9POL` reader - EE9POL"]
pub struct EE9POL_R(crate::FieldReader<bool, bool>);
impl EE9POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE9POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE9POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE9POL` writer - EE9POL"]
pub struct EE9POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9POL_W<'a> {
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
#[doc = "Field `EE9SNS` reader - EE9SNS"]
pub struct EE9SNS_R(crate::FieldReader<u8, u8>);
impl EE9SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE9SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE9SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE9SNS` writer - EE9SNS"]
pub struct EE9SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE9SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `EE10SRC` reader - EE10SRC"]
pub struct EE10SRC_R(crate::FieldReader<u8, u8>);
impl EE10SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE10SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE10SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE10SRC` writer - EE10SRC"]
pub struct EE10SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `EE10POL` reader - EE10POL"]
pub struct EE10POL_R(crate::FieldReader<bool, bool>);
impl EE10POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EE10POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE10POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE10POL` writer - EE10POL"]
pub struct EE10POL_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `EE10SNS` reader - EE10SNS"]
pub struct EE10SNS_R(crate::FieldReader<u8, u8>);
impl EE10SNS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EE10SNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EE10SNS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EE10SNS` writer - EE10SNS"]
pub struct EE10SNS_W<'a> {
    w: &'a mut W,
}
impl<'a> EE10SNS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - EE6SRC"]
    #[inline(always)]
    pub fn ee6src(&self) -> EE6SRC_R {
        EE6SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - EE6POL"]
    #[inline(always)]
    pub fn ee6pol(&self) -> EE6POL_R {
        EE6POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - EE6SNS"]
    #[inline(always)]
    pub fn ee6sns(&self) -> EE6SNS_R {
        EE6SNS_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - EE7SRC"]
    #[inline(always)]
    pub fn ee7src(&self) -> EE7SRC_R {
        EE7SRC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - EE7POL"]
    #[inline(always)]
    pub fn ee7pol(&self) -> EE7POL_R {
        EE7POL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - EE7SNS"]
    #[inline(always)]
    pub fn ee7sns(&self) -> EE7SNS_R {
        EE7SNS_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - EE8SRC"]
    #[inline(always)]
    pub fn ee8src(&self) -> EE8SRC_R {
        EE8SRC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - EE8POL"]
    #[inline(always)]
    pub fn ee8pol(&self) -> EE8POL_R {
        EE8POL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - EE8SNS"]
    #[inline(always)]
    pub fn ee8sns(&self) -> EE8SNS_R {
        EE8SNS_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - EE9SRC"]
    #[inline(always)]
    pub fn ee9src(&self) -> EE9SRC_R {
        EE9SRC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - EE9POL"]
    #[inline(always)]
    pub fn ee9pol(&self) -> EE9POL_R {
        EE9POL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - EE9SNS"]
    #[inline(always)]
    pub fn ee9sns(&self) -> EE9SNS_R {
        EE9SNS_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - EE10SRC"]
    #[inline(always)]
    pub fn ee10src(&self) -> EE10SRC_R {
        EE10SRC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - EE10POL"]
    #[inline(always)]
    pub fn ee10pol(&self) -> EE10POL_R {
        EE10POL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - EE10SNS"]
    #[inline(always)]
    pub fn ee10sns(&self) -> EE10SNS_R {
        EE10SNS_R::new(((self.bits >> 27) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - EE6SRC"]
    #[inline(always)]
    pub fn ee6src(&mut self) -> EE6SRC_W {
        EE6SRC_W { w: self }
    }
    #[doc = "Bit 2 - EE6POL"]
    #[inline(always)]
    pub fn ee6pol(&mut self) -> EE6POL_W {
        EE6POL_W { w: self }
    }
    #[doc = "Bits 3:4 - EE6SNS"]
    #[inline(always)]
    pub fn ee6sns(&mut self) -> EE6SNS_W {
        EE6SNS_W { w: self }
    }
    #[doc = "Bits 6:7 - EE7SRC"]
    #[inline(always)]
    pub fn ee7src(&mut self) -> EE7SRC_W {
        EE7SRC_W { w: self }
    }
    #[doc = "Bit 8 - EE7POL"]
    #[inline(always)]
    pub fn ee7pol(&mut self) -> EE7POL_W {
        EE7POL_W { w: self }
    }
    #[doc = "Bits 9:10 - EE7SNS"]
    #[inline(always)]
    pub fn ee7sns(&mut self) -> EE7SNS_W {
        EE7SNS_W { w: self }
    }
    #[doc = "Bits 12:13 - EE8SRC"]
    #[inline(always)]
    pub fn ee8src(&mut self) -> EE8SRC_W {
        EE8SRC_W { w: self }
    }
    #[doc = "Bit 14 - EE8POL"]
    #[inline(always)]
    pub fn ee8pol(&mut self) -> EE8POL_W {
        EE8POL_W { w: self }
    }
    #[doc = "Bits 15:16 - EE8SNS"]
    #[inline(always)]
    pub fn ee8sns(&mut self) -> EE8SNS_W {
        EE8SNS_W { w: self }
    }
    #[doc = "Bits 18:19 - EE9SRC"]
    #[inline(always)]
    pub fn ee9src(&mut self) -> EE9SRC_W {
        EE9SRC_W { w: self }
    }
    #[doc = "Bit 20 - EE9POL"]
    #[inline(always)]
    pub fn ee9pol(&mut self) -> EE9POL_W {
        EE9POL_W { w: self }
    }
    #[doc = "Bits 21:22 - EE9SNS"]
    #[inline(always)]
    pub fn ee9sns(&mut self) -> EE9SNS_W {
        EE9SNS_W { w: self }
    }
    #[doc = "Bits 24:25 - EE10SRC"]
    #[inline(always)]
    pub fn ee10src(&mut self) -> EE10SRC_W {
        EE10SRC_W { w: self }
    }
    #[doc = "Bit 26 - EE10POL"]
    #[inline(always)]
    pub fn ee10pol(&mut self) -> EE10POL_W {
        EE10POL_W { w: self }
    }
    #[doc = "Bits 27:28 - EE10SNS"]
    #[inline(always)]
    pub fn ee10sns(&mut self) -> EE10SNS_W {
        EE10SNS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr2](index.html) module"]
pub struct EECR2_SPEC;
impl crate::RegisterSpec for EECR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr2::R](R) reader structure"]
impl crate::Readable for EECR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr2::W](W) writer structure"]
impl crate::Writable for EECR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EECR2 to value 0"]
impl crate::Resettable for EECR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
