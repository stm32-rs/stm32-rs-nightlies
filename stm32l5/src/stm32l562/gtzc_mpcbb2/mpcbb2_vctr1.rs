#[doc = "Register `MPCBB2_VCTR1` reader"]
pub struct R(crate::R<MPCBB2_VCTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR1` writer"]
pub struct W(crate::W<MPCBB2_VCTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR1_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B32` reader - B32"]
pub struct B32_R(crate::FieldReader<bool, bool>);
impl B32_R {
    pub(crate) fn new(bits: bool) -> Self {
        B32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B32` writer - B32"]
pub struct B32_W<'a> {
    w: &'a mut W,
}
impl<'a> B32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B33` reader - B33"]
pub struct B33_R(crate::FieldReader<bool, bool>);
impl B33_R {
    pub(crate) fn new(bits: bool) -> Self {
        B33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B33` writer - B33"]
pub struct B33_W<'a> {
    w: &'a mut W,
}
impl<'a> B33_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B34` reader - B34"]
pub struct B34_R(crate::FieldReader<bool, bool>);
impl B34_R {
    pub(crate) fn new(bits: bool) -> Self {
        B34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B34` writer - B34"]
pub struct B34_W<'a> {
    w: &'a mut W,
}
impl<'a> B34_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B35` reader - B35"]
pub struct B35_R(crate::FieldReader<bool, bool>);
impl B35_R {
    pub(crate) fn new(bits: bool) -> Self {
        B35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B35` writer - B35"]
pub struct B35_W<'a> {
    w: &'a mut W,
}
impl<'a> B35_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B36` reader - B36"]
pub struct B36_R(crate::FieldReader<bool, bool>);
impl B36_R {
    pub(crate) fn new(bits: bool) -> Self {
        B36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B36` writer - B36"]
pub struct B36_W<'a> {
    w: &'a mut W,
}
impl<'a> B36_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B37` reader - B37"]
pub struct B37_R(crate::FieldReader<bool, bool>);
impl B37_R {
    pub(crate) fn new(bits: bool) -> Self {
        B37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B37` writer - B37"]
pub struct B37_W<'a> {
    w: &'a mut W,
}
impl<'a> B37_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B38` reader - B38"]
pub struct B38_R(crate::FieldReader<bool, bool>);
impl B38_R {
    pub(crate) fn new(bits: bool) -> Self {
        B38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B38` writer - B38"]
pub struct B38_W<'a> {
    w: &'a mut W,
}
impl<'a> B38_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B39` reader - B39"]
pub struct B39_R(crate::FieldReader<bool, bool>);
impl B39_R {
    pub(crate) fn new(bits: bool) -> Self {
        B39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B39` writer - B39"]
pub struct B39_W<'a> {
    w: &'a mut W,
}
impl<'a> B39_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B40` reader - B40"]
pub struct B40_R(crate::FieldReader<bool, bool>);
impl B40_R {
    pub(crate) fn new(bits: bool) -> Self {
        B40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B40` writer - B40"]
pub struct B40_W<'a> {
    w: &'a mut W,
}
impl<'a> B40_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B41` reader - B41"]
pub struct B41_R(crate::FieldReader<bool, bool>);
impl B41_R {
    pub(crate) fn new(bits: bool) -> Self {
        B41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B41` writer - B41"]
pub struct B41_W<'a> {
    w: &'a mut W,
}
impl<'a> B41_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B42` reader - B42"]
pub struct B42_R(crate::FieldReader<bool, bool>);
impl B42_R {
    pub(crate) fn new(bits: bool) -> Self {
        B42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B42` writer - B42"]
pub struct B42_W<'a> {
    w: &'a mut W,
}
impl<'a> B42_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B43` reader - B43"]
pub struct B43_R(crate::FieldReader<bool, bool>);
impl B43_R {
    pub(crate) fn new(bits: bool) -> Self {
        B43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B43` writer - B43"]
pub struct B43_W<'a> {
    w: &'a mut W,
}
impl<'a> B43_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B44` reader - B44"]
pub struct B44_R(crate::FieldReader<bool, bool>);
impl B44_R {
    pub(crate) fn new(bits: bool) -> Self {
        B44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B44` writer - B44"]
pub struct B44_W<'a> {
    w: &'a mut W,
}
impl<'a> B44_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B45` reader - B45"]
pub struct B45_R(crate::FieldReader<bool, bool>);
impl B45_R {
    pub(crate) fn new(bits: bool) -> Self {
        B45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B45` writer - B45"]
pub struct B45_W<'a> {
    w: &'a mut W,
}
impl<'a> B45_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B46` reader - B46"]
pub struct B46_R(crate::FieldReader<bool, bool>);
impl B46_R {
    pub(crate) fn new(bits: bool) -> Self {
        B46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B46` writer - B46"]
pub struct B46_W<'a> {
    w: &'a mut W,
}
impl<'a> B46_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B47` reader - B47"]
pub struct B47_R(crate::FieldReader<bool, bool>);
impl B47_R {
    pub(crate) fn new(bits: bool) -> Self {
        B47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B47` writer - B47"]
pub struct B47_W<'a> {
    w: &'a mut W,
}
impl<'a> B47_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B48` reader - B48"]
pub struct B48_R(crate::FieldReader<bool, bool>);
impl B48_R {
    pub(crate) fn new(bits: bool) -> Self {
        B48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B48` writer - B48"]
pub struct B48_W<'a> {
    w: &'a mut W,
}
impl<'a> B48_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B49` reader - B49"]
pub struct B49_R(crate::FieldReader<bool, bool>);
impl B49_R {
    pub(crate) fn new(bits: bool) -> Self {
        B49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B49` writer - B49"]
pub struct B49_W<'a> {
    w: &'a mut W,
}
impl<'a> B49_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B50` reader - B50"]
pub struct B50_R(crate::FieldReader<bool, bool>);
impl B50_R {
    pub(crate) fn new(bits: bool) -> Self {
        B50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B50_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B50` writer - B50"]
pub struct B50_W<'a> {
    w: &'a mut W,
}
impl<'a> B50_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B51` reader - B51"]
pub struct B51_R(crate::FieldReader<bool, bool>);
impl B51_R {
    pub(crate) fn new(bits: bool) -> Self {
        B51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B51_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B51` writer - B51"]
pub struct B51_W<'a> {
    w: &'a mut W,
}
impl<'a> B51_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B52` reader - B52"]
pub struct B52_R(crate::FieldReader<bool, bool>);
impl B52_R {
    pub(crate) fn new(bits: bool) -> Self {
        B52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B52_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B52` writer - B52"]
pub struct B52_W<'a> {
    w: &'a mut W,
}
impl<'a> B52_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B53` reader - B53"]
pub struct B53_R(crate::FieldReader<bool, bool>);
impl B53_R {
    pub(crate) fn new(bits: bool) -> Self {
        B53_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B53_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B53` writer - B53"]
pub struct B53_W<'a> {
    w: &'a mut W,
}
impl<'a> B53_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B54` reader - B54"]
pub struct B54_R(crate::FieldReader<bool, bool>);
impl B54_R {
    pub(crate) fn new(bits: bool) -> Self {
        B54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B54_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B54` writer - B54"]
pub struct B54_W<'a> {
    w: &'a mut W,
}
impl<'a> B54_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B55` reader - B55"]
pub struct B55_R(crate::FieldReader<bool, bool>);
impl B55_R {
    pub(crate) fn new(bits: bool) -> Self {
        B55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B55_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B55` writer - B55"]
pub struct B55_W<'a> {
    w: &'a mut W,
}
impl<'a> B55_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B56` reader - B56"]
pub struct B56_R(crate::FieldReader<bool, bool>);
impl B56_R {
    pub(crate) fn new(bits: bool) -> Self {
        B56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B56_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B56` writer - B56"]
pub struct B56_W<'a> {
    w: &'a mut W,
}
impl<'a> B56_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B57` reader - B57"]
pub struct B57_R(crate::FieldReader<bool, bool>);
impl B57_R {
    pub(crate) fn new(bits: bool) -> Self {
        B57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B57_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B57` writer - B57"]
pub struct B57_W<'a> {
    w: &'a mut W,
}
impl<'a> B57_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B58` reader - B58"]
pub struct B58_R(crate::FieldReader<bool, bool>);
impl B58_R {
    pub(crate) fn new(bits: bool) -> Self {
        B58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B58_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B58` writer - B58"]
pub struct B58_W<'a> {
    w: &'a mut W,
}
impl<'a> B58_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B59` reader - B59"]
pub struct B59_R(crate::FieldReader<bool, bool>);
impl B59_R {
    pub(crate) fn new(bits: bool) -> Self {
        B59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B59_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B59` writer - B59"]
pub struct B59_W<'a> {
    w: &'a mut W,
}
impl<'a> B59_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B60` reader - B60"]
pub struct B60_R(crate::FieldReader<bool, bool>);
impl B60_R {
    pub(crate) fn new(bits: bool) -> Self {
        B60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B60_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B60` writer - B60"]
pub struct B60_W<'a> {
    w: &'a mut W,
}
impl<'a> B60_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B61` reader - B61"]
pub struct B61_R(crate::FieldReader<bool, bool>);
impl B61_R {
    pub(crate) fn new(bits: bool) -> Self {
        B61_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B61_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B61` writer - B61"]
pub struct B61_W<'a> {
    w: &'a mut W,
}
impl<'a> B61_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B62` reader - B62"]
pub struct B62_R(crate::FieldReader<bool, bool>);
impl B62_R {
    pub(crate) fn new(bits: bool) -> Self {
        B62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B62_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B62` writer - B62"]
pub struct B62_W<'a> {
    w: &'a mut W,
}
impl<'a> B62_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B63` reader - B63"]
pub struct B63_R(crate::FieldReader<bool, bool>);
impl B63_R {
    pub(crate) fn new(bits: bool) -> Self {
        B63_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B63_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B63` writer - B63"]
pub struct B63_W<'a> {
    w: &'a mut W,
}
impl<'a> B63_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B32"]
    #[inline(always)]
    pub fn b32(&self) -> B32_R {
        B32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B33"]
    #[inline(always)]
    pub fn b33(&self) -> B33_R {
        B33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B34"]
    #[inline(always)]
    pub fn b34(&self) -> B34_R {
        B34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B35"]
    #[inline(always)]
    pub fn b35(&self) -> B35_R {
        B35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B36"]
    #[inline(always)]
    pub fn b36(&self) -> B36_R {
        B36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B37"]
    #[inline(always)]
    pub fn b37(&self) -> B37_R {
        B37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B38"]
    #[inline(always)]
    pub fn b38(&self) -> B38_R {
        B38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B39"]
    #[inline(always)]
    pub fn b39(&self) -> B39_R {
        B39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B40"]
    #[inline(always)]
    pub fn b40(&self) -> B40_R {
        B40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B41"]
    #[inline(always)]
    pub fn b41(&self) -> B41_R {
        B41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B42"]
    #[inline(always)]
    pub fn b42(&self) -> B42_R {
        B42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B43"]
    #[inline(always)]
    pub fn b43(&self) -> B43_R {
        B43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B44"]
    #[inline(always)]
    pub fn b44(&self) -> B44_R {
        B44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B45"]
    #[inline(always)]
    pub fn b45(&self) -> B45_R {
        B45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B46"]
    #[inline(always)]
    pub fn b46(&self) -> B46_R {
        B46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B47"]
    #[inline(always)]
    pub fn b47(&self) -> B47_R {
        B47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B48"]
    #[inline(always)]
    pub fn b48(&self) -> B48_R {
        B48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B49"]
    #[inline(always)]
    pub fn b49(&self) -> B49_R {
        B49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B50"]
    #[inline(always)]
    pub fn b50(&self) -> B50_R {
        B50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B51"]
    #[inline(always)]
    pub fn b51(&self) -> B51_R {
        B51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B52"]
    #[inline(always)]
    pub fn b52(&self) -> B52_R {
        B52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B53"]
    #[inline(always)]
    pub fn b53(&self) -> B53_R {
        B53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B54"]
    #[inline(always)]
    pub fn b54(&self) -> B54_R {
        B54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B55"]
    #[inline(always)]
    pub fn b55(&self) -> B55_R {
        B55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B56"]
    #[inline(always)]
    pub fn b56(&self) -> B56_R {
        B56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B57"]
    #[inline(always)]
    pub fn b57(&self) -> B57_R {
        B57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B58"]
    #[inline(always)]
    pub fn b58(&self) -> B58_R {
        B58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B59"]
    #[inline(always)]
    pub fn b59(&self) -> B59_R {
        B59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B60"]
    #[inline(always)]
    pub fn b60(&self) -> B60_R {
        B60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B61"]
    #[inline(always)]
    pub fn b61(&self) -> B61_R {
        B61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B62"]
    #[inline(always)]
    pub fn b62(&self) -> B62_R {
        B62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B63"]
    #[inline(always)]
    pub fn b63(&self) -> B63_R {
        B63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B32"]
    #[inline(always)]
    pub fn b32(&mut self) -> B32_W {
        B32_W { w: self }
    }
    #[doc = "Bit 1 - B33"]
    #[inline(always)]
    pub fn b33(&mut self) -> B33_W {
        B33_W { w: self }
    }
    #[doc = "Bit 2 - B34"]
    #[inline(always)]
    pub fn b34(&mut self) -> B34_W {
        B34_W { w: self }
    }
    #[doc = "Bit 3 - B35"]
    #[inline(always)]
    pub fn b35(&mut self) -> B35_W {
        B35_W { w: self }
    }
    #[doc = "Bit 4 - B36"]
    #[inline(always)]
    pub fn b36(&mut self) -> B36_W {
        B36_W { w: self }
    }
    #[doc = "Bit 5 - B37"]
    #[inline(always)]
    pub fn b37(&mut self) -> B37_W {
        B37_W { w: self }
    }
    #[doc = "Bit 6 - B38"]
    #[inline(always)]
    pub fn b38(&mut self) -> B38_W {
        B38_W { w: self }
    }
    #[doc = "Bit 7 - B39"]
    #[inline(always)]
    pub fn b39(&mut self) -> B39_W {
        B39_W { w: self }
    }
    #[doc = "Bit 8 - B40"]
    #[inline(always)]
    pub fn b40(&mut self) -> B40_W {
        B40_W { w: self }
    }
    #[doc = "Bit 9 - B41"]
    #[inline(always)]
    pub fn b41(&mut self) -> B41_W {
        B41_W { w: self }
    }
    #[doc = "Bit 10 - B42"]
    #[inline(always)]
    pub fn b42(&mut self) -> B42_W {
        B42_W { w: self }
    }
    #[doc = "Bit 11 - B43"]
    #[inline(always)]
    pub fn b43(&mut self) -> B43_W {
        B43_W { w: self }
    }
    #[doc = "Bit 12 - B44"]
    #[inline(always)]
    pub fn b44(&mut self) -> B44_W {
        B44_W { w: self }
    }
    #[doc = "Bit 13 - B45"]
    #[inline(always)]
    pub fn b45(&mut self) -> B45_W {
        B45_W { w: self }
    }
    #[doc = "Bit 14 - B46"]
    #[inline(always)]
    pub fn b46(&mut self) -> B46_W {
        B46_W { w: self }
    }
    #[doc = "Bit 15 - B47"]
    #[inline(always)]
    pub fn b47(&mut self) -> B47_W {
        B47_W { w: self }
    }
    #[doc = "Bit 16 - B48"]
    #[inline(always)]
    pub fn b48(&mut self) -> B48_W {
        B48_W { w: self }
    }
    #[doc = "Bit 17 - B49"]
    #[inline(always)]
    pub fn b49(&mut self) -> B49_W {
        B49_W { w: self }
    }
    #[doc = "Bit 18 - B50"]
    #[inline(always)]
    pub fn b50(&mut self) -> B50_W {
        B50_W { w: self }
    }
    #[doc = "Bit 19 - B51"]
    #[inline(always)]
    pub fn b51(&mut self) -> B51_W {
        B51_W { w: self }
    }
    #[doc = "Bit 20 - B52"]
    #[inline(always)]
    pub fn b52(&mut self) -> B52_W {
        B52_W { w: self }
    }
    #[doc = "Bit 21 - B53"]
    #[inline(always)]
    pub fn b53(&mut self) -> B53_W {
        B53_W { w: self }
    }
    #[doc = "Bit 22 - B54"]
    #[inline(always)]
    pub fn b54(&mut self) -> B54_W {
        B54_W { w: self }
    }
    #[doc = "Bit 23 - B55"]
    #[inline(always)]
    pub fn b55(&mut self) -> B55_W {
        B55_W { w: self }
    }
    #[doc = "Bit 24 - B56"]
    #[inline(always)]
    pub fn b56(&mut self) -> B56_W {
        B56_W { w: self }
    }
    #[doc = "Bit 25 - B57"]
    #[inline(always)]
    pub fn b57(&mut self) -> B57_W {
        B57_W { w: self }
    }
    #[doc = "Bit 26 - B58"]
    #[inline(always)]
    pub fn b58(&mut self) -> B58_W {
        B58_W { w: self }
    }
    #[doc = "Bit 27 - B59"]
    #[inline(always)]
    pub fn b59(&mut self) -> B59_W {
        B59_W { w: self }
    }
    #[doc = "Bit 28 - B60"]
    #[inline(always)]
    pub fn b60(&mut self) -> B60_W {
        B60_W { w: self }
    }
    #[doc = "Bit 29 - B61"]
    #[inline(always)]
    pub fn b61(&mut self) -> B61_W {
        B61_W { w: self }
    }
    #[doc = "Bit 30 - B62"]
    #[inline(always)]
    pub fn b62(&mut self) -> B62_W {
        B62_W { w: self }
    }
    #[doc = "Bit 31 - B63"]
    #[inline(always)]
    pub fn b63(&mut self) -> B63_W {
        B63_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr1](index.html) module"]
pub struct MPCBB2_VCTR1_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr1::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr1::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR1 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
