#[doc = "Register `MPCBB2_VCTR3` reader"]
pub struct R(crate::R<MPCBB2_VCTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR3` writer"]
pub struct W(crate::W<MPCBB2_VCTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR3_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B96` reader - B96"]
pub struct B96_R(crate::FieldReader<bool, bool>);
impl B96_R {
    pub(crate) fn new(bits: bool) -> Self {
        B96_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B96_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B96` writer - B96"]
pub struct B96_W<'a> {
    w: &'a mut W,
}
impl<'a> B96_W<'a> {
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
#[doc = "Field `B97` reader - B97"]
pub struct B97_R(crate::FieldReader<bool, bool>);
impl B97_R {
    pub(crate) fn new(bits: bool) -> Self {
        B97_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B97_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B97` writer - B97"]
pub struct B97_W<'a> {
    w: &'a mut W,
}
impl<'a> B97_W<'a> {
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
#[doc = "Field `B98` reader - B98"]
pub struct B98_R(crate::FieldReader<bool, bool>);
impl B98_R {
    pub(crate) fn new(bits: bool) -> Self {
        B98_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B98_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B98` writer - B98"]
pub struct B98_W<'a> {
    w: &'a mut W,
}
impl<'a> B98_W<'a> {
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
#[doc = "Field `B99` reader - B99"]
pub struct B99_R(crate::FieldReader<bool, bool>);
impl B99_R {
    pub(crate) fn new(bits: bool) -> Self {
        B99_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B99_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B99` writer - B99"]
pub struct B99_W<'a> {
    w: &'a mut W,
}
impl<'a> B99_W<'a> {
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
#[doc = "Field `B100` reader - B100"]
pub struct B100_R(crate::FieldReader<bool, bool>);
impl B100_R {
    pub(crate) fn new(bits: bool) -> Self {
        B100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B100_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B100` writer - B100"]
pub struct B100_W<'a> {
    w: &'a mut W,
}
impl<'a> B100_W<'a> {
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
#[doc = "Field `B101` reader - B101"]
pub struct B101_R(crate::FieldReader<bool, bool>);
impl B101_R {
    pub(crate) fn new(bits: bool) -> Self {
        B101_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B101_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B101` writer - B101"]
pub struct B101_W<'a> {
    w: &'a mut W,
}
impl<'a> B101_W<'a> {
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
#[doc = "Field `B102` reader - B102"]
pub struct B102_R(crate::FieldReader<bool, bool>);
impl B102_R {
    pub(crate) fn new(bits: bool) -> Self {
        B102_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B102_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B102` writer - B102"]
pub struct B102_W<'a> {
    w: &'a mut W,
}
impl<'a> B102_W<'a> {
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
#[doc = "Field `B103` reader - B103"]
pub struct B103_R(crate::FieldReader<bool, bool>);
impl B103_R {
    pub(crate) fn new(bits: bool) -> Self {
        B103_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B103_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B103` writer - B103"]
pub struct B103_W<'a> {
    w: &'a mut W,
}
impl<'a> B103_W<'a> {
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
#[doc = "Field `B104` reader - B104"]
pub struct B104_R(crate::FieldReader<bool, bool>);
impl B104_R {
    pub(crate) fn new(bits: bool) -> Self {
        B104_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B104_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B104` writer - B104"]
pub struct B104_W<'a> {
    w: &'a mut W,
}
impl<'a> B104_W<'a> {
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
#[doc = "Field `B105` reader - B105"]
pub struct B105_R(crate::FieldReader<bool, bool>);
impl B105_R {
    pub(crate) fn new(bits: bool) -> Self {
        B105_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B105_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B105` writer - B105"]
pub struct B105_W<'a> {
    w: &'a mut W,
}
impl<'a> B105_W<'a> {
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
#[doc = "Field `B106` reader - B106"]
pub struct B106_R(crate::FieldReader<bool, bool>);
impl B106_R {
    pub(crate) fn new(bits: bool) -> Self {
        B106_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B106_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B106` writer - B106"]
pub struct B106_W<'a> {
    w: &'a mut W,
}
impl<'a> B106_W<'a> {
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
#[doc = "Field `B107` reader - B107"]
pub struct B107_R(crate::FieldReader<bool, bool>);
impl B107_R {
    pub(crate) fn new(bits: bool) -> Self {
        B107_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B107_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B107` writer - B107"]
pub struct B107_W<'a> {
    w: &'a mut W,
}
impl<'a> B107_W<'a> {
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
#[doc = "Field `B108` reader - B108"]
pub struct B108_R(crate::FieldReader<bool, bool>);
impl B108_R {
    pub(crate) fn new(bits: bool) -> Self {
        B108_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B108_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B108` writer - B108"]
pub struct B108_W<'a> {
    w: &'a mut W,
}
impl<'a> B108_W<'a> {
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
#[doc = "Field `B109` reader - B109"]
pub struct B109_R(crate::FieldReader<bool, bool>);
impl B109_R {
    pub(crate) fn new(bits: bool) -> Self {
        B109_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B109_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B109` writer - B109"]
pub struct B109_W<'a> {
    w: &'a mut W,
}
impl<'a> B109_W<'a> {
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
#[doc = "Field `B110` reader - B110"]
pub struct B110_R(crate::FieldReader<bool, bool>);
impl B110_R {
    pub(crate) fn new(bits: bool) -> Self {
        B110_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B110_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B110` writer - B110"]
pub struct B110_W<'a> {
    w: &'a mut W,
}
impl<'a> B110_W<'a> {
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
#[doc = "Field `B111` reader - B111"]
pub struct B111_R(crate::FieldReader<bool, bool>);
impl B111_R {
    pub(crate) fn new(bits: bool) -> Self {
        B111_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B111_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B111` writer - B111"]
pub struct B111_W<'a> {
    w: &'a mut W,
}
impl<'a> B111_W<'a> {
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
#[doc = "Field `B112` reader - B112"]
pub struct B112_R(crate::FieldReader<bool, bool>);
impl B112_R {
    pub(crate) fn new(bits: bool) -> Self {
        B112_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B112_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B112` writer - B112"]
pub struct B112_W<'a> {
    w: &'a mut W,
}
impl<'a> B112_W<'a> {
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
#[doc = "Field `B113` reader - B113"]
pub struct B113_R(crate::FieldReader<bool, bool>);
impl B113_R {
    pub(crate) fn new(bits: bool) -> Self {
        B113_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B113_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B113` writer - B113"]
pub struct B113_W<'a> {
    w: &'a mut W,
}
impl<'a> B113_W<'a> {
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
#[doc = "Field `B114` reader - B114"]
pub struct B114_R(crate::FieldReader<bool, bool>);
impl B114_R {
    pub(crate) fn new(bits: bool) -> Self {
        B114_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B114_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B114` writer - B114"]
pub struct B114_W<'a> {
    w: &'a mut W,
}
impl<'a> B114_W<'a> {
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
#[doc = "Field `B115` reader - B115"]
pub struct B115_R(crate::FieldReader<bool, bool>);
impl B115_R {
    pub(crate) fn new(bits: bool) -> Self {
        B115_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B115_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B115` writer - B115"]
pub struct B115_W<'a> {
    w: &'a mut W,
}
impl<'a> B115_W<'a> {
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
#[doc = "Field `B116` reader - B116"]
pub struct B116_R(crate::FieldReader<bool, bool>);
impl B116_R {
    pub(crate) fn new(bits: bool) -> Self {
        B116_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B116_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B116` writer - B116"]
pub struct B116_W<'a> {
    w: &'a mut W,
}
impl<'a> B116_W<'a> {
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
#[doc = "Field `B117` reader - B117"]
pub struct B117_R(crate::FieldReader<bool, bool>);
impl B117_R {
    pub(crate) fn new(bits: bool) -> Self {
        B117_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B117_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B117` writer - B117"]
pub struct B117_W<'a> {
    w: &'a mut W,
}
impl<'a> B117_W<'a> {
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
#[doc = "Field `B118` reader - B118"]
pub struct B118_R(crate::FieldReader<bool, bool>);
impl B118_R {
    pub(crate) fn new(bits: bool) -> Self {
        B118_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B118_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B118` writer - B118"]
pub struct B118_W<'a> {
    w: &'a mut W,
}
impl<'a> B118_W<'a> {
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
#[doc = "Field `B119` reader - B119"]
pub struct B119_R(crate::FieldReader<bool, bool>);
impl B119_R {
    pub(crate) fn new(bits: bool) -> Self {
        B119_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B119_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B119` writer - B119"]
pub struct B119_W<'a> {
    w: &'a mut W,
}
impl<'a> B119_W<'a> {
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
#[doc = "Field `B120` reader - B120"]
pub struct B120_R(crate::FieldReader<bool, bool>);
impl B120_R {
    pub(crate) fn new(bits: bool) -> Self {
        B120_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B120_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B120` writer - B120"]
pub struct B120_W<'a> {
    w: &'a mut W,
}
impl<'a> B120_W<'a> {
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
#[doc = "Field `B121` reader - B121"]
pub struct B121_R(crate::FieldReader<bool, bool>);
impl B121_R {
    pub(crate) fn new(bits: bool) -> Self {
        B121_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B121_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B121` writer - B121"]
pub struct B121_W<'a> {
    w: &'a mut W,
}
impl<'a> B121_W<'a> {
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
#[doc = "Field `B122` reader - B122"]
pub struct B122_R(crate::FieldReader<bool, bool>);
impl B122_R {
    pub(crate) fn new(bits: bool) -> Self {
        B122_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B122_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B122` writer - B122"]
pub struct B122_W<'a> {
    w: &'a mut W,
}
impl<'a> B122_W<'a> {
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
#[doc = "Field `B123` reader - B123"]
pub struct B123_R(crate::FieldReader<bool, bool>);
impl B123_R {
    pub(crate) fn new(bits: bool) -> Self {
        B123_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B123_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B123` writer - B123"]
pub struct B123_W<'a> {
    w: &'a mut W,
}
impl<'a> B123_W<'a> {
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
#[doc = "Field `B124` reader - B124"]
pub struct B124_R(crate::FieldReader<bool, bool>);
impl B124_R {
    pub(crate) fn new(bits: bool) -> Self {
        B124_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B124_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B124` writer - B124"]
pub struct B124_W<'a> {
    w: &'a mut W,
}
impl<'a> B124_W<'a> {
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
#[doc = "Field `B125` reader - B125"]
pub struct B125_R(crate::FieldReader<bool, bool>);
impl B125_R {
    pub(crate) fn new(bits: bool) -> Self {
        B125_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B125_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B125` writer - B125"]
pub struct B125_W<'a> {
    w: &'a mut W,
}
impl<'a> B125_W<'a> {
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
#[doc = "Field `B126` reader - B126"]
pub struct B126_R(crate::FieldReader<bool, bool>);
impl B126_R {
    pub(crate) fn new(bits: bool) -> Self {
        B126_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B126_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B126` writer - B126"]
pub struct B126_W<'a> {
    w: &'a mut W,
}
impl<'a> B126_W<'a> {
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
#[doc = "Field `B127` reader - B127"]
pub struct B127_R(crate::FieldReader<bool, bool>);
impl B127_R {
    pub(crate) fn new(bits: bool) -> Self {
        B127_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B127_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B127` writer - B127"]
pub struct B127_W<'a> {
    w: &'a mut W,
}
impl<'a> B127_W<'a> {
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
    #[doc = "Bit 0 - B96"]
    #[inline(always)]
    pub fn b96(&self) -> B96_R {
        B96_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B97"]
    #[inline(always)]
    pub fn b97(&self) -> B97_R {
        B97_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B98"]
    #[inline(always)]
    pub fn b98(&self) -> B98_R {
        B98_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B99"]
    #[inline(always)]
    pub fn b99(&self) -> B99_R {
        B99_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B100"]
    #[inline(always)]
    pub fn b100(&self) -> B100_R {
        B100_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B101"]
    #[inline(always)]
    pub fn b101(&self) -> B101_R {
        B101_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B102"]
    #[inline(always)]
    pub fn b102(&self) -> B102_R {
        B102_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B103"]
    #[inline(always)]
    pub fn b103(&self) -> B103_R {
        B103_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B104"]
    #[inline(always)]
    pub fn b104(&self) -> B104_R {
        B104_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B105"]
    #[inline(always)]
    pub fn b105(&self) -> B105_R {
        B105_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B106"]
    #[inline(always)]
    pub fn b106(&self) -> B106_R {
        B106_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B107"]
    #[inline(always)]
    pub fn b107(&self) -> B107_R {
        B107_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B108"]
    #[inline(always)]
    pub fn b108(&self) -> B108_R {
        B108_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B109"]
    #[inline(always)]
    pub fn b109(&self) -> B109_R {
        B109_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B110"]
    #[inline(always)]
    pub fn b110(&self) -> B110_R {
        B110_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B111"]
    #[inline(always)]
    pub fn b111(&self) -> B111_R {
        B111_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B112"]
    #[inline(always)]
    pub fn b112(&self) -> B112_R {
        B112_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B113"]
    #[inline(always)]
    pub fn b113(&self) -> B113_R {
        B113_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B114"]
    #[inline(always)]
    pub fn b114(&self) -> B114_R {
        B114_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B115"]
    #[inline(always)]
    pub fn b115(&self) -> B115_R {
        B115_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B116"]
    #[inline(always)]
    pub fn b116(&self) -> B116_R {
        B116_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B117"]
    #[inline(always)]
    pub fn b117(&self) -> B117_R {
        B117_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B118"]
    #[inline(always)]
    pub fn b118(&self) -> B118_R {
        B118_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B119"]
    #[inline(always)]
    pub fn b119(&self) -> B119_R {
        B119_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B120"]
    #[inline(always)]
    pub fn b120(&self) -> B120_R {
        B120_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B121"]
    #[inline(always)]
    pub fn b121(&self) -> B121_R {
        B121_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B122"]
    #[inline(always)]
    pub fn b122(&self) -> B122_R {
        B122_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B123"]
    #[inline(always)]
    pub fn b123(&self) -> B123_R {
        B123_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B124"]
    #[inline(always)]
    pub fn b124(&self) -> B124_R {
        B124_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B125"]
    #[inline(always)]
    pub fn b125(&self) -> B125_R {
        B125_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B126"]
    #[inline(always)]
    pub fn b126(&self) -> B126_R {
        B126_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B127"]
    #[inline(always)]
    pub fn b127(&self) -> B127_R {
        B127_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B96"]
    #[inline(always)]
    pub fn b96(&mut self) -> B96_W {
        B96_W { w: self }
    }
    #[doc = "Bit 1 - B97"]
    #[inline(always)]
    pub fn b97(&mut self) -> B97_W {
        B97_W { w: self }
    }
    #[doc = "Bit 2 - B98"]
    #[inline(always)]
    pub fn b98(&mut self) -> B98_W {
        B98_W { w: self }
    }
    #[doc = "Bit 3 - B99"]
    #[inline(always)]
    pub fn b99(&mut self) -> B99_W {
        B99_W { w: self }
    }
    #[doc = "Bit 4 - B100"]
    #[inline(always)]
    pub fn b100(&mut self) -> B100_W {
        B100_W { w: self }
    }
    #[doc = "Bit 5 - B101"]
    #[inline(always)]
    pub fn b101(&mut self) -> B101_W {
        B101_W { w: self }
    }
    #[doc = "Bit 6 - B102"]
    #[inline(always)]
    pub fn b102(&mut self) -> B102_W {
        B102_W { w: self }
    }
    #[doc = "Bit 7 - B103"]
    #[inline(always)]
    pub fn b103(&mut self) -> B103_W {
        B103_W { w: self }
    }
    #[doc = "Bit 8 - B104"]
    #[inline(always)]
    pub fn b104(&mut self) -> B104_W {
        B104_W { w: self }
    }
    #[doc = "Bit 9 - B105"]
    #[inline(always)]
    pub fn b105(&mut self) -> B105_W {
        B105_W { w: self }
    }
    #[doc = "Bit 10 - B106"]
    #[inline(always)]
    pub fn b106(&mut self) -> B106_W {
        B106_W { w: self }
    }
    #[doc = "Bit 11 - B107"]
    #[inline(always)]
    pub fn b107(&mut self) -> B107_W {
        B107_W { w: self }
    }
    #[doc = "Bit 12 - B108"]
    #[inline(always)]
    pub fn b108(&mut self) -> B108_W {
        B108_W { w: self }
    }
    #[doc = "Bit 13 - B109"]
    #[inline(always)]
    pub fn b109(&mut self) -> B109_W {
        B109_W { w: self }
    }
    #[doc = "Bit 14 - B110"]
    #[inline(always)]
    pub fn b110(&mut self) -> B110_W {
        B110_W { w: self }
    }
    #[doc = "Bit 15 - B111"]
    #[inline(always)]
    pub fn b111(&mut self) -> B111_W {
        B111_W { w: self }
    }
    #[doc = "Bit 16 - B112"]
    #[inline(always)]
    pub fn b112(&mut self) -> B112_W {
        B112_W { w: self }
    }
    #[doc = "Bit 17 - B113"]
    #[inline(always)]
    pub fn b113(&mut self) -> B113_W {
        B113_W { w: self }
    }
    #[doc = "Bit 18 - B114"]
    #[inline(always)]
    pub fn b114(&mut self) -> B114_W {
        B114_W { w: self }
    }
    #[doc = "Bit 19 - B115"]
    #[inline(always)]
    pub fn b115(&mut self) -> B115_W {
        B115_W { w: self }
    }
    #[doc = "Bit 20 - B116"]
    #[inline(always)]
    pub fn b116(&mut self) -> B116_W {
        B116_W { w: self }
    }
    #[doc = "Bit 21 - B117"]
    #[inline(always)]
    pub fn b117(&mut self) -> B117_W {
        B117_W { w: self }
    }
    #[doc = "Bit 22 - B118"]
    #[inline(always)]
    pub fn b118(&mut self) -> B118_W {
        B118_W { w: self }
    }
    #[doc = "Bit 23 - B119"]
    #[inline(always)]
    pub fn b119(&mut self) -> B119_W {
        B119_W { w: self }
    }
    #[doc = "Bit 24 - B120"]
    #[inline(always)]
    pub fn b120(&mut self) -> B120_W {
        B120_W { w: self }
    }
    #[doc = "Bit 25 - B121"]
    #[inline(always)]
    pub fn b121(&mut self) -> B121_W {
        B121_W { w: self }
    }
    #[doc = "Bit 26 - B122"]
    #[inline(always)]
    pub fn b122(&mut self) -> B122_W {
        B122_W { w: self }
    }
    #[doc = "Bit 27 - B123"]
    #[inline(always)]
    pub fn b123(&mut self) -> B123_W {
        B123_W { w: self }
    }
    #[doc = "Bit 28 - B124"]
    #[inline(always)]
    pub fn b124(&mut self) -> B124_W {
        B124_W { w: self }
    }
    #[doc = "Bit 29 - B125"]
    #[inline(always)]
    pub fn b125(&mut self) -> B125_W {
        B125_W { w: self }
    }
    #[doc = "Bit 30 - B126"]
    #[inline(always)]
    pub fn b126(&mut self) -> B126_W {
        B126_W { w: self }
    }
    #[doc = "Bit 31 - B127"]
    #[inline(always)]
    pub fn b127(&mut self) -> B127_W {
        B127_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr3](index.html) module"]
pub struct MPCBB2_VCTR3_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr3::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr3::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR3 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
