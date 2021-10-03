#[doc = "Register `PUCRA` reader"]
pub struct R(crate::R<PUCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRA` writer"]
pub struct W(crate::W<PUCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRA_SPEC>;
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
impl From<crate::W<PUCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU15` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU15_R(crate::FieldReader<bool, bool>);
impl PU15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU15` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU15_W<'a> {
    w: &'a mut W,
}
impl<'a> PU15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU14` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU14_R(crate::FieldReader<bool, bool>);
impl PU14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU14` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU14_W<'a> {
    w: &'a mut W,
}
impl<'a> PU14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU13` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU13_R(crate::FieldReader<bool, bool>);
impl PU13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU13` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU13_W<'a> {
    w: &'a mut W,
}
impl<'a> PU13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU12` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU12_R(crate::FieldReader<bool, bool>);
impl PU12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU12` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU12_W<'a> {
    w: &'a mut W,
}
impl<'a> PU12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU11` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU11_R(crate::FieldReader<bool, bool>);
impl PU11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU11` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU11_W<'a> {
    w: &'a mut W,
}
impl<'a> PU11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU10` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU10_R(crate::FieldReader<bool, bool>);
impl PU10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU10` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU10_W<'a> {
    w: &'a mut W,
}
impl<'a> PU10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU9` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU9_R(crate::FieldReader<bool, bool>);
impl PU9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU9` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU9_W<'a> {
    w: &'a mut W,
}
impl<'a> PU9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU8` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU8_R(crate::FieldReader<bool, bool>);
impl PU8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU8` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU8_W<'a> {
    w: &'a mut W,
}
impl<'a> PU8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU7` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU7_R(crate::FieldReader<bool, bool>);
impl PU7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU7` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU7_W<'a> {
    w: &'a mut W,
}
impl<'a> PU7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU6` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU6_R(crate::FieldReader<bool, bool>);
impl PU6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU6` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU6_W<'a> {
    w: &'a mut W,
}
impl<'a> PU6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU5` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU5_R(crate::FieldReader<bool, bool>);
impl PU5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU5` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU5_W<'a> {
    w: &'a mut W,
}
impl<'a> PU5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU4` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU4_R(crate::FieldReader<bool, bool>);
impl PU4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU4` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU4_W<'a> {
    w: &'a mut W,
}
impl<'a> PU4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU3` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU3_R(crate::FieldReader<bool, bool>);
impl PU3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU3` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU3_W<'a> {
    w: &'a mut W,
}
impl<'a> PU3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU2` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU2_R(crate::FieldReader<bool, bool>);
impl PU2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU2` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU2_W<'a> {
    w: &'a mut W,
}
impl<'a> PU2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU1` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU1_R(crate::FieldReader<bool, bool>);
impl PU1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU1` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU1_W<'a> {
    w: &'a mut W,
}
impl<'a> PU1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
#[doc = "Field `PU0` reader - Port A pull-up bit y (y=0..15)"]
pub struct PU0_R(crate::FieldReader<bool, bool>);
impl PU0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU0` writer - Port A pull-up bit y (y=0..15)"]
pub struct PU0_W<'a> {
    w: &'a mut W,
}
impl<'a> PU0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
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
impl R {
    #[doc = "Bit 15 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W {
        PU15_W { w: self }
    }
    #[doc = "Bit 14 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W {
        PU14_W { w: self }
    }
    #[doc = "Bit 13 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W {
        PU13_W { w: self }
    }
    #[doc = "Bit 12 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu12(&mut self) -> PU12_W {
        PU12_W { w: self }
    }
    #[doc = "Bit 11 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu11(&mut self) -> PU11_W {
        PU11_W { w: self }
    }
    #[doc = "Bit 10 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu10(&mut self) -> PU10_W {
        PU10_W { w: self }
    }
    #[doc = "Bit 9 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W {
        PU9_W { w: self }
    }
    #[doc = "Bit 8 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W {
        PU8_W { w: self }
    }
    #[doc = "Bit 7 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W {
        PU7_W { w: self }
    }
    #[doc = "Bit 6 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W {
        PU6_W { w: self }
    }
    #[doc = "Bit 5 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W {
        PU5_W { w: self }
    }
    #[doc = "Bit 4 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W {
        PU4_W { w: self }
    }
    #[doc = "Bit 3 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W {
        PU3_W { w: self }
    }
    #[doc = "Bit 2 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W {
        PU2_W { w: self }
    }
    #[doc = "Bit 1 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W {
        PU1_W { w: self }
    }
    #[doc = "Bit 0 - Port A pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W {
        PU0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port A pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucra](index.html) module"]
pub struct PUCRA_SPEC;
impl crate::RegisterSpec for PUCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucra::R](R) reader structure"]
impl crate::Readable for PUCRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucra::W](W) writer structure"]
impl crate::Writable for PUCRA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCRA to value 0"]
impl crate::Resettable for PUCRA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
