#[doc = "Register `FFA1R` reader"]
pub struct R(crate::R<FFA1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFA1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFA1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFA1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFA1R` writer"]
pub struct W(crate::W<FFA1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFA1R_SPEC>;
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
impl From<crate::W<FFA1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFA1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FFA0` reader - Filter FIFO assignment for filter 0"]
pub struct FFA0_R(crate::FieldReader<bool, bool>);
impl FFA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA0` writer - Filter FIFO assignment for filter 0"]
pub struct FFA0_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA0_W<'a> {
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
#[doc = "Field `FFA1` reader - Filter FIFO assignment for filter 1"]
pub struct FFA1_R(crate::FieldReader<bool, bool>);
impl FFA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA1` writer - Filter FIFO assignment for filter 1"]
pub struct FFA1_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA1_W<'a> {
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
#[doc = "Field `FFA2` reader - Filter FIFO assignment for filter 2"]
pub struct FFA2_R(crate::FieldReader<bool, bool>);
impl FFA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA2` writer - Filter FIFO assignment for filter 2"]
pub struct FFA2_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA2_W<'a> {
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
#[doc = "Field `FFA3` reader - Filter FIFO assignment for filter 3"]
pub struct FFA3_R(crate::FieldReader<bool, bool>);
impl FFA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA3` writer - Filter FIFO assignment for filter 3"]
pub struct FFA3_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA3_W<'a> {
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
#[doc = "Field `FFA4` reader - Filter FIFO assignment for filter 4"]
pub struct FFA4_R(crate::FieldReader<bool, bool>);
impl FFA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA4` writer - Filter FIFO assignment for filter 4"]
pub struct FFA4_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA4_W<'a> {
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
#[doc = "Field `FFA5` reader - Filter FIFO assignment for filter 5"]
pub struct FFA5_R(crate::FieldReader<bool, bool>);
impl FFA5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA5` writer - Filter FIFO assignment for filter 5"]
pub struct FFA5_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA5_W<'a> {
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
#[doc = "Field `FFA6` reader - Filter FIFO assignment for filter 6"]
pub struct FFA6_R(crate::FieldReader<bool, bool>);
impl FFA6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA6` writer - Filter FIFO assignment for filter 6"]
pub struct FFA6_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA6_W<'a> {
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
#[doc = "Field `FFA7` reader - Filter FIFO assignment for filter 7"]
pub struct FFA7_R(crate::FieldReader<bool, bool>);
impl FFA7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA7` writer - Filter FIFO assignment for filter 7"]
pub struct FFA7_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA7_W<'a> {
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
#[doc = "Field `FFA8` reader - Filter FIFO assignment for filter 8"]
pub struct FFA8_R(crate::FieldReader<bool, bool>);
impl FFA8_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA8` writer - Filter FIFO assignment for filter 8"]
pub struct FFA8_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA8_W<'a> {
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
#[doc = "Field `FFA9` reader - Filter FIFO assignment for filter 9"]
pub struct FFA9_R(crate::FieldReader<bool, bool>);
impl FFA9_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA9` writer - Filter FIFO assignment for filter 9"]
pub struct FFA9_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA9_W<'a> {
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
#[doc = "Field `FFA10` reader - Filter FIFO assignment for filter 10"]
pub struct FFA10_R(crate::FieldReader<bool, bool>);
impl FFA10_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA10` writer - Filter FIFO assignment for filter 10"]
pub struct FFA10_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA10_W<'a> {
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
#[doc = "Field `FFA11` reader - Filter FIFO assignment for filter 11"]
pub struct FFA11_R(crate::FieldReader<bool, bool>);
impl FFA11_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA11` writer - Filter FIFO assignment for filter 11"]
pub struct FFA11_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA11_W<'a> {
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
#[doc = "Field `FFA12` reader - Filter FIFO assignment for filter 12"]
pub struct FFA12_R(crate::FieldReader<bool, bool>);
impl FFA12_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA12` writer - Filter FIFO assignment for filter 12"]
pub struct FFA12_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA12_W<'a> {
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
#[doc = "Field `FFA13` reader - Filter FIFO assignment for filter 13"]
pub struct FFA13_R(crate::FieldReader<bool, bool>);
impl FFA13_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFA13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFA13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFA13` writer - Filter FIFO assignment for filter 13"]
pub struct FFA13_W<'a> {
    w: &'a mut W,
}
impl<'a> FFA13_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&self) -> FFA0_R {
        FFA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&self) -> FFA1_R {
        FFA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&self) -> FFA2_R {
        FFA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&self) -> FFA3_R {
        FFA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&self) -> FFA4_R {
        FFA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&self) -> FFA5_R {
        FFA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&self) -> FFA6_R {
        FFA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&self) -> FFA7_R {
        FFA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&self) -> FFA8_R {
        FFA8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&self) -> FFA9_R {
        FFA9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&self) -> FFA10_R {
        FFA10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&self) -> FFA11_R {
        FFA11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&self) -> FFA12_R {
        FFA12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&self) -> FFA13_R {
        FFA13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter FIFO assignment for filter 0"]
    #[inline(always)]
    pub fn ffa0(&mut self) -> FFA0_W {
        FFA0_W { w: self }
    }
    #[doc = "Bit 1 - Filter FIFO assignment for filter 1"]
    #[inline(always)]
    pub fn ffa1(&mut self) -> FFA1_W {
        FFA1_W { w: self }
    }
    #[doc = "Bit 2 - Filter FIFO assignment for filter 2"]
    #[inline(always)]
    pub fn ffa2(&mut self) -> FFA2_W {
        FFA2_W { w: self }
    }
    #[doc = "Bit 3 - Filter FIFO assignment for filter 3"]
    #[inline(always)]
    pub fn ffa3(&mut self) -> FFA3_W {
        FFA3_W { w: self }
    }
    #[doc = "Bit 4 - Filter FIFO assignment for filter 4"]
    #[inline(always)]
    pub fn ffa4(&mut self) -> FFA4_W {
        FFA4_W { w: self }
    }
    #[doc = "Bit 5 - Filter FIFO assignment for filter 5"]
    #[inline(always)]
    pub fn ffa5(&mut self) -> FFA5_W {
        FFA5_W { w: self }
    }
    #[doc = "Bit 6 - Filter FIFO assignment for filter 6"]
    #[inline(always)]
    pub fn ffa6(&mut self) -> FFA6_W {
        FFA6_W { w: self }
    }
    #[doc = "Bit 7 - Filter FIFO assignment for filter 7"]
    #[inline(always)]
    pub fn ffa7(&mut self) -> FFA7_W {
        FFA7_W { w: self }
    }
    #[doc = "Bit 8 - Filter FIFO assignment for filter 8"]
    #[inline(always)]
    pub fn ffa8(&mut self) -> FFA8_W {
        FFA8_W { w: self }
    }
    #[doc = "Bit 9 - Filter FIFO assignment for filter 9"]
    #[inline(always)]
    pub fn ffa9(&mut self) -> FFA9_W {
        FFA9_W { w: self }
    }
    #[doc = "Bit 10 - Filter FIFO assignment for filter 10"]
    #[inline(always)]
    pub fn ffa10(&mut self) -> FFA10_W {
        FFA10_W { w: self }
    }
    #[doc = "Bit 11 - Filter FIFO assignment for filter 11"]
    #[inline(always)]
    pub fn ffa11(&mut self) -> FFA11_W {
        FFA11_W { w: self }
    }
    #[doc = "Bit 12 - Filter FIFO assignment for filter 12"]
    #[inline(always)]
    pub fn ffa12(&mut self) -> FFA12_W {
        FFA12_W { w: self }
    }
    #[doc = "Bit 13 - Filter FIFO assignment for filter 13"]
    #[inline(always)]
    pub fn ffa13(&mut self) -> FFA13_W {
        FFA13_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "filter FIFO assignment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffa1r](index.html) module"]
pub struct FFA1R_SPEC;
impl crate::RegisterSpec for FFA1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffa1r::R](R) reader structure"]
impl crate::Readable for FFA1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffa1r::W](W) writer structure"]
impl crate::Writable for FFA1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FFA1R to value 0"]
impl crate::Resettable for FFA1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
