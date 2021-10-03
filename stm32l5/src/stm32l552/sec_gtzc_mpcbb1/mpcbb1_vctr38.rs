#[doc = "Register `MPCBB1_VCTR38` reader"]
pub struct R(crate::R<MPCBB1_VCTR38_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR38_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR38_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR38_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR38` writer"]
pub struct W(crate::W<MPCBB1_VCTR38_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR38_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR38_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR38_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1216` reader - B1216"]
pub struct B1216_R(crate::FieldReader<bool, bool>);
impl B1216_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1216_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1216_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1216` writer - B1216"]
pub struct B1216_W<'a> {
    w: &'a mut W,
}
impl<'a> B1216_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1217` reader - B1217"]
pub struct B1217_R(crate::FieldReader<bool, bool>);
impl B1217_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1217_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1217_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1217` writer - B1217"]
pub struct B1217_W<'a> {
    w: &'a mut W,
}
impl<'a> B1217_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1218` reader - B1218"]
pub struct B1218_R(crate::FieldReader<bool, bool>);
impl B1218_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1218_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1218_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1218` writer - B1218"]
pub struct B1218_W<'a> {
    w: &'a mut W,
}
impl<'a> B1218_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1219` reader - B1219"]
pub struct B1219_R(crate::FieldReader<bool, bool>);
impl B1219_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1219_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1219_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1219` writer - B1219"]
pub struct B1219_W<'a> {
    w: &'a mut W,
}
impl<'a> B1219_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1220` reader - B1220"]
pub struct B1220_R(crate::FieldReader<bool, bool>);
impl B1220_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1220_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1220_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1220` writer - B1220"]
pub struct B1220_W<'a> {
    w: &'a mut W,
}
impl<'a> B1220_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1221` reader - B1221"]
pub struct B1221_R(crate::FieldReader<bool, bool>);
impl B1221_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1221_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1221_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1221` writer - B1221"]
pub struct B1221_W<'a> {
    w: &'a mut W,
}
impl<'a> B1221_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1222` reader - B1222"]
pub struct B1222_R(crate::FieldReader<bool, bool>);
impl B1222_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1222_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1222_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1222` writer - B1222"]
pub struct B1222_W<'a> {
    w: &'a mut W,
}
impl<'a> B1222_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1223` reader - B1223"]
pub struct B1223_R(crate::FieldReader<bool, bool>);
impl B1223_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1223_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1223_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1223` writer - B1223"]
pub struct B1223_W<'a> {
    w: &'a mut W,
}
impl<'a> B1223_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1224` reader - B1224"]
pub struct B1224_R(crate::FieldReader<bool, bool>);
impl B1224_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1224_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1224_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1224` writer - B1224"]
pub struct B1224_W<'a> {
    w: &'a mut W,
}
impl<'a> B1224_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1225` reader - B1225"]
pub struct B1225_R(crate::FieldReader<bool, bool>);
impl B1225_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1225_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1225_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1225` writer - B1225"]
pub struct B1225_W<'a> {
    w: &'a mut W,
}
impl<'a> B1225_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1226` reader - B1226"]
pub struct B1226_R(crate::FieldReader<bool, bool>);
impl B1226_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1226_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1226_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1226` writer - B1226"]
pub struct B1226_W<'a> {
    w: &'a mut W,
}
impl<'a> B1226_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1227` reader - B1227"]
pub struct B1227_R(crate::FieldReader<bool, bool>);
impl B1227_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1227_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1227_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1227` writer - B1227"]
pub struct B1227_W<'a> {
    w: &'a mut W,
}
impl<'a> B1227_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1228` reader - B1228"]
pub struct B1228_R(crate::FieldReader<bool, bool>);
impl B1228_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1228_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1228_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1228` writer - B1228"]
pub struct B1228_W<'a> {
    w: &'a mut W,
}
impl<'a> B1228_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1229` reader - B1229"]
pub struct B1229_R(crate::FieldReader<bool, bool>);
impl B1229_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1229_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1229_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1229` writer - B1229"]
pub struct B1229_W<'a> {
    w: &'a mut W,
}
impl<'a> B1229_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1230` reader - B1230"]
pub struct B1230_R(crate::FieldReader<bool, bool>);
impl B1230_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1230_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1230_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1230` writer - B1230"]
pub struct B1230_W<'a> {
    w: &'a mut W,
}
impl<'a> B1230_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1231` reader - B1231"]
pub struct B1231_R(crate::FieldReader<bool, bool>);
impl B1231_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1231_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1231_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1231` writer - B1231"]
pub struct B1231_W<'a> {
    w: &'a mut W,
}
impl<'a> B1231_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1232` reader - B1232"]
pub struct B1232_R(crate::FieldReader<bool, bool>);
impl B1232_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1232_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1232_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1232` writer - B1232"]
pub struct B1232_W<'a> {
    w: &'a mut W,
}
impl<'a> B1232_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1233` reader - B1233"]
pub struct B1233_R(crate::FieldReader<bool, bool>);
impl B1233_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1233_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1233_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1233` writer - B1233"]
pub struct B1233_W<'a> {
    w: &'a mut W,
}
impl<'a> B1233_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1234` reader - B1234"]
pub struct B1234_R(crate::FieldReader<bool, bool>);
impl B1234_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1234_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1234_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1234` writer - B1234"]
pub struct B1234_W<'a> {
    w: &'a mut W,
}
impl<'a> B1234_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1235` reader - B1235"]
pub struct B1235_R(crate::FieldReader<bool, bool>);
impl B1235_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1235_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1235_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1235` writer - B1235"]
pub struct B1235_W<'a> {
    w: &'a mut W,
}
impl<'a> B1235_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1236` reader - B1236"]
pub struct B1236_R(crate::FieldReader<bool, bool>);
impl B1236_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1236_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1236_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1236` writer - B1236"]
pub struct B1236_W<'a> {
    w: &'a mut W,
}
impl<'a> B1236_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1237` reader - B1237"]
pub struct B1237_R(crate::FieldReader<bool, bool>);
impl B1237_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1237_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1237_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1237` writer - B1237"]
pub struct B1237_W<'a> {
    w: &'a mut W,
}
impl<'a> B1237_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1238` reader - B1238"]
pub struct B1238_R(crate::FieldReader<bool, bool>);
impl B1238_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1238_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1238_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1238` writer - B1238"]
pub struct B1238_W<'a> {
    w: &'a mut W,
}
impl<'a> B1238_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1239` reader - B1239"]
pub struct B1239_R(crate::FieldReader<bool, bool>);
impl B1239_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1239_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1239_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1239` writer - B1239"]
pub struct B1239_W<'a> {
    w: &'a mut W,
}
impl<'a> B1239_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1240` reader - B1240"]
pub struct B1240_R(crate::FieldReader<bool, bool>);
impl B1240_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1240_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1240_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1240` writer - B1240"]
pub struct B1240_W<'a> {
    w: &'a mut W,
}
impl<'a> B1240_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1241` reader - B1241"]
pub struct B1241_R(crate::FieldReader<bool, bool>);
impl B1241_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1241_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1241_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1241` writer - B1241"]
pub struct B1241_W<'a> {
    w: &'a mut W,
}
impl<'a> B1241_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1242` reader - B1242"]
pub struct B1242_R(crate::FieldReader<bool, bool>);
impl B1242_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1242_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1242_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1242` writer - B1242"]
pub struct B1242_W<'a> {
    w: &'a mut W,
}
impl<'a> B1242_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1243` reader - B1243"]
pub struct B1243_R(crate::FieldReader<bool, bool>);
impl B1243_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1243_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1243_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1243` writer - B1243"]
pub struct B1243_W<'a> {
    w: &'a mut W,
}
impl<'a> B1243_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1244` reader - B1244"]
pub struct B1244_R(crate::FieldReader<bool, bool>);
impl B1244_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1244_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1244_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1244` writer - B1244"]
pub struct B1244_W<'a> {
    w: &'a mut W,
}
impl<'a> B1244_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1245` reader - B1245"]
pub struct B1245_R(crate::FieldReader<bool, bool>);
impl B1245_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1245_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1245_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1245` writer - B1245"]
pub struct B1245_W<'a> {
    w: &'a mut W,
}
impl<'a> B1245_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1246` reader - B1246"]
pub struct B1246_R(crate::FieldReader<bool, bool>);
impl B1246_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1246_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1246_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1246` writer - B1246"]
pub struct B1246_W<'a> {
    w: &'a mut W,
}
impl<'a> B1246_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1247` reader - B1247"]
pub struct B1247_R(crate::FieldReader<bool, bool>);
impl B1247_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1247_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1247_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1247` writer - B1247"]
pub struct B1247_W<'a> {
    w: &'a mut W,
}
impl<'a> B1247_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1216"]
    #[inline(always)]
    pub fn b1216(&self) -> B1216_R {
        B1216_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1217"]
    #[inline(always)]
    pub fn b1217(&self) -> B1217_R {
        B1217_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1218"]
    #[inline(always)]
    pub fn b1218(&self) -> B1218_R {
        B1218_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1219"]
    #[inline(always)]
    pub fn b1219(&self) -> B1219_R {
        B1219_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1220"]
    #[inline(always)]
    pub fn b1220(&self) -> B1220_R {
        B1220_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1221"]
    #[inline(always)]
    pub fn b1221(&self) -> B1221_R {
        B1221_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1222"]
    #[inline(always)]
    pub fn b1222(&self) -> B1222_R {
        B1222_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1223"]
    #[inline(always)]
    pub fn b1223(&self) -> B1223_R {
        B1223_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1224"]
    #[inline(always)]
    pub fn b1224(&self) -> B1224_R {
        B1224_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1225"]
    #[inline(always)]
    pub fn b1225(&self) -> B1225_R {
        B1225_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1226"]
    #[inline(always)]
    pub fn b1226(&self) -> B1226_R {
        B1226_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1227"]
    #[inline(always)]
    pub fn b1227(&self) -> B1227_R {
        B1227_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1228"]
    #[inline(always)]
    pub fn b1228(&self) -> B1228_R {
        B1228_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1229"]
    #[inline(always)]
    pub fn b1229(&self) -> B1229_R {
        B1229_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1230"]
    #[inline(always)]
    pub fn b1230(&self) -> B1230_R {
        B1230_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1231"]
    #[inline(always)]
    pub fn b1231(&self) -> B1231_R {
        B1231_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1232"]
    #[inline(always)]
    pub fn b1232(&self) -> B1232_R {
        B1232_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1233"]
    #[inline(always)]
    pub fn b1233(&self) -> B1233_R {
        B1233_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1234"]
    #[inline(always)]
    pub fn b1234(&self) -> B1234_R {
        B1234_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1235"]
    #[inline(always)]
    pub fn b1235(&self) -> B1235_R {
        B1235_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1236"]
    #[inline(always)]
    pub fn b1236(&self) -> B1236_R {
        B1236_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1237"]
    #[inline(always)]
    pub fn b1237(&self) -> B1237_R {
        B1237_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1238"]
    #[inline(always)]
    pub fn b1238(&self) -> B1238_R {
        B1238_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1239"]
    #[inline(always)]
    pub fn b1239(&self) -> B1239_R {
        B1239_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1240"]
    #[inline(always)]
    pub fn b1240(&self) -> B1240_R {
        B1240_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1241"]
    #[inline(always)]
    pub fn b1241(&self) -> B1241_R {
        B1241_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1242"]
    #[inline(always)]
    pub fn b1242(&self) -> B1242_R {
        B1242_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1243"]
    #[inline(always)]
    pub fn b1243(&self) -> B1243_R {
        B1243_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1244"]
    #[inline(always)]
    pub fn b1244(&self) -> B1244_R {
        B1244_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1245"]
    #[inline(always)]
    pub fn b1245(&self) -> B1245_R {
        B1245_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1246"]
    #[inline(always)]
    pub fn b1246(&self) -> B1246_R {
        B1246_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1247"]
    #[inline(always)]
    pub fn b1247(&self) -> B1247_R {
        B1247_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1216"]
    #[inline(always)]
    pub fn b1216(&mut self) -> B1216_W {
        B1216_W { w: self }
    }
    #[doc = "Bit 1 - B1217"]
    #[inline(always)]
    pub fn b1217(&mut self) -> B1217_W {
        B1217_W { w: self }
    }
    #[doc = "Bit 2 - B1218"]
    #[inline(always)]
    pub fn b1218(&mut self) -> B1218_W {
        B1218_W { w: self }
    }
    #[doc = "Bit 3 - B1219"]
    #[inline(always)]
    pub fn b1219(&mut self) -> B1219_W {
        B1219_W { w: self }
    }
    #[doc = "Bit 4 - B1220"]
    #[inline(always)]
    pub fn b1220(&mut self) -> B1220_W {
        B1220_W { w: self }
    }
    #[doc = "Bit 5 - B1221"]
    #[inline(always)]
    pub fn b1221(&mut self) -> B1221_W {
        B1221_W { w: self }
    }
    #[doc = "Bit 6 - B1222"]
    #[inline(always)]
    pub fn b1222(&mut self) -> B1222_W {
        B1222_W { w: self }
    }
    #[doc = "Bit 7 - B1223"]
    #[inline(always)]
    pub fn b1223(&mut self) -> B1223_W {
        B1223_W { w: self }
    }
    #[doc = "Bit 8 - B1224"]
    #[inline(always)]
    pub fn b1224(&mut self) -> B1224_W {
        B1224_W { w: self }
    }
    #[doc = "Bit 9 - B1225"]
    #[inline(always)]
    pub fn b1225(&mut self) -> B1225_W {
        B1225_W { w: self }
    }
    #[doc = "Bit 10 - B1226"]
    #[inline(always)]
    pub fn b1226(&mut self) -> B1226_W {
        B1226_W { w: self }
    }
    #[doc = "Bit 11 - B1227"]
    #[inline(always)]
    pub fn b1227(&mut self) -> B1227_W {
        B1227_W { w: self }
    }
    #[doc = "Bit 12 - B1228"]
    #[inline(always)]
    pub fn b1228(&mut self) -> B1228_W {
        B1228_W { w: self }
    }
    #[doc = "Bit 13 - B1229"]
    #[inline(always)]
    pub fn b1229(&mut self) -> B1229_W {
        B1229_W { w: self }
    }
    #[doc = "Bit 14 - B1230"]
    #[inline(always)]
    pub fn b1230(&mut self) -> B1230_W {
        B1230_W { w: self }
    }
    #[doc = "Bit 15 - B1231"]
    #[inline(always)]
    pub fn b1231(&mut self) -> B1231_W {
        B1231_W { w: self }
    }
    #[doc = "Bit 16 - B1232"]
    #[inline(always)]
    pub fn b1232(&mut self) -> B1232_W {
        B1232_W { w: self }
    }
    #[doc = "Bit 17 - B1233"]
    #[inline(always)]
    pub fn b1233(&mut self) -> B1233_W {
        B1233_W { w: self }
    }
    #[doc = "Bit 18 - B1234"]
    #[inline(always)]
    pub fn b1234(&mut self) -> B1234_W {
        B1234_W { w: self }
    }
    #[doc = "Bit 19 - B1235"]
    #[inline(always)]
    pub fn b1235(&mut self) -> B1235_W {
        B1235_W { w: self }
    }
    #[doc = "Bit 20 - B1236"]
    #[inline(always)]
    pub fn b1236(&mut self) -> B1236_W {
        B1236_W { w: self }
    }
    #[doc = "Bit 21 - B1237"]
    #[inline(always)]
    pub fn b1237(&mut self) -> B1237_W {
        B1237_W { w: self }
    }
    #[doc = "Bit 22 - B1238"]
    #[inline(always)]
    pub fn b1238(&mut self) -> B1238_W {
        B1238_W { w: self }
    }
    #[doc = "Bit 23 - B1239"]
    #[inline(always)]
    pub fn b1239(&mut self) -> B1239_W {
        B1239_W { w: self }
    }
    #[doc = "Bit 24 - B1240"]
    #[inline(always)]
    pub fn b1240(&mut self) -> B1240_W {
        B1240_W { w: self }
    }
    #[doc = "Bit 25 - B1241"]
    #[inline(always)]
    pub fn b1241(&mut self) -> B1241_W {
        B1241_W { w: self }
    }
    #[doc = "Bit 26 - B1242"]
    #[inline(always)]
    pub fn b1242(&mut self) -> B1242_W {
        B1242_W { w: self }
    }
    #[doc = "Bit 27 - B1243"]
    #[inline(always)]
    pub fn b1243(&mut self) -> B1243_W {
        B1243_W { w: self }
    }
    #[doc = "Bit 28 - B1244"]
    #[inline(always)]
    pub fn b1244(&mut self) -> B1244_W {
        B1244_W { w: self }
    }
    #[doc = "Bit 29 - B1245"]
    #[inline(always)]
    pub fn b1245(&mut self) -> B1245_W {
        B1245_W { w: self }
    }
    #[doc = "Bit 30 - B1246"]
    #[inline(always)]
    pub fn b1246(&mut self) -> B1246_W {
        B1246_W { w: self }
    }
    #[doc = "Bit 31 - B1247"]
    #[inline(always)]
    pub fn b1247(&mut self) -> B1247_W {
        B1247_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr38](index.html) module"]
pub struct MPCBB1_VCTR38_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR38_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr38::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR38_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr38::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR38_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR38 to value 0"]
impl crate::Resettable for MPCBB1_VCTR38_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
