#[doc = "Register `MPCBB1_VCTR40` reader"]
pub struct R(crate::R<MPCBB1_VCTR40_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_VCTR40_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_VCTR40_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_VCTR40_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_VCTR40` writer"]
pub struct W(crate::W<MPCBB1_VCTR40_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_VCTR40_SPEC>;
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
impl From<crate::W<MPCBB1_VCTR40_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_VCTR40_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1280` reader - B1280"]
pub struct B1280_R(crate::FieldReader<bool, bool>);
impl B1280_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1280_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1280_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1280` writer - B1280"]
pub struct B1280_W<'a> {
    w: &'a mut W,
}
impl<'a> B1280_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1281` reader - B1281"]
pub struct B1281_R(crate::FieldReader<bool, bool>);
impl B1281_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1281_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1281_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1281` writer - B1281"]
pub struct B1281_W<'a> {
    w: &'a mut W,
}
impl<'a> B1281_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1282` reader - B1282"]
pub struct B1282_R(crate::FieldReader<bool, bool>);
impl B1282_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1282_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1282_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1282` writer - B1282"]
pub struct B1282_W<'a> {
    w: &'a mut W,
}
impl<'a> B1282_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1283` reader - B1283"]
pub struct B1283_R(crate::FieldReader<bool, bool>);
impl B1283_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1283_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1283_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1283` writer - B1283"]
pub struct B1283_W<'a> {
    w: &'a mut W,
}
impl<'a> B1283_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1284` reader - B1284"]
pub struct B1284_R(crate::FieldReader<bool, bool>);
impl B1284_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1284_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1284_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1284` writer - B1284"]
pub struct B1284_W<'a> {
    w: &'a mut W,
}
impl<'a> B1284_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1285` reader - B1285"]
pub struct B1285_R(crate::FieldReader<bool, bool>);
impl B1285_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1285_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1285_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1285` writer - B1285"]
pub struct B1285_W<'a> {
    w: &'a mut W,
}
impl<'a> B1285_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1286` reader - B1286"]
pub struct B1286_R(crate::FieldReader<bool, bool>);
impl B1286_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1286_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1286_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1286` writer - B1286"]
pub struct B1286_W<'a> {
    w: &'a mut W,
}
impl<'a> B1286_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1287` reader - B1287"]
pub struct B1287_R(crate::FieldReader<bool, bool>);
impl B1287_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1287_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1287_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1287` writer - B1287"]
pub struct B1287_W<'a> {
    w: &'a mut W,
}
impl<'a> B1287_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1288` reader - B1288"]
pub struct B1288_R(crate::FieldReader<bool, bool>);
impl B1288_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1288_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1288_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1288` writer - B1288"]
pub struct B1288_W<'a> {
    w: &'a mut W,
}
impl<'a> B1288_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1289` reader - B1289"]
pub struct B1289_R(crate::FieldReader<bool, bool>);
impl B1289_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1289_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1289_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1289` writer - B1289"]
pub struct B1289_W<'a> {
    w: &'a mut W,
}
impl<'a> B1289_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1290` reader - B1290"]
pub struct B1290_R(crate::FieldReader<bool, bool>);
impl B1290_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1290_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1290_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1290` writer - B1290"]
pub struct B1290_W<'a> {
    w: &'a mut W,
}
impl<'a> B1290_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1291` reader - B1291"]
pub struct B1291_R(crate::FieldReader<bool, bool>);
impl B1291_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1291_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1291_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1291` writer - B1291"]
pub struct B1291_W<'a> {
    w: &'a mut W,
}
impl<'a> B1291_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1292` reader - B1292"]
pub struct B1292_R(crate::FieldReader<bool, bool>);
impl B1292_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1292_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1292_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1292` writer - B1292"]
pub struct B1292_W<'a> {
    w: &'a mut W,
}
impl<'a> B1292_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1293` reader - B1293"]
pub struct B1293_R(crate::FieldReader<bool, bool>);
impl B1293_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1293_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1293_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1293` writer - B1293"]
pub struct B1293_W<'a> {
    w: &'a mut W,
}
impl<'a> B1293_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1294` reader - B1294"]
pub struct B1294_R(crate::FieldReader<bool, bool>);
impl B1294_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1294_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1294_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1294` writer - B1294"]
pub struct B1294_W<'a> {
    w: &'a mut W,
}
impl<'a> B1294_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1295` reader - B1295"]
pub struct B1295_R(crate::FieldReader<bool, bool>);
impl B1295_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1295_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1295_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1295` writer - B1295"]
pub struct B1295_W<'a> {
    w: &'a mut W,
}
impl<'a> B1295_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1296` reader - B1296"]
pub struct B1296_R(crate::FieldReader<bool, bool>);
impl B1296_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1296_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1296_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1296` writer - B1296"]
pub struct B1296_W<'a> {
    w: &'a mut W,
}
impl<'a> B1296_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1297` reader - B1297"]
pub struct B1297_R(crate::FieldReader<bool, bool>);
impl B1297_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1297_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1297_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1297` writer - B1297"]
pub struct B1297_W<'a> {
    w: &'a mut W,
}
impl<'a> B1297_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1298` reader - B1298"]
pub struct B1298_R(crate::FieldReader<bool, bool>);
impl B1298_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1298_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1298_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1298` writer - B1298"]
pub struct B1298_W<'a> {
    w: &'a mut W,
}
impl<'a> B1298_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1299` reader - B1299"]
pub struct B1299_R(crate::FieldReader<bool, bool>);
impl B1299_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1299_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1299_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1299` writer - B1299"]
pub struct B1299_W<'a> {
    w: &'a mut W,
}
impl<'a> B1299_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1300` reader - B1300"]
pub struct B1300_R(crate::FieldReader<bool, bool>);
impl B1300_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1300_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1300_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1300` writer - B1300"]
pub struct B1300_W<'a> {
    w: &'a mut W,
}
impl<'a> B1300_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1301` reader - B1301"]
pub struct B1301_R(crate::FieldReader<bool, bool>);
impl B1301_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1301_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1301_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1301` writer - B1301"]
pub struct B1301_W<'a> {
    w: &'a mut W,
}
impl<'a> B1301_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1302` reader - B1302"]
pub struct B1302_R(crate::FieldReader<bool, bool>);
impl B1302_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1302_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1302_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1302` writer - B1302"]
pub struct B1302_W<'a> {
    w: &'a mut W,
}
impl<'a> B1302_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1303` reader - B1303"]
pub struct B1303_R(crate::FieldReader<bool, bool>);
impl B1303_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1303_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1303_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1303` writer - B1303"]
pub struct B1303_W<'a> {
    w: &'a mut W,
}
impl<'a> B1303_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1304` reader - B1304"]
pub struct B1304_R(crate::FieldReader<bool, bool>);
impl B1304_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1304_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1304_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1304` writer - B1304"]
pub struct B1304_W<'a> {
    w: &'a mut W,
}
impl<'a> B1304_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1305` reader - B1305"]
pub struct B1305_R(crate::FieldReader<bool, bool>);
impl B1305_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1305_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1305_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1305` writer - B1305"]
pub struct B1305_W<'a> {
    w: &'a mut W,
}
impl<'a> B1305_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1306` reader - B1306"]
pub struct B1306_R(crate::FieldReader<bool, bool>);
impl B1306_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1306_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1306_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1306` writer - B1306"]
pub struct B1306_W<'a> {
    w: &'a mut W,
}
impl<'a> B1306_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1307` reader - B1307"]
pub struct B1307_R(crate::FieldReader<bool, bool>);
impl B1307_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1307_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1307_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1307` writer - B1307"]
pub struct B1307_W<'a> {
    w: &'a mut W,
}
impl<'a> B1307_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1308` reader - B1308"]
pub struct B1308_R(crate::FieldReader<bool, bool>);
impl B1308_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1308_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1308_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1308` writer - B1308"]
pub struct B1308_W<'a> {
    w: &'a mut W,
}
impl<'a> B1308_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1309` reader - B1309"]
pub struct B1309_R(crate::FieldReader<bool, bool>);
impl B1309_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1309_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1309_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1309` writer - B1309"]
pub struct B1309_W<'a> {
    w: &'a mut W,
}
impl<'a> B1309_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1310` reader - B1310"]
pub struct B1310_R(crate::FieldReader<bool, bool>);
impl B1310_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1310_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1310_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1310` writer - B1310"]
pub struct B1310_W<'a> {
    w: &'a mut W,
}
impl<'a> B1310_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B1311` reader - B1311"]
pub struct B1311_R(crate::FieldReader<bool, bool>);
impl B1311_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1311_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1311_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1311` writer - B1311"]
pub struct B1311_W<'a> {
    w: &'a mut W,
}
impl<'a> B1311_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B1280"]
    #[inline(always)]
    pub fn b1280(&self) -> B1280_R {
        B1280_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B1281"]
    #[inline(always)]
    pub fn b1281(&self) -> B1281_R {
        B1281_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B1282"]
    #[inline(always)]
    pub fn b1282(&self) -> B1282_R {
        B1282_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B1283"]
    #[inline(always)]
    pub fn b1283(&self) -> B1283_R {
        B1283_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B1284"]
    #[inline(always)]
    pub fn b1284(&self) -> B1284_R {
        B1284_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B1285"]
    #[inline(always)]
    pub fn b1285(&self) -> B1285_R {
        B1285_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B1286"]
    #[inline(always)]
    pub fn b1286(&self) -> B1286_R {
        B1286_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B1287"]
    #[inline(always)]
    pub fn b1287(&self) -> B1287_R {
        B1287_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B1288"]
    #[inline(always)]
    pub fn b1288(&self) -> B1288_R {
        B1288_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B1289"]
    #[inline(always)]
    pub fn b1289(&self) -> B1289_R {
        B1289_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B1290"]
    #[inline(always)]
    pub fn b1290(&self) -> B1290_R {
        B1290_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B1291"]
    #[inline(always)]
    pub fn b1291(&self) -> B1291_R {
        B1291_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B1292"]
    #[inline(always)]
    pub fn b1292(&self) -> B1292_R {
        B1292_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B1293"]
    #[inline(always)]
    pub fn b1293(&self) -> B1293_R {
        B1293_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B1294"]
    #[inline(always)]
    pub fn b1294(&self) -> B1294_R {
        B1294_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B1295"]
    #[inline(always)]
    pub fn b1295(&self) -> B1295_R {
        B1295_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B1296"]
    #[inline(always)]
    pub fn b1296(&self) -> B1296_R {
        B1296_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B1297"]
    #[inline(always)]
    pub fn b1297(&self) -> B1297_R {
        B1297_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B1298"]
    #[inline(always)]
    pub fn b1298(&self) -> B1298_R {
        B1298_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B1299"]
    #[inline(always)]
    pub fn b1299(&self) -> B1299_R {
        B1299_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B1300"]
    #[inline(always)]
    pub fn b1300(&self) -> B1300_R {
        B1300_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B1301"]
    #[inline(always)]
    pub fn b1301(&self) -> B1301_R {
        B1301_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B1302"]
    #[inline(always)]
    pub fn b1302(&self) -> B1302_R {
        B1302_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B1303"]
    #[inline(always)]
    pub fn b1303(&self) -> B1303_R {
        B1303_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B1304"]
    #[inline(always)]
    pub fn b1304(&self) -> B1304_R {
        B1304_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B1305"]
    #[inline(always)]
    pub fn b1305(&self) -> B1305_R {
        B1305_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B1306"]
    #[inline(always)]
    pub fn b1306(&self) -> B1306_R {
        B1306_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B1307"]
    #[inline(always)]
    pub fn b1307(&self) -> B1307_R {
        B1307_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B1308"]
    #[inline(always)]
    pub fn b1308(&self) -> B1308_R {
        B1308_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B1309"]
    #[inline(always)]
    pub fn b1309(&self) -> B1309_R {
        B1309_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B1310"]
    #[inline(always)]
    pub fn b1310(&self) -> B1310_R {
        B1310_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B1311"]
    #[inline(always)]
    pub fn b1311(&self) -> B1311_R {
        B1311_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B1280"]
    #[inline(always)]
    pub fn b1280(&mut self) -> B1280_W {
        B1280_W { w: self }
    }
    #[doc = "Bit 1 - B1281"]
    #[inline(always)]
    pub fn b1281(&mut self) -> B1281_W {
        B1281_W { w: self }
    }
    #[doc = "Bit 2 - B1282"]
    #[inline(always)]
    pub fn b1282(&mut self) -> B1282_W {
        B1282_W { w: self }
    }
    #[doc = "Bit 3 - B1283"]
    #[inline(always)]
    pub fn b1283(&mut self) -> B1283_W {
        B1283_W { w: self }
    }
    #[doc = "Bit 4 - B1284"]
    #[inline(always)]
    pub fn b1284(&mut self) -> B1284_W {
        B1284_W { w: self }
    }
    #[doc = "Bit 5 - B1285"]
    #[inline(always)]
    pub fn b1285(&mut self) -> B1285_W {
        B1285_W { w: self }
    }
    #[doc = "Bit 6 - B1286"]
    #[inline(always)]
    pub fn b1286(&mut self) -> B1286_W {
        B1286_W { w: self }
    }
    #[doc = "Bit 7 - B1287"]
    #[inline(always)]
    pub fn b1287(&mut self) -> B1287_W {
        B1287_W { w: self }
    }
    #[doc = "Bit 8 - B1288"]
    #[inline(always)]
    pub fn b1288(&mut self) -> B1288_W {
        B1288_W { w: self }
    }
    #[doc = "Bit 9 - B1289"]
    #[inline(always)]
    pub fn b1289(&mut self) -> B1289_W {
        B1289_W { w: self }
    }
    #[doc = "Bit 10 - B1290"]
    #[inline(always)]
    pub fn b1290(&mut self) -> B1290_W {
        B1290_W { w: self }
    }
    #[doc = "Bit 11 - B1291"]
    #[inline(always)]
    pub fn b1291(&mut self) -> B1291_W {
        B1291_W { w: self }
    }
    #[doc = "Bit 12 - B1292"]
    #[inline(always)]
    pub fn b1292(&mut self) -> B1292_W {
        B1292_W { w: self }
    }
    #[doc = "Bit 13 - B1293"]
    #[inline(always)]
    pub fn b1293(&mut self) -> B1293_W {
        B1293_W { w: self }
    }
    #[doc = "Bit 14 - B1294"]
    #[inline(always)]
    pub fn b1294(&mut self) -> B1294_W {
        B1294_W { w: self }
    }
    #[doc = "Bit 15 - B1295"]
    #[inline(always)]
    pub fn b1295(&mut self) -> B1295_W {
        B1295_W { w: self }
    }
    #[doc = "Bit 16 - B1296"]
    #[inline(always)]
    pub fn b1296(&mut self) -> B1296_W {
        B1296_W { w: self }
    }
    #[doc = "Bit 17 - B1297"]
    #[inline(always)]
    pub fn b1297(&mut self) -> B1297_W {
        B1297_W { w: self }
    }
    #[doc = "Bit 18 - B1298"]
    #[inline(always)]
    pub fn b1298(&mut self) -> B1298_W {
        B1298_W { w: self }
    }
    #[doc = "Bit 19 - B1299"]
    #[inline(always)]
    pub fn b1299(&mut self) -> B1299_W {
        B1299_W { w: self }
    }
    #[doc = "Bit 20 - B1300"]
    #[inline(always)]
    pub fn b1300(&mut self) -> B1300_W {
        B1300_W { w: self }
    }
    #[doc = "Bit 21 - B1301"]
    #[inline(always)]
    pub fn b1301(&mut self) -> B1301_W {
        B1301_W { w: self }
    }
    #[doc = "Bit 22 - B1302"]
    #[inline(always)]
    pub fn b1302(&mut self) -> B1302_W {
        B1302_W { w: self }
    }
    #[doc = "Bit 23 - B1303"]
    #[inline(always)]
    pub fn b1303(&mut self) -> B1303_W {
        B1303_W { w: self }
    }
    #[doc = "Bit 24 - B1304"]
    #[inline(always)]
    pub fn b1304(&mut self) -> B1304_W {
        B1304_W { w: self }
    }
    #[doc = "Bit 25 - B1305"]
    #[inline(always)]
    pub fn b1305(&mut self) -> B1305_W {
        B1305_W { w: self }
    }
    #[doc = "Bit 26 - B1306"]
    #[inline(always)]
    pub fn b1306(&mut self) -> B1306_W {
        B1306_W { w: self }
    }
    #[doc = "Bit 27 - B1307"]
    #[inline(always)]
    pub fn b1307(&mut self) -> B1307_W {
        B1307_W { w: self }
    }
    #[doc = "Bit 28 - B1308"]
    #[inline(always)]
    pub fn b1308(&mut self) -> B1308_W {
        B1308_W { w: self }
    }
    #[doc = "Bit 29 - B1309"]
    #[inline(always)]
    pub fn b1309(&mut self) -> B1309_W {
        B1309_W { w: self }
    }
    #[doc = "Bit 30 - B1310"]
    #[inline(always)]
    pub fn b1310(&mut self) -> B1310_W {
        B1310_W { w: self }
    }
    #[doc = "Bit 31 - B1311"]
    #[inline(always)]
    pub fn b1311(&mut self) -> B1311_W {
        B1311_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_vctr40](index.html) module"]
pub struct MPCBB1_VCTR40_SPEC;
impl crate::RegisterSpec for MPCBB1_VCTR40_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_vctr40::R](R) reader structure"]
impl crate::Readable for MPCBB1_VCTR40_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_vctr40::W](W) writer structure"]
impl crate::Writable for MPCBB1_VCTR40_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_VCTR40 to value 0"]
impl crate::Resettable for MPCBB1_VCTR40_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
