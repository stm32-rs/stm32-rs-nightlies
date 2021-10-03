#[doc = "Register `MPCBB1_VCTR55` reader"]
pub struct R(crate::R<MPCBB1_VCTR55_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR55_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR55_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR55_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR55` writer"]
pub struct W(crate::W<MPCBB1_VCTR55_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR55_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR55_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR55_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1760` reader - B1760"]
pub struct B1760_R(crate::FieldReader<bool, bool>);
impl B1760_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1760_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1760_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1760` writer - B1760"]
pub struct B1760_W<'a> {
    w: &'a mut W,
}
impl<'a> B1760_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1761` reader - B1761"]
pub struct B1761_R(crate::FieldReader<bool, bool>);
impl B1761_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1761_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1761_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1761` writer - B1761"]
pub struct B1761_W<'a> {
    w: &'a mut W,
}
impl<'a> B1761_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1762` reader - B1762"]
pub struct B1762_R(crate::FieldReader<bool, bool>);
impl B1762_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1762_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1762_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1762` writer - B1762"]
pub struct B1762_W<'a> {
    w: &'a mut W,
}
impl<'a> B1762_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1763` reader - B1763"]
pub struct B1763_R(crate::FieldReader<bool, bool>);
impl B1763_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1763_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1763_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1763` writer - B1763"]
pub struct B1763_W<'a> {
    w: &'a mut W,
}
impl<'a> B1763_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1764` reader - B1764"]
pub struct B1764_R(crate::FieldReader<bool, bool>);
impl B1764_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1764_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1764_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1764` writer - B1764"]
pub struct B1764_W<'a> {
    w: &'a mut W,
}
impl<'a> B1764_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1765` reader - B1765"]
pub struct B1765_R(crate::FieldReader<bool, bool>);
impl B1765_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1765_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1765_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1765` writer - B1765"]
pub struct B1765_W<'a> {
    w: &'a mut W,
}
impl<'a> B1765_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1766` reader - B1766"]
pub struct B1766_R(crate::FieldReader<bool, bool>);
impl B1766_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1766_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1766_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1766` writer - B1766"]
pub struct B1766_W<'a> {
    w: &'a mut W,
}
impl<'a> B1766_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1767` reader - B1767"]
pub struct B1767_R(crate::FieldReader<bool, bool>);
impl B1767_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1767_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1767_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1767` writer - B1767"]
pub struct B1767_W<'a> {
    w: &'a mut W,
}
impl<'a> B1767_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1768` reader - B1768"]
pub struct B1768_R(crate::FieldReader<bool, bool>);
impl B1768_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1768_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1768_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1768` writer - B1768"]
pub struct B1768_W<'a> {
    w: &'a mut W,
}
impl<'a> B1768_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1769` reader - B1769"]
pub struct B1769_R(crate::FieldReader<bool, bool>);
impl B1769_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1769_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1769_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1769` writer - B1769"]
pub struct B1769_W<'a> {
    w: &'a mut W,
}
impl<'a> B1769_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1770` reader - B1770"]
pub struct B1770_R(crate::FieldReader<bool, bool>);
impl B1770_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1770_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1770_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1770` writer - B1770"]
pub struct B1770_W<'a> {
    w: &'a mut W,
}
impl<'a> B1770_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1771` reader - B1771"]
pub struct B1771_R(crate::FieldReader<bool, bool>);
impl B1771_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1771_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1771_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1771` writer - B1771"]
pub struct B1771_W<'a> {
    w: &'a mut W,
}
impl<'a> B1771_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1772` reader - B1772"]
pub struct B1772_R(crate::FieldReader<bool, bool>);
impl B1772_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1772_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1772_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1772` writer - B1772"]
pub struct B1772_W<'a> {
    w: &'a mut W,
}
impl<'a> B1772_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1773` reader - B1773"]
pub struct B1773_R(crate::FieldReader<bool, bool>);
impl B1773_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1773_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1773_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1773` writer - B1773"]
pub struct B1773_W<'a> {
    w: &'a mut W,
}
impl<'a> B1773_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1774` reader - B1774"]
pub struct B1774_R(crate::FieldReader<bool, bool>);
impl B1774_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1774_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1774_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1774` writer - B1774"]
pub struct B1774_W<'a> {
    w: &'a mut W,
}
impl<'a> B1774_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1775` reader - B1775"]
pub struct B1775_R(crate::FieldReader<bool, bool>);
impl B1775_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1775_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1775_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1775` writer - B1775"]
pub struct B1775_W<'a> {
    w: &'a mut W,
}
impl<'a> B1775_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1776` reader - B1776"]
pub struct B1776_R(crate::FieldReader<bool, bool>);
impl B1776_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1776_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1776_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1776` writer - B1776"]
pub struct B1776_W<'a> {
    w: &'a mut W,
}
impl<'a> B1776_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1777` reader - B1777"]
pub struct B1777_R(crate::FieldReader<bool, bool>);
impl B1777_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1777_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1777_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1777` writer - B1777"]
pub struct B1777_W<'a> {
    w: &'a mut W,
}
impl<'a> B1777_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1778` reader - B1778"]
pub struct B1778_R(crate::FieldReader<bool, bool>);
impl B1778_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1778_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1778_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1778` writer - B1778"]
pub struct B1778_W<'a> {
    w: &'a mut W,
}
impl<'a> B1778_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1779` reader - B1779"]
pub struct B1779_R(crate::FieldReader<bool, bool>);
impl B1779_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1779_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1779_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1779` writer - B1779"]
pub struct B1779_W<'a> {
    w: &'a mut W,
}
impl<'a> B1779_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1780` reader - B1780"]
pub struct B1780_R(crate::FieldReader<bool, bool>);
impl B1780_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1780_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1780_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1780` writer - B1780"]
pub struct B1780_W<'a> {
    w: &'a mut W,
}
impl<'a> B1780_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1781` reader - B1781"]
pub struct B1781_R(crate::FieldReader<bool, bool>);
impl B1781_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1781_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1781_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1781` writer - B1781"]
pub struct B1781_W<'a> {
    w: &'a mut W,
}
impl<'a> B1781_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1782` reader - B1782"]
pub struct B1782_R(crate::FieldReader<bool, bool>);
impl B1782_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1782_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1782_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1782` writer - B1782"]
pub struct B1782_W<'a> {
    w: &'a mut W,
}
impl<'a> B1782_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1783` reader - B1783"]
pub struct B1783_R(crate::FieldReader<bool, bool>);
impl B1783_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1783_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1783_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1783` writer - B1783"]
pub struct B1783_W<'a> {
    w: &'a mut W,
}
impl<'a> B1783_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1784` reader - B1784"]
pub struct B1784_R(crate::FieldReader<bool, bool>);
impl B1784_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1784_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1784_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1784` writer - B1784"]
pub struct B1784_W<'a> {
    w: &'a mut W,
}
impl<'a> B1784_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1785` reader - B1785"]
pub struct B1785_R(crate::FieldReader<bool, bool>);
impl B1785_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1785_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1785_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1785` writer - B1785"]
pub struct B1785_W<'a> {
    w: &'a mut W,
}
impl<'a> B1785_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1786` reader - B1786"]
pub struct B1786_R(crate::FieldReader<bool, bool>);
impl B1786_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1786_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1786_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1786` writer - B1786"]
pub struct B1786_W<'a> {
    w: &'a mut W,
}
impl<'a> B1786_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1787` reader - B1787"]
pub struct B1787_R(crate::FieldReader<bool, bool>);
impl B1787_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1787_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1787_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1787` writer - B1787"]
pub struct B1787_W<'a> {
    w: &'a mut W,
}
impl<'a> B1787_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1788` reader - B1788"]
pub struct B1788_R(crate::FieldReader<bool, bool>);
impl B1788_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1788_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1788_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1788` writer - B1788"]
pub struct B1788_W<'a> {
    w: &'a mut W,
}
impl<'a> B1788_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1789` reader - B1789"]
pub struct B1789_R(crate::FieldReader<bool, bool>);
impl B1789_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1789_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1789_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1789` writer - B1789"]
pub struct B1789_W<'a> {
    w: &'a mut W,
}
impl<'a> B1789_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1790` reader - B1790"]
pub struct B1790_R(crate::FieldReader<bool, bool>);
impl B1790_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1790_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1790_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1790` writer - B1790"]
pub struct B1790_W<'a> {
    w: &'a mut W,
}
impl<'a> B1790_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1791` reader - B1791"]
pub struct B1791_R(crate::FieldReader<bool, bool>);
impl B1791_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1791_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1791_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1791` writer - B1791"]
pub struct B1791_W<'a> {
    w: &'a mut W,
}
impl<'a> B1791_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1760"]
    #[inline(always)]
    pub fn b1760(&self) -> B1760_R {
        B1760_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1761"]
    #[inline(always)]
    pub fn b1761(&self) -> B1761_R {
        B1761_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1762"]
    #[inline(always)]
    pub fn b1762(&self) -> B1762_R {
        B1762_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1763"]
    #[inline(always)]
    pub fn b1763(&self) -> B1763_R {
        B1763_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1764"]
    #[inline(always)]
    pub fn b1764(&self) -> B1764_R {
        B1764_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1765"]
    #[inline(always)]
    pub fn b1765(&self) -> B1765_R {
        B1765_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1766"]
    #[inline(always)]
    pub fn b1766(&self) -> B1766_R {
        B1766_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1767"]
    #[inline(always)]
    pub fn b1767(&self) -> B1767_R {
        B1767_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1768"]
    #[inline(always)]
    pub fn b1768(&self) -> B1768_R {
        B1768_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1769"]
    #[inline(always)]
    pub fn b1769(&self) -> B1769_R {
        B1769_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1770"]
    #[inline(always)]
    pub fn b1770(&self) -> B1770_R {
        B1770_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1771"]
    #[inline(always)]
    pub fn b1771(&self) -> B1771_R {
        B1771_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1772"]
    #[inline(always)]
    pub fn b1772(&self) -> B1772_R {
        B1772_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1773"]
    #[inline(always)]
    pub fn b1773(&self) -> B1773_R {
        B1773_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1774"]
    #[inline(always)]
    pub fn b1774(&self) -> B1774_R {
        B1774_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1775"]
    #[inline(always)]
    pub fn b1775(&self) -> B1775_R {
        B1775_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1776"]
    #[inline(always)]
    pub fn b1776(&self) -> B1776_R {
        B1776_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1777"]
    #[inline(always)]
    pub fn b1777(&self) -> B1777_R {
        B1777_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1778"]
    #[inline(always)]
    pub fn b1778(&self) -> B1778_R {
        B1778_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1779"]
    #[inline(always)]
    pub fn b1779(&self) -> B1779_R {
        B1779_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1780"]
    #[inline(always)]
    pub fn b1780(&self) -> B1780_R {
        B1780_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1781"]
    #[inline(always)]
    pub fn b1781(&self) -> B1781_R {
        B1781_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1782"]
    #[inline(always)]
    pub fn b1782(&self) -> B1782_R {
        B1782_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1783"]
    #[inline(always)]
    pub fn b1783(&self) -> B1783_R {
        B1783_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1784"]
    #[inline(always)]
    pub fn b1784(&self) -> B1784_R {
        B1784_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1785"]
    #[inline(always)]
    pub fn b1785(&self) -> B1785_R {
        B1785_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1786"]
    #[inline(always)]
    pub fn b1786(&self) -> B1786_R {
        B1786_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1787"]
    #[inline(always)]
    pub fn b1787(&self) -> B1787_R {
        B1787_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1788"]
    #[inline(always)]
    pub fn b1788(&self) -> B1788_R {
        B1788_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1789"]
    #[inline(always)]
    pub fn b1789(&self) -> B1789_R {
        B1789_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1790"]
    #[inline(always)]
    pub fn b1790(&self) -> B1790_R {
        B1790_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1791"]
    #[inline(always)]
    pub fn b1791(&self) -> B1791_R {
        B1791_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1760"]
    #[inline(always)]
    pub fn b1760(&mut self) -> B1760_W {
        B1760_W { w: self }
    }
    #[doc = "Bit 1 - B1761"]
    #[inline(always)]
    pub fn b1761(&mut self) -> B1761_W {
        B1761_W { w: self }
    }
    #[doc = "Bit 2 - B1762"]
    #[inline(always)]
    pub fn b1762(&mut self) -> B1762_W {
        B1762_W { w: self }
    }
    #[doc = "Bit 3 - B1763"]
    #[inline(always)]
    pub fn b1763(&mut self) -> B1763_W {
        B1763_W { w: self }
    }
    #[doc = "Bit 4 - B1764"]
    #[inline(always)]
    pub fn b1764(&mut self) -> B1764_W {
        B1764_W { w: self }
    }
    #[doc = "Bit 5 - B1765"]
    #[inline(always)]
    pub fn b1765(&mut self) -> B1765_W {
        B1765_W { w: self }
    }
    #[doc = "Bit 6 - B1766"]
    #[inline(always)]
    pub fn b1766(&mut self) -> B1766_W {
        B1766_W { w: self }
    }
    #[doc = "Bit 7 - B1767"]
    #[inline(always)]
    pub fn b1767(&mut self) -> B1767_W {
        B1767_W { w: self }
    }
    #[doc = "Bit 8 - B1768"]
    #[inline(always)]
    pub fn b1768(&mut self) -> B1768_W {
        B1768_W { w: self }
    }
    #[doc = "Bit 9 - B1769"]
    #[inline(always)]
    pub fn b1769(&mut self) -> B1769_W {
        B1769_W { w: self }
    }
    #[doc = "Bit 10 - B1770"]
    #[inline(always)]
    pub fn b1770(&mut self) -> B1770_W {
        B1770_W { w: self }
    }
    #[doc = "Bit 11 - B1771"]
    #[inline(always)]
    pub fn b1771(&mut self) -> B1771_W {
        B1771_W { w: self }
    }
    #[doc = "Bit 12 - B1772"]
    #[inline(always)]
    pub fn b1772(&mut self) -> B1772_W {
        B1772_W { w: self }
    }
    #[doc = "Bit 13 - B1773"]
    #[inline(always)]
    pub fn b1773(&mut self) -> B1773_W {
        B1773_W { w: self }
    }
    #[doc = "Bit 14 - B1774"]
    #[inline(always)]
    pub fn b1774(&mut self) -> B1774_W {
        B1774_W { w: self }
    }
    #[doc = "Bit 15 - B1775"]
    #[inline(always)]
    pub fn b1775(&mut self) -> B1775_W {
        B1775_W { w: self }
    }
    #[doc = "Bit 16 - B1776"]
    #[inline(always)]
    pub fn b1776(&mut self) -> B1776_W {
        B1776_W { w: self }
    }
    #[doc = "Bit 17 - B1777"]
    #[inline(always)]
    pub fn b1777(&mut self) -> B1777_W {
        B1777_W { w: self }
    }
    #[doc = "Bit 18 - B1778"]
    #[inline(always)]
    pub fn b1778(&mut self) -> B1778_W {
        B1778_W { w: self }
    }
    #[doc = "Bit 19 - B1779"]
    #[inline(always)]
    pub fn b1779(&mut self) -> B1779_W {
        B1779_W { w: self }
    }
    #[doc = "Bit 20 - B1780"]
    #[inline(always)]
    pub fn b1780(&mut self) -> B1780_W {
        B1780_W { w: self }
    }
    #[doc = "Bit 21 - B1781"]
    #[inline(always)]
    pub fn b1781(&mut self) -> B1781_W {
        B1781_W { w: self }
    }
    #[doc = "Bit 22 - B1782"]
    #[inline(always)]
    pub fn b1782(&mut self) -> B1782_W {
        B1782_W { w: self }
    }
    #[doc = "Bit 23 - B1783"]
    #[inline(always)]
    pub fn b1783(&mut self) -> B1783_W {
        B1783_W { w: self }
    }
    #[doc = "Bit 24 - B1784"]
    #[inline(always)]
    pub fn b1784(&mut self) -> B1784_W {
        B1784_W { w: self }
    }
    #[doc = "Bit 25 - B1785"]
    #[inline(always)]
    pub fn b1785(&mut self) -> B1785_W {
        B1785_W { w: self }
    }
    #[doc = "Bit 26 - B1786"]
    #[inline(always)]
    pub fn b1786(&mut self) -> B1786_W {
        B1786_W { w: self }
    }
    #[doc = "Bit 27 - B1787"]
    #[inline(always)]
    pub fn b1787(&mut self) -> B1787_W {
        B1787_W { w: self }
    }
    #[doc = "Bit 28 - B1788"]
    #[inline(always)]
    pub fn b1788(&mut self) -> B1788_W {
        B1788_W { w: self }
    }
    #[doc = "Bit 29 - B1789"]
    #[inline(always)]
    pub fn b1789(&mut self) -> B1789_W {
        B1789_W { w: self }
    }
    #[doc = "Bit 30 - B1790"]
    #[inline(always)]
    pub fn b1790(&mut self) -> B1790_W {
        B1790_W { w: self }
    }
    #[doc = "Bit 31 - B1791"]
    #[inline(always)]
    pub fn b1791(&mut self) -> B1791_W {
        B1791_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr55](index.html) module"]
pub struct MPCBB1_VCTR55_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR55_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr55::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR55_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr55::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR55_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR55 to value 0"]
impl crate::Resettable for MPCBB1_VCTR55_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
