#[doc = "Register `GPIOG_LCKR` reader"]
pub struct R(crate::R<GPIOG_LCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOG_LCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOG_LCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOG_LCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOG_LCKR` writer"]
pub struct W(crate::W<GPIOG_LCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOG_LCKR_SPEC>;
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
impl From<crate::W<GPIOG_LCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOG_LCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCK0` reader - LCK0"]
pub struct LCK0_R(crate::FieldReader<bool, bool>);
impl LCK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK0` writer - LCK0"]
pub struct LCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK1` reader - LCK1"]
pub struct LCK1_R(crate::FieldReader<bool, bool>);
impl LCK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK1` writer - LCK1"]
pub struct LCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK2` reader - LCK2"]
pub struct LCK2_R(crate::FieldReader<bool, bool>);
impl LCK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK2` writer - LCK2"]
pub struct LCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK3` reader - LCK3"]
pub struct LCK3_R(crate::FieldReader<bool, bool>);
impl LCK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK3` writer - LCK3"]
pub struct LCK3_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK4` reader - LCK4"]
pub struct LCK4_R(crate::FieldReader<bool, bool>);
impl LCK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK4` writer - LCK4"]
pub struct LCK4_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK5` reader - LCK5"]
pub struct LCK5_R(crate::FieldReader<bool, bool>);
impl LCK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK5` writer - LCK5"]
pub struct LCK5_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK6` reader - LCK6"]
pub struct LCK6_R(crate::FieldReader<bool, bool>);
impl LCK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK6` writer - LCK6"]
pub struct LCK6_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK7` reader - LCK7"]
pub struct LCK7_R(crate::FieldReader<bool, bool>);
impl LCK7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK7` writer - LCK7"]
pub struct LCK7_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK8` reader - LCK8"]
pub struct LCK8_R(crate::FieldReader<bool, bool>);
impl LCK8_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK8` writer - LCK8"]
pub struct LCK8_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK9` reader - LCK9"]
pub struct LCK9_R(crate::FieldReader<bool, bool>);
impl LCK9_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK9` writer - LCK9"]
pub struct LCK9_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK10` reader - LCK10"]
pub struct LCK10_R(crate::FieldReader<bool, bool>);
impl LCK10_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK10` writer - LCK10"]
pub struct LCK10_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK11` reader - LCK11"]
pub struct LCK11_R(crate::FieldReader<bool, bool>);
impl LCK11_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK11` writer - LCK11"]
pub struct LCK11_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK12` reader - LCK12"]
pub struct LCK12_R(crate::FieldReader<bool, bool>);
impl LCK12_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK12` writer - LCK12"]
pub struct LCK12_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK13` reader - LCK13"]
pub struct LCK13_R(crate::FieldReader<bool, bool>);
impl LCK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK13` writer - LCK13"]
pub struct LCK13_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK14` reader - LCK14"]
pub struct LCK14_R(crate::FieldReader<bool, bool>);
impl LCK14_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK14` writer - LCK14"]
pub struct LCK14_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCK15` reader - LCK15"]
pub struct LCK15_R(crate::FieldReader<bool, bool>);
impl LCK15_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCK15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCK15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCK15` writer - LCK15"]
pub struct LCK15_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `LCKK` reader - LCKK"]
pub struct LCKK_R(crate::FieldReader<bool, bool>);
impl LCKK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKK` writer - LCKK"]
pub struct LCKK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCKK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
    #[doc = "Bit 0 - LCK0"]
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LCK1"]
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCK2"]
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LCK3"]
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - LCK4"]
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LCK5"]
    #[inline(always)]
    pub fn lck5(&self) -> LCK5_R {
        LCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LCK6"]
    #[inline(always)]
    pub fn lck6(&self) -> LCK6_R {
        LCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCK7"]
    #[inline(always)]
    pub fn lck7(&self) -> LCK7_R {
        LCK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LCK8"]
    #[inline(always)]
    pub fn lck8(&self) -> LCK8_R {
        LCK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCK9"]
    #[inline(always)]
    pub fn lck9(&self) -> LCK9_R {
        LCK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LCK10"]
    #[inline(always)]
    pub fn lck10(&self) -> LCK10_R {
        LCK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LCK11"]
    #[inline(always)]
    pub fn lck11(&self) -> LCK11_R {
        LCK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LCK12"]
    #[inline(always)]
    pub fn lck12(&self) -> LCK12_R {
        LCK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LCK13"]
    #[inline(always)]
    pub fn lck13(&self) -> LCK13_R {
        LCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LCK14"]
    #[inline(always)]
    pub fn lck14(&self) -> LCK14_R {
        LCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LCK15"]
    #[inline(always)]
    pub fn lck15(&self) -> LCK15_R {
        LCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LCKK"]
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCK0"]
    #[inline(always)]
    pub fn lck0(&mut self) -> LCK0_W {
        LCK0_W { w: self }
    }
    #[doc = "Bit 1 - LCK1"]
    #[inline(always)]
    pub fn lck1(&mut self) -> LCK1_W {
        LCK1_W { w: self }
    }
    #[doc = "Bit 2 - LCK2"]
    #[inline(always)]
    pub fn lck2(&mut self) -> LCK2_W {
        LCK2_W { w: self }
    }
    #[doc = "Bit 3 - LCK3"]
    #[inline(always)]
    pub fn lck3(&mut self) -> LCK3_W {
        LCK3_W { w: self }
    }
    #[doc = "Bit 4 - LCK4"]
    #[inline(always)]
    pub fn lck4(&mut self) -> LCK4_W {
        LCK4_W { w: self }
    }
    #[doc = "Bit 5 - LCK5"]
    #[inline(always)]
    pub fn lck5(&mut self) -> LCK5_W {
        LCK5_W { w: self }
    }
    #[doc = "Bit 6 - LCK6"]
    #[inline(always)]
    pub fn lck6(&mut self) -> LCK6_W {
        LCK6_W { w: self }
    }
    #[doc = "Bit 7 - LCK7"]
    #[inline(always)]
    pub fn lck7(&mut self) -> LCK7_W {
        LCK7_W { w: self }
    }
    #[doc = "Bit 8 - LCK8"]
    #[inline(always)]
    pub fn lck8(&mut self) -> LCK8_W {
        LCK8_W { w: self }
    }
    #[doc = "Bit 9 - LCK9"]
    #[inline(always)]
    pub fn lck9(&mut self) -> LCK9_W {
        LCK9_W { w: self }
    }
    #[doc = "Bit 10 - LCK10"]
    #[inline(always)]
    pub fn lck10(&mut self) -> LCK10_W {
        LCK10_W { w: self }
    }
    #[doc = "Bit 11 - LCK11"]
    #[inline(always)]
    pub fn lck11(&mut self) -> LCK11_W {
        LCK11_W { w: self }
    }
    #[doc = "Bit 12 - LCK12"]
    #[inline(always)]
    pub fn lck12(&mut self) -> LCK12_W {
        LCK12_W { w: self }
    }
    #[doc = "Bit 13 - LCK13"]
    #[inline(always)]
    pub fn lck13(&mut self) -> LCK13_W {
        LCK13_W { w: self }
    }
    #[doc = "Bit 14 - LCK14"]
    #[inline(always)]
    pub fn lck14(&mut self) -> LCK14_W {
        LCK14_W { w: self }
    }
    #[doc = "Bit 15 - LCK15"]
    #[inline(always)]
    pub fn lck15(&mut self) -> LCK15_W {
        LCK15_W { w: self }
    }
    #[doc = "Bit 16 - LCKK"]
    #[inline(always)]
    pub fn lckk(&mut self) -> LCKK_W {
        LCKK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to lock the configuration of the port bits when a correct write sequence is applied to bit 16 (LCKK). The value of bits \\[15:0\\]
is used to lock the configuration of the GPIO. During the write sequence, the value of LCKR\\[15:0\\]
must not change. When the LOCK sequence has been applied on a port bit, the value of this port bit can no longer be modified until the next MCU reset or peripheral reset. A specific write sequence is used to write to the GPIOx_LCKR register. Only word access (32-bit long) is allowed during this locking sequence. Each lock bit freezes a specific configuration register (control and alternate function registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiog_lckr](index.html) module"]
pub struct GPIOG_LCKR_SPEC;
impl crate::RegisterSpec for GPIOG_LCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiog_lckr::R](R) reader structure"]
impl crate::Readable for GPIOG_LCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiog_lckr::W](W) writer structure"]
impl crate::Writable for GPIOG_LCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOG_LCKR to value 0"]
impl crate::Resettable for GPIOG_LCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
