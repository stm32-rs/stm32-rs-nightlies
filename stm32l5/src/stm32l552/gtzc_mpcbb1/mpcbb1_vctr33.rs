#[doc = "Register `MPCBB1_VCTR33` reader"]
pub struct R(crate::R<MPCBB1_VCTR33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR33_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR33` writer"]
pub struct W(crate::W<MPCBB1_VCTR33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR33_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR33_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1056` reader - B1056"]
pub struct B1056_R(crate::FieldReader<bool, bool>);
impl B1056_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1056_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1056_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1056` writer - B1056"]
pub struct B1056_W<'a> {
    w: &'a mut W,
}
impl<'a> B1056_W<'a> {
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
#[doc = "Field `B1057` reader - B1057"]
pub struct B1057_R(crate::FieldReader<bool, bool>);
impl B1057_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1057_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1057_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1057` writer - B1057"]
pub struct B1057_W<'a> {
    w: &'a mut W,
}
impl<'a> B1057_W<'a> {
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
#[doc = "Field `B1058` reader - B1058"]
pub struct B1058_R(crate::FieldReader<bool, bool>);
impl B1058_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1058_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1058_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1058` writer - B1058"]
pub struct B1058_W<'a> {
    w: &'a mut W,
}
impl<'a> B1058_W<'a> {
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
#[doc = "Field `B1059` reader - B1059"]
pub struct B1059_R(crate::FieldReader<bool, bool>);
impl B1059_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1059_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1059_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1059` writer - B1059"]
pub struct B1059_W<'a> {
    w: &'a mut W,
}
impl<'a> B1059_W<'a> {
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
#[doc = "Field `B1060` reader - B1060"]
pub struct B1060_R(crate::FieldReader<bool, bool>);
impl B1060_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1060_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1060_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1060` writer - B1060"]
pub struct B1060_W<'a> {
    w: &'a mut W,
}
impl<'a> B1060_W<'a> {
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
#[doc = "Field `B1061` reader - B1061"]
pub struct B1061_R(crate::FieldReader<bool, bool>);
impl B1061_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1061_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1061_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1061` writer - B1061"]
pub struct B1061_W<'a> {
    w: &'a mut W,
}
impl<'a> B1061_W<'a> {
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
#[doc = "Field `B1062` reader - B1062"]
pub struct B1062_R(crate::FieldReader<bool, bool>);
impl B1062_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1062_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1062_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1062` writer - B1062"]
pub struct B1062_W<'a> {
    w: &'a mut W,
}
impl<'a> B1062_W<'a> {
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
#[doc = "Field `B1063` reader - B1063"]
pub struct B1063_R(crate::FieldReader<bool, bool>);
impl B1063_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1063_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1063_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1063` writer - B1063"]
pub struct B1063_W<'a> {
    w: &'a mut W,
}
impl<'a> B1063_W<'a> {
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
#[doc = "Field `B1064` reader - B1064"]
pub struct B1064_R(crate::FieldReader<bool, bool>);
impl B1064_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1064_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1064_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1064` writer - B1064"]
pub struct B1064_W<'a> {
    w: &'a mut W,
}
impl<'a> B1064_W<'a> {
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
#[doc = "Field `B1065` reader - B1065"]
pub struct B1065_R(crate::FieldReader<bool, bool>);
impl B1065_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1065_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1065_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1065` writer - B1065"]
pub struct B1065_W<'a> {
    w: &'a mut W,
}
impl<'a> B1065_W<'a> {
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
#[doc = "Field `B1066` reader - B1066"]
pub struct B1066_R(crate::FieldReader<bool, bool>);
impl B1066_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1066_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1066_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1066` writer - B1066"]
pub struct B1066_W<'a> {
    w: &'a mut W,
}
impl<'a> B1066_W<'a> {
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
#[doc = "Field `B1067` reader - B1067"]
pub struct B1067_R(crate::FieldReader<bool, bool>);
impl B1067_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1067_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1067_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1067` writer - B1067"]
pub struct B1067_W<'a> {
    w: &'a mut W,
}
impl<'a> B1067_W<'a> {
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
#[doc = "Field `B1068` reader - B1068"]
pub struct B1068_R(crate::FieldReader<bool, bool>);
impl B1068_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1068_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1068_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1068` writer - B1068"]
pub struct B1068_W<'a> {
    w: &'a mut W,
}
impl<'a> B1068_W<'a> {
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
#[doc = "Field `B1069` reader - B1069"]
pub struct B1069_R(crate::FieldReader<bool, bool>);
impl B1069_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1069_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1069_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1069` writer - B1069"]
pub struct B1069_W<'a> {
    w: &'a mut W,
}
impl<'a> B1069_W<'a> {
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
#[doc = "Field `B1070` reader - B1070"]
pub struct B1070_R(crate::FieldReader<bool, bool>);
impl B1070_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1070_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1070_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1070` writer - B1070"]
pub struct B1070_W<'a> {
    w: &'a mut W,
}
impl<'a> B1070_W<'a> {
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
#[doc = "Field `B1071` reader - B1071"]
pub struct B1071_R(crate::FieldReader<bool, bool>);
impl B1071_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1071_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1071_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1071` writer - B1071"]
pub struct B1071_W<'a> {
    w: &'a mut W,
}
impl<'a> B1071_W<'a> {
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
#[doc = "Field `B1072` reader - B1072"]
pub struct B1072_R(crate::FieldReader<bool, bool>);
impl B1072_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1072_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1072_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1072` writer - B1072"]
pub struct B1072_W<'a> {
    w: &'a mut W,
}
impl<'a> B1072_W<'a> {
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
#[doc = "Field `B1073` reader - B1073"]
pub struct B1073_R(crate::FieldReader<bool, bool>);
impl B1073_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1073_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1073_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1073` writer - B1073"]
pub struct B1073_W<'a> {
    w: &'a mut W,
}
impl<'a> B1073_W<'a> {
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
#[doc = "Field `B1074` reader - B1074"]
pub struct B1074_R(crate::FieldReader<bool, bool>);
impl B1074_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1074_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1074_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1074` writer - B1074"]
pub struct B1074_W<'a> {
    w: &'a mut W,
}
impl<'a> B1074_W<'a> {
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
#[doc = "Field `B1075` reader - B1075"]
pub struct B1075_R(crate::FieldReader<bool, bool>);
impl B1075_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1075_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1075_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1075` writer - B1075"]
pub struct B1075_W<'a> {
    w: &'a mut W,
}
impl<'a> B1075_W<'a> {
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
#[doc = "Field `B1076` reader - B1076"]
pub struct B1076_R(crate::FieldReader<bool, bool>);
impl B1076_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1076_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1076_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1076` writer - B1076"]
pub struct B1076_W<'a> {
    w: &'a mut W,
}
impl<'a> B1076_W<'a> {
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
#[doc = "Field `B1077` reader - B1077"]
pub struct B1077_R(crate::FieldReader<bool, bool>);
impl B1077_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1077_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1077_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1077` writer - B1077"]
pub struct B1077_W<'a> {
    w: &'a mut W,
}
impl<'a> B1077_W<'a> {
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
#[doc = "Field `B1078` reader - B1078"]
pub struct B1078_R(crate::FieldReader<bool, bool>);
impl B1078_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1078_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1078_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1078` writer - B1078"]
pub struct B1078_W<'a> {
    w: &'a mut W,
}
impl<'a> B1078_W<'a> {
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
#[doc = "Field `B1079` reader - B1079"]
pub struct B1079_R(crate::FieldReader<bool, bool>);
impl B1079_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1079_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1079_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1079` writer - B1079"]
pub struct B1079_W<'a> {
    w: &'a mut W,
}
impl<'a> B1079_W<'a> {
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
#[doc = "Field `B1080` reader - B1080"]
pub struct B1080_R(crate::FieldReader<bool, bool>);
impl B1080_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1080_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1080_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1080` writer - B1080"]
pub struct B1080_W<'a> {
    w: &'a mut W,
}
impl<'a> B1080_W<'a> {
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
#[doc = "Field `B1081` reader - B1081"]
pub struct B1081_R(crate::FieldReader<bool, bool>);
impl B1081_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1081_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1081_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1081` writer - B1081"]
pub struct B1081_W<'a> {
    w: &'a mut W,
}
impl<'a> B1081_W<'a> {
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
#[doc = "Field `B1082` reader - B1082"]
pub struct B1082_R(crate::FieldReader<bool, bool>);
impl B1082_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1082_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1082_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1082` writer - B1082"]
pub struct B1082_W<'a> {
    w: &'a mut W,
}
impl<'a> B1082_W<'a> {
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
#[doc = "Field `B1083` reader - B1083"]
pub struct B1083_R(crate::FieldReader<bool, bool>);
impl B1083_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1083_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1083_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1083` writer - B1083"]
pub struct B1083_W<'a> {
    w: &'a mut W,
}
impl<'a> B1083_W<'a> {
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
#[doc = "Field `B1084` reader - B1084"]
pub struct B1084_R(crate::FieldReader<bool, bool>);
impl B1084_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1084_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1084_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1084` writer - B1084"]
pub struct B1084_W<'a> {
    w: &'a mut W,
}
impl<'a> B1084_W<'a> {
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
#[doc = "Field `B1085` reader - B1085"]
pub struct B1085_R(crate::FieldReader<bool, bool>);
impl B1085_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1085_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1085_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1085` writer - B1085"]
pub struct B1085_W<'a> {
    w: &'a mut W,
}
impl<'a> B1085_W<'a> {
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
#[doc = "Field `B1086` reader - B1086"]
pub struct B1086_R(crate::FieldReader<bool, bool>);
impl B1086_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1086_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1086_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1086` writer - B1086"]
pub struct B1086_W<'a> {
    w: &'a mut W,
}
impl<'a> B1086_W<'a> {
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
#[doc = "Field `B1087` reader - B1087"]
pub struct B1087_R(crate::FieldReader<bool, bool>);
impl B1087_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1087_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1087_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1087` writer - B1087"]
pub struct B1087_W<'a> {
    w: &'a mut W,
}
impl<'a> B1087_W<'a> {
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
    #[doc = "Bit 0 - B1056"]
    #[inline(always)]
    pub fn b1056(&self) -> B1056_R {
        B1056_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1057"]
    #[inline(always)]
    pub fn b1057(&self) -> B1057_R {
        B1057_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1058"]
    #[inline(always)]
    pub fn b1058(&self) -> B1058_R {
        B1058_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1059"]
    #[inline(always)]
    pub fn b1059(&self) -> B1059_R {
        B1059_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1060"]
    #[inline(always)]
    pub fn b1060(&self) -> B1060_R {
        B1060_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1061"]
    #[inline(always)]
    pub fn b1061(&self) -> B1061_R {
        B1061_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1062"]
    #[inline(always)]
    pub fn b1062(&self) -> B1062_R {
        B1062_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1063"]
    #[inline(always)]
    pub fn b1063(&self) -> B1063_R {
        B1063_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1064"]
    #[inline(always)]
    pub fn b1064(&self) -> B1064_R {
        B1064_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1065"]
    #[inline(always)]
    pub fn b1065(&self) -> B1065_R {
        B1065_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1066"]
    #[inline(always)]
    pub fn b1066(&self) -> B1066_R {
        B1066_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1067"]
    #[inline(always)]
    pub fn b1067(&self) -> B1067_R {
        B1067_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1068"]
    #[inline(always)]
    pub fn b1068(&self) -> B1068_R {
        B1068_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1069"]
    #[inline(always)]
    pub fn b1069(&self) -> B1069_R {
        B1069_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1070"]
    #[inline(always)]
    pub fn b1070(&self) -> B1070_R {
        B1070_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1071"]
    #[inline(always)]
    pub fn b1071(&self) -> B1071_R {
        B1071_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1072"]
    #[inline(always)]
    pub fn b1072(&self) -> B1072_R {
        B1072_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1073"]
    #[inline(always)]
    pub fn b1073(&self) -> B1073_R {
        B1073_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1074"]
    #[inline(always)]
    pub fn b1074(&self) -> B1074_R {
        B1074_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1075"]
    #[inline(always)]
    pub fn b1075(&self) -> B1075_R {
        B1075_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1076"]
    #[inline(always)]
    pub fn b1076(&self) -> B1076_R {
        B1076_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1077"]
    #[inline(always)]
    pub fn b1077(&self) -> B1077_R {
        B1077_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1078"]
    #[inline(always)]
    pub fn b1078(&self) -> B1078_R {
        B1078_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1079"]
    #[inline(always)]
    pub fn b1079(&self) -> B1079_R {
        B1079_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1080"]
    #[inline(always)]
    pub fn b1080(&self) -> B1080_R {
        B1080_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1081"]
    #[inline(always)]
    pub fn b1081(&self) -> B1081_R {
        B1081_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1082"]
    #[inline(always)]
    pub fn b1082(&self) -> B1082_R {
        B1082_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1083"]
    #[inline(always)]
    pub fn b1083(&self) -> B1083_R {
        B1083_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1084"]
    #[inline(always)]
    pub fn b1084(&self) -> B1084_R {
        B1084_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1085"]
    #[inline(always)]
    pub fn b1085(&self) -> B1085_R {
        B1085_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1086"]
    #[inline(always)]
    pub fn b1086(&self) -> B1086_R {
        B1086_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1087"]
    #[inline(always)]
    pub fn b1087(&self) -> B1087_R {
        B1087_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1056"]
    #[inline(always)]
    pub fn b1056(&mut self) -> B1056_W {
        B1056_W { w: self }
    }
    #[doc = "Bit 1 - B1057"]
    #[inline(always)]
    pub fn b1057(&mut self) -> B1057_W {
        B1057_W { w: self }
    }
    #[doc = "Bit 2 - B1058"]
    #[inline(always)]
    pub fn b1058(&mut self) -> B1058_W {
        B1058_W { w: self }
    }
    #[doc = "Bit 3 - B1059"]
    #[inline(always)]
    pub fn b1059(&mut self) -> B1059_W {
        B1059_W { w: self }
    }
    #[doc = "Bit 4 - B1060"]
    #[inline(always)]
    pub fn b1060(&mut self) -> B1060_W {
        B1060_W { w: self }
    }
    #[doc = "Bit 5 - B1061"]
    #[inline(always)]
    pub fn b1061(&mut self) -> B1061_W {
        B1061_W { w: self }
    }
    #[doc = "Bit 6 - B1062"]
    #[inline(always)]
    pub fn b1062(&mut self) -> B1062_W {
        B1062_W { w: self }
    }
    #[doc = "Bit 7 - B1063"]
    #[inline(always)]
    pub fn b1063(&mut self) -> B1063_W {
        B1063_W { w: self }
    }
    #[doc = "Bit 8 - B1064"]
    #[inline(always)]
    pub fn b1064(&mut self) -> B1064_W {
        B1064_W { w: self }
    }
    #[doc = "Bit 9 - B1065"]
    #[inline(always)]
    pub fn b1065(&mut self) -> B1065_W {
        B1065_W { w: self }
    }
    #[doc = "Bit 10 - B1066"]
    #[inline(always)]
    pub fn b1066(&mut self) -> B1066_W {
        B1066_W { w: self }
    }
    #[doc = "Bit 11 - B1067"]
    #[inline(always)]
    pub fn b1067(&mut self) -> B1067_W {
        B1067_W { w: self }
    }
    #[doc = "Bit 12 - B1068"]
    #[inline(always)]
    pub fn b1068(&mut self) -> B1068_W {
        B1068_W { w: self }
    }
    #[doc = "Bit 13 - B1069"]
    #[inline(always)]
    pub fn b1069(&mut self) -> B1069_W {
        B1069_W { w: self }
    }
    #[doc = "Bit 14 - B1070"]
    #[inline(always)]
    pub fn b1070(&mut self) -> B1070_W {
        B1070_W { w: self }
    }
    #[doc = "Bit 15 - B1071"]
    #[inline(always)]
    pub fn b1071(&mut self) -> B1071_W {
        B1071_W { w: self }
    }
    #[doc = "Bit 16 - B1072"]
    #[inline(always)]
    pub fn b1072(&mut self) -> B1072_W {
        B1072_W { w: self }
    }
    #[doc = "Bit 17 - B1073"]
    #[inline(always)]
    pub fn b1073(&mut self) -> B1073_W {
        B1073_W { w: self }
    }
    #[doc = "Bit 18 - B1074"]
    #[inline(always)]
    pub fn b1074(&mut self) -> B1074_W {
        B1074_W { w: self }
    }
    #[doc = "Bit 19 - B1075"]
    #[inline(always)]
    pub fn b1075(&mut self) -> B1075_W {
        B1075_W { w: self }
    }
    #[doc = "Bit 20 - B1076"]
    #[inline(always)]
    pub fn b1076(&mut self) -> B1076_W {
        B1076_W { w: self }
    }
    #[doc = "Bit 21 - B1077"]
    #[inline(always)]
    pub fn b1077(&mut self) -> B1077_W {
        B1077_W { w: self }
    }
    #[doc = "Bit 22 - B1078"]
    #[inline(always)]
    pub fn b1078(&mut self) -> B1078_W {
        B1078_W { w: self }
    }
    #[doc = "Bit 23 - B1079"]
    #[inline(always)]
    pub fn b1079(&mut self) -> B1079_W {
        B1079_W { w: self }
    }
    #[doc = "Bit 24 - B1080"]
    #[inline(always)]
    pub fn b1080(&mut self) -> B1080_W {
        B1080_W { w: self }
    }
    #[doc = "Bit 25 - B1081"]
    #[inline(always)]
    pub fn b1081(&mut self) -> B1081_W {
        B1081_W { w: self }
    }
    #[doc = "Bit 26 - B1082"]
    #[inline(always)]
    pub fn b1082(&mut self) -> B1082_W {
        B1082_W { w: self }
    }
    #[doc = "Bit 27 - B1083"]
    #[inline(always)]
    pub fn b1083(&mut self) -> B1083_W {
        B1083_W { w: self }
    }
    #[doc = "Bit 28 - B1084"]
    #[inline(always)]
    pub fn b1084(&mut self) -> B1084_W {
        B1084_W { w: self }
    }
    #[doc = "Bit 29 - B1085"]
    #[inline(always)]
    pub fn b1085(&mut self) -> B1085_W {
        B1085_W { w: self }
    }
    #[doc = "Bit 30 - B1086"]
    #[inline(always)]
    pub fn b1086(&mut self) -> B1086_W {
        B1086_W { w: self }
    }
    #[doc = "Bit 31 - B1087"]
    #[inline(always)]
    pub fn b1087(&mut self) -> B1087_W {
        B1087_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr33](index.html) module"]
pub struct MPCBB1_VCTR33_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr33::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR33_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr33::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR33_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR33 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR33_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
