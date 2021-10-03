#[doc = "Register `MPCBB1_VCTR13` reader"]
pub struct R(crate::R<MPCBB1_VCTR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR13` writer"]
pub struct W(crate::W<MPCBB1_VCTR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR13_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B416` reader - B416"]
pub struct B416_R(crate::FieldReader<bool, bool>);
impl B416_R {
    pub(crate) fn new(bits: bool) -> Self {
        B416_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B416_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B416` writer - B416"]
pub struct B416_W<'a> {
    w: &'a mut W,
}
impl<'a> B416_W<'a> {
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
#[doc = "Field `B417` reader - B417"]
pub struct B417_R(crate::FieldReader<bool, bool>);
impl B417_R {
    pub(crate) fn new(bits: bool) -> Self {
        B417_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B417_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B417` writer - B417"]
pub struct B417_W<'a> {
    w: &'a mut W,
}
impl<'a> B417_W<'a> {
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
#[doc = "Field `B418` reader - B418"]
pub struct B418_R(crate::FieldReader<bool, bool>);
impl B418_R {
    pub(crate) fn new(bits: bool) -> Self {
        B418_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B418_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B418` writer - B418"]
pub struct B418_W<'a> {
    w: &'a mut W,
}
impl<'a> B418_W<'a> {
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
#[doc = "Field `B419` reader - B419"]
pub struct B419_R(crate::FieldReader<bool, bool>);
impl B419_R {
    pub(crate) fn new(bits: bool) -> Self {
        B419_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B419_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B419` writer - B419"]
pub struct B419_W<'a> {
    w: &'a mut W,
}
impl<'a> B419_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `B420` reader - B420"]
pub struct B420_R(crate::FieldReader<bool, bool>);
impl B420_R {
    pub(crate) fn new(bits: bool) -> Self {
        B420_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B420_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B420` writer - B420"]
pub struct B420_W<'a> {
    w: &'a mut W,
}
impl<'a> B420_W<'a> {
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
#[doc = "Field `B421` reader - B421"]
pub struct B421_R(crate::FieldReader<bool, bool>);
impl B421_R {
    pub(crate) fn new(bits: bool) -> Self {
        B421_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B421_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B421` writer - B421"]
pub struct B421_W<'a> {
    w: &'a mut W,
}
impl<'a> B421_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `B422` reader - B422"]
pub struct B422_R(crate::FieldReader<bool, bool>);
impl B422_R {
    pub(crate) fn new(bits: bool) -> Self {
        B422_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B422_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B422` writer - B422"]
pub struct B422_W<'a> {
    w: &'a mut W,
}
impl<'a> B422_W<'a> {
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
#[doc = "Field `B423` reader - B423"]
pub struct B423_R(crate::FieldReader<bool, bool>);
impl B423_R {
    pub(crate) fn new(bits: bool) -> Self {
        B423_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B423_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B423` writer - B423"]
pub struct B423_W<'a> {
    w: &'a mut W,
}
impl<'a> B423_W<'a> {
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
#[doc = "Field `B424` reader - B424"]
pub struct B424_R(crate::FieldReader<bool, bool>);
impl B424_R {
    pub(crate) fn new(bits: bool) -> Self {
        B424_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B424_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B424` writer - B424"]
pub struct B424_W<'a> {
    w: &'a mut W,
}
impl<'a> B424_W<'a> {
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
#[doc = "Field `B425` reader - B425"]
pub struct B425_R(crate::FieldReader<bool, bool>);
impl B425_R {
    pub(crate) fn new(bits: bool) -> Self {
        B425_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B425_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B425` writer - B425"]
pub struct B425_W<'a> {
    w: &'a mut W,
}
impl<'a> B425_W<'a> {
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
#[doc = "Field `B426` reader - B426"]
pub struct B426_R(crate::FieldReader<bool, bool>);
impl B426_R {
    pub(crate) fn new(bits: bool) -> Self {
        B426_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B426_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B426` writer - B426"]
pub struct B426_W<'a> {
    w: &'a mut W,
}
impl<'a> B426_W<'a> {
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
#[doc = "Field `B427` reader - B427"]
pub struct B427_R(crate::FieldReader<bool, bool>);
impl B427_R {
    pub(crate) fn new(bits: bool) -> Self {
        B427_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B427_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B427` writer - B427"]
pub struct B427_W<'a> {
    w: &'a mut W,
}
impl<'a> B427_W<'a> {
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
#[doc = "Field `B428` reader - B428"]
pub struct B428_R(crate::FieldReader<bool, bool>);
impl B428_R {
    pub(crate) fn new(bits: bool) -> Self {
        B428_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B428_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B428` writer - B428"]
pub struct B428_W<'a> {
    w: &'a mut W,
}
impl<'a> B428_W<'a> {
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
#[doc = "Field `B429` reader - B429"]
pub struct B429_R(crate::FieldReader<bool, bool>);
impl B429_R {
    pub(crate) fn new(bits: bool) -> Self {
        B429_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B429_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B429` writer - B429"]
pub struct B429_W<'a> {
    w: &'a mut W,
}
impl<'a> B429_W<'a> {
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
#[doc = "Field `B430` reader - B430"]
pub struct B430_R(crate::FieldReader<bool, bool>);
impl B430_R {
    pub(crate) fn new(bits: bool) -> Self {
        B430_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B430_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B430` writer - B430"]
pub struct B430_W<'a> {
    w: &'a mut W,
}
impl<'a> B430_W<'a> {
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
#[doc = "Field `B431` reader - B431"]
pub struct B431_R(crate::FieldReader<bool, bool>);
impl B431_R {
    pub(crate) fn new(bits: bool) -> Self {
        B431_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B431_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B431` writer - B431"]
pub struct B431_W<'a> {
    w: &'a mut W,
}
impl<'a> B431_W<'a> {
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
#[doc = "Field `B432` reader - B432"]
pub struct B432_R(crate::FieldReader<bool, bool>);
impl B432_R {
    pub(crate) fn new(bits: bool) -> Self {
        B432_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B432_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B432` writer - B432"]
pub struct B432_W<'a> {
    w: &'a mut W,
}
impl<'a> B432_W<'a> {
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
#[doc = "Field `B433` reader - B433"]
pub struct B433_R(crate::FieldReader<bool, bool>);
impl B433_R {
    pub(crate) fn new(bits: bool) -> Self {
        B433_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B433_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B433` writer - B433"]
pub struct B433_W<'a> {
    w: &'a mut W,
}
impl<'a> B433_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `B434` reader - B434"]
pub struct B434_R(crate::FieldReader<bool, bool>);
impl B434_R {
    pub(crate) fn new(bits: bool) -> Self {
        B434_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B434_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B434` writer - B434"]
pub struct B434_W<'a> {
    w: &'a mut W,
}
impl<'a> B434_W<'a> {
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
#[doc = "Field `B435` reader - B435"]
pub struct B435_R(crate::FieldReader<bool, bool>);
impl B435_R {
    pub(crate) fn new(bits: bool) -> Self {
        B435_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B435_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B435` writer - B435"]
pub struct B435_W<'a> {
    w: &'a mut W,
}
impl<'a> B435_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `B436` reader - B436"]
pub struct B436_R(crate::FieldReader<bool, bool>);
impl B436_R {
    pub(crate) fn new(bits: bool) -> Self {
        B436_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B436_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B436` writer - B436"]
pub struct B436_W<'a> {
    w: &'a mut W,
}
impl<'a> B436_W<'a> {
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
#[doc = "Field `B437` reader - B437"]
pub struct B437_R(crate::FieldReader<bool, bool>);
impl B437_R {
    pub(crate) fn new(bits: bool) -> Self {
        B437_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B437_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B437` writer - B437"]
pub struct B437_W<'a> {
    w: &'a mut W,
}
impl<'a> B437_W<'a> {
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
#[doc = "Field `B438` reader - B438"]
pub struct B438_R(crate::FieldReader<bool, bool>);
impl B438_R {
    pub(crate) fn new(bits: bool) -> Self {
        B438_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B438_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B438` writer - B438"]
pub struct B438_W<'a> {
    w: &'a mut W,
}
impl<'a> B438_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `B439` reader - B439"]
pub struct B439_R(crate::FieldReader<bool, bool>);
impl B439_R {
    pub(crate) fn new(bits: bool) -> Self {
        B439_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B439_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B439` writer - B439"]
pub struct B439_W<'a> {
    w: &'a mut W,
}
impl<'a> B439_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `B440` reader - B440"]
pub struct B440_R(crate::FieldReader<bool, bool>);
impl B440_R {
    pub(crate) fn new(bits: bool) -> Self {
        B440_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B440_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B440` writer - B440"]
pub struct B440_W<'a> {
    w: &'a mut W,
}
impl<'a> B440_W<'a> {
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
#[doc = "Field `B441` reader - B441"]
pub struct B441_R(crate::FieldReader<bool, bool>);
impl B441_R {
    pub(crate) fn new(bits: bool) -> Self {
        B441_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B441_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B441` writer - B441"]
pub struct B441_W<'a> {
    w: &'a mut W,
}
impl<'a> B441_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `B442` reader - B442"]
pub struct B442_R(crate::FieldReader<bool, bool>);
impl B442_R {
    pub(crate) fn new(bits: bool) -> Self {
        B442_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B442_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B442` writer - B442"]
pub struct B442_W<'a> {
    w: &'a mut W,
}
impl<'a> B442_W<'a> {
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
#[doc = "Field `B443` reader - B443"]
pub struct B443_R(crate::FieldReader<bool, bool>);
impl B443_R {
    pub(crate) fn new(bits: bool) -> Self {
        B443_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B443_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B443` writer - B443"]
pub struct B443_W<'a> {
    w: &'a mut W,
}
impl<'a> B443_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `B444` reader - B444"]
pub struct B444_R(crate::FieldReader<bool, bool>);
impl B444_R {
    pub(crate) fn new(bits: bool) -> Self {
        B444_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B444_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B444` writer - B444"]
pub struct B444_W<'a> {
    w: &'a mut W,
}
impl<'a> B444_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `B445` reader - B445"]
pub struct B445_R(crate::FieldReader<bool, bool>);
impl B445_R {
    pub(crate) fn new(bits: bool) -> Self {
        B445_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B445_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B445` writer - B445"]
pub struct B445_W<'a> {
    w: &'a mut W,
}
impl<'a> B445_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `B446` reader - B446"]
pub struct B446_R(crate::FieldReader<bool, bool>);
impl B446_R {
    pub(crate) fn new(bits: bool) -> Self {
        B446_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B446_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B446` writer - B446"]
pub struct B446_W<'a> {
    w: &'a mut W,
}
impl<'a> B446_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `B447` reader - B447"]
pub struct B447_R(crate::FieldReader<bool, bool>);
impl B447_R {
    pub(crate) fn new(bits: bool) -> Self {
        B447_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B447_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B447` writer - B447"]
pub struct B447_W<'a> {
    w: &'a mut W,
}
impl<'a> B447_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - B416"]
    #[inline(always)]
    pub fn b416(&self) -> B416_R {
        B416_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B417"]
    #[inline(always)]
    pub fn b417(&self) -> B417_R {
        B417_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B418"]
    #[inline(always)]
    pub fn b418(&self) -> B418_R {
        B418_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B419"]
    #[inline(always)]
    pub fn b419(&self) -> B419_R {
        B419_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B420"]
    #[inline(always)]
    pub fn b420(&self) -> B420_R {
        B420_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B421"]
    #[inline(always)]
    pub fn b421(&self) -> B421_R {
        B421_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B422"]
    #[inline(always)]
    pub fn b422(&self) -> B422_R {
        B422_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B423"]
    #[inline(always)]
    pub fn b423(&self) -> B423_R {
        B423_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B424"]
    #[inline(always)]
    pub fn b424(&self) -> B424_R {
        B424_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B425"]
    #[inline(always)]
    pub fn b425(&self) -> B425_R {
        B425_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B426"]
    #[inline(always)]
    pub fn b426(&self) -> B426_R {
        B426_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B427"]
    #[inline(always)]
    pub fn b427(&self) -> B427_R {
        B427_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B428"]
    #[inline(always)]
    pub fn b428(&self) -> B428_R {
        B428_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B429"]
    #[inline(always)]
    pub fn b429(&self) -> B429_R {
        B429_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B430"]
    #[inline(always)]
    pub fn b430(&self) -> B430_R {
        B430_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B431"]
    #[inline(always)]
    pub fn b431(&self) -> B431_R {
        B431_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B432"]
    #[inline(always)]
    pub fn b432(&self) -> B432_R {
        B432_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B433"]
    #[inline(always)]
    pub fn b433(&self) -> B433_R {
        B433_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B434"]
    #[inline(always)]
    pub fn b434(&self) -> B434_R {
        B434_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B435"]
    #[inline(always)]
    pub fn b435(&self) -> B435_R {
        B435_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B436"]
    #[inline(always)]
    pub fn b436(&self) -> B436_R {
        B436_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B437"]
    #[inline(always)]
    pub fn b437(&self) -> B437_R {
        B437_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B438"]
    #[inline(always)]
    pub fn b438(&self) -> B438_R {
        B438_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B439"]
    #[inline(always)]
    pub fn b439(&self) -> B439_R {
        B439_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B440"]
    #[inline(always)]
    pub fn b440(&self) -> B440_R {
        B440_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B441"]
    #[inline(always)]
    pub fn b441(&self) -> B441_R {
        B441_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B442"]
    #[inline(always)]
    pub fn b442(&self) -> B442_R {
        B442_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B443"]
    #[inline(always)]
    pub fn b443(&self) -> B443_R {
        B443_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B444"]
    #[inline(always)]
    pub fn b444(&self) -> B444_R {
        B444_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B445"]
    #[inline(always)]
    pub fn b445(&self) -> B445_R {
        B445_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B446"]
    #[inline(always)]
    pub fn b446(&self) -> B446_R {
        B446_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B447"]
    #[inline(always)]
    pub fn b447(&self) -> B447_R {
        B447_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B416"]
    #[inline(always)]
    pub fn b416(&mut self) -> B416_W {
        B416_W { w: self }
    }
    #[doc = "Bit 1 - B417"]
    #[inline(always)]
    pub fn b417(&mut self) -> B417_W {
        B417_W { w: self }
    }
    #[doc = "Bit 2 - B418"]
    #[inline(always)]
    pub fn b418(&mut self) -> B418_W {
        B418_W { w: self }
    }
    #[doc = "Bit 3 - B419"]
    #[inline(always)]
    pub fn b419(&mut self) -> B419_W {
        B419_W { w: self }
    }
    #[doc = "Bit 4 - B420"]
    #[inline(always)]
    pub fn b420(&mut self) -> B420_W {
        B420_W { w: self }
    }
    #[doc = "Bit 5 - B421"]
    #[inline(always)]
    pub fn b421(&mut self) -> B421_W {
        B421_W { w: self }
    }
    #[doc = "Bit 6 - B422"]
    #[inline(always)]
    pub fn b422(&mut self) -> B422_W {
        B422_W { w: self }
    }
    #[doc = "Bit 7 - B423"]
    #[inline(always)]
    pub fn b423(&mut self) -> B423_W {
        B423_W { w: self }
    }
    #[doc = "Bit 8 - B424"]
    #[inline(always)]
    pub fn b424(&mut self) -> B424_W {
        B424_W { w: self }
    }
    #[doc = "Bit 9 - B425"]
    #[inline(always)]
    pub fn b425(&mut self) -> B425_W {
        B425_W { w: self }
    }
    #[doc = "Bit 10 - B426"]
    #[inline(always)]
    pub fn b426(&mut self) -> B426_W {
        B426_W { w: self }
    }
    #[doc = "Bit 11 - B427"]
    #[inline(always)]
    pub fn b427(&mut self) -> B427_W {
        B427_W { w: self }
    }
    #[doc = "Bit 12 - B428"]
    #[inline(always)]
    pub fn b428(&mut self) -> B428_W {
        B428_W { w: self }
    }
    #[doc = "Bit 13 - B429"]
    #[inline(always)]
    pub fn b429(&mut self) -> B429_W {
        B429_W { w: self }
    }
    #[doc = "Bit 14 - B430"]
    #[inline(always)]
    pub fn b430(&mut self) -> B430_W {
        B430_W { w: self }
    }
    #[doc = "Bit 15 - B431"]
    #[inline(always)]
    pub fn b431(&mut self) -> B431_W {
        B431_W { w: self }
    }
    #[doc = "Bit 16 - B432"]
    #[inline(always)]
    pub fn b432(&mut self) -> B432_W {
        B432_W { w: self }
    }
    #[doc = "Bit 17 - B433"]
    #[inline(always)]
    pub fn b433(&mut self) -> B433_W {
        B433_W { w: self }
    }
    #[doc = "Bit 18 - B434"]
    #[inline(always)]
    pub fn b434(&mut self) -> B434_W {
        B434_W { w: self }
    }
    #[doc = "Bit 19 - B435"]
    #[inline(always)]
    pub fn b435(&mut self) -> B435_W {
        B435_W { w: self }
    }
    #[doc = "Bit 20 - B436"]
    #[inline(always)]
    pub fn b436(&mut self) -> B436_W {
        B436_W { w: self }
    }
    #[doc = "Bit 21 - B437"]
    #[inline(always)]
    pub fn b437(&mut self) -> B437_W {
        B437_W { w: self }
    }
    #[doc = "Bit 22 - B438"]
    #[inline(always)]
    pub fn b438(&mut self) -> B438_W {
        B438_W { w: self }
    }
    #[doc = "Bit 23 - B439"]
    #[inline(always)]
    pub fn b439(&mut self) -> B439_W {
        B439_W { w: self }
    }
    #[doc = "Bit 24 - B440"]
    #[inline(always)]
    pub fn b440(&mut self) -> B440_W {
        B440_W { w: self }
    }
    #[doc = "Bit 25 - B441"]
    #[inline(always)]
    pub fn b441(&mut self) -> B441_W {
        B441_W { w: self }
    }
    #[doc = "Bit 26 - B442"]
    #[inline(always)]
    pub fn b442(&mut self) -> B442_W {
        B442_W { w: self }
    }
    #[doc = "Bit 27 - B443"]
    #[inline(always)]
    pub fn b443(&mut self) -> B443_W {
        B443_W { w: self }
    }
    #[doc = "Bit 28 - B444"]
    #[inline(always)]
    pub fn b444(&mut self) -> B444_W {
        B444_W { w: self }
    }
    #[doc = "Bit 29 - B445"]
    #[inline(always)]
    pub fn b445(&mut self) -> B445_W {
        B445_W { w: self }
    }
    #[doc = "Bit 30 - B446"]
    #[inline(always)]
    pub fn b446(&mut self) -> B446_W {
        B446_W { w: self }
    }
    #[doc = "Bit 31 - B447"]
    #[inline(always)]
    pub fn b447(&mut self) -> B447_W {
        B447_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr13](index.html) module"]
pub struct MPCBB1_VCTR13_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr13::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr13::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR13 to value 0"]
impl crate::Resettable for MPCBB1_VCTR13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
