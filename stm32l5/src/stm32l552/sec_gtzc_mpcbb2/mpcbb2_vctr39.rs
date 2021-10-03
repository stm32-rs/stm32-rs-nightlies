#[doc = "Register `MPCBB2_VCTR39` reader"]
pub struct R(crate::R<MPCBB2_VCTR39_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR39_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR39_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR39_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR39` writer"]
pub struct W(crate::W<MPCBB2_VCTR39_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR39_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR39_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR39_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1248` reader - B1248"]
pub struct B1248_R(crate::FieldReader<bool, bool>);
impl B1248_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1248_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1248_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1248` writer - B1248"]
pub struct B1248_W<'a> {
    w: &'a mut W,
}
impl<'a> B1248_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1249` reader - B1249"]
pub struct B1249_R(crate::FieldReader<bool, bool>);
impl B1249_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1249_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1249_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1249` writer - B1249"]
pub struct B1249_W<'a> {
    w: &'a mut W,
}
impl<'a> B1249_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1250` reader - B1250"]
pub struct B1250_R(crate::FieldReader<bool, bool>);
impl B1250_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1250_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1250_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1250` writer - B1250"]
pub struct B1250_W<'a> {
    w: &'a mut W,
}
impl<'a> B1250_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1251` reader - B1251"]
pub struct B1251_R(crate::FieldReader<bool, bool>);
impl B1251_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1251_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1251_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1251` writer - B1251"]
pub struct B1251_W<'a> {
    w: &'a mut W,
}
impl<'a> B1251_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1252` reader - B1252"]
pub struct B1252_R(crate::FieldReader<bool, bool>);
impl B1252_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1252_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1252_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1252` writer - B1252"]
pub struct B1252_W<'a> {
    w: &'a mut W,
}
impl<'a> B1252_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1253` reader - B1253"]
pub struct B1253_R(crate::FieldReader<bool, bool>);
impl B1253_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1253_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1253_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1253` writer - B1253"]
pub struct B1253_W<'a> {
    w: &'a mut W,
}
impl<'a> B1253_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1254` reader - B1254"]
pub struct B1254_R(crate::FieldReader<bool, bool>);
impl B1254_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1254_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1254_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1254` writer - B1254"]
pub struct B1254_W<'a> {
    w: &'a mut W,
}
impl<'a> B1254_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1255` reader - B1255"]
pub struct B1255_R(crate::FieldReader<bool, bool>);
impl B1255_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1255_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1255_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1255` writer - B1255"]
pub struct B1255_W<'a> {
    w: &'a mut W,
}
impl<'a> B1255_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1256` reader - B1256"]
pub struct B1256_R(crate::FieldReader<bool, bool>);
impl B1256_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1256_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1256_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1256` writer - B1256"]
pub struct B1256_W<'a> {
    w: &'a mut W,
}
impl<'a> B1256_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1257` reader - B1257"]
pub struct B1257_R(crate::FieldReader<bool, bool>);
impl B1257_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1257_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1257_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1257` writer - B1257"]
pub struct B1257_W<'a> {
    w: &'a mut W,
}
impl<'a> B1257_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1258` reader - B1258"]
pub struct B1258_R(crate::FieldReader<bool, bool>);
impl B1258_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1258_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1258_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1258` writer - B1258"]
pub struct B1258_W<'a> {
    w: &'a mut W,
}
impl<'a> B1258_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1259` reader - B1259"]
pub struct B1259_R(crate::FieldReader<bool, bool>);
impl B1259_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1259_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1259_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1259` writer - B1259"]
pub struct B1259_W<'a> {
    w: &'a mut W,
}
impl<'a> B1259_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1260` reader - B1260"]
pub struct B1260_R(crate::FieldReader<bool, bool>);
impl B1260_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1260_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1260_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1260` writer - B1260"]
pub struct B1260_W<'a> {
    w: &'a mut W,
}
impl<'a> B1260_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1261` reader - B1261"]
pub struct B1261_R(crate::FieldReader<bool, bool>);
impl B1261_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1261_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1261_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1261` writer - B1261"]
pub struct B1261_W<'a> {
    w: &'a mut W,
}
impl<'a> B1261_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1262` reader - B1262"]
pub struct B1262_R(crate::FieldReader<bool, bool>);
impl B1262_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1262_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1262_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1262` writer - B1262"]
pub struct B1262_W<'a> {
    w: &'a mut W,
}
impl<'a> B1262_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1263` reader - B1263"]
pub struct B1263_R(crate::FieldReader<bool, bool>);
impl B1263_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1263_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1263_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1263` writer - B1263"]
pub struct B1263_W<'a> {
    w: &'a mut W,
}
impl<'a> B1263_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1264` reader - B1264"]
pub struct B1264_R(crate::FieldReader<bool, bool>);
impl B1264_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1264_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1264_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1264` writer - B1264"]
pub struct B1264_W<'a> {
    w: &'a mut W,
}
impl<'a> B1264_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1265` reader - B1265"]
pub struct B1265_R(crate::FieldReader<bool, bool>);
impl B1265_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1265_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1265_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1265` writer - B1265"]
pub struct B1265_W<'a> {
    w: &'a mut W,
}
impl<'a> B1265_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1266` reader - B1266"]
pub struct B1266_R(crate::FieldReader<bool, bool>);
impl B1266_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1266_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1266_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1266` writer - B1266"]
pub struct B1266_W<'a> {
    w: &'a mut W,
}
impl<'a> B1266_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1267` reader - B1267"]
pub struct B1267_R(crate::FieldReader<bool, bool>);
impl B1267_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1267_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1267_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1267` writer - B1267"]
pub struct B1267_W<'a> {
    w: &'a mut W,
}
impl<'a> B1267_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1268` reader - B1268"]
pub struct B1268_R(crate::FieldReader<bool, bool>);
impl B1268_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1268_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1268_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1268` writer - B1268"]
pub struct B1268_W<'a> {
    w: &'a mut W,
}
impl<'a> B1268_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1269` reader - B1269"]
pub struct B1269_R(crate::FieldReader<bool, bool>);
impl B1269_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1269_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1269_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1269` writer - B1269"]
pub struct B1269_W<'a> {
    w: &'a mut W,
}
impl<'a> B1269_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1270` reader - B1270"]
pub struct B1270_R(crate::FieldReader<bool, bool>);
impl B1270_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1270_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1270_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1270` writer - B1270"]
pub struct B1270_W<'a> {
    w: &'a mut W,
}
impl<'a> B1270_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1271` reader - B1271"]
pub struct B1271_R(crate::FieldReader<bool, bool>);
impl B1271_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1271_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1271_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1271` writer - B1271"]
pub struct B1271_W<'a> {
    w: &'a mut W,
}
impl<'a> B1271_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1272` reader - B1272"]
pub struct B1272_R(crate::FieldReader<bool, bool>);
impl B1272_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1272_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1272_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1272` writer - B1272"]
pub struct B1272_W<'a> {
    w: &'a mut W,
}
impl<'a> B1272_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1273` reader - B1273"]
pub struct B1273_R(crate::FieldReader<bool, bool>);
impl B1273_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1273_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1273_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1273` writer - B1273"]
pub struct B1273_W<'a> {
    w: &'a mut W,
}
impl<'a> B1273_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1274` reader - B1274"]
pub struct B1274_R(crate::FieldReader<bool, bool>);
impl B1274_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1274_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1274_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1274` writer - B1274"]
pub struct B1274_W<'a> {
    w: &'a mut W,
}
impl<'a> B1274_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1275` reader - B1275"]
pub struct B1275_R(crate::FieldReader<bool, bool>);
impl B1275_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1275_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1275_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1275` writer - B1275"]
pub struct B1275_W<'a> {
    w: &'a mut W,
}
impl<'a> B1275_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1276` reader - B1276"]
pub struct B1276_R(crate::FieldReader<bool, bool>);
impl B1276_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1276_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1276_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1276` writer - B1276"]
pub struct B1276_W<'a> {
    w: &'a mut W,
}
impl<'a> B1276_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1277` reader - B1277"]
pub struct B1277_R(crate::FieldReader<bool, bool>);
impl B1277_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1277_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1277_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1277` writer - B1277"]
pub struct B1277_W<'a> {
    w: &'a mut W,
}
impl<'a> B1277_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1278` reader - B1278"]
pub struct B1278_R(crate::FieldReader<bool, bool>);
impl B1278_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1278_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1278_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1278` writer - B1278"]
pub struct B1278_W<'a> {
    w: &'a mut W,
}
impl<'a> B1278_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1279` reader - B1279"]
pub struct B1279_R(crate::FieldReader<bool, bool>);
impl B1279_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1279_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1279_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1279` writer - B1279"]
pub struct B1279_W<'a> {
    w: &'a mut W,
}
impl<'a> B1279_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1248"]
    #[inline(always)]
    pub fn b1248(&self) -> B1248_R {
        B1248_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1249"]
    #[inline(always)]
    pub fn b1249(&self) -> B1249_R {
        B1249_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1250"]
    #[inline(always)]
    pub fn b1250(&self) -> B1250_R {
        B1250_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1251"]
    #[inline(always)]
    pub fn b1251(&self) -> B1251_R {
        B1251_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1252"]
    #[inline(always)]
    pub fn b1252(&self) -> B1252_R {
        B1252_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1253"]
    #[inline(always)]
    pub fn b1253(&self) -> B1253_R {
        B1253_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1254"]
    #[inline(always)]
    pub fn b1254(&self) -> B1254_R {
        B1254_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1255"]
    #[inline(always)]
    pub fn b1255(&self) -> B1255_R {
        B1255_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1256"]
    #[inline(always)]
    pub fn b1256(&self) -> B1256_R {
        B1256_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1257"]
    #[inline(always)]
    pub fn b1257(&self) -> B1257_R {
        B1257_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1258"]
    #[inline(always)]
    pub fn b1258(&self) -> B1258_R {
        B1258_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1259"]
    #[inline(always)]
    pub fn b1259(&self) -> B1259_R {
        B1259_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1260"]
    #[inline(always)]
    pub fn b1260(&self) -> B1260_R {
        B1260_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1261"]
    #[inline(always)]
    pub fn b1261(&self) -> B1261_R {
        B1261_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1262"]
    #[inline(always)]
    pub fn b1262(&self) -> B1262_R {
        B1262_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1263"]
    #[inline(always)]
    pub fn b1263(&self) -> B1263_R {
        B1263_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1264"]
    #[inline(always)]
    pub fn b1264(&self) -> B1264_R {
        B1264_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1265"]
    #[inline(always)]
    pub fn b1265(&self) -> B1265_R {
        B1265_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1266"]
    #[inline(always)]
    pub fn b1266(&self) -> B1266_R {
        B1266_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1267"]
    #[inline(always)]
    pub fn b1267(&self) -> B1267_R {
        B1267_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1268"]
    #[inline(always)]
    pub fn b1268(&self) -> B1268_R {
        B1268_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1269"]
    #[inline(always)]
    pub fn b1269(&self) -> B1269_R {
        B1269_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1270"]
    #[inline(always)]
    pub fn b1270(&self) -> B1270_R {
        B1270_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1271"]
    #[inline(always)]
    pub fn b1271(&self) -> B1271_R {
        B1271_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1272"]
    #[inline(always)]
    pub fn b1272(&self) -> B1272_R {
        B1272_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1273"]
    #[inline(always)]
    pub fn b1273(&self) -> B1273_R {
        B1273_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1274"]
    #[inline(always)]
    pub fn b1274(&self) -> B1274_R {
        B1274_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1275"]
    #[inline(always)]
    pub fn b1275(&self) -> B1275_R {
        B1275_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1276"]
    #[inline(always)]
    pub fn b1276(&self) -> B1276_R {
        B1276_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1277"]
    #[inline(always)]
    pub fn b1277(&self) -> B1277_R {
        B1277_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1278"]
    #[inline(always)]
    pub fn b1278(&self) -> B1278_R {
        B1278_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1279"]
    #[inline(always)]
    pub fn b1279(&self) -> B1279_R {
        B1279_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1248"]
    #[inline(always)]
    pub fn b1248(&mut self) -> B1248_W {
        B1248_W { w: self }
    }
    #[doc = "Bit 1 - B1249"]
    #[inline(always)]
    pub fn b1249(&mut self) -> B1249_W {
        B1249_W { w: self }
    }
    #[doc = "Bit 2 - B1250"]
    #[inline(always)]
    pub fn b1250(&mut self) -> B1250_W {
        B1250_W { w: self }
    }
    #[doc = "Bit 3 - B1251"]
    #[inline(always)]
    pub fn b1251(&mut self) -> B1251_W {
        B1251_W { w: self }
    }
    #[doc = "Bit 4 - B1252"]
    #[inline(always)]
    pub fn b1252(&mut self) -> B1252_W {
        B1252_W { w: self }
    }
    #[doc = "Bit 5 - B1253"]
    #[inline(always)]
    pub fn b1253(&mut self) -> B1253_W {
        B1253_W { w: self }
    }
    #[doc = "Bit 6 - B1254"]
    #[inline(always)]
    pub fn b1254(&mut self) -> B1254_W {
        B1254_W { w: self }
    }
    #[doc = "Bit 7 - B1255"]
    #[inline(always)]
    pub fn b1255(&mut self) -> B1255_W {
        B1255_W { w: self }
    }
    #[doc = "Bit 8 - B1256"]
    #[inline(always)]
    pub fn b1256(&mut self) -> B1256_W {
        B1256_W { w: self }
    }
    #[doc = "Bit 9 - B1257"]
    #[inline(always)]
    pub fn b1257(&mut self) -> B1257_W {
        B1257_W { w: self }
    }
    #[doc = "Bit 10 - B1258"]
    #[inline(always)]
    pub fn b1258(&mut self) -> B1258_W {
        B1258_W { w: self }
    }
    #[doc = "Bit 11 - B1259"]
    #[inline(always)]
    pub fn b1259(&mut self) -> B1259_W {
        B1259_W { w: self }
    }
    #[doc = "Bit 12 - B1260"]
    #[inline(always)]
    pub fn b1260(&mut self) -> B1260_W {
        B1260_W { w: self }
    }
    #[doc = "Bit 13 - B1261"]
    #[inline(always)]
    pub fn b1261(&mut self) -> B1261_W {
        B1261_W { w: self }
    }
    #[doc = "Bit 14 - B1262"]
    #[inline(always)]
    pub fn b1262(&mut self) -> B1262_W {
        B1262_W { w: self }
    }
    #[doc = "Bit 15 - B1263"]
    #[inline(always)]
    pub fn b1263(&mut self) -> B1263_W {
        B1263_W { w: self }
    }
    #[doc = "Bit 16 - B1264"]
    #[inline(always)]
    pub fn b1264(&mut self) -> B1264_W {
        B1264_W { w: self }
    }
    #[doc = "Bit 17 - B1265"]
    #[inline(always)]
    pub fn b1265(&mut self) -> B1265_W {
        B1265_W { w: self }
    }
    #[doc = "Bit 18 - B1266"]
    #[inline(always)]
    pub fn b1266(&mut self) -> B1266_W {
        B1266_W { w: self }
    }
    #[doc = "Bit 19 - B1267"]
    #[inline(always)]
    pub fn b1267(&mut self) -> B1267_W {
        B1267_W { w: self }
    }
    #[doc = "Bit 20 - B1268"]
    #[inline(always)]
    pub fn b1268(&mut self) -> B1268_W {
        B1268_W { w: self }
    }
    #[doc = "Bit 21 - B1269"]
    #[inline(always)]
    pub fn b1269(&mut self) -> B1269_W {
        B1269_W { w: self }
    }
    #[doc = "Bit 22 - B1270"]
    #[inline(always)]
    pub fn b1270(&mut self) -> B1270_W {
        B1270_W { w: self }
    }
    #[doc = "Bit 23 - B1271"]
    #[inline(always)]
    pub fn b1271(&mut self) -> B1271_W {
        B1271_W { w: self }
    }
    #[doc = "Bit 24 - B1272"]
    #[inline(always)]
    pub fn b1272(&mut self) -> B1272_W {
        B1272_W { w: self }
    }
    #[doc = "Bit 25 - B1273"]
    #[inline(always)]
    pub fn b1273(&mut self) -> B1273_W {
        B1273_W { w: self }
    }
    #[doc = "Bit 26 - B1274"]
    #[inline(always)]
    pub fn b1274(&mut self) -> B1274_W {
        B1274_W { w: self }
    }
    #[doc = "Bit 27 - B1275"]
    #[inline(always)]
    pub fn b1275(&mut self) -> B1275_W {
        B1275_W { w: self }
    }
    #[doc = "Bit 28 - B1276"]
    #[inline(always)]
    pub fn b1276(&mut self) -> B1276_W {
        B1276_W { w: self }
    }
    #[doc = "Bit 29 - B1277"]
    #[inline(always)]
    pub fn b1277(&mut self) -> B1277_W {
        B1277_W { w: self }
    }
    #[doc = "Bit 30 - B1278"]
    #[inline(always)]
    pub fn b1278(&mut self) -> B1278_W {
        B1278_W { w: self }
    }
    #[doc = "Bit 31 - B1279"]
    #[inline(always)]
    pub fn b1279(&mut self) -> B1279_W {
        B1279_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr39](index.html) module"]
pub struct MPCBB2_VCTR39_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr39::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR39_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr39::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR39_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR39 to value 0"]
impl crate::Resettable for MPCBB2_VCTR39_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
