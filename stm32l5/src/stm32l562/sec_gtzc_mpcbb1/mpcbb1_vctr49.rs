#[doc = "Register `MPCBB1_VCTR49` reader"]
pub struct R(crate::R<MPCBB1_VCTR49_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR49_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR49_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR49_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR49` writer"]
pub struct W(crate::W<MPCBB1_VCTR49_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR49_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR49_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR49_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1568` reader - B1568"]
pub struct B1568_R(crate::FieldReader<bool, bool>);
impl B1568_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1568_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1568_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1568` writer - B1568"]
pub struct B1568_W<'a> {
    w: &'a mut W,
}
impl<'a> B1568_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1569` reader - B1569"]
pub struct B1569_R(crate::FieldReader<bool, bool>);
impl B1569_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1569_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1569_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1569` writer - B1569"]
pub struct B1569_W<'a> {
    w: &'a mut W,
}
impl<'a> B1569_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1570` reader - B1570"]
pub struct B1570_R(crate::FieldReader<bool, bool>);
impl B1570_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1570_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1570_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1570` writer - B1570"]
pub struct B1570_W<'a> {
    w: &'a mut W,
}
impl<'a> B1570_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1571` reader - B1571"]
pub struct B1571_R(crate::FieldReader<bool, bool>);
impl B1571_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1571_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1571_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1571` writer - B1571"]
pub struct B1571_W<'a> {
    w: &'a mut W,
}
impl<'a> B1571_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1572` reader - B1572"]
pub struct B1572_R(crate::FieldReader<bool, bool>);
impl B1572_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1572_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1572_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1572` writer - B1572"]
pub struct B1572_W<'a> {
    w: &'a mut W,
}
impl<'a> B1572_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1573` reader - B1573"]
pub struct B1573_R(crate::FieldReader<bool, bool>);
impl B1573_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1573_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1573_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1573` writer - B1573"]
pub struct B1573_W<'a> {
    w: &'a mut W,
}
impl<'a> B1573_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1574` reader - B1574"]
pub struct B1574_R(crate::FieldReader<bool, bool>);
impl B1574_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1574_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1574_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1574` writer - B1574"]
pub struct B1574_W<'a> {
    w: &'a mut W,
}
impl<'a> B1574_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1575` reader - B1575"]
pub struct B1575_R(crate::FieldReader<bool, bool>);
impl B1575_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1575_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1575_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1575` writer - B1575"]
pub struct B1575_W<'a> {
    w: &'a mut W,
}
impl<'a> B1575_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1576` reader - B1576"]
pub struct B1576_R(crate::FieldReader<bool, bool>);
impl B1576_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1576_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1576_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1576` writer - B1576"]
pub struct B1576_W<'a> {
    w: &'a mut W,
}
impl<'a> B1576_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1577` reader - B1577"]
pub struct B1577_R(crate::FieldReader<bool, bool>);
impl B1577_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1577_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1577_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1577` writer - B1577"]
pub struct B1577_W<'a> {
    w: &'a mut W,
}
impl<'a> B1577_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1578` reader - B1578"]
pub struct B1578_R(crate::FieldReader<bool, bool>);
impl B1578_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1578_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1578_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1578` writer - B1578"]
pub struct B1578_W<'a> {
    w: &'a mut W,
}
impl<'a> B1578_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1579` reader - B1579"]
pub struct B1579_R(crate::FieldReader<bool, bool>);
impl B1579_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1579_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1579_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1579` writer - B1579"]
pub struct B1579_W<'a> {
    w: &'a mut W,
}
impl<'a> B1579_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1580` reader - B1580"]
pub struct B1580_R(crate::FieldReader<bool, bool>);
impl B1580_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1580_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1580_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1580` writer - B1580"]
pub struct B1580_W<'a> {
    w: &'a mut W,
}
impl<'a> B1580_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1581` reader - B1581"]
pub struct B1581_R(crate::FieldReader<bool, bool>);
impl B1581_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1581_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1581_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1581` writer - B1581"]
pub struct B1581_W<'a> {
    w: &'a mut W,
}
impl<'a> B1581_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1582` reader - B1582"]
pub struct B1582_R(crate::FieldReader<bool, bool>);
impl B1582_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1582_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1582_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1582` writer - B1582"]
pub struct B1582_W<'a> {
    w: &'a mut W,
}
impl<'a> B1582_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1583` reader - B1583"]
pub struct B1583_R(crate::FieldReader<bool, bool>);
impl B1583_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1583_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1583_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1583` writer - B1583"]
pub struct B1583_W<'a> {
    w: &'a mut W,
}
impl<'a> B1583_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1584` reader - B1584"]
pub struct B1584_R(crate::FieldReader<bool, bool>);
impl B1584_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1584_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1584_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1584` writer - B1584"]
pub struct B1584_W<'a> {
    w: &'a mut W,
}
impl<'a> B1584_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1585` reader - B1585"]
pub struct B1585_R(crate::FieldReader<bool, bool>);
impl B1585_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1585_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1585_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1585` writer - B1585"]
pub struct B1585_W<'a> {
    w: &'a mut W,
}
impl<'a> B1585_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1586` reader - B1586"]
pub struct B1586_R(crate::FieldReader<bool, bool>);
impl B1586_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1586_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1586_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1586` writer - B1586"]
pub struct B1586_W<'a> {
    w: &'a mut W,
}
impl<'a> B1586_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1587` reader - B1587"]
pub struct B1587_R(crate::FieldReader<bool, bool>);
impl B1587_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1587_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1587_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1587` writer - B1587"]
pub struct B1587_W<'a> {
    w: &'a mut W,
}
impl<'a> B1587_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1588` reader - B1588"]
pub struct B1588_R(crate::FieldReader<bool, bool>);
impl B1588_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1588_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1588_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1588` writer - B1588"]
pub struct B1588_W<'a> {
    w: &'a mut W,
}
impl<'a> B1588_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1589` reader - B1589"]
pub struct B1589_R(crate::FieldReader<bool, bool>);
impl B1589_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1589_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1589_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1589` writer - B1589"]
pub struct B1589_W<'a> {
    w: &'a mut W,
}
impl<'a> B1589_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1590` reader - B1590"]
pub struct B1590_R(crate::FieldReader<bool, bool>);
impl B1590_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1590_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1590_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1590` writer - B1590"]
pub struct B1590_W<'a> {
    w: &'a mut W,
}
impl<'a> B1590_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1591` reader - B1591"]
pub struct B1591_R(crate::FieldReader<bool, bool>);
impl B1591_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1591_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1591_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1591` writer - B1591"]
pub struct B1591_W<'a> {
    w: &'a mut W,
}
impl<'a> B1591_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1592` reader - B1592"]
pub struct B1592_R(crate::FieldReader<bool, bool>);
impl B1592_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1592_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1592_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1592` writer - B1592"]
pub struct B1592_W<'a> {
    w: &'a mut W,
}
impl<'a> B1592_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1593` reader - B1593"]
pub struct B1593_R(crate::FieldReader<bool, bool>);
impl B1593_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1593_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1593_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1593` writer - B1593"]
pub struct B1593_W<'a> {
    w: &'a mut W,
}
impl<'a> B1593_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1594` reader - B1594"]
pub struct B1594_R(crate::FieldReader<bool, bool>);
impl B1594_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1594_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1594_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1594` writer - B1594"]
pub struct B1594_W<'a> {
    w: &'a mut W,
}
impl<'a> B1594_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1595` reader - B1595"]
pub struct B1595_R(crate::FieldReader<bool, bool>);
impl B1595_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1595_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1595_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1595` writer - B1595"]
pub struct B1595_W<'a> {
    w: &'a mut W,
}
impl<'a> B1595_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1596` reader - B1596"]
pub struct B1596_R(crate::FieldReader<bool, bool>);
impl B1596_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1596_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1596_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1596` writer - B1596"]
pub struct B1596_W<'a> {
    w: &'a mut W,
}
impl<'a> B1596_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1597` reader - B1597"]
pub struct B1597_R(crate::FieldReader<bool, bool>);
impl B1597_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1597_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1597_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1597` writer - B1597"]
pub struct B1597_W<'a> {
    w: &'a mut W,
}
impl<'a> B1597_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1598` reader - B1598"]
pub struct B1598_R(crate::FieldReader<bool, bool>);
impl B1598_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1598_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1598_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1598` writer - B1598"]
pub struct B1598_W<'a> {
    w: &'a mut W,
}
impl<'a> B1598_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1599` reader - B1599"]
pub struct B1599_R(crate::FieldReader<bool, bool>);
impl B1599_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1599_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1599_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1599` writer - B1599"]
pub struct B1599_W<'a> {
    w: &'a mut W,
}
impl<'a> B1599_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1568"]
    #[inline(always)]
    pub fn b1568(&self) -> B1568_R {
        B1568_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1569"]
    #[inline(always)]
    pub fn b1569(&self) -> B1569_R {
        B1569_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1570"]
    #[inline(always)]
    pub fn b1570(&self) -> B1570_R {
        B1570_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1571"]
    #[inline(always)]
    pub fn b1571(&self) -> B1571_R {
        B1571_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1572"]
    #[inline(always)]
    pub fn b1572(&self) -> B1572_R {
        B1572_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1573"]
    #[inline(always)]
    pub fn b1573(&self) -> B1573_R {
        B1573_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1574"]
    #[inline(always)]
    pub fn b1574(&self) -> B1574_R {
        B1574_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1575"]
    #[inline(always)]
    pub fn b1575(&self) -> B1575_R {
        B1575_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1576"]
    #[inline(always)]
    pub fn b1576(&self) -> B1576_R {
        B1576_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1577"]
    #[inline(always)]
    pub fn b1577(&self) -> B1577_R {
        B1577_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1578"]
    #[inline(always)]
    pub fn b1578(&self) -> B1578_R {
        B1578_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1579"]
    #[inline(always)]
    pub fn b1579(&self) -> B1579_R {
        B1579_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1580"]
    #[inline(always)]
    pub fn b1580(&self) -> B1580_R {
        B1580_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1581"]
    #[inline(always)]
    pub fn b1581(&self) -> B1581_R {
        B1581_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1582"]
    #[inline(always)]
    pub fn b1582(&self) -> B1582_R {
        B1582_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1583"]
    #[inline(always)]
    pub fn b1583(&self) -> B1583_R {
        B1583_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1584"]
    #[inline(always)]
    pub fn b1584(&self) -> B1584_R {
        B1584_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1585"]
    #[inline(always)]
    pub fn b1585(&self) -> B1585_R {
        B1585_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1586"]
    #[inline(always)]
    pub fn b1586(&self) -> B1586_R {
        B1586_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1587"]
    #[inline(always)]
    pub fn b1587(&self) -> B1587_R {
        B1587_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1588"]
    #[inline(always)]
    pub fn b1588(&self) -> B1588_R {
        B1588_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1589"]
    #[inline(always)]
    pub fn b1589(&self) -> B1589_R {
        B1589_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1590"]
    #[inline(always)]
    pub fn b1590(&self) -> B1590_R {
        B1590_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1591"]
    #[inline(always)]
    pub fn b1591(&self) -> B1591_R {
        B1591_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1592"]
    #[inline(always)]
    pub fn b1592(&self) -> B1592_R {
        B1592_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1593"]
    #[inline(always)]
    pub fn b1593(&self) -> B1593_R {
        B1593_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1594"]
    #[inline(always)]
    pub fn b1594(&self) -> B1594_R {
        B1594_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1595"]
    #[inline(always)]
    pub fn b1595(&self) -> B1595_R {
        B1595_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1596"]
    #[inline(always)]
    pub fn b1596(&self) -> B1596_R {
        B1596_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1597"]
    #[inline(always)]
    pub fn b1597(&self) -> B1597_R {
        B1597_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1598"]
    #[inline(always)]
    pub fn b1598(&self) -> B1598_R {
        B1598_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1599"]
    #[inline(always)]
    pub fn b1599(&self) -> B1599_R {
        B1599_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1568"]
    #[inline(always)]
    pub fn b1568(&mut self) -> B1568_W {
        B1568_W { w: self }
    }
    #[doc = "Bit 1 - B1569"]
    #[inline(always)]
    pub fn b1569(&mut self) -> B1569_W {
        B1569_W { w: self }
    }
    #[doc = "Bit 2 - B1570"]
    #[inline(always)]
    pub fn b1570(&mut self) -> B1570_W {
        B1570_W { w: self }
    }
    #[doc = "Bit 3 - B1571"]
    #[inline(always)]
    pub fn b1571(&mut self) -> B1571_W {
        B1571_W { w: self }
    }
    #[doc = "Bit 4 - B1572"]
    #[inline(always)]
    pub fn b1572(&mut self) -> B1572_W {
        B1572_W { w: self }
    }
    #[doc = "Bit 5 - B1573"]
    #[inline(always)]
    pub fn b1573(&mut self) -> B1573_W {
        B1573_W { w: self }
    }
    #[doc = "Bit 6 - B1574"]
    #[inline(always)]
    pub fn b1574(&mut self) -> B1574_W {
        B1574_W { w: self }
    }
    #[doc = "Bit 7 - B1575"]
    #[inline(always)]
    pub fn b1575(&mut self) -> B1575_W {
        B1575_W { w: self }
    }
    #[doc = "Bit 8 - B1576"]
    #[inline(always)]
    pub fn b1576(&mut self) -> B1576_W {
        B1576_W { w: self }
    }
    #[doc = "Bit 9 - B1577"]
    #[inline(always)]
    pub fn b1577(&mut self) -> B1577_W {
        B1577_W { w: self }
    }
    #[doc = "Bit 10 - B1578"]
    #[inline(always)]
    pub fn b1578(&mut self) -> B1578_W {
        B1578_W { w: self }
    }
    #[doc = "Bit 11 - B1579"]
    #[inline(always)]
    pub fn b1579(&mut self) -> B1579_W {
        B1579_W { w: self }
    }
    #[doc = "Bit 12 - B1580"]
    #[inline(always)]
    pub fn b1580(&mut self) -> B1580_W {
        B1580_W { w: self }
    }
    #[doc = "Bit 13 - B1581"]
    #[inline(always)]
    pub fn b1581(&mut self) -> B1581_W {
        B1581_W { w: self }
    }
    #[doc = "Bit 14 - B1582"]
    #[inline(always)]
    pub fn b1582(&mut self) -> B1582_W {
        B1582_W { w: self }
    }
    #[doc = "Bit 15 - B1583"]
    #[inline(always)]
    pub fn b1583(&mut self) -> B1583_W {
        B1583_W { w: self }
    }
    #[doc = "Bit 16 - B1584"]
    #[inline(always)]
    pub fn b1584(&mut self) -> B1584_W {
        B1584_W { w: self }
    }
    #[doc = "Bit 17 - B1585"]
    #[inline(always)]
    pub fn b1585(&mut self) -> B1585_W {
        B1585_W { w: self }
    }
    #[doc = "Bit 18 - B1586"]
    #[inline(always)]
    pub fn b1586(&mut self) -> B1586_W {
        B1586_W { w: self }
    }
    #[doc = "Bit 19 - B1587"]
    #[inline(always)]
    pub fn b1587(&mut self) -> B1587_W {
        B1587_W { w: self }
    }
    #[doc = "Bit 20 - B1588"]
    #[inline(always)]
    pub fn b1588(&mut self) -> B1588_W {
        B1588_W { w: self }
    }
    #[doc = "Bit 21 - B1589"]
    #[inline(always)]
    pub fn b1589(&mut self) -> B1589_W {
        B1589_W { w: self }
    }
    #[doc = "Bit 22 - B1590"]
    #[inline(always)]
    pub fn b1590(&mut self) -> B1590_W {
        B1590_W { w: self }
    }
    #[doc = "Bit 23 - B1591"]
    #[inline(always)]
    pub fn b1591(&mut self) -> B1591_W {
        B1591_W { w: self }
    }
    #[doc = "Bit 24 - B1592"]
    #[inline(always)]
    pub fn b1592(&mut self) -> B1592_W {
        B1592_W { w: self }
    }
    #[doc = "Bit 25 - B1593"]
    #[inline(always)]
    pub fn b1593(&mut self) -> B1593_W {
        B1593_W { w: self }
    }
    #[doc = "Bit 26 - B1594"]
    #[inline(always)]
    pub fn b1594(&mut self) -> B1594_W {
        B1594_W { w: self }
    }
    #[doc = "Bit 27 - B1595"]
    #[inline(always)]
    pub fn b1595(&mut self) -> B1595_W {
        B1595_W { w: self }
    }
    #[doc = "Bit 28 - B1596"]
    #[inline(always)]
    pub fn b1596(&mut self) -> B1596_W {
        B1596_W { w: self }
    }
    #[doc = "Bit 29 - B1597"]
    #[inline(always)]
    pub fn b1597(&mut self) -> B1597_W {
        B1597_W { w: self }
    }
    #[doc = "Bit 30 - B1598"]
    #[inline(always)]
    pub fn b1598(&mut self) -> B1598_W {
        B1598_W { w: self }
    }
    #[doc = "Bit 31 - B1599"]
    #[inline(always)]
    pub fn b1599(&mut self) -> B1599_W {
        B1599_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr49](index.html) module"]
pub struct MPCBB1_VCTR49_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR49_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr49::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR49_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr49::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR49_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR49 to value 0"]
impl crate::Resettable for MPCBB1_VCTR49_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
