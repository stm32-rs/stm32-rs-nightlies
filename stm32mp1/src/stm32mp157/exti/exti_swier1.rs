#[doc = "Register `EXTI_SWIER1` reader"]
pub struct R(crate::R<EXTI_SWIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_SWIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_SWIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_SWIER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_SWIER1` writer"]
pub struct W(crate::W<EXTI_SWIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_SWIER1_SPEC>;
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
impl From<crate::W<EXTI_SWIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_SWIER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI0` reader - SWI0"]
pub struct SWI0_R(crate::FieldReader<bool, bool>);
impl SWI0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI0` writer - SWI0"]
pub struct SWI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI1` reader - SWI1"]
pub struct SWI1_R(crate::FieldReader<bool, bool>);
impl SWI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI1` writer - SWI1"]
pub struct SWI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI2` reader - SWI2"]
pub struct SWI2_R(crate::FieldReader<bool, bool>);
impl SWI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI2` writer - SWI2"]
pub struct SWI2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI3` reader - SWI3"]
pub struct SWI3_R(crate::FieldReader<bool, bool>);
impl SWI3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI3` writer - SWI3"]
pub struct SWI3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI4` reader - SWI4"]
pub struct SWI4_R(crate::FieldReader<bool, bool>);
impl SWI4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI4` writer - SWI4"]
pub struct SWI4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI5` reader - SWI5"]
pub struct SWI5_R(crate::FieldReader<bool, bool>);
impl SWI5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI5` writer - SWI5"]
pub struct SWI5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI6` reader - SWI6"]
pub struct SWI6_R(crate::FieldReader<bool, bool>);
impl SWI6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI6` writer - SWI6"]
pub struct SWI6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI7` reader - SWI7"]
pub struct SWI7_R(crate::FieldReader<bool, bool>);
impl SWI7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI7` writer - SWI7"]
pub struct SWI7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI8` reader - SWI8"]
pub struct SWI8_R(crate::FieldReader<bool, bool>);
impl SWI8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI8` writer - SWI8"]
pub struct SWI8_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI9` reader - SWI9"]
pub struct SWI9_R(crate::FieldReader<bool, bool>);
impl SWI9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI9` writer - SWI9"]
pub struct SWI9_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI10` reader - SWI10"]
pub struct SWI10_R(crate::FieldReader<bool, bool>);
impl SWI10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI10` writer - SWI10"]
pub struct SWI10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI11` reader - SWI11"]
pub struct SWI11_R(crate::FieldReader<bool, bool>);
impl SWI11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI11` writer - SWI11"]
pub struct SWI11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI12` reader - SWI12"]
pub struct SWI12_R(crate::FieldReader<bool, bool>);
impl SWI12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI12` writer - SWI12"]
pub struct SWI12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI13` reader - SWI13"]
pub struct SWI13_R(crate::FieldReader<bool, bool>);
impl SWI13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI13` writer - SWI13"]
pub struct SWI13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI14` reader - SWI14"]
pub struct SWI14_R(crate::FieldReader<bool, bool>);
impl SWI14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI14` writer - SWI14"]
pub struct SWI14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI15` reader - SWI15"]
pub struct SWI15_R(crate::FieldReader<bool, bool>);
impl SWI15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI15` writer - SWI15"]
pub struct SWI15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `SWI16` reader - SWI16"]
pub struct SWI16_R(crate::FieldReader<bool, bool>);
impl SWI16_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWI16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWI16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWI16` writer - SWI16"]
pub struct SWI16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWI16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 0 - SWI0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SWI1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SWI2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SWI3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SWI4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SWI5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SWI6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SWI7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SWI8"]
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SWI9"]
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SWI10"]
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SWI11"]
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SWI12"]
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SWI13"]
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SWI14"]
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SWI15"]
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SWI16"]
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SWI0"]
    #[inline(always)]
    pub fn swi0(&mut self) -> SWI0_W {
        SWI0_W { w: self }
    }
    #[doc = "Bit 1 - SWI1"]
    #[inline(always)]
    pub fn swi1(&mut self) -> SWI1_W {
        SWI1_W { w: self }
    }
    #[doc = "Bit 2 - SWI2"]
    #[inline(always)]
    pub fn swi2(&mut self) -> SWI2_W {
        SWI2_W { w: self }
    }
    #[doc = "Bit 3 - SWI3"]
    #[inline(always)]
    pub fn swi3(&mut self) -> SWI3_W {
        SWI3_W { w: self }
    }
    #[doc = "Bit 4 - SWI4"]
    #[inline(always)]
    pub fn swi4(&mut self) -> SWI4_W {
        SWI4_W { w: self }
    }
    #[doc = "Bit 5 - SWI5"]
    #[inline(always)]
    pub fn swi5(&mut self) -> SWI5_W {
        SWI5_W { w: self }
    }
    #[doc = "Bit 6 - SWI6"]
    #[inline(always)]
    pub fn swi6(&mut self) -> SWI6_W {
        SWI6_W { w: self }
    }
    #[doc = "Bit 7 - SWI7"]
    #[inline(always)]
    pub fn swi7(&mut self) -> SWI7_W {
        SWI7_W { w: self }
    }
    #[doc = "Bit 8 - SWI8"]
    #[inline(always)]
    pub fn swi8(&mut self) -> SWI8_W {
        SWI8_W { w: self }
    }
    #[doc = "Bit 9 - SWI9"]
    #[inline(always)]
    pub fn swi9(&mut self) -> SWI9_W {
        SWI9_W { w: self }
    }
    #[doc = "Bit 10 - SWI10"]
    #[inline(always)]
    pub fn swi10(&mut self) -> SWI10_W {
        SWI10_W { w: self }
    }
    #[doc = "Bit 11 - SWI11"]
    #[inline(always)]
    pub fn swi11(&mut self) -> SWI11_W {
        SWI11_W { w: self }
    }
    #[doc = "Bit 12 - SWI12"]
    #[inline(always)]
    pub fn swi12(&mut self) -> SWI12_W {
        SWI12_W { w: self }
    }
    #[doc = "Bit 13 - SWI13"]
    #[inline(always)]
    pub fn swi13(&mut self) -> SWI13_W {
        SWI13_W { w: self }
    }
    #[doc = "Bit 14 - SWI14"]
    #[inline(always)]
    pub fn swi14(&mut self) -> SWI14_W {
        SWI14_W { w: self }
    }
    #[doc = "Bit 15 - SWI15"]
    #[inline(always)]
    pub fn swi15(&mut self) -> SWI15_W {
        SWI15_W { w: self }
    }
    #[doc = "Bit 16 - SWI16"]
    #[inline(always)]
    pub fn swi16(&mut self) -> SWI16_W {
        SWI16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_swier1](index.html) module"]
pub struct EXTI_SWIER1_SPEC;
impl crate::RegisterSpec for EXTI_SWIER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_swier1::R](R) reader structure"]
impl crate::Readable for EXTI_SWIER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_swier1::W](W) writer structure"]
impl crate::Writable for EXTI_SWIER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_SWIER1 to value 0"]
impl crate::Resettable for EXTI_SWIER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
