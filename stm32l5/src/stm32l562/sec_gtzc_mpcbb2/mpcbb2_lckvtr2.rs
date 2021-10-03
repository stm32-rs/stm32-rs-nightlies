#[doc = "Register `MPCBB2_LCKVTR2` reader"]
pub struct R(crate::R<MPCBB2_LCKVTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB2_LCKVTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB2_LCKVTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB2_LCKVTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB2_LCKVTR2` writer"]
pub struct W(crate::W<MPCBB2_LCKVTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB2_LCKVTR2_SPEC>;
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
impl From<crate::W<MPCBB2_LCKVTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB2_LCKVTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCKSB32` reader - LCKSB32"]
pub struct LCKSB32_R(crate::FieldReader<bool, bool>);
impl LCKSB32_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB32` writer - LCKSB32"]
pub struct LCKSB32_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB32_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB33` reader - LCKSB33"]
pub struct LCKSB33_R(crate::FieldReader<bool, bool>);
impl LCKSB33_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB33` writer - LCKSB33"]
pub struct LCKSB33_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB33_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB34` reader - LCKSB34"]
pub struct LCKSB34_R(crate::FieldReader<bool, bool>);
impl LCKSB34_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB34` writer - LCKSB34"]
pub struct LCKSB34_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB34_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB35` reader - LCKSB35"]
pub struct LCKSB35_R(crate::FieldReader<bool, bool>);
impl LCKSB35_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB35` writer - LCKSB35"]
pub struct LCKSB35_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB35_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB36` reader - LCKSB36"]
pub struct LCKSB36_R(crate::FieldReader<bool, bool>);
impl LCKSB36_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB36` writer - LCKSB36"]
pub struct LCKSB36_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB36_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB37` reader - LCKSB37"]
pub struct LCKSB37_R(crate::FieldReader<bool, bool>);
impl LCKSB37_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB37` writer - LCKSB37"]
pub struct LCKSB37_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB37_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB38` reader - LCKSB38"]
pub struct LCKSB38_R(crate::FieldReader<bool, bool>);
impl LCKSB38_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB38` writer - LCKSB38"]
pub struct LCKSB38_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB38_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB39` reader - LCKSB39"]
pub struct LCKSB39_R(crate::FieldReader<bool, bool>);
impl LCKSB39_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB39` writer - LCKSB39"]
pub struct LCKSB39_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB39_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB40` reader - LCKSB40"]
pub struct LCKSB40_R(crate::FieldReader<bool, bool>);
impl LCKSB40_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB40` writer - LCKSB40"]
pub struct LCKSB40_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB40_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB41` reader - LCKSB41"]
pub struct LCKSB41_R(crate::FieldReader<bool, bool>);
impl LCKSB41_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB41` writer - LCKSB41"]
pub struct LCKSB41_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB41_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB42` reader - LCKSB42"]
pub struct LCKSB42_R(crate::FieldReader<bool, bool>);
impl LCKSB42_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB42` writer - LCKSB42"]
pub struct LCKSB42_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB42_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB43` reader - LCKSB43"]
pub struct LCKSB43_R(crate::FieldReader<bool, bool>);
impl LCKSB43_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB43` writer - LCKSB43"]
pub struct LCKSB43_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB43_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB44` reader - LCKSB44"]
pub struct LCKSB44_R(crate::FieldReader<bool, bool>);
impl LCKSB44_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB44` writer - LCKSB44"]
pub struct LCKSB44_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB44_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB45` reader - LCKSB45"]
pub struct LCKSB45_R(crate::FieldReader<bool, bool>);
impl LCKSB45_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB45` writer - LCKSB45"]
pub struct LCKSB45_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB45_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB46` reader - LCKSB46"]
pub struct LCKSB46_R(crate::FieldReader<bool, bool>);
impl LCKSB46_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB46` writer - LCKSB46"]
pub struct LCKSB46_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB46_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB47` reader - LCKSB47"]
pub struct LCKSB47_R(crate::FieldReader<bool, bool>);
impl LCKSB47_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB47` writer - LCKSB47"]
pub struct LCKSB47_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB47_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB48` reader - LCKSB48"]
pub struct LCKSB48_R(crate::FieldReader<bool, bool>);
impl LCKSB48_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB48` writer - LCKSB48"]
pub struct LCKSB48_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB48_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB49` reader - LCKSB49"]
pub struct LCKSB49_R(crate::FieldReader<bool, bool>);
impl LCKSB49_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB49` writer - LCKSB49"]
pub struct LCKSB49_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB49_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB50` reader - LCKSB50"]
pub struct LCKSB50_R(crate::FieldReader<bool, bool>);
impl LCKSB50_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB50_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB50_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB50` writer - LCKSB50"]
pub struct LCKSB50_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB50_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB51` reader - LCKSB51"]
pub struct LCKSB51_R(crate::FieldReader<bool, bool>);
impl LCKSB51_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB51_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB51_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB51` writer - LCKSB51"]
pub struct LCKSB51_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB51_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB52` reader - LCKSB52"]
pub struct LCKSB52_R(crate::FieldReader<bool, bool>);
impl LCKSB52_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB52_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB52_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB52` writer - LCKSB52"]
pub struct LCKSB52_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB52_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB53` reader - LCKSB53"]
pub struct LCKSB53_R(crate::FieldReader<bool, bool>);
impl LCKSB53_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB53_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB53_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB53` writer - LCKSB53"]
pub struct LCKSB53_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB53_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB54` reader - LCKSB54"]
pub struct LCKSB54_R(crate::FieldReader<bool, bool>);
impl LCKSB54_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB54_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB54_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB54` writer - LCKSB54"]
pub struct LCKSB54_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB54_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB55` reader - LCKSB55"]
pub struct LCKSB55_R(crate::FieldReader<bool, bool>);
impl LCKSB55_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB55_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB55_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB55` writer - LCKSB55"]
pub struct LCKSB55_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB55_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB56` reader - LCKSB56"]
pub struct LCKSB56_R(crate::FieldReader<bool, bool>);
impl LCKSB56_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB56_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB56_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB56` writer - LCKSB56"]
pub struct LCKSB56_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB56_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB57` reader - LCKSB57"]
pub struct LCKSB57_R(crate::FieldReader<bool, bool>);
impl LCKSB57_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB57_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB57_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB57` writer - LCKSB57"]
pub struct LCKSB57_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB57_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB58` reader - LCKSB58"]
pub struct LCKSB58_R(crate::FieldReader<bool, bool>);
impl LCKSB58_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB58_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB58_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB58` writer - LCKSB58"]
pub struct LCKSB58_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB58_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB59` reader - LCKSB59"]
pub struct LCKSB59_R(crate::FieldReader<bool, bool>);
impl LCKSB59_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB59_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB59_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB59` writer - LCKSB59"]
pub struct LCKSB59_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB59_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB60` reader - LCKSB60"]
pub struct LCKSB60_R(crate::FieldReader<bool, bool>);
impl LCKSB60_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB60_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB60_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB60` writer - LCKSB60"]
pub struct LCKSB60_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB60_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB61` reader - LCKSB61"]
pub struct LCKSB61_R(crate::FieldReader<bool, bool>);
impl LCKSB61_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB61_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB61_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB61` writer - LCKSB61"]
pub struct LCKSB61_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB61_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB62` reader - LCKSB62"]
pub struct LCKSB62_R(crate::FieldReader<bool, bool>);
impl LCKSB62_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB62_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB62_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB62` writer - LCKSB62"]
pub struct LCKSB62_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB62_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB63` reader - LCKSB63"]
pub struct LCKSB63_R(crate::FieldReader<bool, bool>);
impl LCKSB63_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB63_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB63_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB63` writer - LCKSB63"]
pub struct LCKSB63_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB63_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - LCKSB32"]
    #[inline(always)]
    pub fn lcksb32(&self) -> LCKSB32_R {
        LCKSB32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCKSB33"]
    #[inline(always)]
    pub fn lcksb33(&self) -> LCKSB33_R {
        LCKSB33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCKSB34"]
    #[inline(always)]
    pub fn lcksb34(&self) -> LCKSB34_R {
        LCKSB34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCKSB35"]
    #[inline(always)]
    pub fn lcksb35(&self) -> LCKSB35_R {
        LCKSB35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCKSB36"]
    #[inline(always)]
    pub fn lcksb36(&self) -> LCKSB36_R {
        LCKSB36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCKSB37"]
    #[inline(always)]
    pub fn lcksb37(&self) -> LCKSB37_R {
        LCKSB37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCKSB38"]
    #[inline(always)]
    pub fn lcksb38(&self) -> LCKSB38_R {
        LCKSB38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCKSB39"]
    #[inline(always)]
    pub fn lcksb39(&self) -> LCKSB39_R {
        LCKSB39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCKSB40"]
    #[inline(always)]
    pub fn lcksb40(&self) -> LCKSB40_R {
        LCKSB40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCKSB41"]
    #[inline(always)]
    pub fn lcksb41(&self) -> LCKSB41_R {
        LCKSB41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCKSB42"]
    #[inline(always)]
    pub fn lcksb42(&self) -> LCKSB42_R {
        LCKSB42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCKSB43"]
    #[inline(always)]
    pub fn lcksb43(&self) -> LCKSB43_R {
        LCKSB43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCKSB44"]
    #[inline(always)]
    pub fn lcksb44(&self) -> LCKSB44_R {
        LCKSB44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCKSB45"]
    #[inline(always)]
    pub fn lcksb45(&self) -> LCKSB45_R {
        LCKSB45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCKSB46"]
    #[inline(always)]
    pub fn lcksb46(&self) -> LCKSB46_R {
        LCKSB46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCKSB47"]
    #[inline(always)]
    pub fn lcksb47(&self) -> LCKSB47_R {
        LCKSB47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LCKSB48"]
    #[inline(always)]
    pub fn lcksb48(&self) -> LCKSB48_R {
        LCKSB48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LCKSB49"]
    #[inline(always)]
    pub fn lcksb49(&self) -> LCKSB49_R {
        LCKSB49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LCKSB50"]
    #[inline(always)]
    pub fn lcksb50(&self) -> LCKSB50_R {
        LCKSB50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LCKSB51"]
    #[inline(always)]
    pub fn lcksb51(&self) -> LCKSB51_R {
        LCKSB51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LCKSB52"]
    #[inline(always)]
    pub fn lcksb52(&self) -> LCKSB52_R {
        LCKSB52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LCKSB53"]
    #[inline(always)]
    pub fn lcksb53(&self) -> LCKSB53_R {
        LCKSB53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LCKSB54"]
    #[inline(always)]
    pub fn lcksb54(&self) -> LCKSB54_R {
        LCKSB54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LCKSB55"]
    #[inline(always)]
    pub fn lcksb55(&self) -> LCKSB55_R {
        LCKSB55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LCKSB56"]
    #[inline(always)]
    pub fn lcksb56(&self) -> LCKSB56_R {
        LCKSB56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LCKSB57"]
    #[inline(always)]
    pub fn lcksb57(&self) -> LCKSB57_R {
        LCKSB57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LCKSB58"]
    #[inline(always)]
    pub fn lcksb58(&self) -> LCKSB58_R {
        LCKSB58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LCKSB59"]
    #[inline(always)]
    pub fn lcksb59(&self) -> LCKSB59_R {
        LCKSB59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LCKSB60"]
    #[inline(always)]
    pub fn lcksb60(&self) -> LCKSB60_R {
        LCKSB60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LCKSB61"]
    #[inline(always)]
    pub fn lcksb61(&self) -> LCKSB61_R {
        LCKSB61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LCKSB62"]
    #[inline(always)]
    pub fn lcksb62(&self) -> LCKSB62_R {
        LCKSB62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LCKSB63"]
    #[inline(always)]
    pub fn lcksb63(&self) -> LCKSB63_R {
        LCKSB63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCKSB32"]
    #[inline(always)]
    pub fn lcksb32(&mut self) -> LCKSB32_W {
        LCKSB32_W { w: self }
    }
    #[doc = "Bit 1 - LCKSB33"]
    #[inline(always)]
    pub fn lcksb33(&mut self) -> LCKSB33_W {
        LCKSB33_W { w: self }
    }
    #[doc = "Bit 2 - LCKSB34"]
    #[inline(always)]
    pub fn lcksb34(&mut self) -> LCKSB34_W {
        LCKSB34_W { w: self }
    }
    #[doc = "Bit 3 - LCKSB35"]
    #[inline(always)]
    pub fn lcksb35(&mut self) -> LCKSB35_W {
        LCKSB35_W { w: self }
    }
    #[doc = "Bit 4 - LCKSB36"]
    #[inline(always)]
    pub fn lcksb36(&mut self) -> LCKSB36_W {
        LCKSB36_W { w: self }
    }
    #[doc = "Bit 5 - LCKSB37"]
    #[inline(always)]
    pub fn lcksb37(&mut self) -> LCKSB37_W {
        LCKSB37_W { w: self }
    }
    #[doc = "Bit 6 - LCKSB38"]
    #[inline(always)]
    pub fn lcksb38(&mut self) -> LCKSB38_W {
        LCKSB38_W { w: self }
    }
    #[doc = "Bit 7 - LCKSB39"]
    #[inline(always)]
    pub fn lcksb39(&mut self) -> LCKSB39_W {
        LCKSB39_W { w: self }
    }
    #[doc = "Bit 8 - LCKSB40"]
    #[inline(always)]
    pub fn lcksb40(&mut self) -> LCKSB40_W {
        LCKSB40_W { w: self }
    }
    #[doc = "Bit 9 - LCKSB41"]
    #[inline(always)]
    pub fn lcksb41(&mut self) -> LCKSB41_W {
        LCKSB41_W { w: self }
    }
    #[doc = "Bit 10 - LCKSB42"]
    #[inline(always)]
    pub fn lcksb42(&mut self) -> LCKSB42_W {
        LCKSB42_W { w: self }
    }
    #[doc = "Bit 11 - LCKSB43"]
    #[inline(always)]
    pub fn lcksb43(&mut self) -> LCKSB43_W {
        LCKSB43_W { w: self }
    }
    #[doc = "Bit 12 - LCKSB44"]
    #[inline(always)]
    pub fn lcksb44(&mut self) -> LCKSB44_W {
        LCKSB44_W { w: self }
    }
    #[doc = "Bit 13 - LCKSB45"]
    #[inline(always)]
    pub fn lcksb45(&mut self) -> LCKSB45_W {
        LCKSB45_W { w: self }
    }
    #[doc = "Bit 14 - LCKSB46"]
    #[inline(always)]
    pub fn lcksb46(&mut self) -> LCKSB46_W {
        LCKSB46_W { w: self }
    }
    #[doc = "Bit 15 - LCKSB47"]
    #[inline(always)]
    pub fn lcksb47(&mut self) -> LCKSB47_W {
        LCKSB47_W { w: self }
    }
    #[doc = "Bit 16 - LCKSB48"]
    #[inline(always)]
    pub fn lcksb48(&mut self) -> LCKSB48_W {
        LCKSB48_W { w: self }
    }
    #[doc = "Bit 17 - LCKSB49"]
    #[inline(always)]
    pub fn lcksb49(&mut self) -> LCKSB49_W {
        LCKSB49_W { w: self }
    }
    #[doc = "Bit 18 - LCKSB50"]
    #[inline(always)]
    pub fn lcksb50(&mut self) -> LCKSB50_W {
        LCKSB50_W { w: self }
    }
    #[doc = "Bit 19 - LCKSB51"]
    #[inline(always)]
    pub fn lcksb51(&mut self) -> LCKSB51_W {
        LCKSB51_W { w: self }
    }
    #[doc = "Bit 20 - LCKSB52"]
    #[inline(always)]
    pub fn lcksb52(&mut self) -> LCKSB52_W {
        LCKSB52_W { w: self }
    }
    #[doc = "Bit 21 - LCKSB53"]
    #[inline(always)]
    pub fn lcksb53(&mut self) -> LCKSB53_W {
        LCKSB53_W { w: self }
    }
    #[doc = "Bit 22 - LCKSB54"]
    #[inline(always)]
    pub fn lcksb54(&mut self) -> LCKSB54_W {
        LCKSB54_W { w: self }
    }
    #[doc = "Bit 23 - LCKSB55"]
    #[inline(always)]
    pub fn lcksb55(&mut self) -> LCKSB55_W {
        LCKSB55_W { w: self }
    }
    #[doc = "Bit 24 - LCKSB56"]
    #[inline(always)]
    pub fn lcksb56(&mut self) -> LCKSB56_W {
        LCKSB56_W { w: self }
    }
    #[doc = "Bit 25 - LCKSB57"]
    #[inline(always)]
    pub fn lcksb57(&mut self) -> LCKSB57_W {
        LCKSB57_W { w: self }
    }
    #[doc = "Bit 26 - LCKSB58"]
    #[inline(always)]
    pub fn lcksb58(&mut self) -> LCKSB58_W {
        LCKSB58_W { w: self }
    }
    #[doc = "Bit 27 - LCKSB59"]
    #[inline(always)]
    pub fn lcksb59(&mut self) -> LCKSB59_W {
        LCKSB59_W { w: self }
    }
    #[doc = "Bit 28 - LCKSB60"]
    #[inline(always)]
    pub fn lcksb60(&mut self) -> LCKSB60_W {
        LCKSB60_W { w: self }
    }
    #[doc = "Bit 29 - LCKSB61"]
    #[inline(always)]
    pub fn lcksb61(&mut self) -> LCKSB61_W {
        LCKSB61_W { w: self }
    }
    #[doc = "Bit 30 - LCKSB62"]
    #[inline(always)]
    pub fn lcksb62(&mut self) -> LCKSB62_W {
        LCKSB62_W { w: self }
    }
    #[doc = "Bit 31 - LCKSB63"]
    #[inline(always)]
    pub fn lcksb63(&mut self) -> LCKSB63_W {
        LCKSB63_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb2_lckvtr2](index.html) module"]
pub struct MPCBB2_LCKVTR2_SPEC;
impl crate::RegisterSpec for MPCBB2_LCKVTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb2_lckvtr2::R](R) reader structure"]
impl crate::Readable for MPCBB2_LCKVTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb2_lckvtr2::W](W) writer structure"]
impl crate::Writable for MPCBB2_LCKVTR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB2_LCKVTR2 to value 0"]
impl crate::Resettable for MPCBB2_LCKVTR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
