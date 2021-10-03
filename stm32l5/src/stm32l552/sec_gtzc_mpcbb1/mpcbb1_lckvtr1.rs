#[doc = "Register `MPCBB1_LCKVTR1` reader"]
pub struct R(crate::R<MPCBB1_LCKVTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCBB1_LCKVTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCBB1_LCKVTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCBB1_LCKVTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCBB1_LCKVTR1` writer"]
pub struct W(crate::W<MPCBB1_LCKVTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCBB1_LCKVTR1_SPEC>;
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
impl From<crate::W<MPCBB1_LCKVTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCBB1_LCKVTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCKSB0` reader - LCKSB0"]
pub struct LCKSB0_R(crate::FieldReader<bool, bool>);
impl LCKSB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB0` writer - LCKSB0"]
pub struct LCKSB0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB1` reader - LCKSB1"]
pub struct LCKSB1_R(crate::FieldReader<bool, bool>);
impl LCKSB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB1` writer - LCKSB1"]
pub struct LCKSB1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB2` reader - LCKSB2"]
pub struct LCKSB2_R(crate::FieldReader<bool, bool>);
impl LCKSB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB2` writer - LCKSB2"]
pub struct LCKSB2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB3` reader - LCKSB3"]
pub struct LCKSB3_R(crate::FieldReader<bool, bool>);
impl LCKSB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB3` writer - LCKSB3"]
pub struct LCKSB3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB4` reader - LCKSB4"]
pub struct LCKSB4_R(crate::FieldReader<bool, bool>);
impl LCKSB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB4` writer - LCKSB4"]
pub struct LCKSB4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB5` reader - LCKSB5"]
pub struct LCKSB5_R(crate::FieldReader<bool, bool>);
impl LCKSB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB5` writer - LCKSB5"]
pub struct LCKSB5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB6` reader - LCKSB6"]
pub struct LCKSB6_R(crate::FieldReader<bool, bool>);
impl LCKSB6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB6` writer - LCKSB6"]
pub struct LCKSB6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB7` reader - LCKSB7"]
pub struct LCKSB7_R(crate::FieldReader<bool, bool>);
impl LCKSB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB7` writer - LCKSB7"]
pub struct LCKSB7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB8` reader - LCKSB8"]
pub struct LCKSB8_R(crate::FieldReader<bool, bool>);
impl LCKSB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB8` writer - LCKSB8"]
pub struct LCKSB8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB9` reader - LCKSB9"]
pub struct LCKSB9_R(crate::FieldReader<bool, bool>);
impl LCKSB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB9` writer - LCKSB9"]
pub struct LCKSB9_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB10` reader - LCKSB10"]
pub struct LCKSB10_R(crate::FieldReader<bool, bool>);
impl LCKSB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB10` writer - LCKSB10"]
pub struct LCKSB10_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB11` reader - LCKSB11"]
pub struct LCKSB11_R(crate::FieldReader<bool, bool>);
impl LCKSB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB11` writer - LCKSB11"]
pub struct LCKSB11_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB12` reader - LCKSB12"]
pub struct LCKSB12_R(crate::FieldReader<bool, bool>);
impl LCKSB12_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB12` writer - LCKSB12"]
pub struct LCKSB12_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB13` reader - LCKSB13"]
pub struct LCKSB13_R(crate::FieldReader<bool, bool>);
impl LCKSB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB13` writer - LCKSB13"]
pub struct LCKSB13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB14` reader - LCKSB14"]
pub struct LCKSB14_R(crate::FieldReader<bool, bool>);
impl LCKSB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB14` writer - LCKSB14"]
pub struct LCKSB14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB15` reader - LCKSB15"]
pub struct LCKSB15_R(crate::FieldReader<bool, bool>);
impl LCKSB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB15` writer - LCKSB15"]
pub struct LCKSB15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB16` reader - LCKSB16"]
pub struct LCKSB16_R(crate::FieldReader<bool, bool>);
impl LCKSB16_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB16` writer - LCKSB16"]
pub struct LCKSB16_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB17` reader - LCKSB17"]
pub struct LCKSB17_R(crate::FieldReader<bool, bool>);
impl LCKSB17_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB17` writer - LCKSB17"]
pub struct LCKSB17_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB18` reader - LCKSB18"]
pub struct LCKSB18_R(crate::FieldReader<bool, bool>);
impl LCKSB18_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB18` writer - LCKSB18"]
pub struct LCKSB18_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB19` reader - LCKSB19"]
pub struct LCKSB19_R(crate::FieldReader<bool, bool>);
impl LCKSB19_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB19` writer - LCKSB19"]
pub struct LCKSB19_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB20` reader - LCKSB20"]
pub struct LCKSB20_R(crate::FieldReader<bool, bool>);
impl LCKSB20_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB20` writer - LCKSB20"]
pub struct LCKSB20_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB21` reader - LCKSB21"]
pub struct LCKSB21_R(crate::FieldReader<bool, bool>);
impl LCKSB21_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB21` writer - LCKSB21"]
pub struct LCKSB21_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB22` reader - LCKSB22"]
pub struct LCKSB22_R(crate::FieldReader<bool, bool>);
impl LCKSB22_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB22` writer - LCKSB22"]
pub struct LCKSB22_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB23` reader - LCKSB23"]
pub struct LCKSB23_R(crate::FieldReader<bool, bool>);
impl LCKSB23_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB23` writer - LCKSB23"]
pub struct LCKSB23_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB24` reader - LCKSB24"]
pub struct LCKSB24_R(crate::FieldReader<bool, bool>);
impl LCKSB24_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB24` writer - LCKSB24"]
pub struct LCKSB24_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB25` reader - LCKSB25"]
pub struct LCKSB25_R(crate::FieldReader<bool, bool>);
impl LCKSB25_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB25` writer - LCKSB25"]
pub struct LCKSB25_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB26` reader - LCKSB26"]
pub struct LCKSB26_R(crate::FieldReader<bool, bool>);
impl LCKSB26_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB26` writer - LCKSB26"]
pub struct LCKSB26_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB27` reader - LCKSB27"]
pub struct LCKSB27_R(crate::FieldReader<bool, bool>);
impl LCKSB27_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB27` writer - LCKSB27"]
pub struct LCKSB27_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB28` reader - LCKSB28"]
pub struct LCKSB28_R(crate::FieldReader<bool, bool>);
impl LCKSB28_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB28` writer - LCKSB28"]
pub struct LCKSB28_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB29` reader - LCKSB29"]
pub struct LCKSB29_R(crate::FieldReader<bool, bool>);
impl LCKSB29_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB29` writer - LCKSB29"]
pub struct LCKSB29_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB30` reader - LCKSB30"]
pub struct LCKSB30_R(crate::FieldReader<bool, bool>);
impl LCKSB30_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB30` writer - LCKSB30"]
pub struct LCKSB30_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKSB31` reader - LCKSB31"]
pub struct LCKSB31_R(crate::FieldReader<bool, bool>);
impl LCKSB31_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKSB31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKSB31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKSB31` writer - LCKSB31"]
pub struct LCKSB31_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKSB31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - LCKSB0"]
    #[inline(always)]
    pub fn lcksb0(&self) -> LCKSB0_R {
        LCKSB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCKSB1"]
    #[inline(always)]
    pub fn lcksb1(&self) -> LCKSB1_R {
        LCKSB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCKSB2"]
    #[inline(always)]
    pub fn lcksb2(&self) -> LCKSB2_R {
        LCKSB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCKSB3"]
    #[inline(always)]
    pub fn lcksb3(&self) -> LCKSB3_R {
        LCKSB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCKSB4"]
    #[inline(always)]
    pub fn lcksb4(&self) -> LCKSB4_R {
        LCKSB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCKSB5"]
    #[inline(always)]
    pub fn lcksb5(&self) -> LCKSB5_R {
        LCKSB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCKSB6"]
    #[inline(always)]
    pub fn lcksb6(&self) -> LCKSB6_R {
        LCKSB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCKSB7"]
    #[inline(always)]
    pub fn lcksb7(&self) -> LCKSB7_R {
        LCKSB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCKSB8"]
    #[inline(always)]
    pub fn lcksb8(&self) -> LCKSB8_R {
        LCKSB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCKSB9"]
    #[inline(always)]
    pub fn lcksb9(&self) -> LCKSB9_R {
        LCKSB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCKSB10"]
    #[inline(always)]
    pub fn lcksb10(&self) -> LCKSB10_R {
        LCKSB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCKSB11"]
    #[inline(always)]
    pub fn lcksb11(&self) -> LCKSB11_R {
        LCKSB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCKSB12"]
    #[inline(always)]
    pub fn lcksb12(&self) -> LCKSB12_R {
        LCKSB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCKSB13"]
    #[inline(always)]
    pub fn lcksb13(&self) -> LCKSB13_R {
        LCKSB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCKSB14"]
    #[inline(always)]
    pub fn lcksb14(&self) -> LCKSB14_R {
        LCKSB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCKSB15"]
    #[inline(always)]
    pub fn lcksb15(&self) -> LCKSB15_R {
        LCKSB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LCKSB16"]
    #[inline(always)]
    pub fn lcksb16(&self) -> LCKSB16_R {
        LCKSB16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - LCKSB17"]
    #[inline(always)]
    pub fn lcksb17(&self) -> LCKSB17_R {
        LCKSB17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - LCKSB18"]
    #[inline(always)]
    pub fn lcksb18(&self) -> LCKSB18_R {
        LCKSB18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LCKSB19"]
    #[inline(always)]
    pub fn lcksb19(&self) -> LCKSB19_R {
        LCKSB19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LCKSB20"]
    #[inline(always)]
    pub fn lcksb20(&self) -> LCKSB20_R {
        LCKSB20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LCKSB21"]
    #[inline(always)]
    pub fn lcksb21(&self) -> LCKSB21_R {
        LCKSB21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - LCKSB22"]
    #[inline(always)]
    pub fn lcksb22(&self) -> LCKSB22_R {
        LCKSB22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - LCKSB23"]
    #[inline(always)]
    pub fn lcksb23(&self) -> LCKSB23_R {
        LCKSB23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LCKSB24"]
    #[inline(always)]
    pub fn lcksb24(&self) -> LCKSB24_R {
        LCKSB24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LCKSB25"]
    #[inline(always)]
    pub fn lcksb25(&self) -> LCKSB25_R {
        LCKSB25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LCKSB26"]
    #[inline(always)]
    pub fn lcksb26(&self) -> LCKSB26_R {
        LCKSB26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LCKSB27"]
    #[inline(always)]
    pub fn lcksb27(&self) -> LCKSB27_R {
        LCKSB27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LCKSB28"]
    #[inline(always)]
    pub fn lcksb28(&self) -> LCKSB28_R {
        LCKSB28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LCKSB29"]
    #[inline(always)]
    pub fn lcksb29(&self) -> LCKSB29_R {
        LCKSB29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LCKSB30"]
    #[inline(always)]
    pub fn lcksb30(&self) -> LCKSB30_R {
        LCKSB30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LCKSB31"]
    #[inline(always)]
    pub fn lcksb31(&self) -> LCKSB31_R {
        LCKSB31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCKSB0"]
    #[inline(always)]
    pub fn lcksb0(&mut self) -> LCKSB0_W {
        LCKSB0_W { w: self }
    }
    #[doc = "Bit 1 - LCKSB1"]
    #[inline(always)]
    pub fn lcksb1(&mut self) -> LCKSB1_W {
        LCKSB1_W { w: self }
    }
    #[doc = "Bit 2 - LCKSB2"]
    #[inline(always)]
    pub fn lcksb2(&mut self) -> LCKSB2_W {
        LCKSB2_W { w: self }
    }
    #[doc = "Bit 3 - LCKSB3"]
    #[inline(always)]
    pub fn lcksb3(&mut self) -> LCKSB3_W {
        LCKSB3_W { w: self }
    }
    #[doc = "Bit 4 - LCKSB4"]
    #[inline(always)]
    pub fn lcksb4(&mut self) -> LCKSB4_W {
        LCKSB4_W { w: self }
    }
    #[doc = "Bit 5 - LCKSB5"]
    #[inline(always)]
    pub fn lcksb5(&mut self) -> LCKSB5_W {
        LCKSB5_W { w: self }
    }
    #[doc = "Bit 6 - LCKSB6"]
    #[inline(always)]
    pub fn lcksb6(&mut self) -> LCKSB6_W {
        LCKSB6_W { w: self }
    }
    #[doc = "Bit 7 - LCKSB7"]
    #[inline(always)]
    pub fn lcksb7(&mut self) -> LCKSB7_W {
        LCKSB7_W { w: self }
    }
    #[doc = "Bit 8 - LCKSB8"]
    #[inline(always)]
    pub fn lcksb8(&mut self) -> LCKSB8_W {
        LCKSB8_W { w: self }
    }
    #[doc = "Bit 9 - LCKSB9"]
    #[inline(always)]
    pub fn lcksb9(&mut self) -> LCKSB9_W {
        LCKSB9_W { w: self }
    }
    #[doc = "Bit 10 - LCKSB10"]
    #[inline(always)]
    pub fn lcksb10(&mut self) -> LCKSB10_W {
        LCKSB10_W { w: self }
    }
    #[doc = "Bit 11 - LCKSB11"]
    #[inline(always)]
    pub fn lcksb11(&mut self) -> LCKSB11_W {
        LCKSB11_W { w: self }
    }
    #[doc = "Bit 12 - LCKSB12"]
    #[inline(always)]
    pub fn lcksb12(&mut self) -> LCKSB12_W {
        LCKSB12_W { w: self }
    }
    #[doc = "Bit 13 - LCKSB13"]
    #[inline(always)]
    pub fn lcksb13(&mut self) -> LCKSB13_W {
        LCKSB13_W { w: self }
    }
    #[doc = "Bit 14 - LCKSB14"]
    #[inline(always)]
    pub fn lcksb14(&mut self) -> LCKSB14_W {
        LCKSB14_W { w: self }
    }
    #[doc = "Bit 15 - LCKSB15"]
    #[inline(always)]
    pub fn lcksb15(&mut self) -> LCKSB15_W {
        LCKSB15_W { w: self }
    }
    #[doc = "Bit 16 - LCKSB16"]
    #[inline(always)]
    pub fn lcksb16(&mut self) -> LCKSB16_W {
        LCKSB16_W { w: self }
    }
    #[doc = "Bit 17 - LCKSB17"]
    #[inline(always)]
    pub fn lcksb17(&mut self) -> LCKSB17_W {
        LCKSB17_W { w: self }
    }
    #[doc = "Bit 18 - LCKSB18"]
    #[inline(always)]
    pub fn lcksb18(&mut self) -> LCKSB18_W {
        LCKSB18_W { w: self }
    }
    #[doc = "Bit 19 - LCKSB19"]
    #[inline(always)]
    pub fn lcksb19(&mut self) -> LCKSB19_W {
        LCKSB19_W { w: self }
    }
    #[doc = "Bit 20 - LCKSB20"]
    #[inline(always)]
    pub fn lcksb20(&mut self) -> LCKSB20_W {
        LCKSB20_W { w: self }
    }
    #[doc = "Bit 21 - LCKSB21"]
    #[inline(always)]
    pub fn lcksb21(&mut self) -> LCKSB21_W {
        LCKSB21_W { w: self }
    }
    #[doc = "Bit 22 - LCKSB22"]
    #[inline(always)]
    pub fn lcksb22(&mut self) -> LCKSB22_W {
        LCKSB22_W { w: self }
    }
    #[doc = "Bit 23 - LCKSB23"]
    #[inline(always)]
    pub fn lcksb23(&mut self) -> LCKSB23_W {
        LCKSB23_W { w: self }
    }
    #[doc = "Bit 24 - LCKSB24"]
    #[inline(always)]
    pub fn lcksb24(&mut self) -> LCKSB24_W {
        LCKSB24_W { w: self }
    }
    #[doc = "Bit 25 - LCKSB25"]
    #[inline(always)]
    pub fn lcksb25(&mut self) -> LCKSB25_W {
        LCKSB25_W { w: self }
    }
    #[doc = "Bit 26 - LCKSB26"]
    #[inline(always)]
    pub fn lcksb26(&mut self) -> LCKSB26_W {
        LCKSB26_W { w: self }
    }
    #[doc = "Bit 27 - LCKSB27"]
    #[inline(always)]
    pub fn lcksb27(&mut self) -> LCKSB27_W {
        LCKSB27_W { w: self }
    }
    #[doc = "Bit 28 - LCKSB28"]
    #[inline(always)]
    pub fn lcksb28(&mut self) -> LCKSB28_W {
        LCKSB28_W { w: self }
    }
    #[doc = "Bit 29 - LCKSB29"]
    #[inline(always)]
    pub fn lcksb29(&mut self) -> LCKSB29_W {
        LCKSB29_W { w: self }
    }
    #[doc = "Bit 30 - LCKSB30"]
    #[inline(always)]
    pub fn lcksb30(&mut self) -> LCKSB30_W {
        LCKSB30_W { w: self }
    }
    #[doc = "Bit 31 - LCKSB31"]
    #[inline(always)]
    pub fn lcksb31(&mut self) -> LCKSB31_W {
        LCKSB31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcbb1_lckvtr1](index.html) module"]
pub struct MPCBB1_LCKVTR1_SPEC;
impl crate::RegisterSpec for MPCBB1_LCKVTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcbb1_lckvtr1::R](R) reader structure"]
impl crate::Readable for MPCBB1_LCKVTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcbb1_lckvtr1::W](W) writer structure"]
impl crate::Writable for MPCBB1_LCKVTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPCBB1_LCKVTR1 to value 0"]
impl crate::Resettable for MPCBB1_LCKVTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
