#[doc = "Register `IER0` reader"]
pub struct R(crate::R<IER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IER0` writer"]
pub struct W(crate::W<IER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER0_SPEC>;
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
impl From<crate::W<IER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AE0IE` reader - AE0IE"]
pub struct AE0IE_R(crate::FieldReader<bool, bool>);
impl AE0IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE0IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE0IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE0IE` writer - AE0IE"]
pub struct AE0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE0IE_W<'a> {
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
#[doc = "Field `AE1IE` reader - AE1IE"]
pub struct AE1IE_R(crate::FieldReader<bool, bool>);
impl AE1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE1IE` writer - AE1IE"]
pub struct AE1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE1IE_W<'a> {
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
#[doc = "Field `AE2IE` reader - AE2IE"]
pub struct AE2IE_R(crate::FieldReader<bool, bool>);
impl AE2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE2IE` writer - AE2IE"]
pub struct AE2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE2IE_W<'a> {
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
#[doc = "Field `AE3IE` reader - AE3IE"]
pub struct AE3IE_R(crate::FieldReader<bool, bool>);
impl AE3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE3IE` writer - AE3IE"]
pub struct AE3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE3IE_W<'a> {
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
#[doc = "Field `AE4IE` reader - AE4IE"]
pub struct AE4IE_R(crate::FieldReader<bool, bool>);
impl AE4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE4IE` writer - AE4IE"]
pub struct AE4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE4IE_W<'a> {
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
#[doc = "Field `AE5IE` reader - AE5IE"]
pub struct AE5IE_R(crate::FieldReader<bool, bool>);
impl AE5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE5IE` writer - AE5IE"]
pub struct AE5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE5IE_W<'a> {
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
#[doc = "Field `AE6IE` reader - AE6IE"]
pub struct AE6IE_R(crate::FieldReader<bool, bool>);
impl AE6IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE6IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE6IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE6IE` writer - AE6IE"]
pub struct AE6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE6IE_W<'a> {
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
#[doc = "Field `AE7IE` reader - AE7IE"]
pub struct AE7IE_R(crate::FieldReader<bool, bool>);
impl AE7IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE7IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE7IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE7IE` writer - AE7IE"]
pub struct AE7IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE7IE_W<'a> {
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
#[doc = "Field `AE8IE` reader - AE8IE"]
pub struct AE8IE_R(crate::FieldReader<bool, bool>);
impl AE8IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE8IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE8IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE8IE` writer - AE8IE"]
pub struct AE8IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE8IE_W<'a> {
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
#[doc = "Field `AE9IE` reader - AE9IE"]
pub struct AE9IE_R(crate::FieldReader<bool, bool>);
impl AE9IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE9IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE9IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE9IE` writer - AE9IE"]
pub struct AE9IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE9IE_W<'a> {
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
#[doc = "Field `AE10IE` reader - AE10IE"]
pub struct AE10IE_R(crate::FieldReader<bool, bool>);
impl AE10IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE10IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE10IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE10IE` writer - AE10IE"]
pub struct AE10IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE10IE_W<'a> {
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
#[doc = "Field `AE11IE` reader - AE11IE"]
pub struct AE11IE_R(crate::FieldReader<bool, bool>);
impl AE11IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE11IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE11IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE11IE` writer - AE11IE"]
pub struct AE11IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE11IE_W<'a> {
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
#[doc = "Field `AE12IE` reader - AE12IE"]
pub struct AE12IE_R(crate::FieldReader<bool, bool>);
impl AE12IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE12IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE12IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE12IE` writer - AE12IE"]
pub struct AE12IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE12IE_W<'a> {
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
#[doc = "Field `AE13IE` reader - AE13IE"]
pub struct AE13IE_R(crate::FieldReader<bool, bool>);
impl AE13IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE13IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE13IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE13IE` writer - AE13IE"]
pub struct AE13IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE13IE_W<'a> {
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
#[doc = "Field `AE14IE` reader - AE14IE"]
pub struct AE14IE_R(crate::FieldReader<bool, bool>);
impl AE14IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE14IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE14IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE14IE` writer - AE14IE"]
pub struct AE14IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE14IE_W<'a> {
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
#[doc = "Field `AE15IE` reader - AE15IE"]
pub struct AE15IE_R(crate::FieldReader<bool, bool>);
impl AE15IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AE15IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AE15IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AE15IE` writer - AE15IE"]
pub struct AE15IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE15IE_W<'a> {
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
#[doc = "Field `PE0IE` reader - PE0IE"]
pub struct PE0IE_R(crate::FieldReader<bool, bool>);
impl PE0IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE0IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE0IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE0IE` writer - PE0IE"]
pub struct PE0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE0IE_W<'a> {
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
#[doc = "Field `PE1IE` reader - PE1IE"]
pub struct PE1IE_R(crate::FieldReader<bool, bool>);
impl PE1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE1IE` writer - PE1IE"]
pub struct PE1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE1IE_W<'a> {
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
#[doc = "Field `PE2IE` reader - PE2IE"]
pub struct PE2IE_R(crate::FieldReader<bool, bool>);
impl PE2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE2IE` writer - PE2IE"]
pub struct PE2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE2IE_W<'a> {
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
#[doc = "Field `PE3IE` reader - PE3IE"]
pub struct PE3IE_R(crate::FieldReader<bool, bool>);
impl PE3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE3IE` writer - PE3IE"]
pub struct PE3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE3IE_W<'a> {
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
#[doc = "Field `PE4IE` reader - PE4IE"]
pub struct PE4IE_R(crate::FieldReader<bool, bool>);
impl PE4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PE4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PE4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PE4IE` writer - PE4IE"]
pub struct PE4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE4IE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - AE0IE"]
    #[inline(always)]
    pub fn ae0ie(&self) -> AE0IE_R {
        AE0IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AE1IE"]
    #[inline(always)]
    pub fn ae1ie(&self) -> AE1IE_R {
        AE1IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AE2IE"]
    #[inline(always)]
    pub fn ae2ie(&self) -> AE2IE_R {
        AE2IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AE3IE"]
    #[inline(always)]
    pub fn ae3ie(&self) -> AE3IE_R {
        AE3IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AE4IE"]
    #[inline(always)]
    pub fn ae4ie(&self) -> AE4IE_R {
        AE4IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AE5IE"]
    #[inline(always)]
    pub fn ae5ie(&self) -> AE5IE_R {
        AE5IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AE6IE"]
    #[inline(always)]
    pub fn ae6ie(&self) -> AE6IE_R {
        AE6IE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AE7IE"]
    #[inline(always)]
    pub fn ae7ie(&self) -> AE7IE_R {
        AE7IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AE8IE"]
    #[inline(always)]
    pub fn ae8ie(&self) -> AE8IE_R {
        AE8IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AE9IE"]
    #[inline(always)]
    pub fn ae9ie(&self) -> AE9IE_R {
        AE9IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AE10IE"]
    #[inline(always)]
    pub fn ae10ie(&self) -> AE10IE_R {
        AE10IE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AE11IE"]
    #[inline(always)]
    pub fn ae11ie(&self) -> AE11IE_R {
        AE11IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AE12IE"]
    #[inline(always)]
    pub fn ae12ie(&self) -> AE12IE_R {
        AE12IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AE13IE"]
    #[inline(always)]
    pub fn ae13ie(&self) -> AE13IE_R {
        AE13IE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AE14IE"]
    #[inline(always)]
    pub fn ae14ie(&self) -> AE14IE_R {
        AE14IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AE15IE"]
    #[inline(always)]
    pub fn ae15ie(&self) -> AE15IE_R {
        AE15IE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PE0IE"]
    #[inline(always)]
    pub fn pe0ie(&self) -> PE0IE_R {
        PE0IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PE1IE"]
    #[inline(always)]
    pub fn pe1ie(&self) -> PE1IE_R {
        PE1IE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PE2IE"]
    #[inline(always)]
    pub fn pe2ie(&self) -> PE2IE_R {
        PE2IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PE3IE"]
    #[inline(always)]
    pub fn pe3ie(&self) -> PE3IE_R {
        PE3IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PE4IE"]
    #[inline(always)]
    pub fn pe4ie(&self) -> PE4IE_R {
        PE4IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AE0IE"]
    #[inline(always)]
    pub fn ae0ie(&mut self) -> AE0IE_W {
        AE0IE_W { w: self }
    }
    #[doc = "Bit 1 - AE1IE"]
    #[inline(always)]
    pub fn ae1ie(&mut self) -> AE1IE_W {
        AE1IE_W { w: self }
    }
    #[doc = "Bit 2 - AE2IE"]
    #[inline(always)]
    pub fn ae2ie(&mut self) -> AE2IE_W {
        AE2IE_W { w: self }
    }
    #[doc = "Bit 3 - AE3IE"]
    #[inline(always)]
    pub fn ae3ie(&mut self) -> AE3IE_W {
        AE3IE_W { w: self }
    }
    #[doc = "Bit 4 - AE4IE"]
    #[inline(always)]
    pub fn ae4ie(&mut self) -> AE4IE_W {
        AE4IE_W { w: self }
    }
    #[doc = "Bit 5 - AE5IE"]
    #[inline(always)]
    pub fn ae5ie(&mut self) -> AE5IE_W {
        AE5IE_W { w: self }
    }
    #[doc = "Bit 6 - AE6IE"]
    #[inline(always)]
    pub fn ae6ie(&mut self) -> AE6IE_W {
        AE6IE_W { w: self }
    }
    #[doc = "Bit 7 - AE7IE"]
    #[inline(always)]
    pub fn ae7ie(&mut self) -> AE7IE_W {
        AE7IE_W { w: self }
    }
    #[doc = "Bit 8 - AE8IE"]
    #[inline(always)]
    pub fn ae8ie(&mut self) -> AE8IE_W {
        AE8IE_W { w: self }
    }
    #[doc = "Bit 9 - AE9IE"]
    #[inline(always)]
    pub fn ae9ie(&mut self) -> AE9IE_W {
        AE9IE_W { w: self }
    }
    #[doc = "Bit 10 - AE10IE"]
    #[inline(always)]
    pub fn ae10ie(&mut self) -> AE10IE_W {
        AE10IE_W { w: self }
    }
    #[doc = "Bit 11 - AE11IE"]
    #[inline(always)]
    pub fn ae11ie(&mut self) -> AE11IE_W {
        AE11IE_W { w: self }
    }
    #[doc = "Bit 12 - AE12IE"]
    #[inline(always)]
    pub fn ae12ie(&mut self) -> AE12IE_W {
        AE12IE_W { w: self }
    }
    #[doc = "Bit 13 - AE13IE"]
    #[inline(always)]
    pub fn ae13ie(&mut self) -> AE13IE_W {
        AE13IE_W { w: self }
    }
    #[doc = "Bit 14 - AE14IE"]
    #[inline(always)]
    pub fn ae14ie(&mut self) -> AE14IE_W {
        AE14IE_W { w: self }
    }
    #[doc = "Bit 15 - AE15IE"]
    #[inline(always)]
    pub fn ae15ie(&mut self) -> AE15IE_W {
        AE15IE_W { w: self }
    }
    #[doc = "Bit 16 - PE0IE"]
    #[inline(always)]
    pub fn pe0ie(&mut self) -> PE0IE_W {
        PE0IE_W { w: self }
    }
    #[doc = "Bit 17 - PE1IE"]
    #[inline(always)]
    pub fn pe1ie(&mut self) -> PE1IE_W {
        PE1IE_W { w: self }
    }
    #[doc = "Bit 18 - PE2IE"]
    #[inline(always)]
    pub fn pe2ie(&mut self) -> PE2IE_W {
        PE2IE_W { w: self }
    }
    #[doc = "Bit 19 - PE3IE"]
    #[inline(always)]
    pub fn pe3ie(&mut self) -> PE3IE_W {
        PE3IE_W { w: self }
    }
    #[doc = "Bit 20 - PE4IE"]
    #[inline(always)]
    pub fn pe4ie(&mut self) -> PE4IE_W {
        PE4IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host interrupt enable register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier0](index.html) module"]
pub struct IER0_SPEC;
impl crate::RegisterSpec for IER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ier0::R](R) reader structure"]
impl crate::Readable for IER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ier0::W](W) writer structure"]
impl crate::Writable for IER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER0 to value 0"]
impl crate::Resettable for IER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
