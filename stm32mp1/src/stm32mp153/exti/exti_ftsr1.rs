#[doc = "Register `EXTI_FTSR1` reader"]
pub struct R(crate::R<EXTI_FTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_FTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_FTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_FTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_FTSR1` writer"]
pub struct W(crate::W<EXTI_FTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_FTSR1_SPEC>;
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
impl From<crate::W<EXTI_FTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_FTSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FT0` reader - FT0"]
pub struct FT0_R(crate::FieldReader<bool, bool>);
impl FT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT0` writer - FT0"]
pub struct FT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FT0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT1` reader - FT1"]
pub struct FT1_R(crate::FieldReader<bool, bool>);
impl FT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT1` writer - FT1"]
pub struct FT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FT1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT2` reader - FT2"]
pub struct FT2_R(crate::FieldReader<bool, bool>);
impl FT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT2` writer - FT2"]
pub struct FT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FT2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT3` reader - FT3"]
pub struct FT3_R(crate::FieldReader<bool, bool>);
impl FT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT3` writer - FT3"]
pub struct FT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FT3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT4` reader - FT4"]
pub struct FT4_R(crate::FieldReader<bool, bool>);
impl FT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT4` writer - FT4"]
pub struct FT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FT4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT5` reader - FT5"]
pub struct FT5_R(crate::FieldReader<bool, bool>);
impl FT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT5` writer - FT5"]
pub struct FT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FT5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT6` reader - FT6"]
pub struct FT6_R(crate::FieldReader<bool, bool>);
impl FT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT6` writer - FT6"]
pub struct FT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FT6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT7` reader - FT7"]
pub struct FT7_R(crate::FieldReader<bool, bool>);
impl FT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT7` writer - FT7"]
pub struct FT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FT7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT8` reader - FT8"]
pub struct FT8_R(crate::FieldReader<bool, bool>);
impl FT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT8` writer - FT8"]
pub struct FT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FT8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT9` reader - FT9"]
pub struct FT9_R(crate::FieldReader<bool, bool>);
impl FT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT9` writer - FT9"]
pub struct FT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FT9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT10` reader - FT10"]
pub struct FT10_R(crate::FieldReader<bool, bool>);
impl FT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT10` writer - FT10"]
pub struct FT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FT10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT11` reader - FT11"]
pub struct FT11_R(crate::FieldReader<bool, bool>);
impl FT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT11` writer - FT11"]
pub struct FT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FT11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT12` reader - FT12"]
pub struct FT12_R(crate::FieldReader<bool, bool>);
impl FT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT12` writer - FT12"]
pub struct FT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FT12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT13` reader - FT13"]
pub struct FT13_R(crate::FieldReader<bool, bool>);
impl FT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT13` writer - FT13"]
pub struct FT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FT13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT14` reader - FT14"]
pub struct FT14_R(crate::FieldReader<bool, bool>);
impl FT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT14` writer - FT14"]
pub struct FT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FT14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT15` reader - FT15"]
pub struct FT15_R(crate::FieldReader<bool, bool>);
impl FT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT15` writer - FT15"]
pub struct FT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FT15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `FT16` reader - FT16"]
pub struct FT16_R(crate::FieldReader<bool, bool>);
impl FT16_R {
    pub(crate) fn new(bits: bool) -> Self {
        FT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FT16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FT16` writer - FT16"]
pub struct FT16_W<'a> {
    w: &'a mut W,
}
impl<'a> FT16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - FT0"]
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FT1"]
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FT2"]
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FT3"]
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FT4"]
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FT5"]
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FT6"]
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FT7"]
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - FT8"]
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FT9"]
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FT10"]
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FT11"]
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FT12"]
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FT13"]
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FT14"]
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - FT15"]
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - FT16"]
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FT0"]
    #[inline(always)]
    pub fn ft0(&mut self) -> FT0_W {
        FT0_W { w: self }
    }
    #[doc = "Bit 1 - FT1"]
    #[inline(always)]
    pub fn ft1(&mut self) -> FT1_W {
        FT1_W { w: self }
    }
    #[doc = "Bit 2 - FT2"]
    #[inline(always)]
    pub fn ft2(&mut self) -> FT2_W {
        FT2_W { w: self }
    }
    #[doc = "Bit 3 - FT3"]
    #[inline(always)]
    pub fn ft3(&mut self) -> FT3_W {
        FT3_W { w: self }
    }
    #[doc = "Bit 4 - FT4"]
    #[inline(always)]
    pub fn ft4(&mut self) -> FT4_W {
        FT4_W { w: self }
    }
    #[doc = "Bit 5 - FT5"]
    #[inline(always)]
    pub fn ft5(&mut self) -> FT5_W {
        FT5_W { w: self }
    }
    #[doc = "Bit 6 - FT6"]
    #[inline(always)]
    pub fn ft6(&mut self) -> FT6_W {
        FT6_W { w: self }
    }
    #[doc = "Bit 7 - FT7"]
    #[inline(always)]
    pub fn ft7(&mut self) -> FT7_W {
        FT7_W { w: self }
    }
    #[doc = "Bit 8 - FT8"]
    #[inline(always)]
    pub fn ft8(&mut self) -> FT8_W {
        FT8_W { w: self }
    }
    #[doc = "Bit 9 - FT9"]
    #[inline(always)]
    pub fn ft9(&mut self) -> FT9_W {
        FT9_W { w: self }
    }
    #[doc = "Bit 10 - FT10"]
    #[inline(always)]
    pub fn ft10(&mut self) -> FT10_W {
        FT10_W { w: self }
    }
    #[doc = "Bit 11 - FT11"]
    #[inline(always)]
    pub fn ft11(&mut self) -> FT11_W {
        FT11_W { w: self }
    }
    #[doc = "Bit 12 - FT12"]
    #[inline(always)]
    pub fn ft12(&mut self) -> FT12_W {
        FT12_W { w: self }
    }
    #[doc = "Bit 13 - FT13"]
    #[inline(always)]
    pub fn ft13(&mut self) -> FT13_W {
        FT13_W { w: self }
    }
    #[doc = "Bit 14 - FT14"]
    #[inline(always)]
    pub fn ft14(&mut self) -> FT14_W {
        FT14_W { w: self }
    }
    #[doc = "Bit 15 - FT15"]
    #[inline(always)]
    pub fn ft15(&mut self) -> FT15_W {
        FT15_W { w: self }
    }
    #[doc = "Bit 16 - FT16"]
    #[inline(always)]
    pub fn ft16(&mut self) -> FT16_W {
        FT16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains only register bits for configurable events.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_ftsr1](index.html) module"]
pub struct EXTI_FTSR1_SPEC;
impl crate::RegisterSpec for EXTI_FTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_ftsr1::R](R) reader structure"]
impl crate::Readable for EXTI_FTSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_ftsr1::W](W) writer structure"]
impl crate::Writable for EXTI_FTSR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_FTSR1 to value 0"]
impl crate::Resettable for EXTI_FTSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
