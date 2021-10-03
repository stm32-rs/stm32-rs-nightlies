#[doc = "Register `MPCBB1_VCTR34` reader"]
pub struct R(crate::R<MPCBB1_VCTR34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR34` writer"]
pub struct W(crate::W<MPCBB1_VCTR34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR34_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1088` reader - B1088"]
pub struct B1088_R(crate::FieldReader<bool, bool>);
impl B1088_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1088_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1088_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1088` writer - B1088"]
pub struct B1088_W<'a> {
    w: &'a mut W,
}
impl<'a> B1088_W<'a> {
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
#[doc = "Field `B1089` reader - B1089"]
pub struct B1089_R(crate::FieldReader<bool, bool>);
impl B1089_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1089_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1089_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1089` writer - B1089"]
pub struct B1089_W<'a> {
    w: &'a mut W,
}
impl<'a> B1089_W<'a> {
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
#[doc = "Field `B1090` reader - B1090"]
pub struct B1090_R(crate::FieldReader<bool, bool>);
impl B1090_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1090_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1090_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1090` writer - B1090"]
pub struct B1090_W<'a> {
    w: &'a mut W,
}
impl<'a> B1090_W<'a> {
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
#[doc = "Field `B1091` reader - B1091"]
pub struct B1091_R(crate::FieldReader<bool, bool>);
impl B1091_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1091_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1091_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1091` writer - B1091"]
pub struct B1091_W<'a> {
    w: &'a mut W,
}
impl<'a> B1091_W<'a> {
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
#[doc = "Field `B1092` reader - B1092"]
pub struct B1092_R(crate::FieldReader<bool, bool>);
impl B1092_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1092_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1092_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1092` writer - B1092"]
pub struct B1092_W<'a> {
    w: &'a mut W,
}
impl<'a> B1092_W<'a> {
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
#[doc = "Field `B1093` reader - B1093"]
pub struct B1093_R(crate::FieldReader<bool, bool>);
impl B1093_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1093_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1093_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1093` writer - B1093"]
pub struct B1093_W<'a> {
    w: &'a mut W,
}
impl<'a> B1093_W<'a> {
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
#[doc = "Field `B1094` reader - B1094"]
pub struct B1094_R(crate::FieldReader<bool, bool>);
impl B1094_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1094_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1094_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1094` writer - B1094"]
pub struct B1094_W<'a> {
    w: &'a mut W,
}
impl<'a> B1094_W<'a> {
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
#[doc = "Field `B1095` reader - B1095"]
pub struct B1095_R(crate::FieldReader<bool, bool>);
impl B1095_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1095_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1095_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1095` writer - B1095"]
pub struct B1095_W<'a> {
    w: &'a mut W,
}
impl<'a> B1095_W<'a> {
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
#[doc = "Field `B1096` reader - B1096"]
pub struct B1096_R(crate::FieldReader<bool, bool>);
impl B1096_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1096_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1096_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1096` writer - B1096"]
pub struct B1096_W<'a> {
    w: &'a mut W,
}
impl<'a> B1096_W<'a> {
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
#[doc = "Field `B1097` reader - B1097"]
pub struct B1097_R(crate::FieldReader<bool, bool>);
impl B1097_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1097_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1097_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1097` writer - B1097"]
pub struct B1097_W<'a> {
    w: &'a mut W,
}
impl<'a> B1097_W<'a> {
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
#[doc = "Field `B1098` reader - B1098"]
pub struct B1098_R(crate::FieldReader<bool, bool>);
impl B1098_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1098_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1098_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1098` writer - B1098"]
pub struct B1098_W<'a> {
    w: &'a mut W,
}
impl<'a> B1098_W<'a> {
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
#[doc = "Field `B1099` reader - B1099"]
pub struct B1099_R(crate::FieldReader<bool, bool>);
impl B1099_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1099_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1099_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1099` writer - B1099"]
pub struct B1099_W<'a> {
    w: &'a mut W,
}
impl<'a> B1099_W<'a> {
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
#[doc = "Field `B1100` reader - B1100"]
pub struct B1100_R(crate::FieldReader<bool, bool>);
impl B1100_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1100_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1100_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1100` writer - B1100"]
pub struct B1100_W<'a> {
    w: &'a mut W,
}
impl<'a> B1100_W<'a> {
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
#[doc = "Field `B1101` reader - B1101"]
pub struct B1101_R(crate::FieldReader<bool, bool>);
impl B1101_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1101_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1101_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1101` writer - B1101"]
pub struct B1101_W<'a> {
    w: &'a mut W,
}
impl<'a> B1101_W<'a> {
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
#[doc = "Field `B1102` reader - B1102"]
pub struct B1102_R(crate::FieldReader<bool, bool>);
impl B1102_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1102_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1102_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1102` writer - B1102"]
pub struct B1102_W<'a> {
    w: &'a mut W,
}
impl<'a> B1102_W<'a> {
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
#[doc = "Field `B1103` reader - B1103"]
pub struct B1103_R(crate::FieldReader<bool, bool>);
impl B1103_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1103_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1103_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1103` writer - B1103"]
pub struct B1103_W<'a> {
    w: &'a mut W,
}
impl<'a> B1103_W<'a> {
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
#[doc = "Field `B1104` reader - B1104"]
pub struct B1104_R(crate::FieldReader<bool, bool>);
impl B1104_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1104_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1104_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1104` writer - B1104"]
pub struct B1104_W<'a> {
    w: &'a mut W,
}
impl<'a> B1104_W<'a> {
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
#[doc = "Field `B1105` reader - B1105"]
pub struct B1105_R(crate::FieldReader<bool, bool>);
impl B1105_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1105_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1105_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1105` writer - B1105"]
pub struct B1105_W<'a> {
    w: &'a mut W,
}
impl<'a> B1105_W<'a> {
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
#[doc = "Field `B1106` reader - B1106"]
pub struct B1106_R(crate::FieldReader<bool, bool>);
impl B1106_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1106_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1106_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1106` writer - B1106"]
pub struct B1106_W<'a> {
    w: &'a mut W,
}
impl<'a> B1106_W<'a> {
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
#[doc = "Field `B1107` reader - B1107"]
pub struct B1107_R(crate::FieldReader<bool, bool>);
impl B1107_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1107_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1107_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1107` writer - B1107"]
pub struct B1107_W<'a> {
    w: &'a mut W,
}
impl<'a> B1107_W<'a> {
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
#[doc = "Field `B1108` reader - B1108"]
pub struct B1108_R(crate::FieldReader<bool, bool>);
impl B1108_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1108_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1108_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1108` writer - B1108"]
pub struct B1108_W<'a> {
    w: &'a mut W,
}
impl<'a> B1108_W<'a> {
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
#[doc = "Field `B1109` reader - B1109"]
pub struct B1109_R(crate::FieldReader<bool, bool>);
impl B1109_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1109_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1109_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1109` writer - B1109"]
pub struct B1109_W<'a> {
    w: &'a mut W,
}
impl<'a> B1109_W<'a> {
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
#[doc = "Field `B1110` reader - B1110"]
pub struct B1110_R(crate::FieldReader<bool, bool>);
impl B1110_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1110_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1110_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1110` writer - B1110"]
pub struct B1110_W<'a> {
    w: &'a mut W,
}
impl<'a> B1110_W<'a> {
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
#[doc = "Field `B1111` reader - B1111"]
pub struct B1111_R(crate::FieldReader<bool, bool>);
impl B1111_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1111_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1111_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1111` writer - B1111"]
pub struct B1111_W<'a> {
    w: &'a mut W,
}
impl<'a> B1111_W<'a> {
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
#[doc = "Field `B1112` reader - B1112"]
pub struct B1112_R(crate::FieldReader<bool, bool>);
impl B1112_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1112_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1112_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1112` writer - B1112"]
pub struct B1112_W<'a> {
    w: &'a mut W,
}
impl<'a> B1112_W<'a> {
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
#[doc = "Field `B1113` reader - B1113"]
pub struct B1113_R(crate::FieldReader<bool, bool>);
impl B1113_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1113_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1113_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1113` writer - B1113"]
pub struct B1113_W<'a> {
    w: &'a mut W,
}
impl<'a> B1113_W<'a> {
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
#[doc = "Field `B1114` reader - B1114"]
pub struct B1114_R(crate::FieldReader<bool, bool>);
impl B1114_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1114_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1114_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1114` writer - B1114"]
pub struct B1114_W<'a> {
    w: &'a mut W,
}
impl<'a> B1114_W<'a> {
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
#[doc = "Field `B1115` reader - B1115"]
pub struct B1115_R(crate::FieldReader<bool, bool>);
impl B1115_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1115_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1115_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1115` writer - B1115"]
pub struct B1115_W<'a> {
    w: &'a mut W,
}
impl<'a> B1115_W<'a> {
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
#[doc = "Field `B1116` reader - B1116"]
pub struct B1116_R(crate::FieldReader<bool, bool>);
impl B1116_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1116_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1116_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1116` writer - B1116"]
pub struct B1116_W<'a> {
    w: &'a mut W,
}
impl<'a> B1116_W<'a> {
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
#[doc = "Field `B1117` reader - B1117"]
pub struct B1117_R(crate::FieldReader<bool, bool>);
impl B1117_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1117_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1117_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1117` writer - B1117"]
pub struct B1117_W<'a> {
    w: &'a mut W,
}
impl<'a> B1117_W<'a> {
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
#[doc = "Field `B1118` reader - B1118"]
pub struct B1118_R(crate::FieldReader<bool, bool>);
impl B1118_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1118_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1118_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1118` writer - B1118"]
pub struct B1118_W<'a> {
    w: &'a mut W,
}
impl<'a> B1118_W<'a> {
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
#[doc = "Field `B1119` reader - B1119"]
pub struct B1119_R(crate::FieldReader<bool, bool>);
impl B1119_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1119_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1119_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1119` writer - B1119"]
pub struct B1119_W<'a> {
    w: &'a mut W,
}
impl<'a> B1119_W<'a> {
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
    #[doc = "Bit 0 - B1088"]
    #[inline(always)]
    pub fn b1088(&self) -> B1088_R {
        B1088_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1089"]
    #[inline(always)]
    pub fn b1089(&self) -> B1089_R {
        B1089_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1090"]
    #[inline(always)]
    pub fn b1090(&self) -> B1090_R {
        B1090_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1091"]
    #[inline(always)]
    pub fn b1091(&self) -> B1091_R {
        B1091_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1092"]
    #[inline(always)]
    pub fn b1092(&self) -> B1092_R {
        B1092_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1093"]
    #[inline(always)]
    pub fn b1093(&self) -> B1093_R {
        B1093_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1094"]
    #[inline(always)]
    pub fn b1094(&self) -> B1094_R {
        B1094_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1095"]
    #[inline(always)]
    pub fn b1095(&self) -> B1095_R {
        B1095_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1096"]
    #[inline(always)]
    pub fn b1096(&self) -> B1096_R {
        B1096_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1097"]
    #[inline(always)]
    pub fn b1097(&self) -> B1097_R {
        B1097_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1098"]
    #[inline(always)]
    pub fn b1098(&self) -> B1098_R {
        B1098_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1099"]
    #[inline(always)]
    pub fn b1099(&self) -> B1099_R {
        B1099_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1100"]
    #[inline(always)]
    pub fn b1100(&self) -> B1100_R {
        B1100_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1101"]
    #[inline(always)]
    pub fn b1101(&self) -> B1101_R {
        B1101_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1102"]
    #[inline(always)]
    pub fn b1102(&self) -> B1102_R {
        B1102_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1103"]
    #[inline(always)]
    pub fn b1103(&self) -> B1103_R {
        B1103_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1104"]
    #[inline(always)]
    pub fn b1104(&self) -> B1104_R {
        B1104_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1105"]
    #[inline(always)]
    pub fn b1105(&self) -> B1105_R {
        B1105_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1106"]
    #[inline(always)]
    pub fn b1106(&self) -> B1106_R {
        B1106_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1107"]
    #[inline(always)]
    pub fn b1107(&self) -> B1107_R {
        B1107_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1108"]
    #[inline(always)]
    pub fn b1108(&self) -> B1108_R {
        B1108_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1109"]
    #[inline(always)]
    pub fn b1109(&self) -> B1109_R {
        B1109_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1110"]
    #[inline(always)]
    pub fn b1110(&self) -> B1110_R {
        B1110_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1111"]
    #[inline(always)]
    pub fn b1111(&self) -> B1111_R {
        B1111_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1112"]
    #[inline(always)]
    pub fn b1112(&self) -> B1112_R {
        B1112_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1113"]
    #[inline(always)]
    pub fn b1113(&self) -> B1113_R {
        B1113_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1114"]
    #[inline(always)]
    pub fn b1114(&self) -> B1114_R {
        B1114_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1115"]
    #[inline(always)]
    pub fn b1115(&self) -> B1115_R {
        B1115_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1116"]
    #[inline(always)]
    pub fn b1116(&self) -> B1116_R {
        B1116_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1117"]
    #[inline(always)]
    pub fn b1117(&self) -> B1117_R {
        B1117_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1118"]
    #[inline(always)]
    pub fn b1118(&self) -> B1118_R {
        B1118_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1119"]
    #[inline(always)]
    pub fn b1119(&self) -> B1119_R {
        B1119_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1088"]
    #[inline(always)]
    pub fn b1088(&mut self) -> B1088_W {
        B1088_W { w: self }
    }
    #[doc = "Bit 1 - B1089"]
    #[inline(always)]
    pub fn b1089(&mut self) -> B1089_W {
        B1089_W { w: self }
    }
    #[doc = "Bit 2 - B1090"]
    #[inline(always)]
    pub fn b1090(&mut self) -> B1090_W {
        B1090_W { w: self }
    }
    #[doc = "Bit 3 - B1091"]
    #[inline(always)]
    pub fn b1091(&mut self) -> B1091_W {
        B1091_W { w: self }
    }
    #[doc = "Bit 4 - B1092"]
    #[inline(always)]
    pub fn b1092(&mut self) -> B1092_W {
        B1092_W { w: self }
    }
    #[doc = "Bit 5 - B1093"]
    #[inline(always)]
    pub fn b1093(&mut self) -> B1093_W {
        B1093_W { w: self }
    }
    #[doc = "Bit 6 - B1094"]
    #[inline(always)]
    pub fn b1094(&mut self) -> B1094_W {
        B1094_W { w: self }
    }
    #[doc = "Bit 7 - B1095"]
    #[inline(always)]
    pub fn b1095(&mut self) -> B1095_W {
        B1095_W { w: self }
    }
    #[doc = "Bit 8 - B1096"]
    #[inline(always)]
    pub fn b1096(&mut self) -> B1096_W {
        B1096_W { w: self }
    }
    #[doc = "Bit 9 - B1097"]
    #[inline(always)]
    pub fn b1097(&mut self) -> B1097_W {
        B1097_W { w: self }
    }
    #[doc = "Bit 10 - B1098"]
    #[inline(always)]
    pub fn b1098(&mut self) -> B1098_W {
        B1098_W { w: self }
    }
    #[doc = "Bit 11 - B1099"]
    #[inline(always)]
    pub fn b1099(&mut self) -> B1099_W {
        B1099_W { w: self }
    }
    #[doc = "Bit 12 - B1100"]
    #[inline(always)]
    pub fn b1100(&mut self) -> B1100_W {
        B1100_W { w: self }
    }
    #[doc = "Bit 13 - B1101"]
    #[inline(always)]
    pub fn b1101(&mut self) -> B1101_W {
        B1101_W { w: self }
    }
    #[doc = "Bit 14 - B1102"]
    #[inline(always)]
    pub fn b1102(&mut self) -> B1102_W {
        B1102_W { w: self }
    }
    #[doc = "Bit 15 - B1103"]
    #[inline(always)]
    pub fn b1103(&mut self) -> B1103_W {
        B1103_W { w: self }
    }
    #[doc = "Bit 16 - B1104"]
    #[inline(always)]
    pub fn b1104(&mut self) -> B1104_W {
        B1104_W { w: self }
    }
    #[doc = "Bit 17 - B1105"]
    #[inline(always)]
    pub fn b1105(&mut self) -> B1105_W {
        B1105_W { w: self }
    }
    #[doc = "Bit 18 - B1106"]
    #[inline(always)]
    pub fn b1106(&mut self) -> B1106_W {
        B1106_W { w: self }
    }
    #[doc = "Bit 19 - B1107"]
    #[inline(always)]
    pub fn b1107(&mut self) -> B1107_W {
        B1107_W { w: self }
    }
    #[doc = "Bit 20 - B1108"]
    #[inline(always)]
    pub fn b1108(&mut self) -> B1108_W {
        B1108_W { w: self }
    }
    #[doc = "Bit 21 - B1109"]
    #[inline(always)]
    pub fn b1109(&mut self) -> B1109_W {
        B1109_W { w: self }
    }
    #[doc = "Bit 22 - B1110"]
    #[inline(always)]
    pub fn b1110(&mut self) -> B1110_W {
        B1110_W { w: self }
    }
    #[doc = "Bit 23 - B1111"]
    #[inline(always)]
    pub fn b1111(&mut self) -> B1111_W {
        B1111_W { w: self }
    }
    #[doc = "Bit 24 - B1112"]
    #[inline(always)]
    pub fn b1112(&mut self) -> B1112_W {
        B1112_W { w: self }
    }
    #[doc = "Bit 25 - B1113"]
    #[inline(always)]
    pub fn b1113(&mut self) -> B1113_W {
        B1113_W { w: self }
    }
    #[doc = "Bit 26 - B1114"]
    #[inline(always)]
    pub fn b1114(&mut self) -> B1114_W {
        B1114_W { w: self }
    }
    #[doc = "Bit 27 - B1115"]
    #[inline(always)]
    pub fn b1115(&mut self) -> B1115_W {
        B1115_W { w: self }
    }
    #[doc = "Bit 28 - B1116"]
    #[inline(always)]
    pub fn b1116(&mut self) -> B1116_W {
        B1116_W { w: self }
    }
    #[doc = "Bit 29 - B1117"]
    #[inline(always)]
    pub fn b1117(&mut self) -> B1117_W {
        B1117_W { w: self }
    }
    #[doc = "Bit 30 - B1118"]
    #[inline(always)]
    pub fn b1118(&mut self) -> B1118_W {
        B1118_W { w: self }
    }
    #[doc = "Bit 31 - B1119"]
    #[inline(always)]
    pub fn b1119(&mut self) -> B1119_W {
        B1119_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr34](index.html) module"]
pub struct MPCBB1_VCTR34_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr34::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr34::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR34_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR34 to value 0"]
impl crate::Resettable for MPCBB1_VCTR34_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
