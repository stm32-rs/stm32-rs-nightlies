#[doc = "Register `MPCBB2_VCTR47` reader"]
pub struct R(crate::R<MPCBB2_VCTR47_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR47_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR47_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR47_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR47` writer"]
pub struct W(crate::W<MPCBB2_VCTR47_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR47_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR47_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR47_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1504` reader - B1504"]
pub struct B1504_R(crate::FieldReader<bool, bool>);
impl B1504_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1504_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1504_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1504` writer - B1504"]
pub struct B1504_W<'a> {
    w: &'a mut W,
}
impl<'a> B1504_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1505` reader - B1505"]
pub struct B1505_R(crate::FieldReader<bool, bool>);
impl B1505_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1505_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1505_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1505` writer - B1505"]
pub struct B1505_W<'a> {
    w: &'a mut W,
}
impl<'a> B1505_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1506` reader - B1506"]
pub struct B1506_R(crate::FieldReader<bool, bool>);
impl B1506_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1506_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1506_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1506` writer - B1506"]
pub struct B1506_W<'a> {
    w: &'a mut W,
}
impl<'a> B1506_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1507` reader - B1507"]
pub struct B1507_R(crate::FieldReader<bool, bool>);
impl B1507_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1507_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1507_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1507` writer - B1507"]
pub struct B1507_W<'a> {
    w: &'a mut W,
}
impl<'a> B1507_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1508` reader - B1508"]
pub struct B1508_R(crate::FieldReader<bool, bool>);
impl B1508_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1508_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1508_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1508` writer - B1508"]
pub struct B1508_W<'a> {
    w: &'a mut W,
}
impl<'a> B1508_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1509` reader - B1509"]
pub struct B1509_R(crate::FieldReader<bool, bool>);
impl B1509_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1509_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1509_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1509` writer - B1509"]
pub struct B1509_W<'a> {
    w: &'a mut W,
}
impl<'a> B1509_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1510` reader - B1510"]
pub struct B1510_R(crate::FieldReader<bool, bool>);
impl B1510_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1510_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1510_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1510` writer - B1510"]
pub struct B1510_W<'a> {
    w: &'a mut W,
}
impl<'a> B1510_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1511` reader - B1511"]
pub struct B1511_R(crate::FieldReader<bool, bool>);
impl B1511_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1511_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1511_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1511` writer - B1511"]
pub struct B1511_W<'a> {
    w: &'a mut W,
}
impl<'a> B1511_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1512` reader - B1512"]
pub struct B1512_R(crate::FieldReader<bool, bool>);
impl B1512_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1512_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1512_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1512` writer - B1512"]
pub struct B1512_W<'a> {
    w: &'a mut W,
}
impl<'a> B1512_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1513` reader - B1513"]
pub struct B1513_R(crate::FieldReader<bool, bool>);
impl B1513_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1513_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1513_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1513` writer - B1513"]
pub struct B1513_W<'a> {
    w: &'a mut W,
}
impl<'a> B1513_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1514` reader - B1514"]
pub struct B1514_R(crate::FieldReader<bool, bool>);
impl B1514_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1514_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1514_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1514` writer - B1514"]
pub struct B1514_W<'a> {
    w: &'a mut W,
}
impl<'a> B1514_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1515` reader - B1515"]
pub struct B1515_R(crate::FieldReader<bool, bool>);
impl B1515_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1515_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1515_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1515` writer - B1515"]
pub struct B1515_W<'a> {
    w: &'a mut W,
}
impl<'a> B1515_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1516` reader - B1516"]
pub struct B1516_R(crate::FieldReader<bool, bool>);
impl B1516_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1516_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1516_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1516` writer - B1516"]
pub struct B1516_W<'a> {
    w: &'a mut W,
}
impl<'a> B1516_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1517` reader - B1517"]
pub struct B1517_R(crate::FieldReader<bool, bool>);
impl B1517_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1517_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1517_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1517` writer - B1517"]
pub struct B1517_W<'a> {
    w: &'a mut W,
}
impl<'a> B1517_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1518` reader - B1518"]
pub struct B1518_R(crate::FieldReader<bool, bool>);
impl B1518_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1518_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1518_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1518` writer - B1518"]
pub struct B1518_W<'a> {
    w: &'a mut W,
}
impl<'a> B1518_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1519` reader - B1519"]
pub struct B1519_R(crate::FieldReader<bool, bool>);
impl B1519_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1519_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1519_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1519` writer - B1519"]
pub struct B1519_W<'a> {
    w: &'a mut W,
}
impl<'a> B1519_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1520` reader - B1520"]
pub struct B1520_R(crate::FieldReader<bool, bool>);
impl B1520_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1520_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1520_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1520` writer - B1520"]
pub struct B1520_W<'a> {
    w: &'a mut W,
}
impl<'a> B1520_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1521` reader - B1521"]
pub struct B1521_R(crate::FieldReader<bool, bool>);
impl B1521_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1521_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1521_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1521` writer - B1521"]
pub struct B1521_W<'a> {
    w: &'a mut W,
}
impl<'a> B1521_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1522` reader - B1522"]
pub struct B1522_R(crate::FieldReader<bool, bool>);
impl B1522_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1522_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1522_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1522` writer - B1522"]
pub struct B1522_W<'a> {
    w: &'a mut W,
}
impl<'a> B1522_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1523` reader - B1523"]
pub struct B1523_R(crate::FieldReader<bool, bool>);
impl B1523_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1523_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1523_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1523` writer - B1523"]
pub struct B1523_W<'a> {
    w: &'a mut W,
}
impl<'a> B1523_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1524` reader - B1524"]
pub struct B1524_R(crate::FieldReader<bool, bool>);
impl B1524_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1524_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1524_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1524` writer - B1524"]
pub struct B1524_W<'a> {
    w: &'a mut W,
}
impl<'a> B1524_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1525` reader - B1525"]
pub struct B1525_R(crate::FieldReader<bool, bool>);
impl B1525_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1525_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1525_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1525` writer - B1525"]
pub struct B1525_W<'a> {
    w: &'a mut W,
}
impl<'a> B1525_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1526` reader - B1526"]
pub struct B1526_R(crate::FieldReader<bool, bool>);
impl B1526_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1526_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1526_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1526` writer - B1526"]
pub struct B1526_W<'a> {
    w: &'a mut W,
}
impl<'a> B1526_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1527` reader - B1527"]
pub struct B1527_R(crate::FieldReader<bool, bool>);
impl B1527_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1527_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1527_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1527` writer - B1527"]
pub struct B1527_W<'a> {
    w: &'a mut W,
}
impl<'a> B1527_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1528` reader - B1528"]
pub struct B1528_R(crate::FieldReader<bool, bool>);
impl B1528_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1528_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1528_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1528` writer - B1528"]
pub struct B1528_W<'a> {
    w: &'a mut W,
}
impl<'a> B1528_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1529` reader - B1529"]
pub struct B1529_R(crate::FieldReader<bool, bool>);
impl B1529_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1529_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1529_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1529` writer - B1529"]
pub struct B1529_W<'a> {
    w: &'a mut W,
}
impl<'a> B1529_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1530` reader - B1530"]
pub struct B1530_R(crate::FieldReader<bool, bool>);
impl B1530_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1530_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1530_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1530` writer - B1530"]
pub struct B1530_W<'a> {
    w: &'a mut W,
}
impl<'a> B1530_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1531` reader - B1531"]
pub struct B1531_R(crate::FieldReader<bool, bool>);
impl B1531_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1531_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1531_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1531` writer - B1531"]
pub struct B1531_W<'a> {
    w: &'a mut W,
}
impl<'a> B1531_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1532` reader - B1532"]
pub struct B1532_R(crate::FieldReader<bool, bool>);
impl B1532_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1532_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1532_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1532` writer - B1532"]
pub struct B1532_W<'a> {
    w: &'a mut W,
}
impl<'a> B1532_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1533` reader - B1533"]
pub struct B1533_R(crate::FieldReader<bool, bool>);
impl B1533_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1533_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1533_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1533` writer - B1533"]
pub struct B1533_W<'a> {
    w: &'a mut W,
}
impl<'a> B1533_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1534` reader - B1534"]
pub struct B1534_R(crate::FieldReader<bool, bool>);
impl B1534_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1534_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1534_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1534` writer - B1534"]
pub struct B1534_W<'a> {
    w: &'a mut W,
}
impl<'a> B1534_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1535` reader - B1535"]
pub struct B1535_R(crate::FieldReader<bool, bool>);
impl B1535_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1535_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1535_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1535` writer - B1535"]
pub struct B1535_W<'a> {
    w: &'a mut W,
}
impl<'a> B1535_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1504"]
    #[inline(always)]
    pub fn b1504(&self) -> B1504_R {
        B1504_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1505"]
    #[inline(always)]
    pub fn b1505(&self) -> B1505_R {
        B1505_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1506"]
    #[inline(always)]
    pub fn b1506(&self) -> B1506_R {
        B1506_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1507"]
    #[inline(always)]
    pub fn b1507(&self) -> B1507_R {
        B1507_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1508"]
    #[inline(always)]
    pub fn b1508(&self) -> B1508_R {
        B1508_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1509"]
    #[inline(always)]
    pub fn b1509(&self) -> B1509_R {
        B1509_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1510"]
    #[inline(always)]
    pub fn b1510(&self) -> B1510_R {
        B1510_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1511"]
    #[inline(always)]
    pub fn b1511(&self) -> B1511_R {
        B1511_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1512"]
    #[inline(always)]
    pub fn b1512(&self) -> B1512_R {
        B1512_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1513"]
    #[inline(always)]
    pub fn b1513(&self) -> B1513_R {
        B1513_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1514"]
    #[inline(always)]
    pub fn b1514(&self) -> B1514_R {
        B1514_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1515"]
    #[inline(always)]
    pub fn b1515(&self) -> B1515_R {
        B1515_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1516"]
    #[inline(always)]
    pub fn b1516(&self) -> B1516_R {
        B1516_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1517"]
    #[inline(always)]
    pub fn b1517(&self) -> B1517_R {
        B1517_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1518"]
    #[inline(always)]
    pub fn b1518(&self) -> B1518_R {
        B1518_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1519"]
    #[inline(always)]
    pub fn b1519(&self) -> B1519_R {
        B1519_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1520"]
    #[inline(always)]
    pub fn b1520(&self) -> B1520_R {
        B1520_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1521"]
    #[inline(always)]
    pub fn b1521(&self) -> B1521_R {
        B1521_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1522"]
    #[inline(always)]
    pub fn b1522(&self) -> B1522_R {
        B1522_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1523"]
    #[inline(always)]
    pub fn b1523(&self) -> B1523_R {
        B1523_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1524"]
    #[inline(always)]
    pub fn b1524(&self) -> B1524_R {
        B1524_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1525"]
    #[inline(always)]
    pub fn b1525(&self) -> B1525_R {
        B1525_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1526"]
    #[inline(always)]
    pub fn b1526(&self) -> B1526_R {
        B1526_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1527"]
    #[inline(always)]
    pub fn b1527(&self) -> B1527_R {
        B1527_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1528"]
    #[inline(always)]
    pub fn b1528(&self) -> B1528_R {
        B1528_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1529"]
    #[inline(always)]
    pub fn b1529(&self) -> B1529_R {
        B1529_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1530"]
    #[inline(always)]
    pub fn b1530(&self) -> B1530_R {
        B1530_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1531"]
    #[inline(always)]
    pub fn b1531(&self) -> B1531_R {
        B1531_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1532"]
    #[inline(always)]
    pub fn b1532(&self) -> B1532_R {
        B1532_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1533"]
    #[inline(always)]
    pub fn b1533(&self) -> B1533_R {
        B1533_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1534"]
    #[inline(always)]
    pub fn b1534(&self) -> B1534_R {
        B1534_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1535"]
    #[inline(always)]
    pub fn b1535(&self) -> B1535_R {
        B1535_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1504"]
    #[inline(always)]
    pub fn b1504(&mut self) -> B1504_W {
        B1504_W { w: self }
    }
    #[doc = "Bit 1 - B1505"]
    #[inline(always)]
    pub fn b1505(&mut self) -> B1505_W {
        B1505_W { w: self }
    }
    #[doc = "Bit 2 - B1506"]
    #[inline(always)]
    pub fn b1506(&mut self) -> B1506_W {
        B1506_W { w: self }
    }
    #[doc = "Bit 3 - B1507"]
    #[inline(always)]
    pub fn b1507(&mut self) -> B1507_W {
        B1507_W { w: self }
    }
    #[doc = "Bit 4 - B1508"]
    #[inline(always)]
    pub fn b1508(&mut self) -> B1508_W {
        B1508_W { w: self }
    }
    #[doc = "Bit 5 - B1509"]
    #[inline(always)]
    pub fn b1509(&mut self) -> B1509_W {
        B1509_W { w: self }
    }
    #[doc = "Bit 6 - B1510"]
    #[inline(always)]
    pub fn b1510(&mut self) -> B1510_W {
        B1510_W { w: self }
    }
    #[doc = "Bit 7 - B1511"]
    #[inline(always)]
    pub fn b1511(&mut self) -> B1511_W {
        B1511_W { w: self }
    }
    #[doc = "Bit 8 - B1512"]
    #[inline(always)]
    pub fn b1512(&mut self) -> B1512_W {
        B1512_W { w: self }
    }
    #[doc = "Bit 9 - B1513"]
    #[inline(always)]
    pub fn b1513(&mut self) -> B1513_W {
        B1513_W { w: self }
    }
    #[doc = "Bit 10 - B1514"]
    #[inline(always)]
    pub fn b1514(&mut self) -> B1514_W {
        B1514_W { w: self }
    }
    #[doc = "Bit 11 - B1515"]
    #[inline(always)]
    pub fn b1515(&mut self) -> B1515_W {
        B1515_W { w: self }
    }
    #[doc = "Bit 12 - B1516"]
    #[inline(always)]
    pub fn b1516(&mut self) -> B1516_W {
        B1516_W { w: self }
    }
    #[doc = "Bit 13 - B1517"]
    #[inline(always)]
    pub fn b1517(&mut self) -> B1517_W {
        B1517_W { w: self }
    }
    #[doc = "Bit 14 - B1518"]
    #[inline(always)]
    pub fn b1518(&mut self) -> B1518_W {
        B1518_W { w: self }
    }
    #[doc = "Bit 15 - B1519"]
    #[inline(always)]
    pub fn b1519(&mut self) -> B1519_W {
        B1519_W { w: self }
    }
    #[doc = "Bit 16 - B1520"]
    #[inline(always)]
    pub fn b1520(&mut self) -> B1520_W {
        B1520_W { w: self }
    }
    #[doc = "Bit 17 - B1521"]
    #[inline(always)]
    pub fn b1521(&mut self) -> B1521_W {
        B1521_W { w: self }
    }
    #[doc = "Bit 18 - B1522"]
    #[inline(always)]
    pub fn b1522(&mut self) -> B1522_W {
        B1522_W { w: self }
    }
    #[doc = "Bit 19 - B1523"]
    #[inline(always)]
    pub fn b1523(&mut self) -> B1523_W {
        B1523_W { w: self }
    }
    #[doc = "Bit 20 - B1524"]
    #[inline(always)]
    pub fn b1524(&mut self) -> B1524_W {
        B1524_W { w: self }
    }
    #[doc = "Bit 21 - B1525"]
    #[inline(always)]
    pub fn b1525(&mut self) -> B1525_W {
        B1525_W { w: self }
    }
    #[doc = "Bit 22 - B1526"]
    #[inline(always)]
    pub fn b1526(&mut self) -> B1526_W {
        B1526_W { w: self }
    }
    #[doc = "Bit 23 - B1527"]
    #[inline(always)]
    pub fn b1527(&mut self) -> B1527_W {
        B1527_W { w: self }
    }
    #[doc = "Bit 24 - B1528"]
    #[inline(always)]
    pub fn b1528(&mut self) -> B1528_W {
        B1528_W { w: self }
    }
    #[doc = "Bit 25 - B1529"]
    #[inline(always)]
    pub fn b1529(&mut self) -> B1529_W {
        B1529_W { w: self }
    }
    #[doc = "Bit 26 - B1530"]
    #[inline(always)]
    pub fn b1530(&mut self) -> B1530_W {
        B1530_W { w: self }
    }
    #[doc = "Bit 27 - B1531"]
    #[inline(always)]
    pub fn b1531(&mut self) -> B1531_W {
        B1531_W { w: self }
    }
    #[doc = "Bit 28 - B1532"]
    #[inline(always)]
    pub fn b1532(&mut self) -> B1532_W {
        B1532_W { w: self }
    }
    #[doc = "Bit 29 - B1533"]
    #[inline(always)]
    pub fn b1533(&mut self) -> B1533_W {
        B1533_W { w: self }
    }
    #[doc = "Bit 30 - B1534"]
    #[inline(always)]
    pub fn b1534(&mut self) -> B1534_W {
        B1534_W { w: self }
    }
    #[doc = "Bit 31 - B1535"]
    #[inline(always)]
    pub fn b1535(&mut self) -> B1535_W {
        B1535_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr47](index.html) module"]
pub struct MPCBB2_VCTR47_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR47_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr47::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR47_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr47::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR47_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR47 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR47_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
