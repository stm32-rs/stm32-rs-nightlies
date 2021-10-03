#[doc = "Register `PR1` reader"]
pub struct R(crate::R<PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR1` writer"]
pub struct W(crate::W<PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR1_SPEC>;
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
impl From<crate::W<PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIF31` reader - Configurable event inputs Pending bit"]
pub struct PIF31_R(crate::FieldReader<bool, bool>);
impl PIF31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF31` writer - Configurable event inputs Pending bit"]
pub struct PIF31_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF31_W<'a> {
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
#[doc = "Field `PIF0` reader - Configurable event inputs Pending bit"]
pub struct PIF0_R(crate::FieldReader<bool, bool>);
impl PIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF0` writer - Configurable event inputs Pending bit"]
pub struct PIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF0_W<'a> {
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
#[doc = "Field `PIF1` reader - Configurable event inputs Pending bit"]
pub struct PIF1_R(crate::FieldReader<bool, bool>);
impl PIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF1` writer - Configurable event inputs Pending bit"]
pub struct PIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF1_W<'a> {
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
#[doc = "Field `PIF2` reader - Configurable event inputs Pending bit"]
pub struct PIF2_R(crate::FieldReader<bool, bool>);
impl PIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF2` writer - Configurable event inputs Pending bit"]
pub struct PIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF2_W<'a> {
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
#[doc = "Field `PIF3` reader - Configurable event inputs Pending bit"]
pub struct PIF3_R(crate::FieldReader<bool, bool>);
impl PIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF3` writer - Configurable event inputs Pending bit"]
pub struct PIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF3_W<'a> {
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
#[doc = "Field `PIF4` reader - Configurable event inputs Pending bit"]
pub struct PIF4_R(crate::FieldReader<bool, bool>);
impl PIF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF4` writer - Configurable event inputs Pending bit"]
pub struct PIF4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF4_W<'a> {
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
#[doc = "Field `PIF5` reader - Configurable event inputs Pending bit"]
pub struct PIF5_R(crate::FieldReader<bool, bool>);
impl PIF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF5` writer - Configurable event inputs Pending bit"]
pub struct PIF5_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF5_W<'a> {
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
#[doc = "Field `PIF6` reader - Configurable event inputs Pending bit"]
pub struct PIF6_R(crate::FieldReader<bool, bool>);
impl PIF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF6` writer - Configurable event inputs Pending bit"]
pub struct PIF6_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF6_W<'a> {
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
#[doc = "Field `PIF7` reader - Configurable event inputs Pending bit"]
pub struct PIF7_R(crate::FieldReader<bool, bool>);
impl PIF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF7` writer - Configurable event inputs Pending bit"]
pub struct PIF7_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF7_W<'a> {
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
#[doc = "Field `PIF8` reader - Configurable event inputs Pending bit"]
pub struct PIF8_R(crate::FieldReader<bool, bool>);
impl PIF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF8` writer - Configurable event inputs Pending bit"]
pub struct PIF8_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF8_W<'a> {
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
#[doc = "Field `PIF9` reader - Configurable event inputs Pending bit"]
pub struct PIF9_R(crate::FieldReader<bool, bool>);
impl PIF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF9` writer - Configurable event inputs Pending bit"]
pub struct PIF9_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF9_W<'a> {
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
#[doc = "Field `PIF10` reader - Configurable event inputs Pending bit"]
pub struct PIF10_R(crate::FieldReader<bool, bool>);
impl PIF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF10` writer - Configurable event inputs Pending bit"]
pub struct PIF10_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF10_W<'a> {
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
#[doc = "Field `PIF11` reader - Configurable event inputs Pending bit"]
pub struct PIF11_R(crate::FieldReader<bool, bool>);
impl PIF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF11` writer - Configurable event inputs Pending bit"]
pub struct PIF11_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF11_W<'a> {
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
#[doc = "Field `PIF12` reader - Configurable event inputs Pending bit"]
pub struct PIF12_R(crate::FieldReader<bool, bool>);
impl PIF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF12` writer - Configurable event inputs Pending bit"]
pub struct PIF12_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF12_W<'a> {
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
#[doc = "Field `PIF13` reader - Configurable event inputs Pending bit"]
pub struct PIF13_R(crate::FieldReader<bool, bool>);
impl PIF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF13` writer - Configurable event inputs Pending bit"]
pub struct PIF13_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF13_W<'a> {
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
#[doc = "Field `PIF14` reader - Configurable event inputs Pending bit"]
pub struct PIF14_R(crate::FieldReader<bool, bool>);
impl PIF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF14` writer - Configurable event inputs Pending bit"]
pub struct PIF14_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF14_W<'a> {
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
#[doc = "Field `PIF15` reader - Configurable event inputs Pending bit"]
pub struct PIF15_R(crate::FieldReader<bool, bool>);
impl PIF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF15` writer - Configurable event inputs Pending bit"]
pub struct PIF15_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF15_W<'a> {
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
#[doc = "Field `PIF16` reader - Configurable event inputs Pending bit"]
pub struct PIF16_R(crate::FieldReader<bool, bool>);
impl PIF16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF16` writer - Configurable event inputs Pending bit"]
pub struct PIF16_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF16_W<'a> {
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
#[doc = "Field `PIF17` reader - Configurable event inputs Pending bit"]
pub struct PIF17_R(crate::FieldReader<bool, bool>);
impl PIF17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF17` writer - Configurable event inputs Pending bit"]
pub struct PIF17_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF17_W<'a> {
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
#[doc = "Field `PIF18` reader - Configurable event inputs Pending bit"]
pub struct PIF18_R(crate::FieldReader<bool, bool>);
impl PIF18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF18` writer - Configurable event inputs Pending bit"]
pub struct PIF18_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF18_W<'a> {
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
#[doc = "Field `PIF19` reader - Configurable event inputs Pending bit"]
pub struct PIF19_R(crate::FieldReader<bool, bool>);
impl PIF19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF19` writer - Configurable event inputs Pending bit"]
pub struct PIF19_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF19_W<'a> {
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
#[doc = "Field `PIF20` reader - Configurable event inputs Pending bit"]
pub struct PIF20_R(crate::FieldReader<bool, bool>);
impl PIF20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF20` writer - Configurable event inputs Pending bit"]
pub struct PIF20_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF20_W<'a> {
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
#[doc = "Field `PIF21` reader - Configurable event inputs Pending bit"]
pub struct PIF21_R(crate::FieldReader<bool, bool>);
impl PIF21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIF21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIF21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIF21` writer - Configurable event inputs Pending bit"]
pub struct PIF21_W<'a> {
    w: &'a mut W,
}
impl<'a> PIF21_W<'a> {
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
impl R {
    #[doc = "Bit 31 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif31(&self) -> PIF31_R {
        PIF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif17(&self) -> PIF17_R {
        PIF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif18(&self) -> PIF18_R {
        PIF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif19(&self) -> PIF19_R {
        PIF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif20(&self) -> PIF20_R {
        PIF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif31(&mut self) -> PIF31_W {
        PIF31_W { w: self }
    }
    #[doc = "Bit 0 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif0(&mut self) -> PIF0_W {
        PIF0_W { w: self }
    }
    #[doc = "Bit 1 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif1(&mut self) -> PIF1_W {
        PIF1_W { w: self }
    }
    #[doc = "Bit 2 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif2(&mut self) -> PIF2_W {
        PIF2_W { w: self }
    }
    #[doc = "Bit 3 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif3(&mut self) -> PIF3_W {
        PIF3_W { w: self }
    }
    #[doc = "Bit 4 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif4(&mut self) -> PIF4_W {
        PIF4_W { w: self }
    }
    #[doc = "Bit 5 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif5(&mut self) -> PIF5_W {
        PIF5_W { w: self }
    }
    #[doc = "Bit 6 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif6(&mut self) -> PIF6_W {
        PIF6_W { w: self }
    }
    #[doc = "Bit 7 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif7(&mut self) -> PIF7_W {
        PIF7_W { w: self }
    }
    #[doc = "Bit 8 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif8(&mut self) -> PIF8_W {
        PIF8_W { w: self }
    }
    #[doc = "Bit 9 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif9(&mut self) -> PIF9_W {
        PIF9_W { w: self }
    }
    #[doc = "Bit 10 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif10(&mut self) -> PIF10_W {
        PIF10_W { w: self }
    }
    #[doc = "Bit 11 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif11(&mut self) -> PIF11_W {
        PIF11_W { w: self }
    }
    #[doc = "Bit 12 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif12(&mut self) -> PIF12_W {
        PIF12_W { w: self }
    }
    #[doc = "Bit 13 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif13(&mut self) -> PIF13_W {
        PIF13_W { w: self }
    }
    #[doc = "Bit 14 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif14(&mut self) -> PIF14_W {
        PIF14_W { w: self }
    }
    #[doc = "Bit 15 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif15(&mut self) -> PIF15_W {
        PIF15_W { w: self }
    }
    #[doc = "Bit 16 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif16(&mut self) -> PIF16_W {
        PIF16_W { w: self }
    }
    #[doc = "Bit 17 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif17(&mut self) -> PIF17_W {
        PIF17_W { w: self }
    }
    #[doc = "Bit 18 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif18(&mut self) -> PIF18_W {
        PIF18_W { w: self }
    }
    #[doc = "Bit 19 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif19(&mut self) -> PIF19_W {
        PIF19_W { w: self }
    }
    #[doc = "Bit 20 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif20(&mut self) -> PIF20_W {
        PIF20_W { w: self }
    }
    #[doc = "Bit 21 - Configurable event inputs Pending bit"]
    #[inline(always)]
    pub fn pif21(&mut self) -> PIF21_W {
        PIF21_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1](index.html) module"]
pub struct PR1_SPEC;
impl crate::RegisterSpec for PR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr1::R](R) reader structure"]
impl crate::Readable for PR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr1::W](W) writer structure"]
impl crate::Writable for PR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR1 to value 0"]
impl crate::Resettable for PR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
