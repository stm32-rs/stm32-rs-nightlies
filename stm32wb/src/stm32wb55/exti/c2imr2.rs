#[doc = "Register `C2IMR2` reader"]
pub struct R(crate::R<C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR2` writer"]
pub struct W(crate::W<C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR2_SPEC>;
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
impl From<crate::W<C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM0` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM0_R(crate::FieldReader<bool, bool>);
impl IM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM0` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM0_W<'a> {
    w: &'a mut W,
}
impl<'a> IM0_W<'a> {
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
#[doc = "Field `IM1` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM1_R(crate::FieldReader<bool, bool>);
impl IM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM1` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM1_W<'a> {
    w: &'a mut W,
}
impl<'a> IM1_W<'a> {
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
#[doc = "Field `IM2` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM2_R(crate::FieldReader<bool, bool>);
impl IM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM2` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM2_W<'a> {
    w: &'a mut W,
}
impl<'a> IM2_W<'a> {
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
#[doc = "Field `IM3` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM3_R(crate::FieldReader<bool, bool>);
impl IM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM3` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM3_W<'a> {
    w: &'a mut W,
}
impl<'a> IM3_W<'a> {
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
#[doc = "Field `IM4` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM4_R(crate::FieldReader<bool, bool>);
impl IM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM4` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM4_W<'a> {
    w: &'a mut W,
}
impl<'a> IM4_W<'a> {
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
#[doc = "Field `IM5` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM5_R(crate::FieldReader<bool, bool>);
impl IM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM5` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM5_W<'a> {
    w: &'a mut W,
}
impl<'a> IM5_W<'a> {
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
#[doc = "Field `IM6` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM6_R(crate::FieldReader<bool, bool>);
impl IM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM6` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM6_W<'a> {
    w: &'a mut W,
}
impl<'a> IM6_W<'a> {
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
#[doc = "Field `IM7` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM7_R(crate::FieldReader<bool, bool>);
impl IM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM7` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM7_W<'a> {
    w: &'a mut W,
}
impl<'a> IM7_W<'a> {
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
#[doc = "Field `IM8` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM8_R(crate::FieldReader<bool, bool>);
impl IM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM8` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM8_W<'a> {
    w: &'a mut W,
}
impl<'a> IM8_W<'a> {
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
#[doc = "Field `IM9` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM9_R(crate::FieldReader<bool, bool>);
impl IM9_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM9` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM9_W<'a> {
    w: &'a mut W,
}
impl<'a> IM9_W<'a> {
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
#[doc = "Field `IM10` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM10_R(crate::FieldReader<bool, bool>);
impl IM10_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM10` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM10_W<'a> {
    w: &'a mut W,
}
impl<'a> IM10_W<'a> {
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
#[doc = "Field `IM11` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM11_R(crate::FieldReader<bool, bool>);
impl IM11_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM11` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM11_W<'a> {
    w: &'a mut W,
}
impl<'a> IM11_W<'a> {
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
#[doc = "Field `IM12` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM12_R(crate::FieldReader<bool, bool>);
impl IM12_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM12` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM12_W<'a> {
    w: &'a mut W,
}
impl<'a> IM12_W<'a> {
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
#[doc = "Field `IM13` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM13_R(crate::FieldReader<bool, bool>);
impl IM13_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM13` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM13_W<'a> {
    w: &'a mut W,
}
impl<'a> IM13_W<'a> {
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
#[doc = "Field `IM14` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM14_R(crate::FieldReader<bool, bool>);
impl IM14_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM14` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM14_W<'a> {
    w: &'a mut W,
}
impl<'a> IM14_W<'a> {
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
#[doc = "Field `IM15` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM15_R(crate::FieldReader<bool, bool>);
impl IM15_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM15` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM15_W<'a> {
    w: &'a mut W,
}
impl<'a> IM15_W<'a> {
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
#[doc = "Field `IM16` reader - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM16_R(crate::FieldReader<bool, bool>);
impl IM16_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM16` writer - CPUm Wakeup with interrupt Mask on Event input"]
pub struct IM16_W<'a> {
    w: &'a mut W,
}
impl<'a> IM16_W<'a> {
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
    #[doc = "Bit 0 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W {
        IM0_W { w: self }
    }
    #[doc = "Bit 1 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W {
        IM1_W { w: self }
    }
    #[doc = "Bit 2 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W {
        IM2_W { w: self }
    }
    #[doc = "Bit 3 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W {
        IM3_W { w: self }
    }
    #[doc = "Bit 4 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W {
        IM4_W { w: self }
    }
    #[doc = "Bit 5 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W {
        IM5_W { w: self }
    }
    #[doc = "Bit 6 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W {
        IM6_W { w: self }
    }
    #[doc = "Bit 7 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W {
        IM7_W { w: self }
    }
    #[doc = "Bit 8 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W {
        IM8_W { w: self }
    }
    #[doc = "Bit 9 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W {
        IM9_W { w: self }
    }
    #[doc = "Bit 10 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W {
        IM10_W { w: self }
    }
    #[doc = "Bit 11 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W {
        IM11_W { w: self }
    }
    #[doc = "Bit 12 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W {
        IM12_W { w: self }
    }
    #[doc = "Bit 13 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W {
        IM13_W { w: self }
    }
    #[doc = "Bit 14 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W {
        IM14_W { w: self }
    }
    #[doc = "Bit 15 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W {
        IM15_W { w: self }
    }
    #[doc = "Bit 16 - CPUm Wakeup with interrupt Mask on Event input"]
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W {
        IM16_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUm wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](index.html) module"]
pub struct C2IMR2_SPEC;
impl crate::RegisterSpec for C2IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr2::R](R) reader structure"]
impl crate::Readable for C2IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](W) writer structure"]
impl crate::Writable for C2IMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2IMR2 to value 0x0001_fcfd"]
impl crate::Resettable for C2IMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_fcfd
    }
}
