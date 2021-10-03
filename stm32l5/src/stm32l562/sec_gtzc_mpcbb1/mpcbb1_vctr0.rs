#[doc = "Register `MPCBB1_VCTR0` reader"]
pub struct R(crate::R<MPCBB1_VCTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR0` writer"]
pub struct W(crate::W<MPCBB1_VCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR0_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0` reader - B0"]
pub struct B0_R(crate::FieldReader<bool, bool>);
impl B0_R {
    pub(crate) fn new(bits: bool) -> Self {
        B0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0` writer - B0"]
pub struct B0_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1` reader - B1"]
pub struct B1_R(crate::FieldReader<bool, bool>);
impl B1_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1` writer - B1"]
pub struct B1_W<'a> {
    w: &'a mut W,
}
impl<'a> B1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B2` reader - B2"]
pub struct B2_R(crate::FieldReader<bool, bool>);
impl B2_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2` writer - B2"]
pub struct B2_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B3` reader - B3"]
pub struct B3_R(crate::FieldReader<bool, bool>);
impl B3_R {
    pub(crate) fn new(bits: bool) -> Self {
        B3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B3` writer - B3"]
pub struct B3_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B4` reader - B4"]
pub struct B4_R(crate::FieldReader<bool, bool>);
impl B4_R {
    pub(crate) fn new(bits: bool) -> Self {
        B4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B4` writer - B4"]
pub struct B4_W<'a> {
    w: &'a mut W,
}
impl<'a> B4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B5` reader - B5"]
pub struct B5_R(crate::FieldReader<bool, bool>);
impl B5_R {
    pub(crate) fn new(bits: bool) -> Self {
        B5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B5` writer - B5"]
pub struct B5_W<'a> {
    w: &'a mut W,
}
impl<'a> B5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B6` reader - B6"]
pub struct B6_R(crate::FieldReader<bool, bool>);
impl B6_R {
    pub(crate) fn new(bits: bool) -> Self {
        B6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B6` writer - B6"]
pub struct B6_W<'a> {
    w: &'a mut W,
}
impl<'a> B6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B7` reader - B7"]
pub struct B7_R(crate::FieldReader<bool, bool>);
impl B7_R {
    pub(crate) fn new(bits: bool) -> Self {
        B7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B7` writer - B7"]
pub struct B7_W<'a> {
    w: &'a mut W,
}
impl<'a> B7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B8` reader - B8"]
pub struct B8_R(crate::FieldReader<bool, bool>);
impl B8_R {
    pub(crate) fn new(bits: bool) -> Self {
        B8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B8` writer - B8"]
pub struct B8_W<'a> {
    w: &'a mut W,
}
impl<'a> B8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B9` reader - B9"]
pub struct B9_R(crate::FieldReader<bool, bool>);
impl B9_R {
    pub(crate) fn new(bits: bool) -> Self {
        B9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B9` writer - B9"]
pub struct B9_W<'a> {
    w: &'a mut W,
}
impl<'a> B9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B10` reader - B10"]
pub struct B10_R(crate::FieldReader<bool, bool>);
impl B10_R {
    pub(crate) fn new(bits: bool) -> Self {
        B10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B10` writer - B10"]
pub struct B10_W<'a> {
    w: &'a mut W,
}
impl<'a> B10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B11` reader - B11"]
pub struct B11_R(crate::FieldReader<bool, bool>);
impl B11_R {
    pub(crate) fn new(bits: bool) -> Self {
        B11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B11` writer - B11"]
pub struct B11_W<'a> {
    w: &'a mut W,
}
impl<'a> B11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B12` reader - B12"]
pub struct B12_R(crate::FieldReader<bool, bool>);
impl B12_R {
    pub(crate) fn new(bits: bool) -> Self {
        B12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B12` writer - B12"]
pub struct B12_W<'a> {
    w: &'a mut W,
}
impl<'a> B12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B13` reader - B13"]
pub struct B13_R(crate::FieldReader<bool, bool>);
impl B13_R {
    pub(crate) fn new(bits: bool) -> Self {
        B13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B13` writer - B13"]
pub struct B13_W<'a> {
    w: &'a mut W,
}
impl<'a> B13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B14` reader - B14"]
pub struct B14_R(crate::FieldReader<bool, bool>);
impl B14_R {
    pub(crate) fn new(bits: bool) -> Self {
        B14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B14` writer - B14"]
pub struct B14_W<'a> {
    w: &'a mut W,
}
impl<'a> B14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B15` reader - B15"]
pub struct B15_R(crate::FieldReader<bool, bool>);
impl B15_R {
    pub(crate) fn new(bits: bool) -> Self {
        B15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B15` writer - B15"]
pub struct B15_W<'a> {
    w: &'a mut W,
}
impl<'a> B15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B16` reader - B16"]
pub struct B16_R(crate::FieldReader<bool, bool>);
impl B16_R {
    pub(crate) fn new(bits: bool) -> Self {
        B16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B16` writer - B16"]
pub struct B16_W<'a> {
    w: &'a mut W,
}
impl<'a> B16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B17` reader - B17"]
pub struct B17_R(crate::FieldReader<bool, bool>);
impl B17_R {
    pub(crate) fn new(bits: bool) -> Self {
        B17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B17` writer - B17"]
pub struct B17_W<'a> {
    w: &'a mut W,
}
impl<'a> B17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B18` reader - B18"]
pub struct B18_R(crate::FieldReader<bool, bool>);
impl B18_R {
    pub(crate) fn new(bits: bool) -> Self {
        B18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B18` writer - B18"]
pub struct B18_W<'a> {
    w: &'a mut W,
}
impl<'a> B18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B19` reader - B19"]
pub struct B19_R(crate::FieldReader<bool, bool>);
impl B19_R {
    pub(crate) fn new(bits: bool) -> Self {
        B19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B19` writer - B19"]
pub struct B19_W<'a> {
    w: &'a mut W,
}
impl<'a> B19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B20` reader - B20"]
pub struct B20_R(crate::FieldReader<bool, bool>);
impl B20_R {
    pub(crate) fn new(bits: bool) -> Self {
        B20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B20` writer - B20"]
pub struct B20_W<'a> {
    w: &'a mut W,
}
impl<'a> B20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B21` reader - B21"]
pub struct B21_R(crate::FieldReader<bool, bool>);
impl B21_R {
    pub(crate) fn new(bits: bool) -> Self {
        B21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B21` writer - B21"]
pub struct B21_W<'a> {
    w: &'a mut W,
}
impl<'a> B21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B22` reader - B22"]
pub struct B22_R(crate::FieldReader<bool, bool>);
impl B22_R {
    pub(crate) fn new(bits: bool) -> Self {
        B22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B22` writer - B22"]
pub struct B22_W<'a> {
    w: &'a mut W,
}
impl<'a> B22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B23` reader - B23"]
pub struct B23_R(crate::FieldReader<bool, bool>);
impl B23_R {
    pub(crate) fn new(bits: bool) -> Self {
        B23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B23` writer - B23"]
pub struct B23_W<'a> {
    w: &'a mut W,
}
impl<'a> B23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B24` reader - B24"]
pub struct B24_R(crate::FieldReader<bool, bool>);
impl B24_R {
    pub(crate) fn new(bits: bool) -> Self {
        B24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B24` writer - B24"]
pub struct B24_W<'a> {
    w: &'a mut W,
}
impl<'a> B24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B25` reader - B25"]
pub struct B25_R(crate::FieldReader<bool, bool>);
impl B25_R {
    pub(crate) fn new(bits: bool) -> Self {
        B25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B25` writer - B25"]
pub struct B25_W<'a> {
    w: &'a mut W,
}
impl<'a> B25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B26` reader - B26"]
pub struct B26_R(crate::FieldReader<bool, bool>);
impl B26_R {
    pub(crate) fn new(bits: bool) -> Self {
        B26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B26` writer - B26"]
pub struct B26_W<'a> {
    w: &'a mut W,
}
impl<'a> B26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B27` reader - B27"]
pub struct B27_R(crate::FieldReader<bool, bool>);
impl B27_R {
    pub(crate) fn new(bits: bool) -> Self {
        B27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B27` writer - B27"]
pub struct B27_W<'a> {
    w: &'a mut W,
}
impl<'a> B27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B28` reader - B28"]
pub struct B28_R(crate::FieldReader<bool, bool>);
impl B28_R {
    pub(crate) fn new(bits: bool) -> Self {
        B28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B28` writer - B28"]
pub struct B28_W<'a> {
    w: &'a mut W,
}
impl<'a> B28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B29` reader - B29"]
pub struct B29_R(crate::FieldReader<bool, bool>);
impl B29_R {
    pub(crate) fn new(bits: bool) -> Self {
        B29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B29` writer - B29"]
pub struct B29_W<'a> {
    w: &'a mut W,
}
impl<'a> B29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B30` reader - B30"]
pub struct B30_R(crate::FieldReader<bool, bool>);
impl B30_R {
    pub(crate) fn new(bits: bool) -> Self {
        B30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B30` writer - B30"]
pub struct B30_W<'a> {
    w: &'a mut W,
}
impl<'a> B30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B31` reader - B31"]
pub struct B31_R(crate::FieldReader<bool, bool>);
impl B31_R {
    pub(crate) fn new(bits: bool) -> Self {
        B31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B31` writer - B31"]
pub struct B31_W<'a> {
    w: &'a mut W,
}
impl<'a> B31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B0"]
    #[inline(always)]
    pub fn b0(&self) -> B0_R {
        B0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1"]
    #[inline(always)]
    pub fn b1(&self) -> B1_R {
        B1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B2"]
    #[inline(always)]
    pub fn b2(&self) -> B2_R {
        B2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B3"]
    #[inline(always)]
    pub fn b3(&self) -> B3_R {
        B3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B4"]
    #[inline(always)]
    pub fn b4(&self) -> B4_R {
        B4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B5"]
    #[inline(always)]
    pub fn b5(&self) -> B5_R {
        B5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B6"]
    #[inline(always)]
    pub fn b6(&self) -> B6_R {
        B6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B7"]
    #[inline(always)]
    pub fn b7(&self) -> B7_R {
        B7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B8"]
    #[inline(always)]
    pub fn b8(&self) -> B8_R {
        B8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B9"]
    #[inline(always)]
    pub fn b9(&self) -> B9_R {
        B9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B10"]
    #[inline(always)]
    pub fn b10(&self) -> B10_R {
        B10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B11"]
    #[inline(always)]
    pub fn b11(&self) -> B11_R {
        B11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B12"]
    #[inline(always)]
    pub fn b12(&self) -> B12_R {
        B12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B13"]
    #[inline(always)]
    pub fn b13(&self) -> B13_R {
        B13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B14"]
    #[inline(always)]
    pub fn b14(&self) -> B14_R {
        B14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B15"]
    #[inline(always)]
    pub fn b15(&self) -> B15_R {
        B15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B16"]
    #[inline(always)]
    pub fn b16(&self) -> B16_R {
        B16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B17"]
    #[inline(always)]
    pub fn b17(&self) -> B17_R {
        B17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B18"]
    #[inline(always)]
    pub fn b18(&self) -> B18_R {
        B18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B19"]
    #[inline(always)]
    pub fn b19(&self) -> B19_R {
        B19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B20"]
    #[inline(always)]
    pub fn b20(&self) -> B20_R {
        B20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B21"]
    #[inline(always)]
    pub fn b21(&self) -> B21_R {
        B21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B22"]
    #[inline(always)]
    pub fn b22(&self) -> B22_R {
        B22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B23"]
    #[inline(always)]
    pub fn b23(&self) -> B23_R {
        B23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B24"]
    #[inline(always)]
    pub fn b24(&self) -> B24_R {
        B24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B25"]
    #[inline(always)]
    pub fn b25(&self) -> B25_R {
        B25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B26"]
    #[inline(always)]
    pub fn b26(&self) -> B26_R {
        B26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B27"]
    #[inline(always)]
    pub fn b27(&self) -> B27_R {
        B27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B28"]
    #[inline(always)]
    pub fn b28(&self) -> B28_R {
        B28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B29"]
    #[inline(always)]
    pub fn b29(&self) -> B29_R {
        B29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B30"]
    #[inline(always)]
    pub fn b30(&self) -> B30_R {
        B30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B31"]
    #[inline(always)]
    pub fn b31(&self) -> B31_R {
        B31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B0"]
    #[inline(always)]
    pub fn b0(&mut self) -> B0_W {
        B0_W { w: self }
    }
    #[doc = "Bit 1 - B1"]
    #[inline(always)]
    pub fn b1(&mut self) -> B1_W {
        B1_W { w: self }
    }
    #[doc = "Bit 2 - B2"]
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W {
        B2_W { w: self }
    }
    #[doc = "Bit 3 - B3"]
    #[inline(always)]
    pub fn b3(&mut self) -> B3_W {
        B3_W { w: self }
    }
    #[doc = "Bit 4 - B4"]
    #[inline(always)]
    pub fn b4(&mut self) -> B4_W {
        B4_W { w: self }
    }
    #[doc = "Bit 5 - B5"]
    #[inline(always)]
    pub fn b5(&mut self) -> B5_W {
        B5_W { w: self }
    }
    #[doc = "Bit 6 - B6"]
    #[inline(always)]
    pub fn b6(&mut self) -> B6_W {
        B6_W { w: self }
    }
    #[doc = "Bit 7 - B7"]
    #[inline(always)]
    pub fn b7(&mut self) -> B7_W {
        B7_W { w: self }
    }
    #[doc = "Bit 8 - B8"]
    #[inline(always)]
    pub fn b8(&mut self) -> B8_W {
        B8_W { w: self }
    }
    #[doc = "Bit 9 - B9"]
    #[inline(always)]
    pub fn b9(&mut self) -> B9_W {
        B9_W { w: self }
    }
    #[doc = "Bit 10 - B10"]
    #[inline(always)]
    pub fn b10(&mut self) -> B10_W {
        B10_W { w: self }
    }
    #[doc = "Bit 11 - B11"]
    #[inline(always)]
    pub fn b11(&mut self) -> B11_W {
        B11_W { w: self }
    }
    #[doc = "Bit 12 - B12"]
    #[inline(always)]
    pub fn b12(&mut self) -> B12_W {
        B12_W { w: self }
    }
    #[doc = "Bit 13 - B13"]
    #[inline(always)]
    pub fn b13(&mut self) -> B13_W {
        B13_W { w: self }
    }
    #[doc = "Bit 14 - B14"]
    #[inline(always)]
    pub fn b14(&mut self) -> B14_W {
        B14_W { w: self }
    }
    #[doc = "Bit 15 - B15"]
    #[inline(always)]
    pub fn b15(&mut self) -> B15_W {
        B15_W { w: self }
    }
    #[doc = "Bit 16 - B16"]
    #[inline(always)]
    pub fn b16(&mut self) -> B16_W {
        B16_W { w: self }
    }
    #[doc = "Bit 17 - B17"]
    #[inline(always)]
    pub fn b17(&mut self) -> B17_W {
        B17_W { w: self }
    }
    #[doc = "Bit 18 - B18"]
    #[inline(always)]
    pub fn b18(&mut self) -> B18_W {
        B18_W { w: self }
    }
    #[doc = "Bit 19 - B19"]
    #[inline(always)]
    pub fn b19(&mut self) -> B19_W {
        B19_W { w: self }
    }
    #[doc = "Bit 20 - B20"]
    #[inline(always)]
    pub fn b20(&mut self) -> B20_W {
        B20_W { w: self }
    }
    #[doc = "Bit 21 - B21"]
    #[inline(always)]
    pub fn b21(&mut self) -> B21_W {
        B21_W { w: self }
    }
    #[doc = "Bit 22 - B22"]
    #[inline(always)]
    pub fn b22(&mut self) -> B22_W {
        B22_W { w: self }
    }
    #[doc = "Bit 23 - B23"]
    #[inline(always)]
    pub fn b23(&mut self) -> B23_W {
        B23_W { w: self }
    }
    #[doc = "Bit 24 - B24"]
    #[inline(always)]
    pub fn b24(&mut self) -> B24_W {
        B24_W { w: self }
    }
    #[doc = "Bit 25 - B25"]
    #[inline(always)]
    pub fn b25(&mut self) -> B25_W {
        B25_W { w: self }
    }
    #[doc = "Bit 26 - B26"]
    #[inline(always)]
    pub fn b26(&mut self) -> B26_W {
        B26_W { w: self }
    }
    #[doc = "Bit 27 - B27"]
    #[inline(always)]
    pub fn b27(&mut self) -> B27_W {
        B27_W { w: self }
    }
    #[doc = "Bit 28 - B28"]
    #[inline(always)]
    pub fn b28(&mut self) -> B28_W {
        B28_W { w: self }
    }
    #[doc = "Bit 29 - B29"]
    #[inline(always)]
    pub fn b29(&mut self) -> B29_W {
        B29_W { w: self }
    }
    #[doc = "Bit 30 - B30"]
    #[inline(always)]
    pub fn b30(&mut self) -> B30_W {
        B30_W { w: self }
    }
    #[doc = "Bit 31 - B31"]
    #[inline(always)]
    pub fn b31(&mut self) -> B31_W {
        B31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr0](index.html) module"]
pub struct MPCBB1_VCTR0_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr0::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr0::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR0 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
