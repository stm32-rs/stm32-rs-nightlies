#[doc = "Register `MPCBB1_VCTR48` reader"]
pub struct R(crate::R<MPCBB1_VCTR48_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR48_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR48_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR48_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR48` writer"]
pub struct W(crate::W<MPCBB1_VCTR48_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR48_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR48_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR48_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1536` reader - B1536"]
pub struct B1536_R(crate::FieldReader<bool, bool>);
impl B1536_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1536_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1536_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1536` writer - B1536"]
pub struct B1536_W<'a> {
    w: &'a mut W,
}
impl<'a> B1536_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1537` reader - B1537"]
pub struct B1537_R(crate::FieldReader<bool, bool>);
impl B1537_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1537_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1537_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1537` writer - B1537"]
pub struct B1537_W<'a> {
    w: &'a mut W,
}
impl<'a> B1537_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1538` reader - B1538"]
pub struct B1538_R(crate::FieldReader<bool, bool>);
impl B1538_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1538_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1538_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1538` writer - B1538"]
pub struct B1538_W<'a> {
    w: &'a mut W,
}
impl<'a> B1538_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1539` reader - B1539"]
pub struct B1539_R(crate::FieldReader<bool, bool>);
impl B1539_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1539_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1539_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1539` writer - B1539"]
pub struct B1539_W<'a> {
    w: &'a mut W,
}
impl<'a> B1539_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1540` reader - B1540"]
pub struct B1540_R(crate::FieldReader<bool, bool>);
impl B1540_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1540_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1540_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1540` writer - B1540"]
pub struct B1540_W<'a> {
    w: &'a mut W,
}
impl<'a> B1540_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1541` reader - B1541"]
pub struct B1541_R(crate::FieldReader<bool, bool>);
impl B1541_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1541_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1541_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1541` writer - B1541"]
pub struct B1541_W<'a> {
    w: &'a mut W,
}
impl<'a> B1541_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1542` reader - B1542"]
pub struct B1542_R(crate::FieldReader<bool, bool>);
impl B1542_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1542_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1542_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1542` writer - B1542"]
pub struct B1542_W<'a> {
    w: &'a mut W,
}
impl<'a> B1542_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1543` reader - B1543"]
pub struct B1543_R(crate::FieldReader<bool, bool>);
impl B1543_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1543_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1543_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1543` writer - B1543"]
pub struct B1543_W<'a> {
    w: &'a mut W,
}
impl<'a> B1543_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1544` reader - B1544"]
pub struct B1544_R(crate::FieldReader<bool, bool>);
impl B1544_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1544_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1544_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1544` writer - B1544"]
pub struct B1544_W<'a> {
    w: &'a mut W,
}
impl<'a> B1544_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1545` reader - B1545"]
pub struct B1545_R(crate::FieldReader<bool, bool>);
impl B1545_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1545_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1545_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1545` writer - B1545"]
pub struct B1545_W<'a> {
    w: &'a mut W,
}
impl<'a> B1545_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1546` reader - B1546"]
pub struct B1546_R(crate::FieldReader<bool, bool>);
impl B1546_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1546_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1546_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1546` writer - B1546"]
pub struct B1546_W<'a> {
    w: &'a mut W,
}
impl<'a> B1546_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1547` reader - B1547"]
pub struct B1547_R(crate::FieldReader<bool, bool>);
impl B1547_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1547_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1547_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1547` writer - B1547"]
pub struct B1547_W<'a> {
    w: &'a mut W,
}
impl<'a> B1547_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1548` reader - B1548"]
pub struct B1548_R(crate::FieldReader<bool, bool>);
impl B1548_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1548_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1548_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1548` writer - B1548"]
pub struct B1548_W<'a> {
    w: &'a mut W,
}
impl<'a> B1548_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1549` reader - B1549"]
pub struct B1549_R(crate::FieldReader<bool, bool>);
impl B1549_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1549_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1549_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1549` writer - B1549"]
pub struct B1549_W<'a> {
    w: &'a mut W,
}
impl<'a> B1549_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1550` reader - B1550"]
pub struct B1550_R(crate::FieldReader<bool, bool>);
impl B1550_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1550_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1550_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1550` writer - B1550"]
pub struct B1550_W<'a> {
    w: &'a mut W,
}
impl<'a> B1550_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1551` reader - B1551"]
pub struct B1551_R(crate::FieldReader<bool, bool>);
impl B1551_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1551_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1551_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1551` writer - B1551"]
pub struct B1551_W<'a> {
    w: &'a mut W,
}
impl<'a> B1551_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1552` reader - B1552"]
pub struct B1552_R(crate::FieldReader<bool, bool>);
impl B1552_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1552_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1552_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1552` writer - B1552"]
pub struct B1552_W<'a> {
    w: &'a mut W,
}
impl<'a> B1552_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1553` reader - B1553"]
pub struct B1553_R(crate::FieldReader<bool, bool>);
impl B1553_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1553_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1553_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1553` writer - B1553"]
pub struct B1553_W<'a> {
    w: &'a mut W,
}
impl<'a> B1553_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1554` reader - B1554"]
pub struct B1554_R(crate::FieldReader<bool, bool>);
impl B1554_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1554_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1554_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1554` writer - B1554"]
pub struct B1554_W<'a> {
    w: &'a mut W,
}
impl<'a> B1554_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1555` reader - B1555"]
pub struct B1555_R(crate::FieldReader<bool, bool>);
impl B1555_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1555_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1555_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1555` writer - B1555"]
pub struct B1555_W<'a> {
    w: &'a mut W,
}
impl<'a> B1555_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1556` reader - B1556"]
pub struct B1556_R(crate::FieldReader<bool, bool>);
impl B1556_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1556_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1556_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1556` writer - B1556"]
pub struct B1556_W<'a> {
    w: &'a mut W,
}
impl<'a> B1556_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1557` reader - B1557"]
pub struct B1557_R(crate::FieldReader<bool, bool>);
impl B1557_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1557_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1557_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1557` writer - B1557"]
pub struct B1557_W<'a> {
    w: &'a mut W,
}
impl<'a> B1557_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1558` reader - B1558"]
pub struct B1558_R(crate::FieldReader<bool, bool>);
impl B1558_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1558_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1558_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1558` writer - B1558"]
pub struct B1558_W<'a> {
    w: &'a mut W,
}
impl<'a> B1558_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1559` reader - B1559"]
pub struct B1559_R(crate::FieldReader<bool, bool>);
impl B1559_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1559_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1559_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1559` writer - B1559"]
pub struct B1559_W<'a> {
    w: &'a mut W,
}
impl<'a> B1559_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1560` reader - B1560"]
pub struct B1560_R(crate::FieldReader<bool, bool>);
impl B1560_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1560_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1560_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1560` writer - B1560"]
pub struct B1560_W<'a> {
    w: &'a mut W,
}
impl<'a> B1560_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1561` reader - B1561"]
pub struct B1561_R(crate::FieldReader<bool, bool>);
impl B1561_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1561_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1561_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1561` writer - B1561"]
pub struct B1561_W<'a> {
    w: &'a mut W,
}
impl<'a> B1561_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1562` reader - B1562"]
pub struct B1562_R(crate::FieldReader<bool, bool>);
impl B1562_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1562_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1562_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1562` writer - B1562"]
pub struct B1562_W<'a> {
    w: &'a mut W,
}
impl<'a> B1562_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1563` reader - B1563"]
pub struct B1563_R(crate::FieldReader<bool, bool>);
impl B1563_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1563_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1563_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1563` writer - B1563"]
pub struct B1563_W<'a> {
    w: &'a mut W,
}
impl<'a> B1563_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1564` reader - B1564"]
pub struct B1564_R(crate::FieldReader<bool, bool>);
impl B1564_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1564_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1564_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1564` writer - B1564"]
pub struct B1564_W<'a> {
    w: &'a mut W,
}
impl<'a> B1564_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1565` reader - B1565"]
pub struct B1565_R(crate::FieldReader<bool, bool>);
impl B1565_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1565_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1565_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1565` writer - B1565"]
pub struct B1565_W<'a> {
    w: &'a mut W,
}
impl<'a> B1565_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1566` reader - B1566"]
pub struct B1566_R(crate::FieldReader<bool, bool>);
impl B1566_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1566_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1566_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1566` writer - B1566"]
pub struct B1566_W<'a> {
    w: &'a mut W,
}
impl<'a> B1566_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1567` reader - B1567"]
pub struct B1567_R(crate::FieldReader<bool, bool>);
impl B1567_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1567_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1567_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1567` writer - B1567"]
pub struct B1567_W<'a> {
    w: &'a mut W,
}
impl<'a> B1567_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1536"]
    #[inline(always)]
    pub fn b1536(&self) -> B1536_R {
        B1536_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1537"]
    #[inline(always)]
    pub fn b1537(&self) -> B1537_R {
        B1537_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1538"]
    #[inline(always)]
    pub fn b1538(&self) -> B1538_R {
        B1538_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1539"]
    #[inline(always)]
    pub fn b1539(&self) -> B1539_R {
        B1539_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1540"]
    #[inline(always)]
    pub fn b1540(&self) -> B1540_R {
        B1540_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1541"]
    #[inline(always)]
    pub fn b1541(&self) -> B1541_R {
        B1541_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1542"]
    #[inline(always)]
    pub fn b1542(&self) -> B1542_R {
        B1542_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1543"]
    #[inline(always)]
    pub fn b1543(&self) -> B1543_R {
        B1543_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1544"]
    #[inline(always)]
    pub fn b1544(&self) -> B1544_R {
        B1544_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1545"]
    #[inline(always)]
    pub fn b1545(&self) -> B1545_R {
        B1545_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1546"]
    #[inline(always)]
    pub fn b1546(&self) -> B1546_R {
        B1546_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1547"]
    #[inline(always)]
    pub fn b1547(&self) -> B1547_R {
        B1547_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1548"]
    #[inline(always)]
    pub fn b1548(&self) -> B1548_R {
        B1548_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1549"]
    #[inline(always)]
    pub fn b1549(&self) -> B1549_R {
        B1549_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1550"]
    #[inline(always)]
    pub fn b1550(&self) -> B1550_R {
        B1550_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1551"]
    #[inline(always)]
    pub fn b1551(&self) -> B1551_R {
        B1551_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1552"]
    #[inline(always)]
    pub fn b1552(&self) -> B1552_R {
        B1552_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1553"]
    #[inline(always)]
    pub fn b1553(&self) -> B1553_R {
        B1553_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1554"]
    #[inline(always)]
    pub fn b1554(&self) -> B1554_R {
        B1554_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1555"]
    #[inline(always)]
    pub fn b1555(&self) -> B1555_R {
        B1555_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1556"]
    #[inline(always)]
    pub fn b1556(&self) -> B1556_R {
        B1556_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1557"]
    #[inline(always)]
    pub fn b1557(&self) -> B1557_R {
        B1557_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1558"]
    #[inline(always)]
    pub fn b1558(&self) -> B1558_R {
        B1558_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1559"]
    #[inline(always)]
    pub fn b1559(&self) -> B1559_R {
        B1559_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1560"]
    #[inline(always)]
    pub fn b1560(&self) -> B1560_R {
        B1560_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1561"]
    #[inline(always)]
    pub fn b1561(&self) -> B1561_R {
        B1561_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1562"]
    #[inline(always)]
    pub fn b1562(&self) -> B1562_R {
        B1562_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1563"]
    #[inline(always)]
    pub fn b1563(&self) -> B1563_R {
        B1563_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1564"]
    #[inline(always)]
    pub fn b1564(&self) -> B1564_R {
        B1564_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1565"]
    #[inline(always)]
    pub fn b1565(&self) -> B1565_R {
        B1565_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1566"]
    #[inline(always)]
    pub fn b1566(&self) -> B1566_R {
        B1566_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1567"]
    #[inline(always)]
    pub fn b1567(&self) -> B1567_R {
        B1567_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1536"]
    #[inline(always)]
    pub fn b1536(&mut self) -> B1536_W {
        B1536_W { w: self }
    }
    #[doc = "Bit 1 - B1537"]
    #[inline(always)]
    pub fn b1537(&mut self) -> B1537_W {
        B1537_W { w: self }
    }
    #[doc = "Bit 2 - B1538"]
    #[inline(always)]
    pub fn b1538(&mut self) -> B1538_W {
        B1538_W { w: self }
    }
    #[doc = "Bit 3 - B1539"]
    #[inline(always)]
    pub fn b1539(&mut self) -> B1539_W {
        B1539_W { w: self }
    }
    #[doc = "Bit 4 - B1540"]
    #[inline(always)]
    pub fn b1540(&mut self) -> B1540_W {
        B1540_W { w: self }
    }
    #[doc = "Bit 5 - B1541"]
    #[inline(always)]
    pub fn b1541(&mut self) -> B1541_W {
        B1541_W { w: self }
    }
    #[doc = "Bit 6 - B1542"]
    #[inline(always)]
    pub fn b1542(&mut self) -> B1542_W {
        B1542_W { w: self }
    }
    #[doc = "Bit 7 - B1543"]
    #[inline(always)]
    pub fn b1543(&mut self) -> B1543_W {
        B1543_W { w: self }
    }
    #[doc = "Bit 8 - B1544"]
    #[inline(always)]
    pub fn b1544(&mut self) -> B1544_W {
        B1544_W { w: self }
    }
    #[doc = "Bit 9 - B1545"]
    #[inline(always)]
    pub fn b1545(&mut self) -> B1545_W {
        B1545_W { w: self }
    }
    #[doc = "Bit 10 - B1546"]
    #[inline(always)]
    pub fn b1546(&mut self) -> B1546_W {
        B1546_W { w: self }
    }
    #[doc = "Bit 11 - B1547"]
    #[inline(always)]
    pub fn b1547(&mut self) -> B1547_W {
        B1547_W { w: self }
    }
    #[doc = "Bit 12 - B1548"]
    #[inline(always)]
    pub fn b1548(&mut self) -> B1548_W {
        B1548_W { w: self }
    }
    #[doc = "Bit 13 - B1549"]
    #[inline(always)]
    pub fn b1549(&mut self) -> B1549_W {
        B1549_W { w: self }
    }
    #[doc = "Bit 14 - B1550"]
    #[inline(always)]
    pub fn b1550(&mut self) -> B1550_W {
        B1550_W { w: self }
    }
    #[doc = "Bit 15 - B1551"]
    #[inline(always)]
    pub fn b1551(&mut self) -> B1551_W {
        B1551_W { w: self }
    }
    #[doc = "Bit 16 - B1552"]
    #[inline(always)]
    pub fn b1552(&mut self) -> B1552_W {
        B1552_W { w: self }
    }
    #[doc = "Bit 17 - B1553"]
    #[inline(always)]
    pub fn b1553(&mut self) -> B1553_W {
        B1553_W { w: self }
    }
    #[doc = "Bit 18 - B1554"]
    #[inline(always)]
    pub fn b1554(&mut self) -> B1554_W {
        B1554_W { w: self }
    }
    #[doc = "Bit 19 - B1555"]
    #[inline(always)]
    pub fn b1555(&mut self) -> B1555_W {
        B1555_W { w: self }
    }
    #[doc = "Bit 20 - B1556"]
    #[inline(always)]
    pub fn b1556(&mut self) -> B1556_W {
        B1556_W { w: self }
    }
    #[doc = "Bit 21 - B1557"]
    #[inline(always)]
    pub fn b1557(&mut self) -> B1557_W {
        B1557_W { w: self }
    }
    #[doc = "Bit 22 - B1558"]
    #[inline(always)]
    pub fn b1558(&mut self) -> B1558_W {
        B1558_W { w: self }
    }
    #[doc = "Bit 23 - B1559"]
    #[inline(always)]
    pub fn b1559(&mut self) -> B1559_W {
        B1559_W { w: self }
    }
    #[doc = "Bit 24 - B1560"]
    #[inline(always)]
    pub fn b1560(&mut self) -> B1560_W {
        B1560_W { w: self }
    }
    #[doc = "Bit 25 - B1561"]
    #[inline(always)]
    pub fn b1561(&mut self) -> B1561_W {
        B1561_W { w: self }
    }
    #[doc = "Bit 26 - B1562"]
    #[inline(always)]
    pub fn b1562(&mut self) -> B1562_W {
        B1562_W { w: self }
    }
    #[doc = "Bit 27 - B1563"]
    #[inline(always)]
    pub fn b1563(&mut self) -> B1563_W {
        B1563_W { w: self }
    }
    #[doc = "Bit 28 - B1564"]
    #[inline(always)]
    pub fn b1564(&mut self) -> B1564_W {
        B1564_W { w: self }
    }
    #[doc = "Bit 29 - B1565"]
    #[inline(always)]
    pub fn b1565(&mut self) -> B1565_W {
        B1565_W { w: self }
    }
    #[doc = "Bit 30 - B1566"]
    #[inline(always)]
    pub fn b1566(&mut self) -> B1566_W {
        B1566_W { w: self }
    }
    #[doc = "Bit 31 - B1567"]
    #[inline(always)]
    pub fn b1567(&mut self) -> B1567_W {
        B1567_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr48](index.html) module"]
pub struct MPCBB1_VCTR48_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr48::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR48_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr48::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR48_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR48 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB1_VCTR48_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
