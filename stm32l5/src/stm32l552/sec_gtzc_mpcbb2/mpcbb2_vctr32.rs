#[doc = "Register `MPCBB2_VCTR32` reader"]
pub struct R(crate::R<MPCBB2_VCTR32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR32` writer"]
pub struct W(crate::W<MPCBB2_VCTR32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR32_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1024` reader - B1024"]
pub struct B1024_R(crate::FieldReader<bool, bool>);
impl B1024_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1024_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1024_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1024` writer - B1024"]
pub struct B1024_W<'a> {
    w: &'a mut W,
}
impl<'a> B1024_W<'a> {
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
#[doc = "Field `B1025` reader - B1025"]
pub struct B1025_R(crate::FieldReader<bool, bool>);
impl B1025_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1025_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1025_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1025` writer - B1025"]
pub struct B1025_W<'a> {
    w: &'a mut W,
}
impl<'a> B1025_W<'a> {
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
#[doc = "Field `B1026` reader - B1026"]
pub struct B1026_R(crate::FieldReader<bool, bool>);
impl B1026_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1026_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1026_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1026` writer - B1026"]
pub struct B1026_W<'a> {
    w: &'a mut W,
}
impl<'a> B1026_W<'a> {
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
#[doc = "Field `B1027` reader - B1027"]
pub struct B1027_R(crate::FieldReader<bool, bool>);
impl B1027_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1027_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1027_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1027` writer - B1027"]
pub struct B1027_W<'a> {
    w: &'a mut W,
}
impl<'a> B1027_W<'a> {
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
#[doc = "Field `B1028` reader - B1028"]
pub struct B1028_R(crate::FieldReader<bool, bool>);
impl B1028_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1028_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1028_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1028` writer - B1028"]
pub struct B1028_W<'a> {
    w: &'a mut W,
}
impl<'a> B1028_W<'a> {
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
#[doc = "Field `B1029` reader - B1029"]
pub struct B1029_R(crate::FieldReader<bool, bool>);
impl B1029_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1029_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1029_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1029` writer - B1029"]
pub struct B1029_W<'a> {
    w: &'a mut W,
}
impl<'a> B1029_W<'a> {
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
#[doc = "Field `B1030` reader - B1030"]
pub struct B1030_R(crate::FieldReader<bool, bool>);
impl B1030_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1030_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1030_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1030` writer - B1030"]
pub struct B1030_W<'a> {
    w: &'a mut W,
}
impl<'a> B1030_W<'a> {
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
#[doc = "Field `B1031` reader - B1031"]
pub struct B1031_R(crate::FieldReader<bool, bool>);
impl B1031_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1031_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1031_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1031` writer - B1031"]
pub struct B1031_W<'a> {
    w: &'a mut W,
}
impl<'a> B1031_W<'a> {
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
#[doc = "Field `B1032` reader - B1032"]
pub struct B1032_R(crate::FieldReader<bool, bool>);
impl B1032_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1032_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1032_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1032` writer - B1032"]
pub struct B1032_W<'a> {
    w: &'a mut W,
}
impl<'a> B1032_W<'a> {
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
#[doc = "Field `B1033` reader - B1033"]
pub struct B1033_R(crate::FieldReader<bool, bool>);
impl B1033_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1033_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1033_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1033` writer - B1033"]
pub struct B1033_W<'a> {
    w: &'a mut W,
}
impl<'a> B1033_W<'a> {
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
#[doc = "Field `B1034` reader - B1034"]
pub struct B1034_R(crate::FieldReader<bool, bool>);
impl B1034_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1034_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1034_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1034` writer - B1034"]
pub struct B1034_W<'a> {
    w: &'a mut W,
}
impl<'a> B1034_W<'a> {
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
#[doc = "Field `B1035` reader - B1035"]
pub struct B1035_R(crate::FieldReader<bool, bool>);
impl B1035_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1035_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1035_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1035` writer - B1035"]
pub struct B1035_W<'a> {
    w: &'a mut W,
}
impl<'a> B1035_W<'a> {
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
#[doc = "Field `B1036` reader - B1036"]
pub struct B1036_R(crate::FieldReader<bool, bool>);
impl B1036_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1036_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1036_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1036` writer - B1036"]
pub struct B1036_W<'a> {
    w: &'a mut W,
}
impl<'a> B1036_W<'a> {
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
#[doc = "Field `B1037` reader - B1037"]
pub struct B1037_R(crate::FieldReader<bool, bool>);
impl B1037_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1037_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1037_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1037` writer - B1037"]
pub struct B1037_W<'a> {
    w: &'a mut W,
}
impl<'a> B1037_W<'a> {
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
#[doc = "Field `B1038` reader - B1038"]
pub struct B1038_R(crate::FieldReader<bool, bool>);
impl B1038_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1038_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1038_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1038` writer - B1038"]
pub struct B1038_W<'a> {
    w: &'a mut W,
}
impl<'a> B1038_W<'a> {
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
#[doc = "Field `B1039` reader - B1039"]
pub struct B1039_R(crate::FieldReader<bool, bool>);
impl B1039_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1039_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1039_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1039` writer - B1039"]
pub struct B1039_W<'a> {
    w: &'a mut W,
}
impl<'a> B1039_W<'a> {
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
#[doc = "Field `B1040` reader - B1040"]
pub struct B1040_R(crate::FieldReader<bool, bool>);
impl B1040_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1040_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1040_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1040` writer - B1040"]
pub struct B1040_W<'a> {
    w: &'a mut W,
}
impl<'a> B1040_W<'a> {
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
#[doc = "Field `B1041` reader - B1041"]
pub struct B1041_R(crate::FieldReader<bool, bool>);
impl B1041_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1041_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1041_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1041` writer - B1041"]
pub struct B1041_W<'a> {
    w: &'a mut W,
}
impl<'a> B1041_W<'a> {
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
#[doc = "Field `B1042` reader - B1042"]
pub struct B1042_R(crate::FieldReader<bool, bool>);
impl B1042_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1042_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1042_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1042` writer - B1042"]
pub struct B1042_W<'a> {
    w: &'a mut W,
}
impl<'a> B1042_W<'a> {
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
#[doc = "Field `B1043` reader - B1043"]
pub struct B1043_R(crate::FieldReader<bool, bool>);
impl B1043_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1043_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1043_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1043` writer - B1043"]
pub struct B1043_W<'a> {
    w: &'a mut W,
}
impl<'a> B1043_W<'a> {
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
#[doc = "Field `B1044` reader - B1044"]
pub struct B1044_R(crate::FieldReader<bool, bool>);
impl B1044_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1044_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1044_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1044` writer - B1044"]
pub struct B1044_W<'a> {
    w: &'a mut W,
}
impl<'a> B1044_W<'a> {
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
#[doc = "Field `B1045` reader - B1045"]
pub struct B1045_R(crate::FieldReader<bool, bool>);
impl B1045_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1045_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1045_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1045` writer - B1045"]
pub struct B1045_W<'a> {
    w: &'a mut W,
}
impl<'a> B1045_W<'a> {
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
#[doc = "Field `B1046` reader - B1046"]
pub struct B1046_R(crate::FieldReader<bool, bool>);
impl B1046_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1046_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1046_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1046` writer - B1046"]
pub struct B1046_W<'a> {
    w: &'a mut W,
}
impl<'a> B1046_W<'a> {
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
#[doc = "Field `B1047` reader - B1047"]
pub struct B1047_R(crate::FieldReader<bool, bool>);
impl B1047_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1047_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1047_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1047` writer - B1047"]
pub struct B1047_W<'a> {
    w: &'a mut W,
}
impl<'a> B1047_W<'a> {
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
#[doc = "Field `B1048` reader - B1048"]
pub struct B1048_R(crate::FieldReader<bool, bool>);
impl B1048_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1048_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1048_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1048` writer - B1048"]
pub struct B1048_W<'a> {
    w: &'a mut W,
}
impl<'a> B1048_W<'a> {
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
#[doc = "Field `B1049` reader - B1049"]
pub struct B1049_R(crate::FieldReader<bool, bool>);
impl B1049_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1049_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1049_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1049` writer - B1049"]
pub struct B1049_W<'a> {
    w: &'a mut W,
}
impl<'a> B1049_W<'a> {
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
#[doc = "Field `B1050` reader - B1050"]
pub struct B1050_R(crate::FieldReader<bool, bool>);
impl B1050_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1050_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1050_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1050` writer - B1050"]
pub struct B1050_W<'a> {
    w: &'a mut W,
}
impl<'a> B1050_W<'a> {
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
#[doc = "Field `B1051` reader - B1051"]
pub struct B1051_R(crate::FieldReader<bool, bool>);
impl B1051_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1051_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1051_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1051` writer - B1051"]
pub struct B1051_W<'a> {
    w: &'a mut W,
}
impl<'a> B1051_W<'a> {
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
#[doc = "Field `B1052` reader - B1052"]
pub struct B1052_R(crate::FieldReader<bool, bool>);
impl B1052_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1052_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1052_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1052` writer - B1052"]
pub struct B1052_W<'a> {
    w: &'a mut W,
}
impl<'a> B1052_W<'a> {
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
#[doc = "Field `B1053` reader - B1053"]
pub struct B1053_R(crate::FieldReader<bool, bool>);
impl B1053_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1053_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1053_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1053` writer - B1053"]
pub struct B1053_W<'a> {
    w: &'a mut W,
}
impl<'a> B1053_W<'a> {
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
#[doc = "Field `B1054` reader - B1054"]
pub struct B1054_R(crate::FieldReader<bool, bool>);
impl B1054_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1054_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1054_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1054` writer - B1054"]
pub struct B1054_W<'a> {
    w: &'a mut W,
}
impl<'a> B1054_W<'a> {
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
#[doc = "Field `B1055` reader - B1055"]
pub struct B1055_R(crate::FieldReader<bool, bool>);
impl B1055_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1055_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1055_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1055` writer - B1055"]
pub struct B1055_W<'a> {
    w: &'a mut W,
}
impl<'a> B1055_W<'a> {
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
    #[doc = "Bit 0 - B1024"]
    #[inline(always)]
    pub fn b1024(&self) -> B1024_R {
        B1024_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1025"]
    #[inline(always)]
    pub fn b1025(&self) -> B1025_R {
        B1025_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1026"]
    #[inline(always)]
    pub fn b1026(&self) -> B1026_R {
        B1026_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1027"]
    #[inline(always)]
    pub fn b1027(&self) -> B1027_R {
        B1027_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1028"]
    #[inline(always)]
    pub fn b1028(&self) -> B1028_R {
        B1028_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1029"]
    #[inline(always)]
    pub fn b1029(&self) -> B1029_R {
        B1029_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1030"]
    #[inline(always)]
    pub fn b1030(&self) -> B1030_R {
        B1030_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1031"]
    #[inline(always)]
    pub fn b1031(&self) -> B1031_R {
        B1031_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1032"]
    #[inline(always)]
    pub fn b1032(&self) -> B1032_R {
        B1032_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1033"]
    #[inline(always)]
    pub fn b1033(&self) -> B1033_R {
        B1033_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1034"]
    #[inline(always)]
    pub fn b1034(&self) -> B1034_R {
        B1034_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1035"]
    #[inline(always)]
    pub fn b1035(&self) -> B1035_R {
        B1035_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1036"]
    #[inline(always)]
    pub fn b1036(&self) -> B1036_R {
        B1036_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1037"]
    #[inline(always)]
    pub fn b1037(&self) -> B1037_R {
        B1037_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1038"]
    #[inline(always)]
    pub fn b1038(&self) -> B1038_R {
        B1038_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1039"]
    #[inline(always)]
    pub fn b1039(&self) -> B1039_R {
        B1039_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1040"]
    #[inline(always)]
    pub fn b1040(&self) -> B1040_R {
        B1040_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1041"]
    #[inline(always)]
    pub fn b1041(&self) -> B1041_R {
        B1041_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1042"]
    #[inline(always)]
    pub fn b1042(&self) -> B1042_R {
        B1042_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1043"]
    #[inline(always)]
    pub fn b1043(&self) -> B1043_R {
        B1043_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1044"]
    #[inline(always)]
    pub fn b1044(&self) -> B1044_R {
        B1044_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1045"]
    #[inline(always)]
    pub fn b1045(&self) -> B1045_R {
        B1045_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1046"]
    #[inline(always)]
    pub fn b1046(&self) -> B1046_R {
        B1046_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1047"]
    #[inline(always)]
    pub fn b1047(&self) -> B1047_R {
        B1047_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1048"]
    #[inline(always)]
    pub fn b1048(&self) -> B1048_R {
        B1048_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1049"]
    #[inline(always)]
    pub fn b1049(&self) -> B1049_R {
        B1049_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1050"]
    #[inline(always)]
    pub fn b1050(&self) -> B1050_R {
        B1050_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1051"]
    #[inline(always)]
    pub fn b1051(&self) -> B1051_R {
        B1051_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1052"]
    #[inline(always)]
    pub fn b1052(&self) -> B1052_R {
        B1052_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1053"]
    #[inline(always)]
    pub fn b1053(&self) -> B1053_R {
        B1053_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1054"]
    #[inline(always)]
    pub fn b1054(&self) -> B1054_R {
        B1054_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1055"]
    #[inline(always)]
    pub fn b1055(&self) -> B1055_R {
        B1055_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1024"]
    #[inline(always)]
    pub fn b1024(&mut self) -> B1024_W {
        B1024_W { w: self }
    }
    #[doc = "Bit 1 - B1025"]
    #[inline(always)]
    pub fn b1025(&mut self) -> B1025_W {
        B1025_W { w: self }
    }
    #[doc = "Bit 2 - B1026"]
    #[inline(always)]
    pub fn b1026(&mut self) -> B1026_W {
        B1026_W { w: self }
    }
    #[doc = "Bit 3 - B1027"]
    #[inline(always)]
    pub fn b1027(&mut self) -> B1027_W {
        B1027_W { w: self }
    }
    #[doc = "Bit 4 - B1028"]
    #[inline(always)]
    pub fn b1028(&mut self) -> B1028_W {
        B1028_W { w: self }
    }
    #[doc = "Bit 5 - B1029"]
    #[inline(always)]
    pub fn b1029(&mut self) -> B1029_W {
        B1029_W { w: self }
    }
    #[doc = "Bit 6 - B1030"]
    #[inline(always)]
    pub fn b1030(&mut self) -> B1030_W {
        B1030_W { w: self }
    }
    #[doc = "Bit 7 - B1031"]
    #[inline(always)]
    pub fn b1031(&mut self) -> B1031_W {
        B1031_W { w: self }
    }
    #[doc = "Bit 8 - B1032"]
    #[inline(always)]
    pub fn b1032(&mut self) -> B1032_W {
        B1032_W { w: self }
    }
    #[doc = "Bit 9 - B1033"]
    #[inline(always)]
    pub fn b1033(&mut self) -> B1033_W {
        B1033_W { w: self }
    }
    #[doc = "Bit 10 - B1034"]
    #[inline(always)]
    pub fn b1034(&mut self) -> B1034_W {
        B1034_W { w: self }
    }
    #[doc = "Bit 11 - B1035"]
    #[inline(always)]
    pub fn b1035(&mut self) -> B1035_W {
        B1035_W { w: self }
    }
    #[doc = "Bit 12 - B1036"]
    #[inline(always)]
    pub fn b1036(&mut self) -> B1036_W {
        B1036_W { w: self }
    }
    #[doc = "Bit 13 - B1037"]
    #[inline(always)]
    pub fn b1037(&mut self) -> B1037_W {
        B1037_W { w: self }
    }
    #[doc = "Bit 14 - B1038"]
    #[inline(always)]
    pub fn b1038(&mut self) -> B1038_W {
        B1038_W { w: self }
    }
    #[doc = "Bit 15 - B1039"]
    #[inline(always)]
    pub fn b1039(&mut self) -> B1039_W {
        B1039_W { w: self }
    }
    #[doc = "Bit 16 - B1040"]
    #[inline(always)]
    pub fn b1040(&mut self) -> B1040_W {
        B1040_W { w: self }
    }
    #[doc = "Bit 17 - B1041"]
    #[inline(always)]
    pub fn b1041(&mut self) -> B1041_W {
        B1041_W { w: self }
    }
    #[doc = "Bit 18 - B1042"]
    #[inline(always)]
    pub fn b1042(&mut self) -> B1042_W {
        B1042_W { w: self }
    }
    #[doc = "Bit 19 - B1043"]
    #[inline(always)]
    pub fn b1043(&mut self) -> B1043_W {
        B1043_W { w: self }
    }
    #[doc = "Bit 20 - B1044"]
    #[inline(always)]
    pub fn b1044(&mut self) -> B1044_W {
        B1044_W { w: self }
    }
    #[doc = "Bit 21 - B1045"]
    #[inline(always)]
    pub fn b1045(&mut self) -> B1045_W {
        B1045_W { w: self }
    }
    #[doc = "Bit 22 - B1046"]
    #[inline(always)]
    pub fn b1046(&mut self) -> B1046_W {
        B1046_W { w: self }
    }
    #[doc = "Bit 23 - B1047"]
    #[inline(always)]
    pub fn b1047(&mut self) -> B1047_W {
        B1047_W { w: self }
    }
    #[doc = "Bit 24 - B1048"]
    #[inline(always)]
    pub fn b1048(&mut self) -> B1048_W {
        B1048_W { w: self }
    }
    #[doc = "Bit 25 - B1049"]
    #[inline(always)]
    pub fn b1049(&mut self) -> B1049_W {
        B1049_W { w: self }
    }
    #[doc = "Bit 26 - B1050"]
    #[inline(always)]
    pub fn b1050(&mut self) -> B1050_W {
        B1050_W { w: self }
    }
    #[doc = "Bit 27 - B1051"]
    #[inline(always)]
    pub fn b1051(&mut self) -> B1051_W {
        B1051_W { w: self }
    }
    #[doc = "Bit 28 - B1052"]
    #[inline(always)]
    pub fn b1052(&mut self) -> B1052_W {
        B1052_W { w: self }
    }
    #[doc = "Bit 29 - B1053"]
    #[inline(always)]
    pub fn b1053(&mut self) -> B1053_W {
        B1053_W { w: self }
    }
    #[doc = "Bit 30 - B1054"]
    #[inline(always)]
    pub fn b1054(&mut self) -> B1054_W {
        B1054_W { w: self }
    }
    #[doc = "Bit 31 - B1055"]
    #[inline(always)]
    pub fn b1055(&mut self) -> B1055_W {
        B1055_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr32](index.html) module"]
pub struct MPCBB2_VCTR32_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr32::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr32::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR32 to value 0"]
impl crate::Resettable for MPCBB2_VCTR32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
