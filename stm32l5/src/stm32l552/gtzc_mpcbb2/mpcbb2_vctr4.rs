#[doc = "Register `MPCBB2_VCTR4` reader"]
pub struct R(crate::R<MPCBB2_VCTR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR4` writer"]
pub struct W(crate::W<MPCBB2_VCTR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR4_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B128` reader - B128"]
pub struct B128_R(crate::FieldReader<bool, bool>);
impl B128_R {
    pub(crate) fn new(bits: bool) -> Self {
        B128_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B128_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B128` writer - B128"]
pub struct B128_W<'a> {
    w: &'a mut W,
}
impl<'a> B128_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B129` reader - B129"]
pub struct B129_R(crate::FieldReader<bool, bool>);
impl B129_R {
    pub(crate) fn new(bits: bool) -> Self {
        B129_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B129_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B129` writer - B129"]
pub struct B129_W<'a> {
    w: &'a mut W,
}
impl<'a> B129_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B130` reader - B130"]
pub struct B130_R(crate::FieldReader<bool, bool>);
impl B130_R {
    pub(crate) fn new(bits: bool) -> Self {
        B130_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B130_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B130` writer - B130"]
pub struct B130_W<'a> {
    w: &'a mut W,
}
impl<'a> B130_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B131` reader - B131"]
pub struct B131_R(crate::FieldReader<bool, bool>);
impl B131_R {
    pub(crate) fn new(bits: bool) -> Self {
        B131_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B131_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B131` writer - B131"]
pub struct B131_W<'a> {
    w: &'a mut W,
}
impl<'a> B131_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B132` reader - B132"]
pub struct B132_R(crate::FieldReader<bool, bool>);
impl B132_R {
    pub(crate) fn new(bits: bool) -> Self {
        B132_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B132_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B132` writer - B132"]
pub struct B132_W<'a> {
    w: &'a mut W,
}
impl<'a> B132_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B133` reader - B133"]
pub struct B133_R(crate::FieldReader<bool, bool>);
impl B133_R {
    pub(crate) fn new(bits: bool) -> Self {
        B133_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B133_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B133` writer - B133"]
pub struct B133_W<'a> {
    w: &'a mut W,
}
impl<'a> B133_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B134` reader - B134"]
pub struct B134_R(crate::FieldReader<bool, bool>);
impl B134_R {
    pub(crate) fn new(bits: bool) -> Self {
        B134_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B134_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B134` writer - B134"]
pub struct B134_W<'a> {
    w: &'a mut W,
}
impl<'a> B134_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B135` reader - B135"]
pub struct B135_R(crate::FieldReader<bool, bool>);
impl B135_R {
    pub(crate) fn new(bits: bool) -> Self {
        B135_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B135_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B135` writer - B135"]
pub struct B135_W<'a> {
    w: &'a mut W,
}
impl<'a> B135_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B136` reader - B136"]
pub struct B136_R(crate::FieldReader<bool, bool>);
impl B136_R {
    pub(crate) fn new(bits: bool) -> Self {
        B136_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B136_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B136` writer - B136"]
pub struct B136_W<'a> {
    w: &'a mut W,
}
impl<'a> B136_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B137` reader - B137"]
pub struct B137_R(crate::FieldReader<bool, bool>);
impl B137_R {
    pub(crate) fn new(bits: bool) -> Self {
        B137_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B137_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B137` writer - B137"]
pub struct B137_W<'a> {
    w: &'a mut W,
}
impl<'a> B137_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B138` reader - B138"]
pub struct B138_R(crate::FieldReader<bool, bool>);
impl B138_R {
    pub(crate) fn new(bits: bool) -> Self {
        B138_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B138_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B138` writer - B138"]
pub struct B138_W<'a> {
    w: &'a mut W,
}
impl<'a> B138_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B139` reader - B139"]
pub struct B139_R(crate::FieldReader<bool, bool>);
impl B139_R {
    pub(crate) fn new(bits: bool) -> Self {
        B139_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B139_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B139` writer - B139"]
pub struct B139_W<'a> {
    w: &'a mut W,
}
impl<'a> B139_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B140` reader - B140"]
pub struct B140_R(crate::FieldReader<bool, bool>);
impl B140_R {
    pub(crate) fn new(bits: bool) -> Self {
        B140_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B140_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B140` writer - B140"]
pub struct B140_W<'a> {
    w: &'a mut W,
}
impl<'a> B140_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B141` reader - B141"]
pub struct B141_R(crate::FieldReader<bool, bool>);
impl B141_R {
    pub(crate) fn new(bits: bool) -> Self {
        B141_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B141_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B141` writer - B141"]
pub struct B141_W<'a> {
    w: &'a mut W,
}
impl<'a> B141_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B142` reader - B142"]
pub struct B142_R(crate::FieldReader<bool, bool>);
impl B142_R {
    pub(crate) fn new(bits: bool) -> Self {
        B142_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B142_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B142` writer - B142"]
pub struct B142_W<'a> {
    w: &'a mut W,
}
impl<'a> B142_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B143` reader - B143"]
pub struct B143_R(crate::FieldReader<bool, bool>);
impl B143_R {
    pub(crate) fn new(bits: bool) -> Self {
        B143_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B143_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B143` writer - B143"]
pub struct B143_W<'a> {
    w: &'a mut W,
}
impl<'a> B143_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B144` reader - B144"]
pub struct B144_R(crate::FieldReader<bool, bool>);
impl B144_R {
    pub(crate) fn new(bits: bool) -> Self {
        B144_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B144_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B144` writer - B144"]
pub struct B144_W<'a> {
    w: &'a mut W,
}
impl<'a> B144_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B145` reader - B145"]
pub struct B145_R(crate::FieldReader<bool, bool>);
impl B145_R {
    pub(crate) fn new(bits: bool) -> Self {
        B145_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B145_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B145` writer - B145"]
pub struct B145_W<'a> {
    w: &'a mut W,
}
impl<'a> B145_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B146` reader - B146"]
pub struct B146_R(crate::FieldReader<bool, bool>);
impl B146_R {
    pub(crate) fn new(bits: bool) -> Self {
        B146_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B146_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B146` writer - B146"]
pub struct B146_W<'a> {
    w: &'a mut W,
}
impl<'a> B146_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B147` reader - B147"]
pub struct B147_R(crate::FieldReader<bool, bool>);
impl B147_R {
    pub(crate) fn new(bits: bool) -> Self {
        B147_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B147_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B147` writer - B147"]
pub struct B147_W<'a> {
    w: &'a mut W,
}
impl<'a> B147_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B148` reader - B148"]
pub struct B148_R(crate::FieldReader<bool, bool>);
impl B148_R {
    pub(crate) fn new(bits: bool) -> Self {
        B148_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B148_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B148` writer - B148"]
pub struct B148_W<'a> {
    w: &'a mut W,
}
impl<'a> B148_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B149` reader - B149"]
pub struct B149_R(crate::FieldReader<bool, bool>);
impl B149_R {
    pub(crate) fn new(bits: bool) -> Self {
        B149_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B149_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B149` writer - B149"]
pub struct B149_W<'a> {
    w: &'a mut W,
}
impl<'a> B149_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B150` reader - B150"]
pub struct B150_R(crate::FieldReader<bool, bool>);
impl B150_R {
    pub(crate) fn new(bits: bool) -> Self {
        B150_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B150_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B150` writer - B150"]
pub struct B150_W<'a> {
    w: &'a mut W,
}
impl<'a> B150_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B151` reader - B151"]
pub struct B151_R(crate::FieldReader<bool, bool>);
impl B151_R {
    pub(crate) fn new(bits: bool) -> Self {
        B151_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B151_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B151` writer - B151"]
pub struct B151_W<'a> {
    w: &'a mut W,
}
impl<'a> B151_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B152` reader - B152"]
pub struct B152_R(crate::FieldReader<bool, bool>);
impl B152_R {
    pub(crate) fn new(bits: bool) -> Self {
        B152_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B152_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B152` writer - B152"]
pub struct B152_W<'a> {
    w: &'a mut W,
}
impl<'a> B152_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B153` reader - B153"]
pub struct B153_R(crate::FieldReader<bool, bool>);
impl B153_R {
    pub(crate) fn new(bits: bool) -> Self {
        B153_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B153_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B153` writer - B153"]
pub struct B153_W<'a> {
    w: &'a mut W,
}
impl<'a> B153_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B154` reader - B154"]
pub struct B154_R(crate::FieldReader<bool, bool>);
impl B154_R {
    pub(crate) fn new(bits: bool) -> Self {
        B154_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B154_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B154` writer - B154"]
pub struct B154_W<'a> {
    w: &'a mut W,
}
impl<'a> B154_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B155` reader - B155"]
pub struct B155_R(crate::FieldReader<bool, bool>);
impl B155_R {
    pub(crate) fn new(bits: bool) -> Self {
        B155_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B155_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B155` writer - B155"]
pub struct B155_W<'a> {
    w: &'a mut W,
}
impl<'a> B155_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B156` reader - B156"]
pub struct B156_R(crate::FieldReader<bool, bool>);
impl B156_R {
    pub(crate) fn new(bits: bool) -> Self {
        B156_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B156_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B156` writer - B156"]
pub struct B156_W<'a> {
    w: &'a mut W,
}
impl<'a> B156_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B157` reader - B157"]
pub struct B157_R(crate::FieldReader<bool, bool>);
impl B157_R {
    pub(crate) fn new(bits: bool) -> Self {
        B157_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B157_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B157` writer - B157"]
pub struct B157_W<'a> {
    w: &'a mut W,
}
impl<'a> B157_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B158` reader - B158"]
pub struct B158_R(crate::FieldReader<bool, bool>);
impl B158_R {
    pub(crate) fn new(bits: bool) -> Self {
        B158_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B158_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B158` writer - B158"]
pub struct B158_W<'a> {
    w: &'a mut W,
}
impl<'a> B158_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B159` reader - B159"]
pub struct B159_R(crate::FieldReader<bool, bool>);
impl B159_R {
    pub(crate) fn new(bits: bool) -> Self {
        B159_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B159_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B159` writer - B159"]
pub struct B159_W<'a> {
    w: &'a mut W,
}
impl<'a> B159_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B128"]
    #[inline(always)]
    pub fn b128(&self) -> B128_R {
        B128_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B129"]
    #[inline(always)]
    pub fn b129(&self) -> B129_R {
        B129_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B130"]
    #[inline(always)]
    pub fn b130(&self) -> B130_R {
        B130_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B131"]
    #[inline(always)]
    pub fn b131(&self) -> B131_R {
        B131_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B132"]
    #[inline(always)]
    pub fn b132(&self) -> B132_R {
        B132_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B133"]
    #[inline(always)]
    pub fn b133(&self) -> B133_R {
        B133_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B134"]
    #[inline(always)]
    pub fn b134(&self) -> B134_R {
        B134_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B135"]
    #[inline(always)]
    pub fn b135(&self) -> B135_R {
        B135_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B136"]
    #[inline(always)]
    pub fn b136(&self) -> B136_R {
        B136_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B137"]
    #[inline(always)]
    pub fn b137(&self) -> B137_R {
        B137_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B138"]
    #[inline(always)]
    pub fn b138(&self) -> B138_R {
        B138_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B139"]
    #[inline(always)]
    pub fn b139(&self) -> B139_R {
        B139_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B140"]
    #[inline(always)]
    pub fn b140(&self) -> B140_R {
        B140_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B141"]
    #[inline(always)]
    pub fn b141(&self) -> B141_R {
        B141_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B142"]
    #[inline(always)]
    pub fn b142(&self) -> B142_R {
        B142_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B143"]
    #[inline(always)]
    pub fn b143(&self) -> B143_R {
        B143_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B144"]
    #[inline(always)]
    pub fn b144(&self) -> B144_R {
        B144_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B145"]
    #[inline(always)]
    pub fn b145(&self) -> B145_R {
        B145_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B146"]
    #[inline(always)]
    pub fn b146(&self) -> B146_R {
        B146_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B147"]
    #[inline(always)]
    pub fn b147(&self) -> B147_R {
        B147_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B148"]
    #[inline(always)]
    pub fn b148(&self) -> B148_R {
        B148_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B149"]
    #[inline(always)]
    pub fn b149(&self) -> B149_R {
        B149_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B150"]
    #[inline(always)]
    pub fn b150(&self) -> B150_R {
        B150_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B151"]
    #[inline(always)]
    pub fn b151(&self) -> B151_R {
        B151_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B152"]
    #[inline(always)]
    pub fn b152(&self) -> B152_R {
        B152_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B153"]
    #[inline(always)]
    pub fn b153(&self) -> B153_R {
        B153_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B154"]
    #[inline(always)]
    pub fn b154(&self) -> B154_R {
        B154_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B155"]
    #[inline(always)]
    pub fn b155(&self) -> B155_R {
        B155_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B156"]
    #[inline(always)]
    pub fn b156(&self) -> B156_R {
        B156_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B157"]
    #[inline(always)]
    pub fn b157(&self) -> B157_R {
        B157_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B158"]
    #[inline(always)]
    pub fn b158(&self) -> B158_R {
        B158_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B159"]
    #[inline(always)]
    pub fn b159(&self) -> B159_R {
        B159_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B128"]
    #[inline(always)]
    pub fn b128(&mut self) -> B128_W {
        B128_W { w: self }
    }
    #[doc = "Bit 1 - B129"]
    #[inline(always)]
    pub fn b129(&mut self) -> B129_W {
        B129_W { w: self }
    }
    #[doc = "Bit 2 - B130"]
    #[inline(always)]
    pub fn b130(&mut self) -> B130_W {
        B130_W { w: self }
    }
    #[doc = "Bit 3 - B131"]
    #[inline(always)]
    pub fn b131(&mut self) -> B131_W {
        B131_W { w: self }
    }
    #[doc = "Bit 4 - B132"]
    #[inline(always)]
    pub fn b132(&mut self) -> B132_W {
        B132_W { w: self }
    }
    #[doc = "Bit 5 - B133"]
    #[inline(always)]
    pub fn b133(&mut self) -> B133_W {
        B133_W { w: self }
    }
    #[doc = "Bit 6 - B134"]
    #[inline(always)]
    pub fn b134(&mut self) -> B134_W {
        B134_W { w: self }
    }
    #[doc = "Bit 7 - B135"]
    #[inline(always)]
    pub fn b135(&mut self) -> B135_W {
        B135_W { w: self }
    }
    #[doc = "Bit 8 - B136"]
    #[inline(always)]
    pub fn b136(&mut self) -> B136_W {
        B136_W { w: self }
    }
    #[doc = "Bit 9 - B137"]
    #[inline(always)]
    pub fn b137(&mut self) -> B137_W {
        B137_W { w: self }
    }
    #[doc = "Bit 10 - B138"]
    #[inline(always)]
    pub fn b138(&mut self) -> B138_W {
        B138_W { w: self }
    }
    #[doc = "Bit 11 - B139"]
    #[inline(always)]
    pub fn b139(&mut self) -> B139_W {
        B139_W { w: self }
    }
    #[doc = "Bit 12 - B140"]
    #[inline(always)]
    pub fn b140(&mut self) -> B140_W {
        B140_W { w: self }
    }
    #[doc = "Bit 13 - B141"]
    #[inline(always)]
    pub fn b141(&mut self) -> B141_W {
        B141_W { w: self }
    }
    #[doc = "Bit 14 - B142"]
    #[inline(always)]
    pub fn b142(&mut self) -> B142_W {
        B142_W { w: self }
    }
    #[doc = "Bit 15 - B143"]
    #[inline(always)]
    pub fn b143(&mut self) -> B143_W {
        B143_W { w: self }
    }
    #[doc = "Bit 16 - B144"]
    #[inline(always)]
    pub fn b144(&mut self) -> B144_W {
        B144_W { w: self }
    }
    #[doc = "Bit 17 - B145"]
    #[inline(always)]
    pub fn b145(&mut self) -> B145_W {
        B145_W { w: self }
    }
    #[doc = "Bit 18 - B146"]
    #[inline(always)]
    pub fn b146(&mut self) -> B146_W {
        B146_W { w: self }
    }
    #[doc = "Bit 19 - B147"]
    #[inline(always)]
    pub fn b147(&mut self) -> B147_W {
        B147_W { w: self }
    }
    #[doc = "Bit 20 - B148"]
    #[inline(always)]
    pub fn b148(&mut self) -> B148_W {
        B148_W { w: self }
    }
    #[doc = "Bit 21 - B149"]
    #[inline(always)]
    pub fn b149(&mut self) -> B149_W {
        B149_W { w: self }
    }
    #[doc = "Bit 22 - B150"]
    #[inline(always)]
    pub fn b150(&mut self) -> B150_W {
        B150_W { w: self }
    }
    #[doc = "Bit 23 - B151"]
    #[inline(always)]
    pub fn b151(&mut self) -> B151_W {
        B151_W { w: self }
    }
    #[doc = "Bit 24 - B152"]
    #[inline(always)]
    pub fn b152(&mut self) -> B152_W {
        B152_W { w: self }
    }
    #[doc = "Bit 25 - B153"]
    #[inline(always)]
    pub fn b153(&mut self) -> B153_W {
        B153_W { w: self }
    }
    #[doc = "Bit 26 - B154"]
    #[inline(always)]
    pub fn b154(&mut self) -> B154_W {
        B154_W { w: self }
    }
    #[doc = "Bit 27 - B155"]
    #[inline(always)]
    pub fn b155(&mut self) -> B155_W {
        B155_W { w: self }
    }
    #[doc = "Bit 28 - B156"]
    #[inline(always)]
    pub fn b156(&mut self) -> B156_W {
        B156_W { w: self }
    }
    #[doc = "Bit 29 - B157"]
    #[inline(always)]
    pub fn b157(&mut self) -> B157_W {
        B157_W { w: self }
    }
    #[doc = "Bit 30 - B158"]
    #[inline(always)]
    pub fn b158(&mut self) -> B158_W {
        B158_W { w: self }
    }
    #[doc = "Bit 31 - B159"]
    #[inline(always)]
    pub fn b159(&mut self) -> B159_W {
        B159_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr4](index.html) module"]
pub struct MPCBB2_VCTR4_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr4::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr4::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR4 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
