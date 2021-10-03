#[doc = "Register `MPCBB2_VCTR10` reader"]
pub struct R(crate::R<MPCBB2_VCTR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_VCTR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_VCTR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_VCTR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_VCTR10` writer"]
pub struct W(crate::W<MPCBB2_VCTR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_VCTR10_SPEC>;
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
impl From<crate::W<MPCBB2_VCTR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_VCTR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B320` reader - B320"]
pub struct B320_R(crate::FieldReader<bool, bool>);
impl B320_R {
    pub(crate) fn new(bits: bool) -> Self {
        B320_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B320_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B320` writer - B320"]
pub struct B320_W<'a> {
    w: &'a mut W,
}
impl<'a> B320_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B321` reader - B321"]
pub struct B321_R(crate::FieldReader<bool, bool>);
impl B321_R {
    pub(crate) fn new(bits: bool) -> Self {
        B321_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B321_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B321` writer - B321"]
pub struct B321_W<'a> {
    w: &'a mut W,
}
impl<'a> B321_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B322` reader - B322"]
pub struct B322_R(crate::FieldReader<bool, bool>);
impl B322_R {
    pub(crate) fn new(bits: bool) -> Self {
        B322_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B322_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B322` writer - B322"]
pub struct B322_W<'a> {
    w: &'a mut W,
}
impl<'a> B322_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B323` reader - B323"]
pub struct B323_R(crate::FieldReader<bool, bool>);
impl B323_R {
    pub(crate) fn new(bits: bool) -> Self {
        B323_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B323_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B323` writer - B323"]
pub struct B323_W<'a> {
    w: &'a mut W,
}
impl<'a> B323_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B324` reader - B324"]
pub struct B324_R(crate::FieldReader<bool, bool>);
impl B324_R {
    pub(crate) fn new(bits: bool) -> Self {
        B324_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B324_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B324` writer - B324"]
pub struct B324_W<'a> {
    w: &'a mut W,
}
impl<'a> B324_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B325` reader - B325"]
pub struct B325_R(crate::FieldReader<bool, bool>);
impl B325_R {
    pub(crate) fn new(bits: bool) -> Self {
        B325_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B325_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B325` writer - B325"]
pub struct B325_W<'a> {
    w: &'a mut W,
}
impl<'a> B325_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B326` reader - B326"]
pub struct B326_R(crate::FieldReader<bool, bool>);
impl B326_R {
    pub(crate) fn new(bits: bool) -> Self {
        B326_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B326_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B326` writer - B326"]
pub struct B326_W<'a> {
    w: &'a mut W,
}
impl<'a> B326_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B327` reader - B327"]
pub struct B327_R(crate::FieldReader<bool, bool>);
impl B327_R {
    pub(crate) fn new(bits: bool) -> Self {
        B327_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B327_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B327` writer - B327"]
pub struct B327_W<'a> {
    w: &'a mut W,
}
impl<'a> B327_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B328` reader - B328"]
pub struct B328_R(crate::FieldReader<bool, bool>);
impl B328_R {
    pub(crate) fn new(bits: bool) -> Self {
        B328_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B328_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B328` writer - B328"]
pub struct B328_W<'a> {
    w: &'a mut W,
}
impl<'a> B328_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B329` reader - B329"]
pub struct B329_R(crate::FieldReader<bool, bool>);
impl B329_R {
    pub(crate) fn new(bits: bool) -> Self {
        B329_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B329_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B329` writer - B329"]
pub struct B329_W<'a> {
    w: &'a mut W,
}
impl<'a> B329_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B330` reader - B330"]
pub struct B330_R(crate::FieldReader<bool, bool>);
impl B330_R {
    pub(crate) fn new(bits: bool) -> Self {
        B330_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B330_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B330` writer - B330"]
pub struct B330_W<'a> {
    w: &'a mut W,
}
impl<'a> B330_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B331` reader - B331"]
pub struct B331_R(crate::FieldReader<bool, bool>);
impl B331_R {
    pub(crate) fn new(bits: bool) -> Self {
        B331_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B331_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B331` writer - B331"]
pub struct B331_W<'a> {
    w: &'a mut W,
}
impl<'a> B331_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B332` reader - B332"]
pub struct B332_R(crate::FieldReader<bool, bool>);
impl B332_R {
    pub(crate) fn new(bits: bool) -> Self {
        B332_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B332_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B332` writer - B332"]
pub struct B332_W<'a> {
    w: &'a mut W,
}
impl<'a> B332_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B333` reader - B333"]
pub struct B333_R(crate::FieldReader<bool, bool>);
impl B333_R {
    pub(crate) fn new(bits: bool) -> Self {
        B333_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B333_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B333` writer - B333"]
pub struct B333_W<'a> {
    w: &'a mut W,
}
impl<'a> B333_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B334` reader - B334"]
pub struct B334_R(crate::FieldReader<bool, bool>);
impl B334_R {
    pub(crate) fn new(bits: bool) -> Self {
        B334_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B334_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B334` writer - B334"]
pub struct B334_W<'a> {
    w: &'a mut W,
}
impl<'a> B334_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B335` reader - B335"]
pub struct B335_R(crate::FieldReader<bool, bool>);
impl B335_R {
    pub(crate) fn new(bits: bool) -> Self {
        B335_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B335_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B335` writer - B335"]
pub struct B335_W<'a> {
    w: &'a mut W,
}
impl<'a> B335_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B336` reader - B336"]
pub struct B336_R(crate::FieldReader<bool, bool>);
impl B336_R {
    pub(crate) fn new(bits: bool) -> Self {
        B336_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B336_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B336` writer - B336"]
pub struct B336_W<'a> {
    w: &'a mut W,
}
impl<'a> B336_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B337` reader - B337"]
pub struct B337_R(crate::FieldReader<bool, bool>);
impl B337_R {
    pub(crate) fn new(bits: bool) -> Self {
        B337_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B337_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B337` writer - B337"]
pub struct B337_W<'a> {
    w: &'a mut W,
}
impl<'a> B337_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B338` reader - B338"]
pub struct B338_R(crate::FieldReader<bool, bool>);
impl B338_R {
    pub(crate) fn new(bits: bool) -> Self {
        B338_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B338_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B338` writer - B338"]
pub struct B338_W<'a> {
    w: &'a mut W,
}
impl<'a> B338_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B339` reader - B339"]
pub struct B339_R(crate::FieldReader<bool, bool>);
impl B339_R {
    pub(crate) fn new(bits: bool) -> Self {
        B339_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B339_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B339` writer - B339"]
pub struct B339_W<'a> {
    w: &'a mut W,
}
impl<'a> B339_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B340` reader - B340"]
pub struct B340_R(crate::FieldReader<bool, bool>);
impl B340_R {
    pub(crate) fn new(bits: bool) -> Self {
        B340_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B340_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B340` writer - B340"]
pub struct B340_W<'a> {
    w: &'a mut W,
}
impl<'a> B340_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B341` reader - B341"]
pub struct B341_R(crate::FieldReader<bool, bool>);
impl B341_R {
    pub(crate) fn new(bits: bool) -> Self {
        B341_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B341_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B341` writer - B341"]
pub struct B341_W<'a> {
    w: &'a mut W,
}
impl<'a> B341_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B342` reader - B342"]
pub struct B342_R(crate::FieldReader<bool, bool>);
impl B342_R {
    pub(crate) fn new(bits: bool) -> Self {
        B342_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B342_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B342` writer - B342"]
pub struct B342_W<'a> {
    w: &'a mut W,
}
impl<'a> B342_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B343` reader - B343"]
pub struct B343_R(crate::FieldReader<bool, bool>);
impl B343_R {
    pub(crate) fn new(bits: bool) -> Self {
        B343_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B343_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B343` writer - B343"]
pub struct B343_W<'a> {
    w: &'a mut W,
}
impl<'a> B343_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B344` reader - B344"]
pub struct B344_R(crate::FieldReader<bool, bool>);
impl B344_R {
    pub(crate) fn new(bits: bool) -> Self {
        B344_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B344_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B344` writer - B344"]
pub struct B344_W<'a> {
    w: &'a mut W,
}
impl<'a> B344_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B345` reader - B345"]
pub struct B345_R(crate::FieldReader<bool, bool>);
impl B345_R {
    pub(crate) fn new(bits: bool) -> Self {
        B345_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B345_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B345` writer - B345"]
pub struct B345_W<'a> {
    w: &'a mut W,
}
impl<'a> B345_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B346` reader - B346"]
pub struct B346_R(crate::FieldReader<bool, bool>);
impl B346_R {
    pub(crate) fn new(bits: bool) -> Self {
        B346_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B346_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B346` writer - B346"]
pub struct B346_W<'a> {
    w: &'a mut W,
}
impl<'a> B346_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B347` reader - B347"]
pub struct B347_R(crate::FieldReader<bool, bool>);
impl B347_R {
    pub(crate) fn new(bits: bool) -> Self {
        B347_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B347_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B347` writer - B347"]
pub struct B347_W<'a> {
    w: &'a mut W,
}
impl<'a> B347_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B348` reader - B348"]
pub struct B348_R(crate::FieldReader<bool, bool>);
impl B348_R {
    pub(crate) fn new(bits: bool) -> Self {
        B348_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B348_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B348` writer - B348"]
pub struct B348_W<'a> {
    w: &'a mut W,
}
impl<'a> B348_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B349` reader - B349"]
pub struct B349_R(crate::FieldReader<bool, bool>);
impl B349_R {
    pub(crate) fn new(bits: bool) -> Self {
        B349_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B349_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B349` writer - B349"]
pub struct B349_W<'a> {
    w: &'a mut W,
}
impl<'a> B349_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B350` reader - B350"]
pub struct B350_R(crate::FieldReader<bool, bool>);
impl B350_R {
    pub(crate) fn new(bits: bool) -> Self {
        B350_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B350_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B350` writer - B350"]
pub struct B350_W<'a> {
    w: &'a mut W,
}
impl<'a> B350_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `B351` reader - B351"]
pub struct B351_R(crate::FieldReader<bool, bool>);
impl B351_R {
    pub(crate) fn new(bits: bool) -> Self {
        B351_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B351_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B351` writer - B351"]
pub struct B351_W<'a> {
    w: &'a mut W,
}
impl<'a> B351_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - B320"]
    #[inline(always)]
    pub fn b320(&self) -> B320_R {
        B320_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - B321"]
    #[inline(always)]
    pub fn b321(&self) -> B321_R {
        B321_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - B322"]
    #[inline(always)]
    pub fn b322(&self) -> B322_R {
        B322_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - B323"]
    #[inline(always)]
    pub fn b323(&self) -> B323_R {
        B323_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B324"]
    #[inline(always)]
    pub fn b324(&self) -> B324_R {
        B324_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - B325"]
    #[inline(always)]
    pub fn b325(&self) -> B325_R {
        B325_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - B326"]
    #[inline(always)]
    pub fn b326(&self) -> B326_R {
        B326_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - B327"]
    #[inline(always)]
    pub fn b327(&self) -> B327_R {
        B327_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - B328"]
    #[inline(always)]
    pub fn b328(&self) -> B328_R {
        B328_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - B329"]
    #[inline(always)]
    pub fn b329(&self) -> B329_R {
        B329_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - B330"]
    #[inline(always)]
    pub fn b330(&self) -> B330_R {
        B330_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - B331"]
    #[inline(always)]
    pub fn b331(&self) -> B331_R {
        B331_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - B332"]
    #[inline(always)]
    pub fn b332(&self) -> B332_R {
        B332_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - B333"]
    #[inline(always)]
    pub fn b333(&self) -> B333_R {
        B333_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - B334"]
    #[inline(always)]
    pub fn b334(&self) -> B334_R {
        B334_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - B335"]
    #[inline(always)]
    pub fn b335(&self) -> B335_R {
        B335_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - B336"]
    #[inline(always)]
    pub fn b336(&self) -> B336_R {
        B336_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - B337"]
    #[inline(always)]
    pub fn b337(&self) -> B337_R {
        B337_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - B338"]
    #[inline(always)]
    pub fn b338(&self) -> B338_R {
        B338_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - B339"]
    #[inline(always)]
    pub fn b339(&self) -> B339_R {
        B339_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - B340"]
    #[inline(always)]
    pub fn b340(&self) -> B340_R {
        B340_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - B341"]
    #[inline(always)]
    pub fn b341(&self) -> B341_R {
        B341_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - B342"]
    #[inline(always)]
    pub fn b342(&self) -> B342_R {
        B342_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - B343"]
    #[inline(always)]
    pub fn b343(&self) -> B343_R {
        B343_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - B344"]
    #[inline(always)]
    pub fn b344(&self) -> B344_R {
        B344_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - B345"]
    #[inline(always)]
    pub fn b345(&self) -> B345_R {
        B345_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - B346"]
    #[inline(always)]
    pub fn b346(&self) -> B346_R {
        B346_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - B347"]
    #[inline(always)]
    pub fn b347(&self) -> B347_R {
        B347_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - B348"]
    #[inline(always)]
    pub fn b348(&self) -> B348_R {
        B348_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - B349"]
    #[inline(always)]
    pub fn b349(&self) -> B349_R {
        B349_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - B350"]
    #[inline(always)]
    pub fn b350(&self) -> B350_R {
        B350_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - B351"]
    #[inline(always)]
    pub fn b351(&self) -> B351_R {
        B351_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - B320"]
    #[inline(always)]
    pub fn b320(&mut self) -> B320_W {
        B320_W { w: self }
    }
    #[doc = "Bit 1 - B321"]
    #[inline(always)]
    pub fn b321(&mut self) -> B321_W {
        B321_W { w: self }
    }
    #[doc = "Bit 2 - B322"]
    #[inline(always)]
    pub fn b322(&mut self) -> B322_W {
        B322_W { w: self }
    }
    #[doc = "Bit 3 - B323"]
    #[inline(always)]
    pub fn b323(&mut self) -> B323_W {
        B323_W { w: self }
    }
    #[doc = "Bit 4 - B324"]
    #[inline(always)]
    pub fn b324(&mut self) -> B324_W {
        B324_W { w: self }
    }
    #[doc = "Bit 5 - B325"]
    #[inline(always)]
    pub fn b325(&mut self) -> B325_W {
        B325_W { w: self }
    }
    #[doc = "Bit 6 - B326"]
    #[inline(always)]
    pub fn b326(&mut self) -> B326_W {
        B326_W { w: self }
    }
    #[doc = "Bit 7 - B327"]
    #[inline(always)]
    pub fn b327(&mut self) -> B327_W {
        B327_W { w: self }
    }
    #[doc = "Bit 8 - B328"]
    #[inline(always)]
    pub fn b328(&mut self) -> B328_W {
        B328_W { w: self }
    }
    #[doc = "Bit 9 - B329"]
    #[inline(always)]
    pub fn b329(&mut self) -> B329_W {
        B329_W { w: self }
    }
    #[doc = "Bit 10 - B330"]
    #[inline(always)]
    pub fn b330(&mut self) -> B330_W {
        B330_W { w: self }
    }
    #[doc = "Bit 11 - B331"]
    #[inline(always)]
    pub fn b331(&mut self) -> B331_W {
        B331_W { w: self }
    }
    #[doc = "Bit 12 - B332"]
    #[inline(always)]
    pub fn b332(&mut self) -> B332_W {
        B332_W { w: self }
    }
    #[doc = "Bit 13 - B333"]
    #[inline(always)]
    pub fn b333(&mut self) -> B333_W {
        B333_W { w: self }
    }
    #[doc = "Bit 14 - B334"]
    #[inline(always)]
    pub fn b334(&mut self) -> B334_W {
        B334_W { w: self }
    }
    #[doc = "Bit 15 - B335"]
    #[inline(always)]
    pub fn b335(&mut self) -> B335_W {
        B335_W { w: self }
    }
    #[doc = "Bit 16 - B336"]
    #[inline(always)]
    pub fn b336(&mut self) -> B336_W {
        B336_W { w: self }
    }
    #[doc = "Bit 17 - B337"]
    #[inline(always)]
    pub fn b337(&mut self) -> B337_W {
        B337_W { w: self }
    }
    #[doc = "Bit 18 - B338"]
    #[inline(always)]
    pub fn b338(&mut self) -> B338_W {
        B338_W { w: self }
    }
    #[doc = "Bit 19 - B339"]
    #[inline(always)]
    pub fn b339(&mut self) -> B339_W {
        B339_W { w: self }
    }
    #[doc = "Bit 20 - B340"]
    #[inline(always)]
    pub fn b340(&mut self) -> B340_W {
        B340_W { w: self }
    }
    #[doc = "Bit 21 - B341"]
    #[inline(always)]
    pub fn b341(&mut self) -> B341_W {
        B341_W { w: self }
    }
    #[doc = "Bit 22 - B342"]
    #[inline(always)]
    pub fn b342(&mut self) -> B342_W {
        B342_W { w: self }
    }
    #[doc = "Bit 23 - B343"]
    #[inline(always)]
    pub fn b343(&mut self) -> B343_W {
        B343_W { w: self }
    }
    #[doc = "Bit 24 - B344"]
    #[inline(always)]
    pub fn b344(&mut self) -> B344_W {
        B344_W { w: self }
    }
    #[doc = "Bit 25 - B345"]
    #[inline(always)]
    pub fn b345(&mut self) -> B345_W {
        B345_W { w: self }
    }
    #[doc = "Bit 26 - B346"]
    #[inline(always)]
    pub fn b346(&mut self) -> B346_W {
        B346_W { w: self }
    }
    #[doc = "Bit 27 - B347"]
    #[inline(always)]
    pub fn b347(&mut self) -> B347_W {
        B347_W { w: self }
    }
    #[doc = "Bit 28 - B348"]
    #[inline(always)]
    pub fn b348(&mut self) -> B348_W {
        B348_W { w: self }
    }
    #[doc = "Bit 29 - B349"]
    #[inline(always)]
    pub fn b349(&mut self) -> B349_W {
        B349_W { w: self }
    }
    #[doc = "Bit 30 - B350"]
    #[inline(always)]
    pub fn b350(&mut self) -> B350_W {
        B350_W { w: self }
    }
    #[doc = "Bit 31 - B351"]
    #[inline(always)]
    pub fn b351(&mut self) -> B351_W {
        B351_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBBx vector register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_vctr10](index.html) module"]
pub struct MPCBB2_VCTR10_SPEC;
impl crate::RegisterSpec for MPCBB2_VCTR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_vctr10::R](R) reader structure"]
impl crate::Readable for MPCBB2_VCTR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_vctr10::W](W) writer structure"]
impl crate::Writable for MPCBB2_VCTR10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_VCTR10 to value 0xffff_ffff"]
impl crate::Resettable for MPCBB2_VCTR10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
