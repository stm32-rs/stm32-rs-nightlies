#[doc = "Register `MPCBB2_VCTR45` reader"]
pub struct R(crate::R<MPCBB2_VCTR45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR45` writer"]
pub struct W(crate::W<MPCBB2_VCTR45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR45_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1440` reader - B1440"]
pub struct B1440_R(crate::FieldReader<bool, bool>);
impl B1440_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1440_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1440_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1440` writer - B1440"]
pub struct B1440_W<'a> {
    w: &'a mut W,
}
impl<'a> B1440_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1441` reader - B1441"]
pub struct B1441_R(crate::FieldReader<bool, bool>);
impl B1441_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1441_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1441_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1441` writer - B1441"]
pub struct B1441_W<'a> {
    w: &'a mut W,
}
impl<'a> B1441_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1442` reader - B1442"]
pub struct B1442_R(crate::FieldReader<bool, bool>);
impl B1442_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1442_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1442_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1442` writer - B1442"]
pub struct B1442_W<'a> {
    w: &'a mut W,
}
impl<'a> B1442_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1443` reader - B1443"]
pub struct B1443_R(crate::FieldReader<bool, bool>);
impl B1443_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1443_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1443_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1443` writer - B1443"]
pub struct B1443_W<'a> {
    w: &'a mut W,
}
impl<'a> B1443_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1444` reader - B1444"]
pub struct B1444_R(crate::FieldReader<bool, bool>);
impl B1444_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1444_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1444_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1444` writer - B1444"]
pub struct B1444_W<'a> {
    w: &'a mut W,
}
impl<'a> B1444_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1445` reader - B1445"]
pub struct B1445_R(crate::FieldReader<bool, bool>);
impl B1445_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1445_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1445_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1445` writer - B1445"]
pub struct B1445_W<'a> {
    w: &'a mut W,
}
impl<'a> B1445_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1446` reader - B1446"]
pub struct B1446_R(crate::FieldReader<bool, bool>);
impl B1446_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1446_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1446_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1446` writer - B1446"]
pub struct B1446_W<'a> {
    w: &'a mut W,
}
impl<'a> B1446_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1447` reader - B1447"]
pub struct B1447_R(crate::FieldReader<bool, bool>);
impl B1447_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1447_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1447_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1447` writer - B1447"]
pub struct B1447_W<'a> {
    w: &'a mut W,
}
impl<'a> B1447_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1448` reader - B1448"]
pub struct B1448_R(crate::FieldReader<bool, bool>);
impl B1448_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1448_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1448_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1448` writer - B1448"]
pub struct B1448_W<'a> {
    w: &'a mut W,
}
impl<'a> B1448_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1449` reader - B1449"]
pub struct B1449_R(crate::FieldReader<bool, bool>);
impl B1449_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1449_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1449_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1449` writer - B1449"]
pub struct B1449_W<'a> {
    w: &'a mut W,
}
impl<'a> B1449_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1450` reader - B1450"]
pub struct B1450_R(crate::FieldReader<bool, bool>);
impl B1450_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1450_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1450_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1450` writer - B1450"]
pub struct B1450_W<'a> {
    w: &'a mut W,
}
impl<'a> B1450_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1451` reader - B1451"]
pub struct B1451_R(crate::FieldReader<bool, bool>);
impl B1451_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1451_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1451_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1451` writer - B1451"]
pub struct B1451_W<'a> {
    w: &'a mut W,
}
impl<'a> B1451_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1452` reader - B1452"]
pub struct B1452_R(crate::FieldReader<bool, bool>);
impl B1452_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1452_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1452_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1452` writer - B1452"]
pub struct B1452_W<'a> {
    w: &'a mut W,
}
impl<'a> B1452_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1453` reader - B1453"]
pub struct B1453_R(crate::FieldReader<bool, bool>);
impl B1453_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1453_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1453_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1453` writer - B1453"]
pub struct B1453_W<'a> {
    w: &'a mut W,
}
impl<'a> B1453_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1454` reader - B1454"]
pub struct B1454_R(crate::FieldReader<bool, bool>);
impl B1454_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1454_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1454_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1454` writer - B1454"]
pub struct B1454_W<'a> {
    w: &'a mut W,
}
impl<'a> B1454_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1455` reader - B1455"]
pub struct B1455_R(crate::FieldReader<bool, bool>);
impl B1455_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1455_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1455_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1455` writer - B1455"]
pub struct B1455_W<'a> {
    w: &'a mut W,
}
impl<'a> B1455_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1456` reader - B1456"]
pub struct B1456_R(crate::FieldReader<bool, bool>);
impl B1456_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1456_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1456_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1456` writer - B1456"]
pub struct B1456_W<'a> {
    w: &'a mut W,
}
impl<'a> B1456_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1457` reader - B1457"]
pub struct B1457_R(crate::FieldReader<bool, bool>);
impl B1457_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1457_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1457_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1457` writer - B1457"]
pub struct B1457_W<'a> {
    w: &'a mut W,
}
impl<'a> B1457_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1458` reader - B1458"]
pub struct B1458_R(crate::FieldReader<bool, bool>);
impl B1458_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1458_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1458_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1458` writer - B1458"]
pub struct B1458_W<'a> {
    w: &'a mut W,
}
impl<'a> B1458_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1459` reader - B1459"]
pub struct B1459_R(crate::FieldReader<bool, bool>);
impl B1459_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1459_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1459_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1459` writer - B1459"]
pub struct B1459_W<'a> {
    w: &'a mut W,
}
impl<'a> B1459_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1460` reader - B1460"]
pub struct B1460_R(crate::FieldReader<bool, bool>);
impl B1460_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1460_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1460_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1460` writer - B1460"]
pub struct B1460_W<'a> {
    w: &'a mut W,
}
impl<'a> B1460_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1461` reader - B1461"]
pub struct B1461_R(crate::FieldReader<bool, bool>);
impl B1461_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1461_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1461_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1461` writer - B1461"]
pub struct B1461_W<'a> {
    w: &'a mut W,
}
impl<'a> B1461_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1462` reader - B1462"]
pub struct B1462_R(crate::FieldReader<bool, bool>);
impl B1462_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1462_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1462_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1462` writer - B1462"]
pub struct B1462_W<'a> {
    w: &'a mut W,
}
impl<'a> B1462_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1463` reader - B1463"]
pub struct B1463_R(crate::FieldReader<bool, bool>);
impl B1463_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1463_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1463_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1463` writer - B1463"]
pub struct B1463_W<'a> {
    w: &'a mut W,
}
impl<'a> B1463_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1464` reader - B1464"]
pub struct B1464_R(crate::FieldReader<bool, bool>);
impl B1464_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1464_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1464_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1464` writer - B1464"]
pub struct B1464_W<'a> {
    w: &'a mut W,
}
impl<'a> B1464_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1465` reader - B1465"]
pub struct B1465_R(crate::FieldReader<bool, bool>);
impl B1465_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1465_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1465_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1465` writer - B1465"]
pub struct B1465_W<'a> {
    w: &'a mut W,
}
impl<'a> B1465_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1466` reader - B1466"]
pub struct B1466_R(crate::FieldReader<bool, bool>);
impl B1466_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1466_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1466_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1466` writer - B1466"]
pub struct B1466_W<'a> {
    w: &'a mut W,
}
impl<'a> B1466_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1467` reader - B1467"]
pub struct B1467_R(crate::FieldReader<bool, bool>);
impl B1467_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1467_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1467_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1467` writer - B1467"]
pub struct B1467_W<'a> {
    w: &'a mut W,
}
impl<'a> B1467_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1468` reader - B1468"]
pub struct B1468_R(crate::FieldReader<bool, bool>);
impl B1468_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1468_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1468_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1468` writer - B1468"]
pub struct B1468_W<'a> {
    w: &'a mut W,
}
impl<'a> B1468_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1469` reader - B1469"]
pub struct B1469_R(crate::FieldReader<bool, bool>);
impl B1469_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1469_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1469_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1469` writer - B1469"]
pub struct B1469_W<'a> {
    w: &'a mut W,
}
impl<'a> B1469_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1470` reader - B1470"]
pub struct B1470_R(crate::FieldReader<bool, bool>);
impl B1470_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1470_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1470_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1470` writer - B1470"]
pub struct B1470_W<'a> {
    w: &'a mut W,
}
impl<'a> B1470_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1471` reader - B1471"]
pub struct B1471_R(crate::FieldReader<bool, bool>);
impl B1471_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1471_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1471_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1471` writer - B1471"]
pub struct B1471_W<'a> {
    w: &'a mut W,
}
impl<'a> B1471_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1440"]
    #[inline(always)]
    pub fn b1440(&self) -> B1440_R {
        B1440_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1441"]
    #[inline(always)]
    pub fn b1441(&self) -> B1441_R {
        B1441_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1442"]
    #[inline(always)]
    pub fn b1442(&self) -> B1442_R {
        B1442_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1443"]
    #[inline(always)]
    pub fn b1443(&self) -> B1443_R {
        B1443_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1444"]
    #[inline(always)]
    pub fn b1444(&self) -> B1444_R {
        B1444_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1445"]
    #[inline(always)]
    pub fn b1445(&self) -> B1445_R {
        B1445_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1446"]
    #[inline(always)]
    pub fn b1446(&self) -> B1446_R {
        B1446_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1447"]
    #[inline(always)]
    pub fn b1447(&self) -> B1447_R {
        B1447_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1448"]
    #[inline(always)]
    pub fn b1448(&self) -> B1448_R {
        B1448_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1449"]
    #[inline(always)]
    pub fn b1449(&self) -> B1449_R {
        B1449_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1450"]
    #[inline(always)]
    pub fn b1450(&self) -> B1450_R {
        B1450_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1451"]
    #[inline(always)]
    pub fn b1451(&self) -> B1451_R {
        B1451_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1452"]
    #[inline(always)]
    pub fn b1452(&self) -> B1452_R {
        B1452_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1453"]
    #[inline(always)]
    pub fn b1453(&self) -> B1453_R {
        B1453_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1454"]
    #[inline(always)]
    pub fn b1454(&self) -> B1454_R {
        B1454_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1455"]
    #[inline(always)]
    pub fn b1455(&self) -> B1455_R {
        B1455_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1456"]
    #[inline(always)]
    pub fn b1456(&self) -> B1456_R {
        B1456_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1457"]
    #[inline(always)]
    pub fn b1457(&self) -> B1457_R {
        B1457_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1458"]
    #[inline(always)]
    pub fn b1458(&self) -> B1458_R {
        B1458_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1459"]
    #[inline(always)]
    pub fn b1459(&self) -> B1459_R {
        B1459_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1460"]
    #[inline(always)]
    pub fn b1460(&self) -> B1460_R {
        B1460_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1461"]
    #[inline(always)]
    pub fn b1461(&self) -> B1461_R {
        B1461_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1462"]
    #[inline(always)]
    pub fn b1462(&self) -> B1462_R {
        B1462_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1463"]
    #[inline(always)]
    pub fn b1463(&self) -> B1463_R {
        B1463_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1464"]
    #[inline(always)]
    pub fn b1464(&self) -> B1464_R {
        B1464_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1465"]
    #[inline(always)]
    pub fn b1465(&self) -> B1465_R {
        B1465_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1466"]
    #[inline(always)]
    pub fn b1466(&self) -> B1466_R {
        B1466_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1467"]
    #[inline(always)]
    pub fn b1467(&self) -> B1467_R {
        B1467_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1468"]
    #[inline(always)]
    pub fn b1468(&self) -> B1468_R {
        B1468_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1469"]
    #[inline(always)]
    pub fn b1469(&self) -> B1469_R {
        B1469_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1470"]
    #[inline(always)]
    pub fn b1470(&self) -> B1470_R {
        B1470_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1471"]
    #[inline(always)]
    pub fn b1471(&self) -> B1471_R {
        B1471_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1440"]
    #[inline(always)]
    pub fn b1440(&mut self) -> B1440_W {
        B1440_W { w: self }
    }
    #[doc = "Bit 1 - B1441"]
    #[inline(always)]
    pub fn b1441(&mut self) -> B1441_W {
        B1441_W { w: self }
    }
    #[doc = "Bit 2 - B1442"]
    #[inline(always)]
    pub fn b1442(&mut self) -> B1442_W {
        B1442_W { w: self }
    }
    #[doc = "Bit 3 - B1443"]
    #[inline(always)]
    pub fn b1443(&mut self) -> B1443_W {
        B1443_W { w: self }
    }
    #[doc = "Bit 4 - B1444"]
    #[inline(always)]
    pub fn b1444(&mut self) -> B1444_W {
        B1444_W { w: self }
    }
    #[doc = "Bit 5 - B1445"]
    #[inline(always)]
    pub fn b1445(&mut self) -> B1445_W {
        B1445_W { w: self }
    }
    #[doc = "Bit 6 - B1446"]
    #[inline(always)]
    pub fn b1446(&mut self) -> B1446_W {
        B1446_W { w: self }
    }
    #[doc = "Bit 7 - B1447"]
    #[inline(always)]
    pub fn b1447(&mut self) -> B1447_W {
        B1447_W { w: self }
    }
    #[doc = "Bit 8 - B1448"]
    #[inline(always)]
    pub fn b1448(&mut self) -> B1448_W {
        B1448_W { w: self }
    }
    #[doc = "Bit 9 - B1449"]
    #[inline(always)]
    pub fn b1449(&mut self) -> B1449_W {
        B1449_W { w: self }
    }
    #[doc = "Bit 10 - B1450"]
    #[inline(always)]
    pub fn b1450(&mut self) -> B1450_W {
        B1450_W { w: self }
    }
    #[doc = "Bit 11 - B1451"]
    #[inline(always)]
    pub fn b1451(&mut self) -> B1451_W {
        B1451_W { w: self }
    }
    #[doc = "Bit 12 - B1452"]
    #[inline(always)]
    pub fn b1452(&mut self) -> B1452_W {
        B1452_W { w: self }
    }
    #[doc = "Bit 13 - B1453"]
    #[inline(always)]
    pub fn b1453(&mut self) -> B1453_W {
        B1453_W { w: self }
    }
    #[doc = "Bit 14 - B1454"]
    #[inline(always)]
    pub fn b1454(&mut self) -> B1454_W {
        B1454_W { w: self }
    }
    #[doc = "Bit 15 - B1455"]
    #[inline(always)]
    pub fn b1455(&mut self) -> B1455_W {
        B1455_W { w: self }
    }
    #[doc = "Bit 16 - B1456"]
    #[inline(always)]
    pub fn b1456(&mut self) -> B1456_W {
        B1456_W { w: self }
    }
    #[doc = "Bit 17 - B1457"]
    #[inline(always)]
    pub fn b1457(&mut self) -> B1457_W {
        B1457_W { w: self }
    }
    #[doc = "Bit 18 - B1458"]
    #[inline(always)]
    pub fn b1458(&mut self) -> B1458_W {
        B1458_W { w: self }
    }
    #[doc = "Bit 19 - B1459"]
    #[inline(always)]
    pub fn b1459(&mut self) -> B1459_W {
        B1459_W { w: self }
    }
    #[doc = "Bit 20 - B1460"]
    #[inline(always)]
    pub fn b1460(&mut self) -> B1460_W {
        B1460_W { w: self }
    }
    #[doc = "Bit 21 - B1461"]
    #[inline(always)]
    pub fn b1461(&mut self) -> B1461_W {
        B1461_W { w: self }
    }
    #[doc = "Bit 22 - B1462"]
    #[inline(always)]
    pub fn b1462(&mut self) -> B1462_W {
        B1462_W { w: self }
    }
    #[doc = "Bit 23 - B1463"]
    #[inline(always)]
    pub fn b1463(&mut self) -> B1463_W {
        B1463_W { w: self }
    }
    #[doc = "Bit 24 - B1464"]
    #[inline(always)]
    pub fn b1464(&mut self) -> B1464_W {
        B1464_W { w: self }
    }
    #[doc = "Bit 25 - B1465"]
    #[inline(always)]
    pub fn b1465(&mut self) -> B1465_W {
        B1465_W { w: self }
    }
    #[doc = "Bit 26 - B1466"]
    #[inline(always)]
    pub fn b1466(&mut self) -> B1466_W {
        B1466_W { w: self }
    }
    #[doc = "Bit 27 - B1467"]
    #[inline(always)]
    pub fn b1467(&mut self) -> B1467_W {
        B1467_W { w: self }
    }
    #[doc = "Bit 28 - B1468"]
    #[inline(always)]
    pub fn b1468(&mut self) -> B1468_W {
        B1468_W { w: self }
    }
    #[doc = "Bit 29 - B1469"]
    #[inline(always)]
    pub fn b1469(&mut self) -> B1469_W {
        B1469_W { w: self }
    }
    #[doc = "Bit 30 - B1470"]
    #[inline(always)]
    pub fn b1470(&mut self) -> B1470_W {
        B1470_W { w: self }
    }
    #[doc = "Bit 31 - B1471"]
    #[inline(always)]
    pub fn b1471(&mut self) -> B1471_W {
        B1471_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr45](index.html) module"]
pub struct MPCBB2_VCTR45_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr45::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr45::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR45_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR45 to value 0"]
impl crate::Resettable for MPCBB2_VCTR45_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
