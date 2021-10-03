#[doc = "Register `MPCBB2_VCTR43` reader"]
pub struct R(crate::R<MPCBB2_VCTR43_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR43_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR43_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR43_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR43` writer"]
pub struct W(crate::W<MPCBB2_VCTR43_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR43_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR43_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR43_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1376` reader - B1376"]
pub struct B1376_R(crate::FieldReader<bool, bool>);
impl B1376_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1376_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1376_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1376` writer - B1376"]
pub struct B1376_W<'a> {
    w: &'a mut W,
}
impl<'a> B1376_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1377` reader - B1377"]
pub struct B1377_R(crate::FieldReader<bool, bool>);
impl B1377_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1377_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1377_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1377` writer - B1377"]
pub struct B1377_W<'a> {
    w: &'a mut W,
}
impl<'a> B1377_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1378` reader - B1378"]
pub struct B1378_R(crate::FieldReader<bool, bool>);
impl B1378_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1378_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1378_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1378` writer - B1378"]
pub struct B1378_W<'a> {
    w: &'a mut W,
}
impl<'a> B1378_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1379` reader - B1379"]
pub struct B1379_R(crate::FieldReader<bool, bool>);
impl B1379_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1379_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1379_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1379` writer - B1379"]
pub struct B1379_W<'a> {
    w: &'a mut W,
}
impl<'a> B1379_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1380` reader - B1380"]
pub struct B1380_R(crate::FieldReader<bool, bool>);
impl B1380_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1380_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1380_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1380` writer - B1380"]
pub struct B1380_W<'a> {
    w: &'a mut W,
}
impl<'a> B1380_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1381` reader - B1381"]
pub struct B1381_R(crate::FieldReader<bool, bool>);
impl B1381_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1381_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1381_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1381` writer - B1381"]
pub struct B1381_W<'a> {
    w: &'a mut W,
}
impl<'a> B1381_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1382` reader - B1382"]
pub struct B1382_R(crate::FieldReader<bool, bool>);
impl B1382_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1382_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1382_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1382` writer - B1382"]
pub struct B1382_W<'a> {
    w: &'a mut W,
}
impl<'a> B1382_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1383` reader - B1383"]
pub struct B1383_R(crate::FieldReader<bool, bool>);
impl B1383_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1383_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1383_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1383` writer - B1383"]
pub struct B1383_W<'a> {
    w: &'a mut W,
}
impl<'a> B1383_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1384` reader - B1384"]
pub struct B1384_R(crate::FieldReader<bool, bool>);
impl B1384_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1384_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1384_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1384` writer - B1384"]
pub struct B1384_W<'a> {
    w: &'a mut W,
}
impl<'a> B1384_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1385` reader - B1385"]
pub struct B1385_R(crate::FieldReader<bool, bool>);
impl B1385_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1385_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1385_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1385` writer - B1385"]
pub struct B1385_W<'a> {
    w: &'a mut W,
}
impl<'a> B1385_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1386` reader - B1386"]
pub struct B1386_R(crate::FieldReader<bool, bool>);
impl B1386_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1386_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1386_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1386` writer - B1386"]
pub struct B1386_W<'a> {
    w: &'a mut W,
}
impl<'a> B1386_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1387` reader - B1387"]
pub struct B1387_R(crate::FieldReader<bool, bool>);
impl B1387_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1387_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1387_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1387` writer - B1387"]
pub struct B1387_W<'a> {
    w: &'a mut W,
}
impl<'a> B1387_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1388` reader - B1388"]
pub struct B1388_R(crate::FieldReader<bool, bool>);
impl B1388_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1388_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1388_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1388` writer - B1388"]
pub struct B1388_W<'a> {
    w: &'a mut W,
}
impl<'a> B1388_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1389` reader - B1389"]
pub struct B1389_R(crate::FieldReader<bool, bool>);
impl B1389_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1389_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1389_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1389` writer - B1389"]
pub struct B1389_W<'a> {
    w: &'a mut W,
}
impl<'a> B1389_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1390` reader - B1390"]
pub struct B1390_R(crate::FieldReader<bool, bool>);
impl B1390_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1390_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1390_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1390` writer - B1390"]
pub struct B1390_W<'a> {
    w: &'a mut W,
}
impl<'a> B1390_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1391` reader - B1391"]
pub struct B1391_R(crate::FieldReader<bool, bool>);
impl B1391_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1391_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1391_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1391` writer - B1391"]
pub struct B1391_W<'a> {
    w: &'a mut W,
}
impl<'a> B1391_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1392` reader - B1392"]
pub struct B1392_R(crate::FieldReader<bool, bool>);
impl B1392_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1392_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1392_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1392` writer - B1392"]
pub struct B1392_W<'a> {
    w: &'a mut W,
}
impl<'a> B1392_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1393` reader - B1393"]
pub struct B1393_R(crate::FieldReader<bool, bool>);
impl B1393_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1393_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1393_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1393` writer - B1393"]
pub struct B1393_W<'a> {
    w: &'a mut W,
}
impl<'a> B1393_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1394` reader - B1394"]
pub struct B1394_R(crate::FieldReader<bool, bool>);
impl B1394_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1394_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1394_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1394` writer - B1394"]
pub struct B1394_W<'a> {
    w: &'a mut W,
}
impl<'a> B1394_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1395` reader - B1395"]
pub struct B1395_R(crate::FieldReader<bool, bool>);
impl B1395_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1395_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1395_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1395` writer - B1395"]
pub struct B1395_W<'a> {
    w: &'a mut W,
}
impl<'a> B1395_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1396` reader - B1396"]
pub struct B1396_R(crate::FieldReader<bool, bool>);
impl B1396_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1396_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1396_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1396` writer - B1396"]
pub struct B1396_W<'a> {
    w: &'a mut W,
}
impl<'a> B1396_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1397` reader - B1397"]
pub struct B1397_R(crate::FieldReader<bool, bool>);
impl B1397_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1397_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1397_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1397` writer - B1397"]
pub struct B1397_W<'a> {
    w: &'a mut W,
}
impl<'a> B1397_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1398` reader - B1398"]
pub struct B1398_R(crate::FieldReader<bool, bool>);
impl B1398_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1398_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1398_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1398` writer - B1398"]
pub struct B1398_W<'a> {
    w: &'a mut W,
}
impl<'a> B1398_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1399` reader - B1399"]
pub struct B1399_R(crate::FieldReader<bool, bool>);
impl B1399_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1399_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1399_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1399` writer - B1399"]
pub struct B1399_W<'a> {
    w: &'a mut W,
}
impl<'a> B1399_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1400` reader - B1400"]
pub struct B1400_R(crate::FieldReader<bool, bool>);
impl B1400_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1400_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1400_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1400` writer - B1400"]
pub struct B1400_W<'a> {
    w: &'a mut W,
}
impl<'a> B1400_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1401` reader - B1401"]
pub struct B1401_R(crate::FieldReader<bool, bool>);
impl B1401_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1401_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1401_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1401` writer - B1401"]
pub struct B1401_W<'a> {
    w: &'a mut W,
}
impl<'a> B1401_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1402` reader - B1402"]
pub struct B1402_R(crate::FieldReader<bool, bool>);
impl B1402_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1402_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1402_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1402` writer - B1402"]
pub struct B1402_W<'a> {
    w: &'a mut W,
}
impl<'a> B1402_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1403` reader - B1403"]
pub struct B1403_R(crate::FieldReader<bool, bool>);
impl B1403_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1403_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1403_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1403` writer - B1403"]
pub struct B1403_W<'a> {
    w: &'a mut W,
}
impl<'a> B1403_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1404` reader - B1404"]
pub struct B1404_R(crate::FieldReader<bool, bool>);
impl B1404_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1404_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1404_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1404` writer - B1404"]
pub struct B1404_W<'a> {
    w: &'a mut W,
}
impl<'a> B1404_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1405` reader - B1405"]
pub struct B1405_R(crate::FieldReader<bool, bool>);
impl B1405_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1405_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1405_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1405` writer - B1405"]
pub struct B1405_W<'a> {
    w: &'a mut W,
}
impl<'a> B1405_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1406` reader - B1406"]
pub struct B1406_R(crate::FieldReader<bool, bool>);
impl B1406_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1406_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1406_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1406` writer - B1406"]
pub struct B1406_W<'a> {
    w: &'a mut W,
}
impl<'a> B1406_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1407` reader - B1407"]
pub struct B1407_R(crate::FieldReader<bool, bool>);
impl B1407_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1407_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1407_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1407` writer - B1407"]
pub struct B1407_W<'a> {
    w: &'a mut W,
}
impl<'a> B1407_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1376"]
    #[inline(always)]
    pub fn b1376(&self) -> B1376_R {
        B1376_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1377"]
    #[inline(always)]
    pub fn b1377(&self) -> B1377_R {
        B1377_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1378"]
    #[inline(always)]
    pub fn b1378(&self) -> B1378_R {
        B1378_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1379"]
    #[inline(always)]
    pub fn b1379(&self) -> B1379_R {
        B1379_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1380"]
    #[inline(always)]
    pub fn b1380(&self) -> B1380_R {
        B1380_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1381"]
    #[inline(always)]
    pub fn b1381(&self) -> B1381_R {
        B1381_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1382"]
    #[inline(always)]
    pub fn b1382(&self) -> B1382_R {
        B1382_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1383"]
    #[inline(always)]
    pub fn b1383(&self) -> B1383_R {
        B1383_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1384"]
    #[inline(always)]
    pub fn b1384(&self) -> B1384_R {
        B1384_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1385"]
    #[inline(always)]
    pub fn b1385(&self) -> B1385_R {
        B1385_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1386"]
    #[inline(always)]
    pub fn b1386(&self) -> B1386_R {
        B1386_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1387"]
    #[inline(always)]
    pub fn b1387(&self) -> B1387_R {
        B1387_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1388"]
    #[inline(always)]
    pub fn b1388(&self) -> B1388_R {
        B1388_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1389"]
    #[inline(always)]
    pub fn b1389(&self) -> B1389_R {
        B1389_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1390"]
    #[inline(always)]
    pub fn b1390(&self) -> B1390_R {
        B1390_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1391"]
    #[inline(always)]
    pub fn b1391(&self) -> B1391_R {
        B1391_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1392"]
    #[inline(always)]
    pub fn b1392(&self) -> B1392_R {
        B1392_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1393"]
    #[inline(always)]
    pub fn b1393(&self) -> B1393_R {
        B1393_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1394"]
    #[inline(always)]
    pub fn b1394(&self) -> B1394_R {
        B1394_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1395"]
    #[inline(always)]
    pub fn b1395(&self) -> B1395_R {
        B1395_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1396"]
    #[inline(always)]
    pub fn b1396(&self) -> B1396_R {
        B1396_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1397"]
    #[inline(always)]
    pub fn b1397(&self) -> B1397_R {
        B1397_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1398"]
    #[inline(always)]
    pub fn b1398(&self) -> B1398_R {
        B1398_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1399"]
    #[inline(always)]
    pub fn b1399(&self) -> B1399_R {
        B1399_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1400"]
    #[inline(always)]
    pub fn b1400(&self) -> B1400_R {
        B1400_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1401"]
    #[inline(always)]
    pub fn b1401(&self) -> B1401_R {
        B1401_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1402"]
    #[inline(always)]
    pub fn b1402(&self) -> B1402_R {
        B1402_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1403"]
    #[inline(always)]
    pub fn b1403(&self) -> B1403_R {
        B1403_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1404"]
    #[inline(always)]
    pub fn b1404(&self) -> B1404_R {
        B1404_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1405"]
    #[inline(always)]
    pub fn b1405(&self) -> B1405_R {
        B1405_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1406"]
    #[inline(always)]
    pub fn b1406(&self) -> B1406_R {
        B1406_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1407"]
    #[inline(always)]
    pub fn b1407(&self) -> B1407_R {
        B1407_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1376"]
    #[inline(always)]
    pub fn b1376(&mut self) -> B1376_W {
        B1376_W { w: self }
    }
    #[doc = "Bit 1 - B1377"]
    #[inline(always)]
    pub fn b1377(&mut self) -> B1377_W {
        B1377_W { w: self }
    }
    #[doc = "Bit 2 - B1378"]
    #[inline(always)]
    pub fn b1378(&mut self) -> B1378_W {
        B1378_W { w: self }
    }
    #[doc = "Bit 3 - B1379"]
    #[inline(always)]
    pub fn b1379(&mut self) -> B1379_W {
        B1379_W { w: self }
    }
    #[doc = "Bit 4 - B1380"]
    #[inline(always)]
    pub fn b1380(&mut self) -> B1380_W {
        B1380_W { w: self }
    }
    #[doc = "Bit 5 - B1381"]
    #[inline(always)]
    pub fn b1381(&mut self) -> B1381_W {
        B1381_W { w: self }
    }
    #[doc = "Bit 6 - B1382"]
    #[inline(always)]
    pub fn b1382(&mut self) -> B1382_W {
        B1382_W { w: self }
    }
    #[doc = "Bit 7 - B1383"]
    #[inline(always)]
    pub fn b1383(&mut self) -> B1383_W {
        B1383_W { w: self }
    }
    #[doc = "Bit 8 - B1384"]
    #[inline(always)]
    pub fn b1384(&mut self) -> B1384_W {
        B1384_W { w: self }
    }
    #[doc = "Bit 9 - B1385"]
    #[inline(always)]
    pub fn b1385(&mut self) -> B1385_W {
        B1385_W { w: self }
    }
    #[doc = "Bit 10 - B1386"]
    #[inline(always)]
    pub fn b1386(&mut self) -> B1386_W {
        B1386_W { w: self }
    }
    #[doc = "Bit 11 - B1387"]
    #[inline(always)]
    pub fn b1387(&mut self) -> B1387_W {
        B1387_W { w: self }
    }
    #[doc = "Bit 12 - B1388"]
    #[inline(always)]
    pub fn b1388(&mut self) -> B1388_W {
        B1388_W { w: self }
    }
    #[doc = "Bit 13 - B1389"]
    #[inline(always)]
    pub fn b1389(&mut self) -> B1389_W {
        B1389_W { w: self }
    }
    #[doc = "Bit 14 - B1390"]
    #[inline(always)]
    pub fn b1390(&mut self) -> B1390_W {
        B1390_W { w: self }
    }
    #[doc = "Bit 15 - B1391"]
    #[inline(always)]
    pub fn b1391(&mut self) -> B1391_W {
        B1391_W { w: self }
    }
    #[doc = "Bit 16 - B1392"]
    #[inline(always)]
    pub fn b1392(&mut self) -> B1392_W {
        B1392_W { w: self }
    }
    #[doc = "Bit 17 - B1393"]
    #[inline(always)]
    pub fn b1393(&mut self) -> B1393_W {
        B1393_W { w: self }
    }
    #[doc = "Bit 18 - B1394"]
    #[inline(always)]
    pub fn b1394(&mut self) -> B1394_W {
        B1394_W { w: self }
    }
    #[doc = "Bit 19 - B1395"]
    #[inline(always)]
    pub fn b1395(&mut self) -> B1395_W {
        B1395_W { w: self }
    }
    #[doc = "Bit 20 - B1396"]
    #[inline(always)]
    pub fn b1396(&mut self) -> B1396_W {
        B1396_W { w: self }
    }
    #[doc = "Bit 21 - B1397"]
    #[inline(always)]
    pub fn b1397(&mut self) -> B1397_W {
        B1397_W { w: self }
    }
    #[doc = "Bit 22 - B1398"]
    #[inline(always)]
    pub fn b1398(&mut self) -> B1398_W {
        B1398_W { w: self }
    }
    #[doc = "Bit 23 - B1399"]
    #[inline(always)]
    pub fn b1399(&mut self) -> B1399_W {
        B1399_W { w: self }
    }
    #[doc = "Bit 24 - B1400"]
    #[inline(always)]
    pub fn b1400(&mut self) -> B1400_W {
        B1400_W { w: self }
    }
    #[doc = "Bit 25 - B1401"]
    #[inline(always)]
    pub fn b1401(&mut self) -> B1401_W {
        B1401_W { w: self }
    }
    #[doc = "Bit 26 - B1402"]
    #[inline(always)]
    pub fn b1402(&mut self) -> B1402_W {
        B1402_W { w: self }
    }
    #[doc = "Bit 27 - B1403"]
    #[inline(always)]
    pub fn b1403(&mut self) -> B1403_W {
        B1403_W { w: self }
    }
    #[doc = "Bit 28 - B1404"]
    #[inline(always)]
    pub fn b1404(&mut self) -> B1404_W {
        B1404_W { w: self }
    }
    #[doc = "Bit 29 - B1405"]
    #[inline(always)]
    pub fn b1405(&mut self) -> B1405_W {
        B1405_W { w: self }
    }
    #[doc = "Bit 30 - B1406"]
    #[inline(always)]
    pub fn b1406(&mut self) -> B1406_W {
        B1406_W { w: self }
    }
    #[doc = "Bit 31 - B1407"]
    #[inline(always)]
    pub fn b1407(&mut self) -> B1407_W {
        B1407_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr43](index.html) module"]
pub struct MPCBB2_VCTR43_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr43::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR43_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr43::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR43_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR43 to value 0"]
impl crate::Resettable for MPCBB2_VCTR43_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
