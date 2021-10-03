#[doc = "Register `MPCBB1_VCTR31` reader"]
pub struct R(crate::R<MPCBB1_VCTR31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR31` writer"]
pub struct W(crate::W<MPCBB1_VCTR31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR31_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR31_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B992` reader - B992"]
pub struct B992_R(crate::FieldReader<bool, bool>);
impl B992_R {
    pub(crate) fn new(bits: bool) -> Self {
        B992_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B992_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B992` writer - B992"]
pub struct B992_W<'a> {
    w: &'a mut W,
}
impl<'a> B992_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B993` reader - B993"]
pub struct B993_R(crate::FieldReader<bool, bool>);
impl B993_R {
    pub(crate) fn new(bits: bool) -> Self {
        B993_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B993_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B993` writer - B993"]
pub struct B993_W<'a> {
    w: &'a mut W,
}
impl<'a> B993_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B994` reader - B994"]
pub struct B994_R(crate::FieldReader<bool, bool>);
impl B994_R {
    pub(crate) fn new(bits: bool) -> Self {
        B994_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B994_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B994` writer - B994"]
pub struct B994_W<'a> {
    w: &'a mut W,
}
impl<'a> B994_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B995` reader - B995"]
pub struct B995_R(crate::FieldReader<bool, bool>);
impl B995_R {
    pub(crate) fn new(bits: bool) -> Self {
        B995_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B995_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B995` writer - B995"]
pub struct B995_W<'a> {
    w: &'a mut W,
}
impl<'a> B995_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B996` reader - B996"]
pub struct B996_R(crate::FieldReader<bool, bool>);
impl B996_R {
    pub(crate) fn new(bits: bool) -> Self {
        B996_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B996_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B996` writer - B996"]
pub struct B996_W<'a> {
    w: &'a mut W,
}
impl<'a> B996_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B997` reader - B997"]
pub struct B997_R(crate::FieldReader<bool, bool>);
impl B997_R {
    pub(crate) fn new(bits: bool) -> Self {
        B997_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B997_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B997` writer - B997"]
pub struct B997_W<'a> {
    w: &'a mut W,
}
impl<'a> B997_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B998` reader - B998"]
pub struct B998_R(crate::FieldReader<bool, bool>);
impl B998_R {
    pub(crate) fn new(bits: bool) -> Self {
        B998_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B998_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B998` writer - B998"]
pub struct B998_W<'a> {
    w: &'a mut W,
}
impl<'a> B998_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B999` reader - B999"]
pub struct B999_R(crate::FieldReader<bool, bool>);
impl B999_R {
    pub(crate) fn new(bits: bool) -> Self {
        B999_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B999_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B999` writer - B999"]
pub struct B999_W<'a> {
    w: &'a mut W,
}
impl<'a> B999_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1000` reader - B1000"]
pub struct B1000_R(crate::FieldReader<bool, bool>);
impl B1000_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1000_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1000_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1000` writer - B1000"]
pub struct B1000_W<'a> {
    w: &'a mut W,
}
impl<'a> B1000_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1001` reader - B1001"]
pub struct B1001_R(crate::FieldReader<bool, bool>);
impl B1001_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1001_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1001_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1001` writer - B1001"]
pub struct B1001_W<'a> {
    w: &'a mut W,
}
impl<'a> B1001_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1002` reader - B1002"]
pub struct B1002_R(crate::FieldReader<bool, bool>);
impl B1002_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1002_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1002_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1002` writer - B1002"]
pub struct B1002_W<'a> {
    w: &'a mut W,
}
impl<'a> B1002_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1003` reader - B1003"]
pub struct B1003_R(crate::FieldReader<bool, bool>);
impl B1003_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1003_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1003_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1003` writer - B1003"]
pub struct B1003_W<'a> {
    w: &'a mut W,
}
impl<'a> B1003_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1004` reader - B1004"]
pub struct B1004_R(crate::FieldReader<bool, bool>);
impl B1004_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1004_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1004_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1004` writer - B1004"]
pub struct B1004_W<'a> {
    w: &'a mut W,
}
impl<'a> B1004_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1005` reader - B1005"]
pub struct B1005_R(crate::FieldReader<bool, bool>);
impl B1005_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1005_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1005_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1005` writer - B1005"]
pub struct B1005_W<'a> {
    w: &'a mut W,
}
impl<'a> B1005_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1006` reader - B1006"]
pub struct B1006_R(crate::FieldReader<bool, bool>);
impl B1006_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1006_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1006_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1006` writer - B1006"]
pub struct B1006_W<'a> {
    w: &'a mut W,
}
impl<'a> B1006_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1007` reader - B1007"]
pub struct B1007_R(crate::FieldReader<bool, bool>);
impl B1007_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1007_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1007_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1007` writer - B1007"]
pub struct B1007_W<'a> {
    w: &'a mut W,
}
impl<'a> B1007_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1008` reader - B1008"]
pub struct B1008_R(crate::FieldReader<bool, bool>);
impl B1008_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1008_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1008_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1008` writer - B1008"]
pub struct B1008_W<'a> {
    w: &'a mut W,
}
impl<'a> B1008_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1009` reader - B1009"]
pub struct B1009_R(crate::FieldReader<bool, bool>);
impl B1009_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1009_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1009_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1009` writer - B1009"]
pub struct B1009_W<'a> {
    w: &'a mut W,
}
impl<'a> B1009_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1010` reader - B1010"]
pub struct B1010_R(crate::FieldReader<bool, bool>);
impl B1010_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1010_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1010_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1010` writer - B1010"]
pub struct B1010_W<'a> {
    w: &'a mut W,
}
impl<'a> B1010_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1011` reader - B1011"]
pub struct B1011_R(crate::FieldReader<bool, bool>);
impl B1011_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1011_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1011_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1011` writer - B1011"]
pub struct B1011_W<'a> {
    w: &'a mut W,
}
impl<'a> B1011_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1012` reader - B1012"]
pub struct B1012_R(crate::FieldReader<bool, bool>);
impl B1012_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1012_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1012_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1012` writer - B1012"]
pub struct B1012_W<'a> {
    w: &'a mut W,
}
impl<'a> B1012_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1013` reader - B1013"]
pub struct B1013_R(crate::FieldReader<bool, bool>);
impl B1013_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1013_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1013_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1013` writer - B1013"]
pub struct B1013_W<'a> {
    w: &'a mut W,
}
impl<'a> B1013_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1014` reader - B1014"]
pub struct B1014_R(crate::FieldReader<bool, bool>);
impl B1014_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1014_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1014_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1014` writer - B1014"]
pub struct B1014_W<'a> {
    w: &'a mut W,
}
impl<'a> B1014_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1015` reader - B1015"]
pub struct B1015_R(crate::FieldReader<bool, bool>);
impl B1015_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1015_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1015_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1015` writer - B1015"]
pub struct B1015_W<'a> {
    w: &'a mut W,
}
impl<'a> B1015_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1016` reader - B1016"]
pub struct B1016_R(crate::FieldReader<bool, bool>);
impl B1016_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1016_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1016_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1016` writer - B1016"]
pub struct B1016_W<'a> {
    w: &'a mut W,
}
impl<'a> B1016_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1017` reader - B1017"]
pub struct B1017_R(crate::FieldReader<bool, bool>);
impl B1017_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1017_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1017_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1017` writer - B1017"]
pub struct B1017_W<'a> {
    w: &'a mut W,
}
impl<'a> B1017_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1018` reader - B1018"]
pub struct B1018_R(crate::FieldReader<bool, bool>);
impl B1018_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1018_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1018_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1018` writer - B1018"]
pub struct B1018_W<'a> {
    w: &'a mut W,
}
impl<'a> B1018_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1019` reader - B1019"]
pub struct B1019_R(crate::FieldReader<bool, bool>);
impl B1019_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1019_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1019_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1019` writer - B1019"]
pub struct B1019_W<'a> {
    w: &'a mut W,
}
impl<'a> B1019_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1020` reader - B1020"]
pub struct B1020_R(crate::FieldReader<bool, bool>);
impl B1020_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1020_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1020_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1020` writer - B1020"]
pub struct B1020_W<'a> {
    w: &'a mut W,
}
impl<'a> B1020_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1021` reader - B1021"]
pub struct B1021_R(crate::FieldReader<bool, bool>);
impl B1021_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1021_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1021_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1021` writer - B1021"]
pub struct B1021_W<'a> {
    w: &'a mut W,
}
impl<'a> B1021_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1022` reader - B1022"]
pub struct B1022_R(crate::FieldReader<bool, bool>);
impl B1022_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1022_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1022_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1022` writer - B1022"]
pub struct B1022_W<'a> {
    w: &'a mut W,
}
impl<'a> B1022_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1023` reader - B1023"]
pub struct B1023_R(crate::FieldReader<bool, bool>);
impl B1023_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1023_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1023_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1023` writer - B1023"]
pub struct B1023_W<'a> {
    w: &'a mut W,
}
impl<'a> B1023_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B992"]
    #[inline(always)]
    pub fn b992(&self) -> B992_R {
        B992_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B993"]
    #[inline(always)]
    pub fn b993(&self) -> B993_R {
        B993_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B994"]
    #[inline(always)]
    pub fn b994(&self) -> B994_R {
        B994_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B995"]
    #[inline(always)]
    pub fn b995(&self) -> B995_R {
        B995_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B996"]
    #[inline(always)]
    pub fn b996(&self) -> B996_R {
        B996_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B997"]
    #[inline(always)]
    pub fn b997(&self) -> B997_R {
        B997_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B998"]
    #[inline(always)]
    pub fn b998(&self) -> B998_R {
        B998_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B999"]
    #[inline(always)]
    pub fn b999(&self) -> B999_R {
        B999_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1000"]
    #[inline(always)]
    pub fn b1000(&self) -> B1000_R {
        B1000_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1001"]
    #[inline(always)]
    pub fn b1001(&self) -> B1001_R {
        B1001_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1002"]
    #[inline(always)]
    pub fn b1002(&self) -> B1002_R {
        B1002_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1003"]
    #[inline(always)]
    pub fn b1003(&self) -> B1003_R {
        B1003_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1004"]
    #[inline(always)]
    pub fn b1004(&self) -> B1004_R {
        B1004_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1005"]
    #[inline(always)]
    pub fn b1005(&self) -> B1005_R {
        B1005_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1006"]
    #[inline(always)]
    pub fn b1006(&self) -> B1006_R {
        B1006_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1007"]
    #[inline(always)]
    pub fn b1007(&self) -> B1007_R {
        B1007_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1008"]
    #[inline(always)]
    pub fn b1008(&self) -> B1008_R {
        B1008_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1009"]
    #[inline(always)]
    pub fn b1009(&self) -> B1009_R {
        B1009_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1010"]
    #[inline(always)]
    pub fn b1010(&self) -> B1010_R {
        B1010_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1011"]
    #[inline(always)]
    pub fn b1011(&self) -> B1011_R {
        B1011_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1012"]
    #[inline(always)]
    pub fn b1012(&self) -> B1012_R {
        B1012_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1013"]
    #[inline(always)]
    pub fn b1013(&self) -> B1013_R {
        B1013_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1014"]
    #[inline(always)]
    pub fn b1014(&self) -> B1014_R {
        B1014_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1015"]
    #[inline(always)]
    pub fn b1015(&self) -> B1015_R {
        B1015_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1016"]
    #[inline(always)]
    pub fn b1016(&self) -> B1016_R {
        B1016_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1017"]
    #[inline(always)]
    pub fn b1017(&self) -> B1017_R {
        B1017_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1018"]
    #[inline(always)]
    pub fn b1018(&self) -> B1018_R {
        B1018_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1019"]
    #[inline(always)]
    pub fn b1019(&self) -> B1019_R {
        B1019_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1020"]
    #[inline(always)]
    pub fn b1020(&self) -> B1020_R {
        B1020_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1021"]
    #[inline(always)]
    pub fn b1021(&self) -> B1021_R {
        B1021_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1022"]
    #[inline(always)]
    pub fn b1022(&self) -> B1022_R {
        B1022_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1023"]
    #[inline(always)]
    pub fn b1023(&self) -> B1023_R {
        B1023_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B992"]
    #[inline(always)]
    pub fn b992(&mut self) -> B992_W {
        B992_W { w: self }
    }
    #[doc = "Bit 1 - B993"]
    #[inline(always)]
    pub fn b993(&mut self) -> B993_W {
        B993_W { w: self }
    }
    #[doc = "Bit 2 - B994"]
    #[inline(always)]
    pub fn b994(&mut self) -> B994_W {
        B994_W { w: self }
    }
    #[doc = "Bit 3 - B995"]
    #[inline(always)]
    pub fn b995(&mut self) -> B995_W {
        B995_W { w: self }
    }
    #[doc = "Bit 4 - B996"]
    #[inline(always)]
    pub fn b996(&mut self) -> B996_W {
        B996_W { w: self }
    }
    #[doc = "Bit 5 - B997"]
    #[inline(always)]
    pub fn b997(&mut self) -> B997_W {
        B997_W { w: self }
    }
    #[doc = "Bit 6 - B998"]
    #[inline(always)]
    pub fn b998(&mut self) -> B998_W {
        B998_W { w: self }
    }
    #[doc = "Bit 7 - B999"]
    #[inline(always)]
    pub fn b999(&mut self) -> B999_W {
        B999_W { w: self }
    }
    #[doc = "Bit 8 - B1000"]
    #[inline(always)]
    pub fn b1000(&mut self) -> B1000_W {
        B1000_W { w: self }
    }
    #[doc = "Bit 9 - B1001"]
    #[inline(always)]
    pub fn b1001(&mut self) -> B1001_W {
        B1001_W { w: self }
    }
    #[doc = "Bit 10 - B1002"]
    #[inline(always)]
    pub fn b1002(&mut self) -> B1002_W {
        B1002_W { w: self }
    }
    #[doc = "Bit 11 - B1003"]
    #[inline(always)]
    pub fn b1003(&mut self) -> B1003_W {
        B1003_W { w: self }
    }
    #[doc = "Bit 12 - B1004"]
    #[inline(always)]
    pub fn b1004(&mut self) -> B1004_W {
        B1004_W { w: self }
    }
    #[doc = "Bit 13 - B1005"]
    #[inline(always)]
    pub fn b1005(&mut self) -> B1005_W {
        B1005_W { w: self }
    }
    #[doc = "Bit 14 - B1006"]
    #[inline(always)]
    pub fn b1006(&mut self) -> B1006_W {
        B1006_W { w: self }
    }
    #[doc = "Bit 15 - B1007"]
    #[inline(always)]
    pub fn b1007(&mut self) -> B1007_W {
        B1007_W { w: self }
    }
    #[doc = "Bit 16 - B1008"]
    #[inline(always)]
    pub fn b1008(&mut self) -> B1008_W {
        B1008_W { w: self }
    }
    #[doc = "Bit 17 - B1009"]
    #[inline(always)]
    pub fn b1009(&mut self) -> B1009_W {
        B1009_W { w: self }
    }
    #[doc = "Bit 18 - B1010"]
    #[inline(always)]
    pub fn b1010(&mut self) -> B1010_W {
        B1010_W { w: self }
    }
    #[doc = "Bit 19 - B1011"]
    #[inline(always)]
    pub fn b1011(&mut self) -> B1011_W {
        B1011_W { w: self }
    }
    #[doc = "Bit 20 - B1012"]
    #[inline(always)]
    pub fn b1012(&mut self) -> B1012_W {
        B1012_W { w: self }
    }
    #[doc = "Bit 21 - B1013"]
    #[inline(always)]
    pub fn b1013(&mut self) -> B1013_W {
        B1013_W { w: self }
    }
    #[doc = "Bit 22 - B1014"]
    #[inline(always)]
    pub fn b1014(&mut self) -> B1014_W {
        B1014_W { w: self }
    }
    #[doc = "Bit 23 - B1015"]
    #[inline(always)]
    pub fn b1015(&mut self) -> B1015_W {
        B1015_W { w: self }
    }
    #[doc = "Bit 24 - B1016"]
    #[inline(always)]
    pub fn b1016(&mut self) -> B1016_W {
        B1016_W { w: self }
    }
    #[doc = "Bit 25 - B1017"]
    #[inline(always)]
    pub fn b1017(&mut self) -> B1017_W {
        B1017_W { w: self }
    }
    #[doc = "Bit 26 - B1018"]
    #[inline(always)]
    pub fn b1018(&mut self) -> B1018_W {
        B1018_W { w: self }
    }
    #[doc = "Bit 27 - B1019"]
    #[inline(always)]
    pub fn b1019(&mut self) -> B1019_W {
        B1019_W { w: self }
    }
    #[doc = "Bit 28 - B1020"]
    #[inline(always)]
    pub fn b1020(&mut self) -> B1020_W {
        B1020_W { w: self }
    }
    #[doc = "Bit 29 - B1021"]
    #[inline(always)]
    pub fn b1021(&mut self) -> B1021_W {
        B1021_W { w: self }
    }
    #[doc = "Bit 30 - B1022"]
    #[inline(always)]
    pub fn b1022(&mut self) -> B1022_W {
        B1022_W { w: self }
    }
    #[doc = "Bit 31 - B1023"]
    #[inline(always)]
    pub fn b1023(&mut self) -> B1023_W {
        B1023_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr31](index.html) module"]
pub struct MPCBB1_VCTR31_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR31_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr31::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr31::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR31 to value 0"]
impl crate::Resettable for MPCBB1_VCTR31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
