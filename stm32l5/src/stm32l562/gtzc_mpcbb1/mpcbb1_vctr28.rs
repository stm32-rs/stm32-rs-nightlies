#[doc = "Register `MPCBB1_VCTR28` reader"]
pub struct R(crate::R<MPCBB1_VCTR28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR28` writer"]
pub struct W(crate::W<MPCBB1_VCTR28_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR28_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR28_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR28_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B896` reader - B896"]
pub struct B896_R(crate::FieldReader<bool, bool>);
impl B896_R {
    pub(crate) fn new(bits: bool) -> Self {
        B896_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B896_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B896` writer - B896"]
pub struct B896_W<'a> {
    w: &'a mut W,
}
impl<'a> B896_W<'a> {
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
#[doc = "Field `B897` reader - B897"]
pub struct B897_R(crate::FieldReader<bool, bool>);
impl B897_R {
    pub(crate) fn new(bits: bool) -> Self {
        B897_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B897_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B897` writer - B897"]
pub struct B897_W<'a> {
    w: &'a mut W,
}
impl<'a> B897_W<'a> {
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
#[doc = "Field `B898` reader - B898"]
pub struct B898_R(crate::FieldReader<bool, bool>);
impl B898_R {
    pub(crate) fn new(bits: bool) -> Self {
        B898_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B898_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B898` writer - B898"]
pub struct B898_W<'a> {
    w: &'a mut W,
}
impl<'a> B898_W<'a> {
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
#[doc = "Field `B899` reader - B899"]
pub struct B899_R(crate::FieldReader<bool, bool>);
impl B899_R {
    pub(crate) fn new(bits: bool) -> Self {
        B899_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B899_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B899` writer - B899"]
pub struct B899_W<'a> {
    w: &'a mut W,
}
impl<'a> B899_W<'a> {
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
#[doc = "Field `B900` reader - B900"]
pub struct B900_R(crate::FieldReader<bool, bool>);
impl B900_R {
    pub(crate) fn new(bits: bool) -> Self {
        B900_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B900_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B900` writer - B900"]
pub struct B900_W<'a> {
    w: &'a mut W,
}
impl<'a> B900_W<'a> {
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
#[doc = "Field `B901` reader - B901"]
pub struct B901_R(crate::FieldReader<bool, bool>);
impl B901_R {
    pub(crate) fn new(bits: bool) -> Self {
        B901_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B901_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B901` writer - B901"]
pub struct B901_W<'a> {
    w: &'a mut W,
}
impl<'a> B901_W<'a> {
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
#[doc = "Field `B902` reader - B902"]
pub struct B902_R(crate::FieldReader<bool, bool>);
impl B902_R {
    pub(crate) fn new(bits: bool) -> Self {
        B902_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B902_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B902` writer - B902"]
pub struct B902_W<'a> {
    w: &'a mut W,
}
impl<'a> B902_W<'a> {
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
#[doc = "Field `B903` reader - B903"]
pub struct B903_R(crate::FieldReader<bool, bool>);
impl B903_R {
    pub(crate) fn new(bits: bool) -> Self {
        B903_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B903_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B903` writer - B903"]
pub struct B903_W<'a> {
    w: &'a mut W,
}
impl<'a> B903_W<'a> {
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
#[doc = "Field `B904` reader - B904"]
pub struct B904_R(crate::FieldReader<bool, bool>);
impl B904_R {
    pub(crate) fn new(bits: bool) -> Self {
        B904_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B904_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B904` writer - B904"]
pub struct B904_W<'a> {
    w: &'a mut W,
}
impl<'a> B904_W<'a> {
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
#[doc = "Field `B905` reader - B905"]
pub struct B905_R(crate::FieldReader<bool, bool>);
impl B905_R {
    pub(crate) fn new(bits: bool) -> Self {
        B905_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B905_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B905` writer - B905"]
pub struct B905_W<'a> {
    w: &'a mut W,
}
impl<'a> B905_W<'a> {
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
#[doc = "Field `B906` reader - B906"]
pub struct B906_R(crate::FieldReader<bool, bool>);
impl B906_R {
    pub(crate) fn new(bits: bool) -> Self {
        B906_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B906_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B906` writer - B906"]
pub struct B906_W<'a> {
    w: &'a mut W,
}
impl<'a> B906_W<'a> {
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
#[doc = "Field `B907` reader - B907"]
pub struct B907_R(crate::FieldReader<bool, bool>);
impl B907_R {
    pub(crate) fn new(bits: bool) -> Self {
        B907_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B907_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B907` writer - B907"]
pub struct B907_W<'a> {
    w: &'a mut W,
}
impl<'a> B907_W<'a> {
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
#[doc = "Field `B908` reader - B908"]
pub struct B908_R(crate::FieldReader<bool, bool>);
impl B908_R {
    pub(crate) fn new(bits: bool) -> Self {
        B908_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B908_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B908` writer - B908"]
pub struct B908_W<'a> {
    w: &'a mut W,
}
impl<'a> B908_W<'a> {
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
#[doc = "Field `B909` reader - B909"]
pub struct B909_R(crate::FieldReader<bool, bool>);
impl B909_R {
    pub(crate) fn new(bits: bool) -> Self {
        B909_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B909_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B909` writer - B909"]
pub struct B909_W<'a> {
    w: &'a mut W,
}
impl<'a> B909_W<'a> {
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
#[doc = "Field `B910` reader - B910"]
pub struct B910_R(crate::FieldReader<bool, bool>);
impl B910_R {
    pub(crate) fn new(bits: bool) -> Self {
        B910_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B910_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B910` writer - B910"]
pub struct B910_W<'a> {
    w: &'a mut W,
}
impl<'a> B910_W<'a> {
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
#[doc = "Field `B911` reader - B911"]
pub struct B911_R(crate::FieldReader<bool, bool>);
impl B911_R {
    pub(crate) fn new(bits: bool) -> Self {
        B911_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B911_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B911` writer - B911"]
pub struct B911_W<'a> {
    w: &'a mut W,
}
impl<'a> B911_W<'a> {
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
#[doc = "Field `B912` reader - B912"]
pub struct B912_R(crate::FieldReader<bool, bool>);
impl B912_R {
    pub(crate) fn new(bits: bool) -> Self {
        B912_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B912_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B912` writer - B912"]
pub struct B912_W<'a> {
    w: &'a mut W,
}
impl<'a> B912_W<'a> {
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
#[doc = "Field `B913` reader - B913"]
pub struct B913_R(crate::FieldReader<bool, bool>);
impl B913_R {
    pub(crate) fn new(bits: bool) -> Self {
        B913_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B913_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B913` writer - B913"]
pub struct B913_W<'a> {
    w: &'a mut W,
}
impl<'a> B913_W<'a> {
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
#[doc = "Field `B914` reader - B914"]
pub struct B914_R(crate::FieldReader<bool, bool>);
impl B914_R {
    pub(crate) fn new(bits: bool) -> Self {
        B914_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B914_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B914` writer - B914"]
pub struct B914_W<'a> {
    w: &'a mut W,
}
impl<'a> B914_W<'a> {
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
#[doc = "Field `B915` reader - B915"]
pub struct B915_R(crate::FieldReader<bool, bool>);
impl B915_R {
    pub(crate) fn new(bits: bool) -> Self {
        B915_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B915_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B915` writer - B915"]
pub struct B915_W<'a> {
    w: &'a mut W,
}
impl<'a> B915_W<'a> {
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
#[doc = "Field `B916` reader - B916"]
pub struct B916_R(crate::FieldReader<bool, bool>);
impl B916_R {
    pub(crate) fn new(bits: bool) -> Self {
        B916_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B916_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B916` writer - B916"]
pub struct B916_W<'a> {
    w: &'a mut W,
}
impl<'a> B916_W<'a> {
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
#[doc = "Field `B917` reader - B917"]
pub struct B917_R(crate::FieldReader<bool, bool>);
impl B917_R {
    pub(crate) fn new(bits: bool) -> Self {
        B917_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B917_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B917` writer - B917"]
pub struct B917_W<'a> {
    w: &'a mut W,
}
impl<'a> B917_W<'a> {
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
#[doc = "Field `B918` reader - B918"]
pub struct B918_R(crate::FieldReader<bool, bool>);
impl B918_R {
    pub(crate) fn new(bits: bool) -> Self {
        B918_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B918_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B918` writer - B918"]
pub struct B918_W<'a> {
    w: &'a mut W,
}
impl<'a> B918_W<'a> {
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
#[doc = "Field `B919` reader - B919"]
pub struct B919_R(crate::FieldReader<bool, bool>);
impl B919_R {
    pub(crate) fn new(bits: bool) -> Self {
        B919_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B919_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B919` writer - B919"]
pub struct B919_W<'a> {
    w: &'a mut W,
}
impl<'a> B919_W<'a> {
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
#[doc = "Field `B920` reader - B920"]
pub struct B920_R(crate::FieldReader<bool, bool>);
impl B920_R {
    pub(crate) fn new(bits: bool) -> Self {
        B920_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B920_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B920` writer - B920"]
pub struct B920_W<'a> {
    w: &'a mut W,
}
impl<'a> B920_W<'a> {
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
#[doc = "Field `B921` reader - B921"]
pub struct B921_R(crate::FieldReader<bool, bool>);
impl B921_R {
    pub(crate) fn new(bits: bool) -> Self {
        B921_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B921_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B921` writer - B921"]
pub struct B921_W<'a> {
    w: &'a mut W,
}
impl<'a> B921_W<'a> {
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
#[doc = "Field `B922` reader - B922"]
pub struct B922_R(crate::FieldReader<bool, bool>);
impl B922_R {
    pub(crate) fn new(bits: bool) -> Self {
        B922_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B922_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B922` writer - B922"]
pub struct B922_W<'a> {
    w: &'a mut W,
}
impl<'a> B922_W<'a> {
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
#[doc = "Field `B923` reader - B923"]
pub struct B923_R(crate::FieldReader<bool, bool>);
impl B923_R {
    pub(crate) fn new(bits: bool) -> Self {
        B923_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B923_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B923` writer - B923"]
pub struct B923_W<'a> {
    w: &'a mut W,
}
impl<'a> B923_W<'a> {
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
#[doc = "Field `B924` reader - B924"]
pub struct B924_R(crate::FieldReader<bool, bool>);
impl B924_R {
    pub(crate) fn new(bits: bool) -> Self {
        B924_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B924_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B924` writer - B924"]
pub struct B924_W<'a> {
    w: &'a mut W,
}
impl<'a> B924_W<'a> {
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
#[doc = "Field `B925` reader - B925"]
pub struct B925_R(crate::FieldReader<bool, bool>);
impl B925_R {
    pub(crate) fn new(bits: bool) -> Self {
        B925_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B925_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B925` writer - B925"]
pub struct B925_W<'a> {
    w: &'a mut W,
}
impl<'a> B925_W<'a> {
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
#[doc = "Field `B926` reader - B926"]
pub struct B926_R(crate::FieldReader<bool, bool>);
impl B926_R {
    pub(crate) fn new(bits: bool) -> Self {
        B926_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B926_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B926` writer - B926"]
pub struct B926_W<'a> {
    w: &'a mut W,
}
impl<'a> B926_W<'a> {
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
#[doc = "Field `B927` reader - B927"]
pub struct B927_R(crate::FieldReader<bool, bool>);
impl B927_R {
    pub(crate) fn new(bits: bool) -> Self {
        B927_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B927_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B927` writer - B927"]
pub struct B927_W<'a> {
    w: &'a mut W,
}
impl<'a> B927_W<'a> {
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
    #[doc = "Bit 0 - B896"]
    #[inline(always)]
    pub fn b896(&self) -> B896_R {
        B896_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B897"]
    #[inline(always)]
    pub fn b897(&self) -> B897_R {
        B897_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B898"]
    #[inline(always)]
    pub fn b898(&self) -> B898_R {
        B898_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B899"]
    #[inline(always)]
    pub fn b899(&self) -> B899_R {
        B899_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B900"]
    #[inline(always)]
    pub fn b900(&self) -> B900_R {
        B900_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B901"]
    #[inline(always)]
    pub fn b901(&self) -> B901_R {
        B901_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B902"]
    #[inline(always)]
    pub fn b902(&self) -> B902_R {
        B902_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B903"]
    #[inline(always)]
    pub fn b903(&self) -> B903_R {
        B903_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B904"]
    #[inline(always)]
    pub fn b904(&self) -> B904_R {
        B904_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B905"]
    #[inline(always)]
    pub fn b905(&self) -> B905_R {
        B905_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B906"]
    #[inline(always)]
    pub fn b906(&self) -> B906_R {
        B906_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B907"]
    #[inline(always)]
    pub fn b907(&self) -> B907_R {
        B907_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B908"]
    #[inline(always)]
    pub fn b908(&self) -> B908_R {
        B908_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B909"]
    #[inline(always)]
    pub fn b909(&self) -> B909_R {
        B909_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B910"]
    #[inline(always)]
    pub fn b910(&self) -> B910_R {
        B910_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B911"]
    #[inline(always)]
    pub fn b911(&self) -> B911_R {
        B911_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B912"]
    #[inline(always)]
    pub fn b912(&self) -> B912_R {
        B912_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B913"]
    #[inline(always)]
    pub fn b913(&self) -> B913_R {
        B913_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B914"]
    #[inline(always)]
    pub fn b914(&self) -> B914_R {
        B914_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B915"]
    #[inline(always)]
    pub fn b915(&self) -> B915_R {
        B915_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B916"]
    #[inline(always)]
    pub fn b916(&self) -> B916_R {
        B916_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B917"]
    #[inline(always)]
    pub fn b917(&self) -> B917_R {
        B917_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B918"]
    #[inline(always)]
    pub fn b918(&self) -> B918_R {
        B918_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B919"]
    #[inline(always)]
    pub fn b919(&self) -> B919_R {
        B919_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B920"]
    #[inline(always)]
    pub fn b920(&self) -> B920_R {
        B920_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B921"]
    #[inline(always)]
    pub fn b921(&self) -> B921_R {
        B921_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B922"]
    #[inline(always)]
    pub fn b922(&self) -> B922_R {
        B922_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B923"]
    #[inline(always)]
    pub fn b923(&self) -> B923_R {
        B923_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B924"]
    #[inline(always)]
    pub fn b924(&self) -> B924_R {
        B924_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B925"]
    #[inline(always)]
    pub fn b925(&self) -> B925_R {
        B925_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B926"]
    #[inline(always)]
    pub fn b926(&self) -> B926_R {
        B926_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B927"]
    #[inline(always)]
    pub fn b927(&self) -> B927_R {
        B927_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B896"]
    #[inline(always)]
    pub fn b896(&mut self) -> B896_W {
        B896_W { w: self }
    }
    #[doc = "Bit 1 - B897"]
    #[inline(always)]
    pub fn b897(&mut self) -> B897_W {
        B897_W { w: self }
    }
    #[doc = "Bit 2 - B898"]
    #[inline(always)]
    pub fn b898(&mut self) -> B898_W {
        B898_W { w: self }
    }
    #[doc = "Bit 3 - B899"]
    #[inline(always)]
    pub fn b899(&mut self) -> B899_W {
        B899_W { w: self }
    }
    #[doc = "Bit 4 - B900"]
    #[inline(always)]
    pub fn b900(&mut self) -> B900_W {
        B900_W { w: self }
    }
    #[doc = "Bit 5 - B901"]
    #[inline(always)]
    pub fn b901(&mut self) -> B901_W {
        B901_W { w: self }
    }
    #[doc = "Bit 6 - B902"]
    #[inline(always)]
    pub fn b902(&mut self) -> B902_W {
        B902_W { w: self }
    }
    #[doc = "Bit 7 - B903"]
    #[inline(always)]
    pub fn b903(&mut self) -> B903_W {
        B903_W { w: self }
    }
    #[doc = "Bit 8 - B904"]
    #[inline(always)]
    pub fn b904(&mut self) -> B904_W {
        B904_W { w: self }
    }
    #[doc = "Bit 9 - B905"]
    #[inline(always)]
    pub fn b905(&mut self) -> B905_W {
        B905_W { w: self }
    }
    #[doc = "Bit 10 - B906"]
    #[inline(always)]
    pub fn b906(&mut self) -> B906_W {
        B906_W { w: self }
    }
    #[doc = "Bit 11 - B907"]
    #[inline(always)]
    pub fn b907(&mut self) -> B907_W {
        B907_W { w: self }
    }
    #[doc = "Bit 12 - B908"]
    #[inline(always)]
    pub fn b908(&mut self) -> B908_W {
        B908_W { w: self }
    }
    #[doc = "Bit 13 - B909"]
    #[inline(always)]
    pub fn b909(&mut self) -> B909_W {
        B909_W { w: self }
    }
    #[doc = "Bit 14 - B910"]
    #[inline(always)]
    pub fn b910(&mut self) -> B910_W {
        B910_W { w: self }
    }
    #[doc = "Bit 15 - B911"]
    #[inline(always)]
    pub fn b911(&mut self) -> B911_W {
        B911_W { w: self }
    }
    #[doc = "Bit 16 - B912"]
    #[inline(always)]
    pub fn b912(&mut self) -> B912_W {
        B912_W { w: self }
    }
    #[doc = "Bit 17 - B913"]
    #[inline(always)]
    pub fn b913(&mut self) -> B913_W {
        B913_W { w: self }
    }
    #[doc = "Bit 18 - B914"]
    #[inline(always)]
    pub fn b914(&mut self) -> B914_W {
        B914_W { w: self }
    }
    #[doc = "Bit 19 - B915"]
    #[inline(always)]
    pub fn b915(&mut self) -> B915_W {
        B915_W { w: self }
    }
    #[doc = "Bit 20 - B916"]
    #[inline(always)]
    pub fn b916(&mut self) -> B916_W {
        B916_W { w: self }
    }
    #[doc = "Bit 21 - B917"]
    #[inline(always)]
    pub fn b917(&mut self) -> B917_W {
        B917_W { w: self }
    }
    #[doc = "Bit 22 - B918"]
    #[inline(always)]
    pub fn b918(&mut self) -> B918_W {
        B918_W { w: self }
    }
    #[doc = "Bit 23 - B919"]
    #[inline(always)]
    pub fn b919(&mut self) -> B919_W {
        B919_W { w: self }
    }
    #[doc = "Bit 24 - B920"]
    #[inline(always)]
    pub fn b920(&mut self) -> B920_W {
        B920_W { w: self }
    }
    #[doc = "Bit 25 - B921"]
    #[inline(always)]
    pub fn b921(&mut self) -> B921_W {
        B921_W { w: self }
    }
    #[doc = "Bit 26 - B922"]
    #[inline(always)]
    pub fn b922(&mut self) -> B922_W {
        B922_W { w: self }
    }
    #[doc = "Bit 27 - B923"]
    #[inline(always)]
    pub fn b923(&mut self) -> B923_W {
        B923_W { w: self }
    }
    #[doc = "Bit 28 - B924"]
    #[inline(always)]
    pub fn b924(&mut self) -> B924_W {
        B924_W { w: self }
    }
    #[doc = "Bit 29 - B925"]
    #[inline(always)]
    pub fn b925(&mut self) -> B925_W {
        B925_W { w: self }
    }
    #[doc = "Bit 30 - B926"]
    #[inline(always)]
    pub fn b926(&mut self) -> B926_W {
        B926_W { w: self }
    }
    #[doc = "Bit 31 - B927"]
    #[inline(always)]
    pub fn b927(&mut self) -> B927_W {
        B927_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr28](index.html) module"]
pub struct MPCBB1_VCTR28_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr28::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR28_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr28::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR28_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR28 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR28_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
