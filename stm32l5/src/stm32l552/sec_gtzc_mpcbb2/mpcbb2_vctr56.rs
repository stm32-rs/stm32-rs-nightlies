#[doc = "Register `MPCBB2_VCTR56` reader"]
pub struct R(crate::R<MPCBB2_VCTR56_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR56_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR56_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR56_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR56` writer"]
pub struct W(crate::W<MPCBB2_VCTR56_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR56_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR56_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR56_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1792` reader - B1792"]
pub struct B1792_R(crate::FieldReader<bool, bool>);
impl B1792_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1792_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1792_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1792` writer - B1792"]
pub struct B1792_W<'a> {
    w: &'a mut W,
}
impl<'a> B1792_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1793` reader - B1793"]
pub struct B1793_R(crate::FieldReader<bool, bool>);
impl B1793_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1793_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1793_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1793` writer - B1793"]
pub struct B1793_W<'a> {
    w: &'a mut W,
}
impl<'a> B1793_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1794` reader - B1794"]
pub struct B1794_R(crate::FieldReader<bool, bool>);
impl B1794_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1794_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1794_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1794` writer - B1794"]
pub struct B1794_W<'a> {
    w: &'a mut W,
}
impl<'a> B1794_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1795` reader - B1795"]
pub struct B1795_R(crate::FieldReader<bool, bool>);
impl B1795_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1795_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1795_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1795` writer - B1795"]
pub struct B1795_W<'a> {
    w: &'a mut W,
}
impl<'a> B1795_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1796` reader - B1796"]
pub struct B1796_R(crate::FieldReader<bool, bool>);
impl B1796_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1796_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1796_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1796` writer - B1796"]
pub struct B1796_W<'a> {
    w: &'a mut W,
}
impl<'a> B1796_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1797` reader - B1797"]
pub struct B1797_R(crate::FieldReader<bool, bool>);
impl B1797_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1797_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1797_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1797` writer - B1797"]
pub struct B1797_W<'a> {
    w: &'a mut W,
}
impl<'a> B1797_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1798` reader - B1798"]
pub struct B1798_R(crate::FieldReader<bool, bool>);
impl B1798_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1798_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1798_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1798` writer - B1798"]
pub struct B1798_W<'a> {
    w: &'a mut W,
}
impl<'a> B1798_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1799` reader - B1799"]
pub struct B1799_R(crate::FieldReader<bool, bool>);
impl B1799_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1799_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1799_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1799` writer - B1799"]
pub struct B1799_W<'a> {
    w: &'a mut W,
}
impl<'a> B1799_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1800` reader - B1800"]
pub struct B1800_R(crate::FieldReader<bool, bool>);
impl B1800_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1800_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1800_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1800` writer - B1800"]
pub struct B1800_W<'a> {
    w: &'a mut W,
}
impl<'a> B1800_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1801` reader - B1801"]
pub struct B1801_R(crate::FieldReader<bool, bool>);
impl B1801_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1801_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1801_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1801` writer - B1801"]
pub struct B1801_W<'a> {
    w: &'a mut W,
}
impl<'a> B1801_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1802` reader - B1802"]
pub struct B1802_R(crate::FieldReader<bool, bool>);
impl B1802_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1802_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1802_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1802` writer - B1802"]
pub struct B1802_W<'a> {
    w: &'a mut W,
}
impl<'a> B1802_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1803` reader - B1803"]
pub struct B1803_R(crate::FieldReader<bool, bool>);
impl B1803_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1803_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1803_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1803` writer - B1803"]
pub struct B1803_W<'a> {
    w: &'a mut W,
}
impl<'a> B1803_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1804` reader - B1804"]
pub struct B1804_R(crate::FieldReader<bool, bool>);
impl B1804_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1804_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1804_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1804` writer - B1804"]
pub struct B1804_W<'a> {
    w: &'a mut W,
}
impl<'a> B1804_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1805` reader - B1805"]
pub struct B1805_R(crate::FieldReader<bool, bool>);
impl B1805_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1805_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1805_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1805` writer - B1805"]
pub struct B1805_W<'a> {
    w: &'a mut W,
}
impl<'a> B1805_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1806` reader - B1806"]
pub struct B1806_R(crate::FieldReader<bool, bool>);
impl B1806_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1806_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1806_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1806` writer - B1806"]
pub struct B1806_W<'a> {
    w: &'a mut W,
}
impl<'a> B1806_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1807` reader - B1807"]
pub struct B1807_R(crate::FieldReader<bool, bool>);
impl B1807_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1807_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1807_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1807` writer - B1807"]
pub struct B1807_W<'a> {
    w: &'a mut W,
}
impl<'a> B1807_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1808` reader - B1808"]
pub struct B1808_R(crate::FieldReader<bool, bool>);
impl B1808_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1808_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1808_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1808` writer - B1808"]
pub struct B1808_W<'a> {
    w: &'a mut W,
}
impl<'a> B1808_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1809` reader - B1809"]
pub struct B1809_R(crate::FieldReader<bool, bool>);
impl B1809_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1809_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1809_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1809` writer - B1809"]
pub struct B1809_W<'a> {
    w: &'a mut W,
}
impl<'a> B1809_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1810` reader - B1810"]
pub struct B1810_R(crate::FieldReader<bool, bool>);
impl B1810_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1810_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1810_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1810` writer - B1810"]
pub struct B1810_W<'a> {
    w: &'a mut W,
}
impl<'a> B1810_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1811` reader - B1811"]
pub struct B1811_R(crate::FieldReader<bool, bool>);
impl B1811_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1811_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1811_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1811` writer - B1811"]
pub struct B1811_W<'a> {
    w: &'a mut W,
}
impl<'a> B1811_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1812` reader - B1812"]
pub struct B1812_R(crate::FieldReader<bool, bool>);
impl B1812_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1812_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1812_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1812` writer - B1812"]
pub struct B1812_W<'a> {
    w: &'a mut W,
}
impl<'a> B1812_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1813` reader - B1813"]
pub struct B1813_R(crate::FieldReader<bool, bool>);
impl B1813_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1813_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1813_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1813` writer - B1813"]
pub struct B1813_W<'a> {
    w: &'a mut W,
}
impl<'a> B1813_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1814` reader - B1814"]
pub struct B1814_R(crate::FieldReader<bool, bool>);
impl B1814_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1814_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1814_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1814` writer - B1814"]
pub struct B1814_W<'a> {
    w: &'a mut W,
}
impl<'a> B1814_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1815` reader - B1815"]
pub struct B1815_R(crate::FieldReader<bool, bool>);
impl B1815_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1815_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1815_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1815` writer - B1815"]
pub struct B1815_W<'a> {
    w: &'a mut W,
}
impl<'a> B1815_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1816` reader - B1816"]
pub struct B1816_R(crate::FieldReader<bool, bool>);
impl B1816_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1816_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1816_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1816` writer - B1816"]
pub struct B1816_W<'a> {
    w: &'a mut W,
}
impl<'a> B1816_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1817` reader - B1817"]
pub struct B1817_R(crate::FieldReader<bool, bool>);
impl B1817_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1817_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1817_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1817` writer - B1817"]
pub struct B1817_W<'a> {
    w: &'a mut W,
}
impl<'a> B1817_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1818` reader - B1818"]
pub struct B1818_R(crate::FieldReader<bool, bool>);
impl B1818_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1818_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1818_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1818` writer - B1818"]
pub struct B1818_W<'a> {
    w: &'a mut W,
}
impl<'a> B1818_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1819` reader - B1819"]
pub struct B1819_R(crate::FieldReader<bool, bool>);
impl B1819_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1819_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1819_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1819` writer - B1819"]
pub struct B1819_W<'a> {
    w: &'a mut W,
}
impl<'a> B1819_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1820` reader - B1820"]
pub struct B1820_R(crate::FieldReader<bool, bool>);
impl B1820_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1820_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1820_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1820` writer - B1820"]
pub struct B1820_W<'a> {
    w: &'a mut W,
}
impl<'a> B1820_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1821` reader - B1821"]
pub struct B1821_R(crate::FieldReader<bool, bool>);
impl B1821_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1821_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1821_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1821` writer - B1821"]
pub struct B1821_W<'a> {
    w: &'a mut W,
}
impl<'a> B1821_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1822` reader - B1822"]
pub struct B1822_R(crate::FieldReader<bool, bool>);
impl B1822_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1822_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1822_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1822` writer - B1822"]
pub struct B1822_W<'a> {
    w: &'a mut W,
}
impl<'a> B1822_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1823` reader - B1823"]
pub struct B1823_R(crate::FieldReader<bool, bool>);
impl B1823_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1823_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1823_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1823` writer - B1823"]
pub struct B1823_W<'a> {
    w: &'a mut W,
}
impl<'a> B1823_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1792"]
    #[inline(always)]
    pub fn b1792(&self) -> B1792_R {
        B1792_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1793"]
    #[inline(always)]
    pub fn b1793(&self) -> B1793_R {
        B1793_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1794"]
    #[inline(always)]
    pub fn b1794(&self) -> B1794_R {
        B1794_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1795"]
    #[inline(always)]
    pub fn b1795(&self) -> B1795_R {
        B1795_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1796"]
    #[inline(always)]
    pub fn b1796(&self) -> B1796_R {
        B1796_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1797"]
    #[inline(always)]
    pub fn b1797(&self) -> B1797_R {
        B1797_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1798"]
    #[inline(always)]
    pub fn b1798(&self) -> B1798_R {
        B1798_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1799"]
    #[inline(always)]
    pub fn b1799(&self) -> B1799_R {
        B1799_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1800"]
    #[inline(always)]
    pub fn b1800(&self) -> B1800_R {
        B1800_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1801"]
    #[inline(always)]
    pub fn b1801(&self) -> B1801_R {
        B1801_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1802"]
    #[inline(always)]
    pub fn b1802(&self) -> B1802_R {
        B1802_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1803"]
    #[inline(always)]
    pub fn b1803(&self) -> B1803_R {
        B1803_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1804"]
    #[inline(always)]
    pub fn b1804(&self) -> B1804_R {
        B1804_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1805"]
    #[inline(always)]
    pub fn b1805(&self) -> B1805_R {
        B1805_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1806"]
    #[inline(always)]
    pub fn b1806(&self) -> B1806_R {
        B1806_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1807"]
    #[inline(always)]
    pub fn b1807(&self) -> B1807_R {
        B1807_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1808"]
    #[inline(always)]
    pub fn b1808(&self) -> B1808_R {
        B1808_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1809"]
    #[inline(always)]
    pub fn b1809(&self) -> B1809_R {
        B1809_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1810"]
    #[inline(always)]
    pub fn b1810(&self) -> B1810_R {
        B1810_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1811"]
    #[inline(always)]
    pub fn b1811(&self) -> B1811_R {
        B1811_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1812"]
    #[inline(always)]
    pub fn b1812(&self) -> B1812_R {
        B1812_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1813"]
    #[inline(always)]
    pub fn b1813(&self) -> B1813_R {
        B1813_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1814"]
    #[inline(always)]
    pub fn b1814(&self) -> B1814_R {
        B1814_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1815"]
    #[inline(always)]
    pub fn b1815(&self) -> B1815_R {
        B1815_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1816"]
    #[inline(always)]
    pub fn b1816(&self) -> B1816_R {
        B1816_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1817"]
    #[inline(always)]
    pub fn b1817(&self) -> B1817_R {
        B1817_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1818"]
    #[inline(always)]
    pub fn b1818(&self) -> B1818_R {
        B1818_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1819"]
    #[inline(always)]
    pub fn b1819(&self) -> B1819_R {
        B1819_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1820"]
    #[inline(always)]
    pub fn b1820(&self) -> B1820_R {
        B1820_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1821"]
    #[inline(always)]
    pub fn b1821(&self) -> B1821_R {
        B1821_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1822"]
    #[inline(always)]
    pub fn b1822(&self) -> B1822_R {
        B1822_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1823"]
    #[inline(always)]
    pub fn b1823(&self) -> B1823_R {
        B1823_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1792"]
    #[inline(always)]
    pub fn b1792(&mut self) -> B1792_W {
        B1792_W { w: self }
    }
    #[doc = "Bit 1 - B1793"]
    #[inline(always)]
    pub fn b1793(&mut self) -> B1793_W {
        B1793_W { w: self }
    }
    #[doc = "Bit 2 - B1794"]
    #[inline(always)]
    pub fn b1794(&mut self) -> B1794_W {
        B1794_W { w: self }
    }
    #[doc = "Bit 3 - B1795"]
    #[inline(always)]
    pub fn b1795(&mut self) -> B1795_W {
        B1795_W { w: self }
    }
    #[doc = "Bit 4 - B1796"]
    #[inline(always)]
    pub fn b1796(&mut self) -> B1796_W {
        B1796_W { w: self }
    }
    #[doc = "Bit 5 - B1797"]
    #[inline(always)]
    pub fn b1797(&mut self) -> B1797_W {
        B1797_W { w: self }
    }
    #[doc = "Bit 6 - B1798"]
    #[inline(always)]
    pub fn b1798(&mut self) -> B1798_W {
        B1798_W { w: self }
    }
    #[doc = "Bit 7 - B1799"]
    #[inline(always)]
    pub fn b1799(&mut self) -> B1799_W {
        B1799_W { w: self }
    }
    #[doc = "Bit 8 - B1800"]
    #[inline(always)]
    pub fn b1800(&mut self) -> B1800_W {
        B1800_W { w: self }
    }
    #[doc = "Bit 9 - B1801"]
    #[inline(always)]
    pub fn b1801(&mut self) -> B1801_W {
        B1801_W { w: self }
    }
    #[doc = "Bit 10 - B1802"]
    #[inline(always)]
    pub fn b1802(&mut self) -> B1802_W {
        B1802_W { w: self }
    }
    #[doc = "Bit 11 - B1803"]
    #[inline(always)]
    pub fn b1803(&mut self) -> B1803_W {
        B1803_W { w: self }
    }
    #[doc = "Bit 12 - B1804"]
    #[inline(always)]
    pub fn b1804(&mut self) -> B1804_W {
        B1804_W { w: self }
    }
    #[doc = "Bit 13 - B1805"]
    #[inline(always)]
    pub fn b1805(&mut self) -> B1805_W {
        B1805_W { w: self }
    }
    #[doc = "Bit 14 - B1806"]
    #[inline(always)]
    pub fn b1806(&mut self) -> B1806_W {
        B1806_W { w: self }
    }
    #[doc = "Bit 15 - B1807"]
    #[inline(always)]
    pub fn b1807(&mut self) -> B1807_W {
        B1807_W { w: self }
    }
    #[doc = "Bit 16 - B1808"]
    #[inline(always)]
    pub fn b1808(&mut self) -> B1808_W {
        B1808_W { w: self }
    }
    #[doc = "Bit 17 - B1809"]
    #[inline(always)]
    pub fn b1809(&mut self) -> B1809_W {
        B1809_W { w: self }
    }
    #[doc = "Bit 18 - B1810"]
    #[inline(always)]
    pub fn b1810(&mut self) -> B1810_W {
        B1810_W { w: self }
    }
    #[doc = "Bit 19 - B1811"]
    #[inline(always)]
    pub fn b1811(&mut self) -> B1811_W {
        B1811_W { w: self }
    }
    #[doc = "Bit 20 - B1812"]
    #[inline(always)]
    pub fn b1812(&mut self) -> B1812_W {
        B1812_W { w: self }
    }
    #[doc = "Bit 21 - B1813"]
    #[inline(always)]
    pub fn b1813(&mut self) -> B1813_W {
        B1813_W { w: self }
    }
    #[doc = "Bit 22 - B1814"]
    #[inline(always)]
    pub fn b1814(&mut self) -> B1814_W {
        B1814_W { w: self }
    }
    #[doc = "Bit 23 - B1815"]
    #[inline(always)]
    pub fn b1815(&mut self) -> B1815_W {
        B1815_W { w: self }
    }
    #[doc = "Bit 24 - B1816"]
    #[inline(always)]
    pub fn b1816(&mut self) -> B1816_W {
        B1816_W { w: self }
    }
    #[doc = "Bit 25 - B1817"]
    #[inline(always)]
    pub fn b1817(&mut self) -> B1817_W {
        B1817_W { w: self }
    }
    #[doc = "Bit 26 - B1818"]
    #[inline(always)]
    pub fn b1818(&mut self) -> B1818_W {
        B1818_W { w: self }
    }
    #[doc = "Bit 27 - B1819"]
    #[inline(always)]
    pub fn b1819(&mut self) -> B1819_W {
        B1819_W { w: self }
    }
    #[doc = "Bit 28 - B1820"]
    #[inline(always)]
    pub fn b1820(&mut self) -> B1820_W {
        B1820_W { w: self }
    }
    #[doc = "Bit 29 - B1821"]
    #[inline(always)]
    pub fn b1821(&mut self) -> B1821_W {
        B1821_W { w: self }
    }
    #[doc = "Bit 30 - B1822"]
    #[inline(always)]
    pub fn b1822(&mut self) -> B1822_W {
        B1822_W { w: self }
    }
    #[doc = "Bit 31 - B1823"]
    #[inline(always)]
    pub fn b1823(&mut self) -> B1823_W {
        B1823_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr56](index.html) module"]
pub struct MPCBB2_VCTR56_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR56_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr56::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR56_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr56::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR56_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR56 to value 0"]
impl crate::Resettable for MPCBB2_VCTR56_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
