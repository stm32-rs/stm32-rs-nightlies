#[doc = "Register `FR1` reader"]
pub struct R(crate::R<FR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FR1` writer"]
pub struct W(crate::W<FR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FR1_SPEC>;
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
impl From<crate::W<FR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FB0` reader - Filter bits"]
pub struct FB0_R(crate::FieldReader<bool, bool>);
impl FB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB0` writer - Filter bits"]
pub struct FB0_W<'a> {
    w: &'a mut W,
}
impl<'a> FB0_W<'a> {
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
#[doc = "Field `FB1` reader - Filter bits"]
pub struct FB1_R(crate::FieldReader<bool, bool>);
impl FB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB1` writer - Filter bits"]
pub struct FB1_W<'a> {
    w: &'a mut W,
}
impl<'a> FB1_W<'a> {
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
#[doc = "Field `FB2` reader - Filter bits"]
pub struct FB2_R(crate::FieldReader<bool, bool>);
impl FB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB2` writer - Filter bits"]
pub struct FB2_W<'a> {
    w: &'a mut W,
}
impl<'a> FB2_W<'a> {
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
#[doc = "Field `FB3` reader - Filter bits"]
pub struct FB3_R(crate::FieldReader<bool, bool>);
impl FB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB3` writer - Filter bits"]
pub struct FB3_W<'a> {
    w: &'a mut W,
}
impl<'a> FB3_W<'a> {
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
#[doc = "Field `FB4` reader - Filter bits"]
pub struct FB4_R(crate::FieldReader<bool, bool>);
impl FB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB4` writer - Filter bits"]
pub struct FB4_W<'a> {
    w: &'a mut W,
}
impl<'a> FB4_W<'a> {
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
#[doc = "Field `FB5` reader - Filter bits"]
pub struct FB5_R(crate::FieldReader<bool, bool>);
impl FB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB5` writer - Filter bits"]
pub struct FB5_W<'a> {
    w: &'a mut W,
}
impl<'a> FB5_W<'a> {
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
#[doc = "Field `FB6` reader - Filter bits"]
pub struct FB6_R(crate::FieldReader<bool, bool>);
impl FB6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB6` writer - Filter bits"]
pub struct FB6_W<'a> {
    w: &'a mut W,
}
impl<'a> FB6_W<'a> {
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
#[doc = "Field `FB7` reader - Filter bits"]
pub struct FB7_R(crate::FieldReader<bool, bool>);
impl FB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB7` writer - Filter bits"]
pub struct FB7_W<'a> {
    w: &'a mut W,
}
impl<'a> FB7_W<'a> {
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
#[doc = "Field `FB8` reader - Filter bits"]
pub struct FB8_R(crate::FieldReader<bool, bool>);
impl FB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB8` writer - Filter bits"]
pub struct FB8_W<'a> {
    w: &'a mut W,
}
impl<'a> FB8_W<'a> {
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
#[doc = "Field `FB9` reader - Filter bits"]
pub struct FB9_R(crate::FieldReader<bool, bool>);
impl FB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB9` writer - Filter bits"]
pub struct FB9_W<'a> {
    w: &'a mut W,
}
impl<'a> FB9_W<'a> {
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
#[doc = "Field `FB10` reader - Filter bits"]
pub struct FB10_R(crate::FieldReader<bool, bool>);
impl FB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB10` writer - Filter bits"]
pub struct FB10_W<'a> {
    w: &'a mut W,
}
impl<'a> FB10_W<'a> {
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
#[doc = "Field `FB11` reader - Filter bits"]
pub struct FB11_R(crate::FieldReader<bool, bool>);
impl FB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB11` writer - Filter bits"]
pub struct FB11_W<'a> {
    w: &'a mut W,
}
impl<'a> FB11_W<'a> {
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
#[doc = "Field `FB12` reader - Filter bits"]
pub struct FB12_R(crate::FieldReader<bool, bool>);
impl FB12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB12` writer - Filter bits"]
pub struct FB12_W<'a> {
    w: &'a mut W,
}
impl<'a> FB12_W<'a> {
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
#[doc = "Field `FB13` reader - Filter bits"]
pub struct FB13_R(crate::FieldReader<bool, bool>);
impl FB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB13` writer - Filter bits"]
pub struct FB13_W<'a> {
    w: &'a mut W,
}
impl<'a> FB13_W<'a> {
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
#[doc = "Field `FB14` reader - Filter bits"]
pub struct FB14_R(crate::FieldReader<bool, bool>);
impl FB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB14` writer - Filter bits"]
pub struct FB14_W<'a> {
    w: &'a mut W,
}
impl<'a> FB14_W<'a> {
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
#[doc = "Field `FB15` reader - Filter bits"]
pub struct FB15_R(crate::FieldReader<bool, bool>);
impl FB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB15` writer - Filter bits"]
pub struct FB15_W<'a> {
    w: &'a mut W,
}
impl<'a> FB15_W<'a> {
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
#[doc = "Field `FB16` reader - Filter bits"]
pub struct FB16_R(crate::FieldReader<bool, bool>);
impl FB16_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB16` writer - Filter bits"]
pub struct FB16_W<'a> {
    w: &'a mut W,
}
impl<'a> FB16_W<'a> {
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
#[doc = "Field `FB17` reader - Filter bits"]
pub struct FB17_R(crate::FieldReader<bool, bool>);
impl FB17_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB17` writer - Filter bits"]
pub struct FB17_W<'a> {
    w: &'a mut W,
}
impl<'a> FB17_W<'a> {
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
#[doc = "Field `FB18` reader - Filter bits"]
pub struct FB18_R(crate::FieldReader<bool, bool>);
impl FB18_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB18` writer - Filter bits"]
pub struct FB18_W<'a> {
    w: &'a mut W,
}
impl<'a> FB18_W<'a> {
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
#[doc = "Field `FB19` reader - Filter bits"]
pub struct FB19_R(crate::FieldReader<bool, bool>);
impl FB19_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB19` writer - Filter bits"]
pub struct FB19_W<'a> {
    w: &'a mut W,
}
impl<'a> FB19_W<'a> {
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
#[doc = "Field `FB20` reader - Filter bits"]
pub struct FB20_R(crate::FieldReader<bool, bool>);
impl FB20_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB20` writer - Filter bits"]
pub struct FB20_W<'a> {
    w: &'a mut W,
}
impl<'a> FB20_W<'a> {
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
#[doc = "Field `FB21` reader - Filter bits"]
pub struct FB21_R(crate::FieldReader<bool, bool>);
impl FB21_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB21` writer - Filter bits"]
pub struct FB21_W<'a> {
    w: &'a mut W,
}
impl<'a> FB21_W<'a> {
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
#[doc = "Field `FB22` reader - Filter bits"]
pub struct FB22_R(crate::FieldReader<bool, bool>);
impl FB22_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB22` writer - Filter bits"]
pub struct FB22_W<'a> {
    w: &'a mut W,
}
impl<'a> FB22_W<'a> {
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
#[doc = "Field `FB23` reader - Filter bits"]
pub struct FB23_R(crate::FieldReader<bool, bool>);
impl FB23_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB23` writer - Filter bits"]
pub struct FB23_W<'a> {
    w: &'a mut W,
}
impl<'a> FB23_W<'a> {
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
#[doc = "Field `FB24` reader - Filter bits"]
pub struct FB24_R(crate::FieldReader<bool, bool>);
impl FB24_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB24` writer - Filter bits"]
pub struct FB24_W<'a> {
    w: &'a mut W,
}
impl<'a> FB24_W<'a> {
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
#[doc = "Field `FB25` reader - Filter bits"]
pub struct FB25_R(crate::FieldReader<bool, bool>);
impl FB25_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB25` writer - Filter bits"]
pub struct FB25_W<'a> {
    w: &'a mut W,
}
impl<'a> FB25_W<'a> {
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
#[doc = "Field `FB26` reader - Filter bits"]
pub struct FB26_R(crate::FieldReader<bool, bool>);
impl FB26_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB26` writer - Filter bits"]
pub struct FB26_W<'a> {
    w: &'a mut W,
}
impl<'a> FB26_W<'a> {
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
#[doc = "Field `FB27` reader - Filter bits"]
pub struct FB27_R(crate::FieldReader<bool, bool>);
impl FB27_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB27` writer - Filter bits"]
pub struct FB27_W<'a> {
    w: &'a mut W,
}
impl<'a> FB27_W<'a> {
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
#[doc = "Field `FB28` reader - Filter bits"]
pub struct FB28_R(crate::FieldReader<bool, bool>);
impl FB28_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB28` writer - Filter bits"]
pub struct FB28_W<'a> {
    w: &'a mut W,
}
impl<'a> FB28_W<'a> {
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
#[doc = "Field `FB29` reader - Filter bits"]
pub struct FB29_R(crate::FieldReader<bool, bool>);
impl FB29_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB29` writer - Filter bits"]
pub struct FB29_W<'a> {
    w: &'a mut W,
}
impl<'a> FB29_W<'a> {
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
#[doc = "Field `FB30` reader - Filter bits"]
pub struct FB30_R(crate::FieldReader<bool, bool>);
impl FB30_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB30` writer - Filter bits"]
pub struct FB30_W<'a> {
    w: &'a mut W,
}
impl<'a> FB30_W<'a> {
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
#[doc = "Field `FB31` reader - Filter bits"]
pub struct FB31_R(crate::FieldReader<bool, bool>);
impl FB31_R {
    pub(crate) fn new(bits: bool) -> Self {
        FB31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB31` writer - Filter bits"]
pub struct FB31_W<'a> {
    w: &'a mut W,
}
impl<'a> FB31_W<'a> {
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
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&self) -> FB0_R {
        FB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&self) -> FB1_R {
        FB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&self) -> FB2_R {
        FB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&self) -> FB3_R {
        FB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&self) -> FB4_R {
        FB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&self) -> FB5_R {
        FB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&self) -> FB6_R {
        FB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&self) -> FB7_R {
        FB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&self) -> FB8_R {
        FB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&self) -> FB9_R {
        FB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&self) -> FB10_R {
        FB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&self) -> FB11_R {
        FB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&self) -> FB12_R {
        FB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&self) -> FB13_R {
        FB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&self) -> FB14_R {
        FB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&self) -> FB15_R {
        FB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&self) -> FB16_R {
        FB16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&self) -> FB17_R {
        FB17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&self) -> FB18_R {
        FB18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&self) -> FB19_R {
        FB19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&self) -> FB20_R {
        FB20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&self) -> FB21_R {
        FB21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&self) -> FB22_R {
        FB22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&self) -> FB23_R {
        FB23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&self) -> FB24_R {
        FB24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&self) -> FB25_R {
        FB25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&self) -> FB26_R {
        FB26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&self) -> FB27_R {
        FB27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&self) -> FB28_R {
        FB28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&self) -> FB29_R {
        FB29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&self) -> FB30_R {
        FB30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&self) -> FB31_R {
        FB31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter bits"]
    #[inline(always)]
    pub fn fb0(&mut self) -> FB0_W {
        FB0_W { w: self }
    }
    #[doc = "Bit 1 - Filter bits"]
    #[inline(always)]
    pub fn fb1(&mut self) -> FB1_W {
        FB1_W { w: self }
    }
    #[doc = "Bit 2 - Filter bits"]
    #[inline(always)]
    pub fn fb2(&mut self) -> FB2_W {
        FB2_W { w: self }
    }
    #[doc = "Bit 3 - Filter bits"]
    #[inline(always)]
    pub fn fb3(&mut self) -> FB3_W {
        FB3_W { w: self }
    }
    #[doc = "Bit 4 - Filter bits"]
    #[inline(always)]
    pub fn fb4(&mut self) -> FB4_W {
        FB4_W { w: self }
    }
    #[doc = "Bit 5 - Filter bits"]
    #[inline(always)]
    pub fn fb5(&mut self) -> FB5_W {
        FB5_W { w: self }
    }
    #[doc = "Bit 6 - Filter bits"]
    #[inline(always)]
    pub fn fb6(&mut self) -> FB6_W {
        FB6_W { w: self }
    }
    #[doc = "Bit 7 - Filter bits"]
    #[inline(always)]
    pub fn fb7(&mut self) -> FB7_W {
        FB7_W { w: self }
    }
    #[doc = "Bit 8 - Filter bits"]
    #[inline(always)]
    pub fn fb8(&mut self) -> FB8_W {
        FB8_W { w: self }
    }
    #[doc = "Bit 9 - Filter bits"]
    #[inline(always)]
    pub fn fb9(&mut self) -> FB9_W {
        FB9_W { w: self }
    }
    #[doc = "Bit 10 - Filter bits"]
    #[inline(always)]
    pub fn fb10(&mut self) -> FB10_W {
        FB10_W { w: self }
    }
    #[doc = "Bit 11 - Filter bits"]
    #[inline(always)]
    pub fn fb11(&mut self) -> FB11_W {
        FB11_W { w: self }
    }
    #[doc = "Bit 12 - Filter bits"]
    #[inline(always)]
    pub fn fb12(&mut self) -> FB12_W {
        FB12_W { w: self }
    }
    #[doc = "Bit 13 - Filter bits"]
    #[inline(always)]
    pub fn fb13(&mut self) -> FB13_W {
        FB13_W { w: self }
    }
    #[doc = "Bit 14 - Filter bits"]
    #[inline(always)]
    pub fn fb14(&mut self) -> FB14_W {
        FB14_W { w: self }
    }
    #[doc = "Bit 15 - Filter bits"]
    #[inline(always)]
    pub fn fb15(&mut self) -> FB15_W {
        FB15_W { w: self }
    }
    #[doc = "Bit 16 - Filter bits"]
    #[inline(always)]
    pub fn fb16(&mut self) -> FB16_W {
        FB16_W { w: self }
    }
    #[doc = "Bit 17 - Filter bits"]
    #[inline(always)]
    pub fn fb17(&mut self) -> FB17_W {
        FB17_W { w: self }
    }
    #[doc = "Bit 18 - Filter bits"]
    #[inline(always)]
    pub fn fb18(&mut self) -> FB18_W {
        FB18_W { w: self }
    }
    #[doc = "Bit 19 - Filter bits"]
    #[inline(always)]
    pub fn fb19(&mut self) -> FB19_W {
        FB19_W { w: self }
    }
    #[doc = "Bit 20 - Filter bits"]
    #[inline(always)]
    pub fn fb20(&mut self) -> FB20_W {
        FB20_W { w: self }
    }
    #[doc = "Bit 21 - Filter bits"]
    #[inline(always)]
    pub fn fb21(&mut self) -> FB21_W {
        FB21_W { w: self }
    }
    #[doc = "Bit 22 - Filter bits"]
    #[inline(always)]
    pub fn fb22(&mut self) -> FB22_W {
        FB22_W { w: self }
    }
    #[doc = "Bit 23 - Filter bits"]
    #[inline(always)]
    pub fn fb23(&mut self) -> FB23_W {
        FB23_W { w: self }
    }
    #[doc = "Bit 24 - Filter bits"]
    #[inline(always)]
    pub fn fb24(&mut self) -> FB24_W {
        FB24_W { w: self }
    }
    #[doc = "Bit 25 - Filter bits"]
    #[inline(always)]
    pub fn fb25(&mut self) -> FB25_W {
        FB25_W { w: self }
    }
    #[doc = "Bit 26 - Filter bits"]
    #[inline(always)]
    pub fn fb26(&mut self) -> FB26_W {
        FB26_W { w: self }
    }
    #[doc = "Bit 27 - Filter bits"]
    #[inline(always)]
    pub fn fb27(&mut self) -> FB27_W {
        FB27_W { w: self }
    }
    #[doc = "Bit 28 - Filter bits"]
    #[inline(always)]
    pub fn fb28(&mut self) -> FB28_W {
        FB28_W { w: self }
    }
    #[doc = "Bit 29 - Filter bits"]
    #[inline(always)]
    pub fn fb29(&mut self) -> FB29_W {
        FB29_W { w: self }
    }
    #[doc = "Bit 30 - Filter bits"]
    #[inline(always)]
    pub fn fb30(&mut self) -> FB30_W {
        FB30_W { w: self }
    }
    #[doc = "Bit 31 - Filter bits"]
    #[inline(always)]
    pub fn fb31(&mut self) -> FB31_W {
        FB31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter bank 0 register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fr1](index.html) module"]
pub struct FR1_SPEC;
impl crate::RegisterSpec for FR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fr1::R](R) reader structure"]
impl crate::Readable for FR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fr1::W](W) writer structure"]
impl crate::Writable for FR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FR1 to value 0"]
impl crate::Resettable for FR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
