#[doc = "Register `EXTI_TZENR1` reader"]
pub struct R(crate::R<EXTI_TZENR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_TZENR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_TZENR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_TZENR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_TZENR1` writer"]
pub struct W(crate::W<EXTI_TZENR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_TZENR1_SPEC>;
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
impl From<crate::W<EXTI_TZENR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_TZENR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZEN0` reader - TZEN0"]
pub struct TZEN0_R(crate::FieldReader<bool, bool>);
impl TZEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN0` writer - TZEN0"]
pub struct TZEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN1` reader - TZEN1"]
pub struct TZEN1_R(crate::FieldReader<bool, bool>);
impl TZEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN1` writer - TZEN1"]
pub struct TZEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN2` reader - TZEN2"]
pub struct TZEN2_R(crate::FieldReader<bool, bool>);
impl TZEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN2` writer - TZEN2"]
pub struct TZEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN3` reader - TZEN3"]
pub struct TZEN3_R(crate::FieldReader<bool, bool>);
impl TZEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN3` writer - TZEN3"]
pub struct TZEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN4` reader - TZEN4"]
pub struct TZEN4_R(crate::FieldReader<bool, bool>);
impl TZEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN4` writer - TZEN4"]
pub struct TZEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN5` reader - TZEN5"]
pub struct TZEN5_R(crate::FieldReader<bool, bool>);
impl TZEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN5` writer - TZEN5"]
pub struct TZEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN6` reader - TZEN6"]
pub struct TZEN6_R(crate::FieldReader<bool, bool>);
impl TZEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN6` writer - TZEN6"]
pub struct TZEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN7` reader - TZEN7"]
pub struct TZEN7_R(crate::FieldReader<bool, bool>);
impl TZEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN7` writer - TZEN7"]
pub struct TZEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN8` reader - TZEN8"]
pub struct TZEN8_R(crate::FieldReader<bool, bool>);
impl TZEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN8` writer - TZEN8"]
pub struct TZEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN9` reader - TZEN9"]
pub struct TZEN9_R(crate::FieldReader<bool, bool>);
impl TZEN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN9` writer - TZEN9"]
pub struct TZEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN10` reader - TZEN10"]
pub struct TZEN10_R(crate::FieldReader<bool, bool>);
impl TZEN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN10` writer - TZEN10"]
pub struct TZEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN11` reader - TZEN11"]
pub struct TZEN11_R(crate::FieldReader<bool, bool>);
impl TZEN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN11` writer - TZEN11"]
pub struct TZEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN12` reader - TZEN12"]
pub struct TZEN12_R(crate::FieldReader<bool, bool>);
impl TZEN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN12` writer - TZEN12"]
pub struct TZEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN13` reader - TZEN13"]
pub struct TZEN13_R(crate::FieldReader<bool, bool>);
impl TZEN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN13` writer - TZEN13"]
pub struct TZEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN14` reader - TZEN14"]
pub struct TZEN14_R(crate::FieldReader<bool, bool>);
impl TZEN14_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN14` writer - TZEN14"]
pub struct TZEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN15` reader - TZEN15"]
pub struct TZEN15_R(crate::FieldReader<bool, bool>);
impl TZEN15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN15` writer - TZEN15"]
pub struct TZEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN17` reader - TZEN17"]
pub struct TZEN17_R(crate::FieldReader<bool, bool>);
impl TZEN17_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN17` writer - TZEN17"]
pub struct TZEN17_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN18` reader - TZEN18"]
pub struct TZEN18_R(crate::FieldReader<bool, bool>);
impl TZEN18_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN18` writer - TZEN18"]
pub struct TZEN18_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN19` reader - TZEN19"]
pub struct TZEN19_R(crate::FieldReader<bool, bool>);
impl TZEN19_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN19` writer - TZEN19"]
pub struct TZEN19_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN24` reader - TZEN24"]
pub struct TZEN24_R(crate::FieldReader<bool, bool>);
impl TZEN24_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN24` writer - TZEN24"]
pub struct TZEN24_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `TZEN26` reader - TZEN26"]
pub struct TZEN26_R(crate::FieldReader<bool, bool>);
impl TZEN26_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZEN26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZEN26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TZEN26` writer - TZEN26"]
pub struct TZEN26_W<'a> {
    w: &'a mut W,
}
impl<'a> TZEN26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - TZEN0"]
    #[inline(always)]
    pub fn tzen0(&self) -> TZEN0_R {
        TZEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZEN1"]
    #[inline(always)]
    pub fn tzen1(&self) -> TZEN1_R {
        TZEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TZEN2"]
    #[inline(always)]
    pub fn tzen2(&self) -> TZEN2_R {
        TZEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TZEN3"]
    #[inline(always)]
    pub fn tzen3(&self) -> TZEN3_R {
        TZEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TZEN4"]
    #[inline(always)]
    pub fn tzen4(&self) -> TZEN4_R {
        TZEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TZEN5"]
    #[inline(always)]
    pub fn tzen5(&self) -> TZEN5_R {
        TZEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TZEN6"]
    #[inline(always)]
    pub fn tzen6(&self) -> TZEN6_R {
        TZEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TZEN7"]
    #[inline(always)]
    pub fn tzen7(&self) -> TZEN7_R {
        TZEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TZEN8"]
    #[inline(always)]
    pub fn tzen8(&self) -> TZEN8_R {
        TZEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TZEN9"]
    #[inline(always)]
    pub fn tzen9(&self) -> TZEN9_R {
        TZEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TZEN10"]
    #[inline(always)]
    pub fn tzen10(&self) -> TZEN10_R {
        TZEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZEN11"]
    #[inline(always)]
    pub fn tzen11(&self) -> TZEN11_R {
        TZEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZEN12"]
    #[inline(always)]
    pub fn tzen12(&self) -> TZEN12_R {
        TZEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZEN13"]
    #[inline(always)]
    pub fn tzen13(&self) -> TZEN13_R {
        TZEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TZEN14"]
    #[inline(always)]
    pub fn tzen14(&self) -> TZEN14_R {
        TZEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TZEN15"]
    #[inline(always)]
    pub fn tzen15(&self) -> TZEN15_R {
        TZEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TZEN17"]
    #[inline(always)]
    pub fn tzen17(&self) -> TZEN17_R {
        TZEN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TZEN18"]
    #[inline(always)]
    pub fn tzen18(&self) -> TZEN18_R {
        TZEN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - TZEN19"]
    #[inline(always)]
    pub fn tzen19(&self) -> TZEN19_R {
        TZEN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TZEN24"]
    #[inline(always)]
    pub fn tzen24(&self) -> TZEN24_R {
        TZEN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TZEN26"]
    #[inline(always)]
    pub fn tzen26(&self) -> TZEN26_R {
        TZEN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZEN0"]
    #[inline(always)]
    pub fn tzen0(&mut self) -> TZEN0_W {
        TZEN0_W { w: self }
    }
    #[doc = "Bit 1 - TZEN1"]
    #[inline(always)]
    pub fn tzen1(&mut self) -> TZEN1_W {
        TZEN1_W { w: self }
    }
    #[doc = "Bit 2 - TZEN2"]
    #[inline(always)]
    pub fn tzen2(&mut self) -> TZEN2_W {
        TZEN2_W { w: self }
    }
    #[doc = "Bit 3 - TZEN3"]
    #[inline(always)]
    pub fn tzen3(&mut self) -> TZEN3_W {
        TZEN3_W { w: self }
    }
    #[doc = "Bit 4 - TZEN4"]
    #[inline(always)]
    pub fn tzen4(&mut self) -> TZEN4_W {
        TZEN4_W { w: self }
    }
    #[doc = "Bit 5 - TZEN5"]
    #[inline(always)]
    pub fn tzen5(&mut self) -> TZEN5_W {
        TZEN5_W { w: self }
    }
    #[doc = "Bit 6 - TZEN6"]
    #[inline(always)]
    pub fn tzen6(&mut self) -> TZEN6_W {
        TZEN6_W { w: self }
    }
    #[doc = "Bit 7 - TZEN7"]
    #[inline(always)]
    pub fn tzen7(&mut self) -> TZEN7_W {
        TZEN7_W { w: self }
    }
    #[doc = "Bit 8 - TZEN8"]
    #[inline(always)]
    pub fn tzen8(&mut self) -> TZEN8_W {
        TZEN8_W { w: self }
    }
    #[doc = "Bit 9 - TZEN9"]
    #[inline(always)]
    pub fn tzen9(&mut self) -> TZEN9_W {
        TZEN9_W { w: self }
    }
    #[doc = "Bit 10 - TZEN10"]
    #[inline(always)]
    pub fn tzen10(&mut self) -> TZEN10_W {
        TZEN10_W { w: self }
    }
    #[doc = "Bit 11 - TZEN11"]
    #[inline(always)]
    pub fn tzen11(&mut self) -> TZEN11_W {
        TZEN11_W { w: self }
    }
    #[doc = "Bit 12 - TZEN12"]
    #[inline(always)]
    pub fn tzen12(&mut self) -> TZEN12_W {
        TZEN12_W { w: self }
    }
    #[doc = "Bit 13 - TZEN13"]
    #[inline(always)]
    pub fn tzen13(&mut self) -> TZEN13_W {
        TZEN13_W { w: self }
    }
    #[doc = "Bit 14 - TZEN14"]
    #[inline(always)]
    pub fn tzen14(&mut self) -> TZEN14_W {
        TZEN14_W { w: self }
    }
    #[doc = "Bit 15 - TZEN15"]
    #[inline(always)]
    pub fn tzen15(&mut self) -> TZEN15_W {
        TZEN15_W { w: self }
    }
    #[doc = "Bit 17 - TZEN17"]
    #[inline(always)]
    pub fn tzen17(&mut self) -> TZEN17_W {
        TZEN17_W { w: self }
    }
    #[doc = "Bit 18 - TZEN18"]
    #[inline(always)]
    pub fn tzen18(&mut self) -> TZEN18_W {
        TZEN18_W { w: self }
    }
    #[doc = "Bit 19 - TZEN19"]
    #[inline(always)]
    pub fn tzen19(&mut self) -> TZEN19_W {
        TZEN19_W { w: self }
    }
    #[doc = "Bit 24 - TZEN24"]
    #[inline(always)]
    pub fn tzen24(&mut self) -> TZEN24_W {
        TZEN24_W { w: self }
    }
    #[doc = "Bit 26 - TZEN26"]
    #[inline(always)]
    pub fn tzen26(&mut self) -> TZEN26_W {
        TZEN26_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides TrustZone Write access security, a non-secure write access will generate a bus error. A non-secure read will return the register data. Contains only register bits for TrustZone capable Input events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_tzenr1](index.html) module"]
pub struct EXTI_TZENR1_SPEC;
impl crate::RegisterSpec for EXTI_TZENR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_tzenr1::R](R) reader structure"]
impl crate::Readable for EXTI_TZENR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_tzenr1::W](W) writer structure"]
impl crate::Writable for EXTI_TZENR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_TZENR1 to value 0"]
impl crate::Resettable for EXTI_TZENR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
