#[doc = "Register `MPCBB2_VCTR35` reader"]
pub struct R(crate::R<MPCBB2_VCTR35_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR35_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR35_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR35_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR35` writer"]
pub struct W(crate::W<MPCBB2_VCTR35_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR35_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR35_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR35_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1120` reader - B1120"]
pub struct B1120_R(crate::FieldReader<bool, bool>);
impl B1120_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1120_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1120_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1120` writer - B1120"]
pub struct B1120_W<'a> {
    w: &'a mut W,
}
impl<'a> B1120_W<'a> {
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
#[doc = "Field `B1121` reader - B1121"]
pub struct B1121_R(crate::FieldReader<bool, bool>);
impl B1121_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1121_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1121_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1121` writer - B1121"]
pub struct B1121_W<'a> {
    w: &'a mut W,
}
impl<'a> B1121_W<'a> {
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
#[doc = "Field `B1122` reader - B1122"]
pub struct B1122_R(crate::FieldReader<bool, bool>);
impl B1122_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1122_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1122_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1122` writer - B1122"]
pub struct B1122_W<'a> {
    w: &'a mut W,
}
impl<'a> B1122_W<'a> {
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
#[doc = "Field `B1123` reader - B1123"]
pub struct B1123_R(crate::FieldReader<bool, bool>);
impl B1123_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1123_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1123_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1123` writer - B1123"]
pub struct B1123_W<'a> {
    w: &'a mut W,
}
impl<'a> B1123_W<'a> {
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
#[doc = "Field `B1124` reader - B1124"]
pub struct B1124_R(crate::FieldReader<bool, bool>);
impl B1124_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1124_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1124_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1124` writer - B1124"]
pub struct B1124_W<'a> {
    w: &'a mut W,
}
impl<'a> B1124_W<'a> {
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
#[doc = "Field `B1125` reader - B1125"]
pub struct B1125_R(crate::FieldReader<bool, bool>);
impl B1125_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1125_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1125_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1125` writer - B1125"]
pub struct B1125_W<'a> {
    w: &'a mut W,
}
impl<'a> B1125_W<'a> {
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
#[doc = "Field `B1126` reader - B1126"]
pub struct B1126_R(crate::FieldReader<bool, bool>);
impl B1126_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1126_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1126_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1126` writer - B1126"]
pub struct B1126_W<'a> {
    w: &'a mut W,
}
impl<'a> B1126_W<'a> {
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
#[doc = "Field `B1127` reader - B1127"]
pub struct B1127_R(crate::FieldReader<bool, bool>);
impl B1127_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1127_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1127_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1127` writer - B1127"]
pub struct B1127_W<'a> {
    w: &'a mut W,
}
impl<'a> B1127_W<'a> {
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
#[doc = "Field `B1128` reader - B1128"]
pub struct B1128_R(crate::FieldReader<bool, bool>);
impl B1128_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1128` writer - B1128"]
pub struct B1128_W<'a> {
    w: &'a mut W,
}
impl<'a> B1128_W<'a> {
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
#[doc = "Field `B1129` reader - B1129"]
pub struct B1129_R(crate::FieldReader<bool, bool>);
impl B1129_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1129_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1129_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1129` writer - B1129"]
pub struct B1129_W<'a> {
    w: &'a mut W,
}
impl<'a> B1129_W<'a> {
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
#[doc = "Field `B1130` reader - B1130"]
pub struct B1130_R(crate::FieldReader<bool, bool>);
impl B1130_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1130_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1130_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1130` writer - B1130"]
pub struct B1130_W<'a> {
    w: &'a mut W,
}
impl<'a> B1130_W<'a> {
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
#[doc = "Field `B1131` reader - B1131"]
pub struct B1131_R(crate::FieldReader<bool, bool>);
impl B1131_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1131_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1131_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1131` writer - B1131"]
pub struct B1131_W<'a> {
    w: &'a mut W,
}
impl<'a> B1131_W<'a> {
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
#[doc = "Field `B1132` reader - B1132"]
pub struct B1132_R(crate::FieldReader<bool, bool>);
impl B1132_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1132_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1132_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1132` writer - B1132"]
pub struct B1132_W<'a> {
    w: &'a mut W,
}
impl<'a> B1132_W<'a> {
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
#[doc = "Field `B1133` reader - B1133"]
pub struct B1133_R(crate::FieldReader<bool, bool>);
impl B1133_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1133_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1133_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1133` writer - B1133"]
pub struct B1133_W<'a> {
    w: &'a mut W,
}
impl<'a> B1133_W<'a> {
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
#[doc = "Field `B1134` reader - B1134"]
pub struct B1134_R(crate::FieldReader<bool, bool>);
impl B1134_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1134_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1134_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1134` writer - B1134"]
pub struct B1134_W<'a> {
    w: &'a mut W,
}
impl<'a> B1134_W<'a> {
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
#[doc = "Field `B1135` reader - B1135"]
pub struct B1135_R(crate::FieldReader<bool, bool>);
impl B1135_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1135_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1135_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1135` writer - B1135"]
pub struct B1135_W<'a> {
    w: &'a mut W,
}
impl<'a> B1135_W<'a> {
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
#[doc = "Field `B1136` reader - B1136"]
pub struct B1136_R(crate::FieldReader<bool, bool>);
impl B1136_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1136_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1136_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1136` writer - B1136"]
pub struct B1136_W<'a> {
    w: &'a mut W,
}
impl<'a> B1136_W<'a> {
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
#[doc = "Field `B1137` reader - B1137"]
pub struct B1137_R(crate::FieldReader<bool, bool>);
impl B1137_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1137_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1137_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1137` writer - B1137"]
pub struct B1137_W<'a> {
    w: &'a mut W,
}
impl<'a> B1137_W<'a> {
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
#[doc = "Field `B1138` reader - B1138"]
pub struct B1138_R(crate::FieldReader<bool, bool>);
impl B1138_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1138_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1138_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1138` writer - B1138"]
pub struct B1138_W<'a> {
    w: &'a mut W,
}
impl<'a> B1138_W<'a> {
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
#[doc = "Field `B1139` reader - B1139"]
pub struct B1139_R(crate::FieldReader<bool, bool>);
impl B1139_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1139_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1139_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1139` writer - B1139"]
pub struct B1139_W<'a> {
    w: &'a mut W,
}
impl<'a> B1139_W<'a> {
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
#[doc = "Field `B1140` reader - B1140"]
pub struct B1140_R(crate::FieldReader<bool, bool>);
impl B1140_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1140_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1140_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1140` writer - B1140"]
pub struct B1140_W<'a> {
    w: &'a mut W,
}
impl<'a> B1140_W<'a> {
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
#[doc = "Field `B1141` reader - B1141"]
pub struct B1141_R(crate::FieldReader<bool, bool>);
impl B1141_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1141_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1141_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1141` writer - B1141"]
pub struct B1141_W<'a> {
    w: &'a mut W,
}
impl<'a> B1141_W<'a> {
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
#[doc = "Field `B1142` reader - B1142"]
pub struct B1142_R(crate::FieldReader<bool, bool>);
impl B1142_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1142_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1142_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1142` writer - B1142"]
pub struct B1142_W<'a> {
    w: &'a mut W,
}
impl<'a> B1142_W<'a> {
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
#[doc = "Field `B1143` reader - B1143"]
pub struct B1143_R(crate::FieldReader<bool, bool>);
impl B1143_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1143_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1143_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1143` writer - B1143"]
pub struct B1143_W<'a> {
    w: &'a mut W,
}
impl<'a> B1143_W<'a> {
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
#[doc = "Field `B1144` reader - B1144"]
pub struct B1144_R(crate::FieldReader<bool, bool>);
impl B1144_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1144_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1144_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1144` writer - B1144"]
pub struct B1144_W<'a> {
    w: &'a mut W,
}
impl<'a> B1144_W<'a> {
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
#[doc = "Field `B1145` reader - B1145"]
pub struct B1145_R(crate::FieldReader<bool, bool>);
impl B1145_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1145_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1145_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1145` writer - B1145"]
pub struct B1145_W<'a> {
    w: &'a mut W,
}
impl<'a> B1145_W<'a> {
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
#[doc = "Field `B1146` reader - B1146"]
pub struct B1146_R(crate::FieldReader<bool, bool>);
impl B1146_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1146_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1146_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1146` writer - B1146"]
pub struct B1146_W<'a> {
    w: &'a mut W,
}
impl<'a> B1146_W<'a> {
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
#[doc = "Field `B1147` reader - B1147"]
pub struct B1147_R(crate::FieldReader<bool, bool>);
impl B1147_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1147_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1147_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1147` writer - B1147"]
pub struct B1147_W<'a> {
    w: &'a mut W,
}
impl<'a> B1147_W<'a> {
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
#[doc = "Field `B1148` reader - B1148"]
pub struct B1148_R(crate::FieldReader<bool, bool>);
impl B1148_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1148_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1148_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1148` writer - B1148"]
pub struct B1148_W<'a> {
    w: &'a mut W,
}
impl<'a> B1148_W<'a> {
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
#[doc = "Field `B1149` reader - B1149"]
pub struct B1149_R(crate::FieldReader<bool, bool>);
impl B1149_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1149_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1149_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1149` writer - B1149"]
pub struct B1149_W<'a> {
    w: &'a mut W,
}
impl<'a> B1149_W<'a> {
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
#[doc = "Field `B1150` reader - B1150"]
pub struct B1150_R(crate::FieldReader<bool, bool>);
impl B1150_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1150_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1150_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1150` writer - B1150"]
pub struct B1150_W<'a> {
    w: &'a mut W,
}
impl<'a> B1150_W<'a> {
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
#[doc = "Field `B1151` reader - B1151"]
pub struct B1151_R(crate::FieldReader<bool, bool>);
impl B1151_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1151_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1151_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1151` writer - B1151"]
pub struct B1151_W<'a> {
    w: &'a mut W,
}
impl<'a> B1151_W<'a> {
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
    #[doc = "Bit 0 - B1120"]
    #[inline(always)]
    pub fn b1120(&self) -> B1120_R {
        B1120_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1121"]
    #[inline(always)]
    pub fn b1121(&self) -> B1121_R {
        B1121_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1122"]
    #[inline(always)]
    pub fn b1122(&self) -> B1122_R {
        B1122_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1123"]
    #[inline(always)]
    pub fn b1123(&self) -> B1123_R {
        B1123_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1124"]
    #[inline(always)]
    pub fn b1124(&self) -> B1124_R {
        B1124_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1125"]
    #[inline(always)]
    pub fn b1125(&self) -> B1125_R {
        B1125_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1126"]
    #[inline(always)]
    pub fn b1126(&self) -> B1126_R {
        B1126_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1127"]
    #[inline(always)]
    pub fn b1127(&self) -> B1127_R {
        B1127_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1128"]
    #[inline(always)]
    pub fn b1128(&self) -> B1128_R {
        B1128_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1129"]
    #[inline(always)]
    pub fn b1129(&self) -> B1129_R {
        B1129_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1130"]
    #[inline(always)]
    pub fn b1130(&self) -> B1130_R {
        B1130_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1131"]
    #[inline(always)]
    pub fn b1131(&self) -> B1131_R {
        B1131_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1132"]
    #[inline(always)]
    pub fn b1132(&self) -> B1132_R {
        B1132_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1133"]
    #[inline(always)]
    pub fn b1133(&self) -> B1133_R {
        B1133_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1134"]
    #[inline(always)]
    pub fn b1134(&self) -> B1134_R {
        B1134_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1135"]
    #[inline(always)]
    pub fn b1135(&self) -> B1135_R {
        B1135_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1136"]
    #[inline(always)]
    pub fn b1136(&self) -> B1136_R {
        B1136_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1137"]
    #[inline(always)]
    pub fn b1137(&self) -> B1137_R {
        B1137_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1138"]
    #[inline(always)]
    pub fn b1138(&self) -> B1138_R {
        B1138_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1139"]
    #[inline(always)]
    pub fn b1139(&self) -> B1139_R {
        B1139_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1140"]
    #[inline(always)]
    pub fn b1140(&self) -> B1140_R {
        B1140_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1141"]
    #[inline(always)]
    pub fn b1141(&self) -> B1141_R {
        B1141_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1142"]
    #[inline(always)]
    pub fn b1142(&self) -> B1142_R {
        B1142_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1143"]
    #[inline(always)]
    pub fn b1143(&self) -> B1143_R {
        B1143_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1144"]
    #[inline(always)]
    pub fn b1144(&self) -> B1144_R {
        B1144_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1145"]
    #[inline(always)]
    pub fn b1145(&self) -> B1145_R {
        B1145_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1146"]
    #[inline(always)]
    pub fn b1146(&self) -> B1146_R {
        B1146_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1147"]
    #[inline(always)]
    pub fn b1147(&self) -> B1147_R {
        B1147_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1148"]
    #[inline(always)]
    pub fn b1148(&self) -> B1148_R {
        B1148_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1149"]
    #[inline(always)]
    pub fn b1149(&self) -> B1149_R {
        B1149_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1150"]
    #[inline(always)]
    pub fn b1150(&self) -> B1150_R {
        B1150_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1151"]
    #[inline(always)]
    pub fn b1151(&self) -> B1151_R {
        B1151_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1120"]
    #[inline(always)]
    pub fn b1120(&mut self) -> B1120_W {
        B1120_W { w: self }
    }
    #[doc = "Bit 1 - B1121"]
    #[inline(always)]
    pub fn b1121(&mut self) -> B1121_W {
        B1121_W { w: self }
    }
    #[doc = "Bit 2 - B1122"]
    #[inline(always)]
    pub fn b1122(&mut self) -> B1122_W {
        B1122_W { w: self }
    }
    #[doc = "Bit 3 - B1123"]
    #[inline(always)]
    pub fn b1123(&mut self) -> B1123_W {
        B1123_W { w: self }
    }
    #[doc = "Bit 4 - B1124"]
    #[inline(always)]
    pub fn b1124(&mut self) -> B1124_W {
        B1124_W { w: self }
    }
    #[doc = "Bit 5 - B1125"]
    #[inline(always)]
    pub fn b1125(&mut self) -> B1125_W {
        B1125_W { w: self }
    }
    #[doc = "Bit 6 - B1126"]
    #[inline(always)]
    pub fn b1126(&mut self) -> B1126_W {
        B1126_W { w: self }
    }
    #[doc = "Bit 7 - B1127"]
    #[inline(always)]
    pub fn b1127(&mut self) -> B1127_W {
        B1127_W { w: self }
    }
    #[doc = "Bit 8 - B1128"]
    #[inline(always)]
    pub fn b1128(&mut self) -> B1128_W {
        B1128_W { w: self }
    }
    #[doc = "Bit 9 - B1129"]
    #[inline(always)]
    pub fn b1129(&mut self) -> B1129_W {
        B1129_W { w: self }
    }
    #[doc = "Bit 10 - B1130"]
    #[inline(always)]
    pub fn b1130(&mut self) -> B1130_W {
        B1130_W { w: self }
    }
    #[doc = "Bit 11 - B1131"]
    #[inline(always)]
    pub fn b1131(&mut self) -> B1131_W {
        B1131_W { w: self }
    }
    #[doc = "Bit 12 - B1132"]
    #[inline(always)]
    pub fn b1132(&mut self) -> B1132_W {
        B1132_W { w: self }
    }
    #[doc = "Bit 13 - B1133"]
    #[inline(always)]
    pub fn b1133(&mut self) -> B1133_W {
        B1133_W { w: self }
    }
    #[doc = "Bit 14 - B1134"]
    #[inline(always)]
    pub fn b1134(&mut self) -> B1134_W {
        B1134_W { w: self }
    }
    #[doc = "Bit 15 - B1135"]
    #[inline(always)]
    pub fn b1135(&mut self) -> B1135_W {
        B1135_W { w: self }
    }
    #[doc = "Bit 16 - B1136"]
    #[inline(always)]
    pub fn b1136(&mut self) -> B1136_W {
        B1136_W { w: self }
    }
    #[doc = "Bit 17 - B1137"]
    #[inline(always)]
    pub fn b1137(&mut self) -> B1137_W {
        B1137_W { w: self }
    }
    #[doc = "Bit 18 - B1138"]
    #[inline(always)]
    pub fn b1138(&mut self) -> B1138_W {
        B1138_W { w: self }
    }
    #[doc = "Bit 19 - B1139"]
    #[inline(always)]
    pub fn b1139(&mut self) -> B1139_W {
        B1139_W { w: self }
    }
    #[doc = "Bit 20 - B1140"]
    #[inline(always)]
    pub fn b1140(&mut self) -> B1140_W {
        B1140_W { w: self }
    }
    #[doc = "Bit 21 - B1141"]
    #[inline(always)]
    pub fn b1141(&mut self) -> B1141_W {
        B1141_W { w: self }
    }
    #[doc = "Bit 22 - B1142"]
    #[inline(always)]
    pub fn b1142(&mut self) -> B1142_W {
        B1142_W { w: self }
    }
    #[doc = "Bit 23 - B1143"]
    #[inline(always)]
    pub fn b1143(&mut self) -> B1143_W {
        B1143_W { w: self }
    }
    #[doc = "Bit 24 - B1144"]
    #[inline(always)]
    pub fn b1144(&mut self) -> B1144_W {
        B1144_W { w: self }
    }
    #[doc = "Bit 25 - B1145"]
    #[inline(always)]
    pub fn b1145(&mut self) -> B1145_W {
        B1145_W { w: self }
    }
    #[doc = "Bit 26 - B1146"]
    #[inline(always)]
    pub fn b1146(&mut self) -> B1146_W {
        B1146_W { w: self }
    }
    #[doc = "Bit 27 - B1147"]
    #[inline(always)]
    pub fn b1147(&mut self) -> B1147_W {
        B1147_W { w: self }
    }
    #[doc = "Bit 28 - B1148"]
    #[inline(always)]
    pub fn b1148(&mut self) -> B1148_W {
        B1148_W { w: self }
    }
    #[doc = "Bit 29 - B1149"]
    #[inline(always)]
    pub fn b1149(&mut self) -> B1149_W {
        B1149_W { w: self }
    }
    #[doc = "Bit 30 - B1150"]
    #[inline(always)]
    pub fn b1150(&mut self) -> B1150_W {
        B1150_W { w: self }
    }
    #[doc = "Bit 31 - B1151"]
    #[inline(always)]
    pub fn b1151(&mut self) -> B1151_W {
        B1151_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr35](index.html) module"]
pub struct MPCBB2_VCTR35_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR35_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr35::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR35_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr35::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR35_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR35 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR35_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
