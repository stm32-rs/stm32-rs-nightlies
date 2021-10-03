#[doc = "Register `MPCBB2_VCTR50` reader"]
pub struct R(crate::R<MPCBB2_VCTR50_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR50_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR50_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR50_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR50` writer"]
pub struct W(crate::W<MPCBB2_VCTR50_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR50_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR50_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR50_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1600` reader - B1600"]
pub struct B1600_R(crate::FieldReader<bool, bool>);
impl B1600_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1600_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1600_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1600` writer - B1600"]
pub struct B1600_W<'a> {
    w: &'a mut W,
}
impl<'a> B1600_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1601` reader - B1601"]
pub struct B1601_R(crate::FieldReader<bool, bool>);
impl B1601_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1601_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1601_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1601` writer - B1601"]
pub struct B1601_W<'a> {
    w: &'a mut W,
}
impl<'a> B1601_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1602` reader - B1602"]
pub struct B1602_R(crate::FieldReader<bool, bool>);
impl B1602_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1602_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1602_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1602` writer - B1602"]
pub struct B1602_W<'a> {
    w: &'a mut W,
}
impl<'a> B1602_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1603` reader - B1603"]
pub struct B1603_R(crate::FieldReader<bool, bool>);
impl B1603_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1603_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1603_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1603` writer - B1603"]
pub struct B1603_W<'a> {
    w: &'a mut W,
}
impl<'a> B1603_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1604` reader - B1604"]
pub struct B1604_R(crate::FieldReader<bool, bool>);
impl B1604_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1604_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1604_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1604` writer - B1604"]
pub struct B1604_W<'a> {
    w: &'a mut W,
}
impl<'a> B1604_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1605` reader - B1605"]
pub struct B1605_R(crate::FieldReader<bool, bool>);
impl B1605_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1605_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1605_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1605` writer - B1605"]
pub struct B1605_W<'a> {
    w: &'a mut W,
}
impl<'a> B1605_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1606` reader - B1606"]
pub struct B1606_R(crate::FieldReader<bool, bool>);
impl B1606_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1606_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1606_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1606` writer - B1606"]
pub struct B1606_W<'a> {
    w: &'a mut W,
}
impl<'a> B1606_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1607` reader - B1607"]
pub struct B1607_R(crate::FieldReader<bool, bool>);
impl B1607_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1607_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1607_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1607` writer - B1607"]
pub struct B1607_W<'a> {
    w: &'a mut W,
}
impl<'a> B1607_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1608` reader - B1608"]
pub struct B1608_R(crate::FieldReader<bool, bool>);
impl B1608_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1608_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1608_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1608` writer - B1608"]
pub struct B1608_W<'a> {
    w: &'a mut W,
}
impl<'a> B1608_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1609` reader - B1609"]
pub struct B1609_R(crate::FieldReader<bool, bool>);
impl B1609_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1609_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1609_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1609` writer - B1609"]
pub struct B1609_W<'a> {
    w: &'a mut W,
}
impl<'a> B1609_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1610` reader - B1610"]
pub struct B1610_R(crate::FieldReader<bool, bool>);
impl B1610_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1610_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1610_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1610` writer - B1610"]
pub struct B1610_W<'a> {
    w: &'a mut W,
}
impl<'a> B1610_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1611` reader - B1611"]
pub struct B1611_R(crate::FieldReader<bool, bool>);
impl B1611_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1611_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1611_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1611` writer - B1611"]
pub struct B1611_W<'a> {
    w: &'a mut W,
}
impl<'a> B1611_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1612` reader - B1612"]
pub struct B1612_R(crate::FieldReader<bool, bool>);
impl B1612_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1612_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1612_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1612` writer - B1612"]
pub struct B1612_W<'a> {
    w: &'a mut W,
}
impl<'a> B1612_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1613` reader - B1613"]
pub struct B1613_R(crate::FieldReader<bool, bool>);
impl B1613_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1613_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1613_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1613` writer - B1613"]
pub struct B1613_W<'a> {
    w: &'a mut W,
}
impl<'a> B1613_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1614` reader - B1614"]
pub struct B1614_R(crate::FieldReader<bool, bool>);
impl B1614_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1614_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1614_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1614` writer - B1614"]
pub struct B1614_W<'a> {
    w: &'a mut W,
}
impl<'a> B1614_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1615` reader - B1615"]
pub struct B1615_R(crate::FieldReader<bool, bool>);
impl B1615_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1615_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1615_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1615` writer - B1615"]
pub struct B1615_W<'a> {
    w: &'a mut W,
}
impl<'a> B1615_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1616` reader - B1616"]
pub struct B1616_R(crate::FieldReader<bool, bool>);
impl B1616_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1616_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1616_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1616` writer - B1616"]
pub struct B1616_W<'a> {
    w: &'a mut W,
}
impl<'a> B1616_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1617` reader - B1617"]
pub struct B1617_R(crate::FieldReader<bool, bool>);
impl B1617_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1617_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1617_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1617` writer - B1617"]
pub struct B1617_W<'a> {
    w: &'a mut W,
}
impl<'a> B1617_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1618` reader - B1618"]
pub struct B1618_R(crate::FieldReader<bool, bool>);
impl B1618_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1618_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1618_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1618` writer - B1618"]
pub struct B1618_W<'a> {
    w: &'a mut W,
}
impl<'a> B1618_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1619` reader - B1619"]
pub struct B1619_R(crate::FieldReader<bool, bool>);
impl B1619_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1619_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1619_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1619` writer - B1619"]
pub struct B1619_W<'a> {
    w: &'a mut W,
}
impl<'a> B1619_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1620` reader - B1620"]
pub struct B1620_R(crate::FieldReader<bool, bool>);
impl B1620_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1620_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1620_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1620` writer - B1620"]
pub struct B1620_W<'a> {
    w: &'a mut W,
}
impl<'a> B1620_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1621` reader - B1621"]
pub struct B1621_R(crate::FieldReader<bool, bool>);
impl B1621_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1621_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1621_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1621` writer - B1621"]
pub struct B1621_W<'a> {
    w: &'a mut W,
}
impl<'a> B1621_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1622` reader - B1622"]
pub struct B1622_R(crate::FieldReader<bool, bool>);
impl B1622_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1622_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1622_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1622` writer - B1622"]
pub struct B1622_W<'a> {
    w: &'a mut W,
}
impl<'a> B1622_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1623` reader - B1623"]
pub struct B1623_R(crate::FieldReader<bool, bool>);
impl B1623_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1623_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1623_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1623` writer - B1623"]
pub struct B1623_W<'a> {
    w: &'a mut W,
}
impl<'a> B1623_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1624` reader - B1624"]
pub struct B1624_R(crate::FieldReader<bool, bool>);
impl B1624_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1624_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1624_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1624` writer - B1624"]
pub struct B1624_W<'a> {
    w: &'a mut W,
}
impl<'a> B1624_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1625` reader - B1625"]
pub struct B1625_R(crate::FieldReader<bool, bool>);
impl B1625_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1625_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1625_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1625` writer - B1625"]
pub struct B1625_W<'a> {
    w: &'a mut W,
}
impl<'a> B1625_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1626` reader - B1626"]
pub struct B1626_R(crate::FieldReader<bool, bool>);
impl B1626_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1626_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1626_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1626` writer - B1626"]
pub struct B1626_W<'a> {
    w: &'a mut W,
}
impl<'a> B1626_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1627` reader - B1627"]
pub struct B1627_R(crate::FieldReader<bool, bool>);
impl B1627_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1627_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1627_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1627` writer - B1627"]
pub struct B1627_W<'a> {
    w: &'a mut W,
}
impl<'a> B1627_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1628` reader - B1628"]
pub struct B1628_R(crate::FieldReader<bool, bool>);
impl B1628_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1628_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1628_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1628` writer - B1628"]
pub struct B1628_W<'a> {
    w: &'a mut W,
}
impl<'a> B1628_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1629` reader - B1629"]
pub struct B1629_R(crate::FieldReader<bool, bool>);
impl B1629_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1629_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1629_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1629` writer - B1629"]
pub struct B1629_W<'a> {
    w: &'a mut W,
}
impl<'a> B1629_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1630` reader - B1630"]
pub struct B1630_R(crate::FieldReader<bool, bool>);
impl B1630_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1630_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1630_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1630` writer - B1630"]
pub struct B1630_W<'a> {
    w: &'a mut W,
}
impl<'a> B1630_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1631` reader - B1631"]
pub struct B1631_R(crate::FieldReader<bool, bool>);
impl B1631_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1631_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1631_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1631` writer - B1631"]
pub struct B1631_W<'a> {
    w: &'a mut W,
}
impl<'a> B1631_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1600"]
    #[inline(always)]
    pub fn b1600(&self) -> B1600_R {
        B1600_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1601"]
    #[inline(always)]
    pub fn b1601(&self) -> B1601_R {
        B1601_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1602"]
    #[inline(always)]
    pub fn b1602(&self) -> B1602_R {
        B1602_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1603"]
    #[inline(always)]
    pub fn b1603(&self) -> B1603_R {
        B1603_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1604"]
    #[inline(always)]
    pub fn b1604(&self) -> B1604_R {
        B1604_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1605"]
    #[inline(always)]
    pub fn b1605(&self) -> B1605_R {
        B1605_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1606"]
    #[inline(always)]
    pub fn b1606(&self) -> B1606_R {
        B1606_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1607"]
    #[inline(always)]
    pub fn b1607(&self) -> B1607_R {
        B1607_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1608"]
    #[inline(always)]
    pub fn b1608(&self) -> B1608_R {
        B1608_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1609"]
    #[inline(always)]
    pub fn b1609(&self) -> B1609_R {
        B1609_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1610"]
    #[inline(always)]
    pub fn b1610(&self) -> B1610_R {
        B1610_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1611"]
    #[inline(always)]
    pub fn b1611(&self) -> B1611_R {
        B1611_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1612"]
    #[inline(always)]
    pub fn b1612(&self) -> B1612_R {
        B1612_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1613"]
    #[inline(always)]
    pub fn b1613(&self) -> B1613_R {
        B1613_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1614"]
    #[inline(always)]
    pub fn b1614(&self) -> B1614_R {
        B1614_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1615"]
    #[inline(always)]
    pub fn b1615(&self) -> B1615_R {
        B1615_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1616"]
    #[inline(always)]
    pub fn b1616(&self) -> B1616_R {
        B1616_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1617"]
    #[inline(always)]
    pub fn b1617(&self) -> B1617_R {
        B1617_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1618"]
    #[inline(always)]
    pub fn b1618(&self) -> B1618_R {
        B1618_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1619"]
    #[inline(always)]
    pub fn b1619(&self) -> B1619_R {
        B1619_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1620"]
    #[inline(always)]
    pub fn b1620(&self) -> B1620_R {
        B1620_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1621"]
    #[inline(always)]
    pub fn b1621(&self) -> B1621_R {
        B1621_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1622"]
    #[inline(always)]
    pub fn b1622(&self) -> B1622_R {
        B1622_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1623"]
    #[inline(always)]
    pub fn b1623(&self) -> B1623_R {
        B1623_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1624"]
    #[inline(always)]
    pub fn b1624(&self) -> B1624_R {
        B1624_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1625"]
    #[inline(always)]
    pub fn b1625(&self) -> B1625_R {
        B1625_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1626"]
    #[inline(always)]
    pub fn b1626(&self) -> B1626_R {
        B1626_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1627"]
    #[inline(always)]
    pub fn b1627(&self) -> B1627_R {
        B1627_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1628"]
    #[inline(always)]
    pub fn b1628(&self) -> B1628_R {
        B1628_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1629"]
    #[inline(always)]
    pub fn b1629(&self) -> B1629_R {
        B1629_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1630"]
    #[inline(always)]
    pub fn b1630(&self) -> B1630_R {
        B1630_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1631"]
    #[inline(always)]
    pub fn b1631(&self) -> B1631_R {
        B1631_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1600"]
    #[inline(always)]
    pub fn b1600(&mut self) -> B1600_W {
        B1600_W { w: self }
    }
    #[doc = "Bit 1 - B1601"]
    #[inline(always)]
    pub fn b1601(&mut self) -> B1601_W {
        B1601_W { w: self }
    }
    #[doc = "Bit 2 - B1602"]
    #[inline(always)]
    pub fn b1602(&mut self) -> B1602_W {
        B1602_W { w: self }
    }
    #[doc = "Bit 3 - B1603"]
    #[inline(always)]
    pub fn b1603(&mut self) -> B1603_W {
        B1603_W { w: self }
    }
    #[doc = "Bit 4 - B1604"]
    #[inline(always)]
    pub fn b1604(&mut self) -> B1604_W {
        B1604_W { w: self }
    }
    #[doc = "Bit 5 - B1605"]
    #[inline(always)]
    pub fn b1605(&mut self) -> B1605_W {
        B1605_W { w: self }
    }
    #[doc = "Bit 6 - B1606"]
    #[inline(always)]
    pub fn b1606(&mut self) -> B1606_W {
        B1606_W { w: self }
    }
    #[doc = "Bit 7 - B1607"]
    #[inline(always)]
    pub fn b1607(&mut self) -> B1607_W {
        B1607_W { w: self }
    }
    #[doc = "Bit 8 - B1608"]
    #[inline(always)]
    pub fn b1608(&mut self) -> B1608_W {
        B1608_W { w: self }
    }
    #[doc = "Bit 9 - B1609"]
    #[inline(always)]
    pub fn b1609(&mut self) -> B1609_W {
        B1609_W { w: self }
    }
    #[doc = "Bit 10 - B1610"]
    #[inline(always)]
    pub fn b1610(&mut self) -> B1610_W {
        B1610_W { w: self }
    }
    #[doc = "Bit 11 - B1611"]
    #[inline(always)]
    pub fn b1611(&mut self) -> B1611_W {
        B1611_W { w: self }
    }
    #[doc = "Bit 12 - B1612"]
    #[inline(always)]
    pub fn b1612(&mut self) -> B1612_W {
        B1612_W { w: self }
    }
    #[doc = "Bit 13 - B1613"]
    #[inline(always)]
    pub fn b1613(&mut self) -> B1613_W {
        B1613_W { w: self }
    }
    #[doc = "Bit 14 - B1614"]
    #[inline(always)]
    pub fn b1614(&mut self) -> B1614_W {
        B1614_W { w: self }
    }
    #[doc = "Bit 15 - B1615"]
    #[inline(always)]
    pub fn b1615(&mut self) -> B1615_W {
        B1615_W { w: self }
    }
    #[doc = "Bit 16 - B1616"]
    #[inline(always)]
    pub fn b1616(&mut self) -> B1616_W {
        B1616_W { w: self }
    }
    #[doc = "Bit 17 - B1617"]
    #[inline(always)]
    pub fn b1617(&mut self) -> B1617_W {
        B1617_W { w: self }
    }
    #[doc = "Bit 18 - B1618"]
    #[inline(always)]
    pub fn b1618(&mut self) -> B1618_W {
        B1618_W { w: self }
    }
    #[doc = "Bit 19 - B1619"]
    #[inline(always)]
    pub fn b1619(&mut self) -> B1619_W {
        B1619_W { w: self }
    }
    #[doc = "Bit 20 - B1620"]
    #[inline(always)]
    pub fn b1620(&mut self) -> B1620_W {
        B1620_W { w: self }
    }
    #[doc = "Bit 21 - B1621"]
    #[inline(always)]
    pub fn b1621(&mut self) -> B1621_W {
        B1621_W { w: self }
    }
    #[doc = "Bit 22 - B1622"]
    #[inline(always)]
    pub fn b1622(&mut self) -> B1622_W {
        B1622_W { w: self }
    }
    #[doc = "Bit 23 - B1623"]
    #[inline(always)]
    pub fn b1623(&mut self) -> B1623_W {
        B1623_W { w: self }
    }
    #[doc = "Bit 24 - B1624"]
    #[inline(always)]
    pub fn b1624(&mut self) -> B1624_W {
        B1624_W { w: self }
    }
    #[doc = "Bit 25 - B1625"]
    #[inline(always)]
    pub fn b1625(&mut self) -> B1625_W {
        B1625_W { w: self }
    }
    #[doc = "Bit 26 - B1626"]
    #[inline(always)]
    pub fn b1626(&mut self) -> B1626_W {
        B1626_W { w: self }
    }
    #[doc = "Bit 27 - B1627"]
    #[inline(always)]
    pub fn b1627(&mut self) -> B1627_W {
        B1627_W { w: self }
    }
    #[doc = "Bit 28 - B1628"]
    #[inline(always)]
    pub fn b1628(&mut self) -> B1628_W {
        B1628_W { w: self }
    }
    #[doc = "Bit 29 - B1629"]
    #[inline(always)]
    pub fn b1629(&mut self) -> B1629_W {
        B1629_W { w: self }
    }
    #[doc = "Bit 30 - B1630"]
    #[inline(always)]
    pub fn b1630(&mut self) -> B1630_W {
        B1630_W { w: self }
    }
    #[doc = "Bit 31 - B1631"]
    #[inline(always)]
    pub fn b1631(&mut self) -> B1631_W {
        B1631_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr50](index.html) module"]
pub struct MPCBB2_VCTR50_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR50_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr50::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR50_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr50::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR50_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR50 to value 0"]
impl crate::Resettable for MPCBB2_VCTR50_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
