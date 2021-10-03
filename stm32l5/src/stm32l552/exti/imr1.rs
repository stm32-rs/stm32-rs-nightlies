#[doc = "Register `IMR1` reader"]
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR1` writer"]
pub struct W(crate::W<IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR1_SPEC>;
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
impl From<crate::W<IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM0` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM0` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM1` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM1` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM2` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM2` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM3` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM3` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM4` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM4` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM5` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM5` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM6` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM6` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM7` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM7` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM8` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM8` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM9` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM9` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM10` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM10` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM11` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM11` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM12` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM12` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM13` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM13` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM14` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM14` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM15` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM15` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM16` reader - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM16` writer - CPU wakeup with interrupt mask on event input"]
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
#[doc = "Field `IM17` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM17_R(crate::FieldReader<bool, bool>);
impl IM17_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM17` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM17_W<'a> {
    w: &'a mut W,
}
impl<'a> IM17_W<'a> {
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
#[doc = "Field `IM18` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM18_R(crate::FieldReader<bool, bool>);
impl IM18_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM18` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM18_W<'a> {
    w: &'a mut W,
}
impl<'a> IM18_W<'a> {
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
#[doc = "Field `IM19` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM19_R(crate::FieldReader<bool, bool>);
impl IM19_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM19` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM19_W<'a> {
    w: &'a mut W,
}
impl<'a> IM19_W<'a> {
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
#[doc = "Field `IM20` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM20_R(crate::FieldReader<bool, bool>);
impl IM20_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM20` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM20_W<'a> {
    w: &'a mut W,
}
impl<'a> IM20_W<'a> {
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
#[doc = "Field `IM21` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM21_R(crate::FieldReader<bool, bool>);
impl IM21_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM21` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM21_W<'a> {
    w: &'a mut W,
}
impl<'a> IM21_W<'a> {
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
#[doc = "Field `IM22` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM22_R(crate::FieldReader<bool, bool>);
impl IM22_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM22` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM22_W<'a> {
    w: &'a mut W,
}
impl<'a> IM22_W<'a> {
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
#[doc = "Field `IM23` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM23_R(crate::FieldReader<bool, bool>);
impl IM23_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM23` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM23_W<'a> {
    w: &'a mut W,
}
impl<'a> IM23_W<'a> {
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
#[doc = "Field `IM24` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM24_R(crate::FieldReader<bool, bool>);
impl IM24_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM24` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM24_W<'a> {
    w: &'a mut W,
}
impl<'a> IM24_W<'a> {
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
#[doc = "Field `IM25` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM25_R(crate::FieldReader<bool, bool>);
impl IM25_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM25` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM25_W<'a> {
    w: &'a mut W,
}
impl<'a> IM25_W<'a> {
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
#[doc = "Field `IM26` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM26_R(crate::FieldReader<bool, bool>);
impl IM26_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM26` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM26_W<'a> {
    w: &'a mut W,
}
impl<'a> IM26_W<'a> {
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
#[doc = "Field `IM27` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM27_R(crate::FieldReader<bool, bool>);
impl IM27_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM27` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM27_W<'a> {
    w: &'a mut W,
}
impl<'a> IM27_W<'a> {
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
#[doc = "Field `IM28` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM28_R(crate::FieldReader<bool, bool>);
impl IM28_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM28` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM28_W<'a> {
    w: &'a mut W,
}
impl<'a> IM28_W<'a> {
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
#[doc = "Field `IM29` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM29_R(crate::FieldReader<bool, bool>);
impl IM29_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM29` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM29_W<'a> {
    w: &'a mut W,
}
impl<'a> IM29_W<'a> {
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
#[doc = "Field `IM30` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM30_R(crate::FieldReader<bool, bool>);
impl IM30_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM30` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM30_W<'a> {
    w: &'a mut W,
}
impl<'a> IM30_W<'a> {
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
#[doc = "Field `IM31` reader - CPU wakeup with interrupt mask on event input"]
pub struct IM31_R(crate::FieldReader<bool, bool>);
impl IM31_R {
    pub(crate) fn new(bits: bool) -> Self {
        IM31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IM31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IM31` writer - CPU wakeup with interrupt mask on event input"]
pub struct IM31_W<'a> {
    w: &'a mut W,
}
impl<'a> IM31_W<'a> {
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
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im24(&self) -> IM24_R {
        IM24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im26(&self) -> IM26_R {
        IM26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im27(&self) -> IM27_R {
        IM27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im28(&self) -> IM28_R {
        IM28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&self) -> IM29_R {
        IM29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im30(&self) -> IM30_R {
        IM30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im31(&self) -> IM31_R {
        IM31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im0(&mut self) -> IM0_W {
        IM0_W { w: self }
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im1(&mut self) -> IM1_W {
        IM1_W { w: self }
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im2(&mut self) -> IM2_W {
        IM2_W { w: self }
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im3(&mut self) -> IM3_W {
        IM3_W { w: self }
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im4(&mut self) -> IM4_W {
        IM4_W { w: self }
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im5(&mut self) -> IM5_W {
        IM5_W { w: self }
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im6(&mut self) -> IM6_W {
        IM6_W { w: self }
    }
    #[doc = "Bit 7 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im7(&mut self) -> IM7_W {
        IM7_W { w: self }
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im8(&mut self) -> IM8_W {
        IM8_W { w: self }
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im9(&mut self) -> IM9_W {
        IM9_W { w: self }
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im10(&mut self) -> IM10_W {
        IM10_W { w: self }
    }
    #[doc = "Bit 11 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im11(&mut self) -> IM11_W {
        IM11_W { w: self }
    }
    #[doc = "Bit 12 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im12(&mut self) -> IM12_W {
        IM12_W { w: self }
    }
    #[doc = "Bit 13 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im13(&mut self) -> IM13_W {
        IM13_W { w: self }
    }
    #[doc = "Bit 14 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im14(&mut self) -> IM14_W {
        IM14_W { w: self }
    }
    #[doc = "Bit 15 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im15(&mut self) -> IM15_W {
        IM15_W { w: self }
    }
    #[doc = "Bit 16 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im16(&mut self) -> IM16_W {
        IM16_W { w: self }
    }
    #[doc = "Bit 17 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im17(&mut self) -> IM17_W {
        IM17_W { w: self }
    }
    #[doc = "Bit 18 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im18(&mut self) -> IM18_W {
        IM18_W { w: self }
    }
    #[doc = "Bit 19 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im19(&mut self) -> IM19_W {
        IM19_W { w: self }
    }
    #[doc = "Bit 20 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im20(&mut self) -> IM20_W {
        IM20_W { w: self }
    }
    #[doc = "Bit 21 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im21(&mut self) -> IM21_W {
        IM21_W { w: self }
    }
    #[doc = "Bit 22 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im22(&mut self) -> IM22_W {
        IM22_W { w: self }
    }
    #[doc = "Bit 23 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im23(&mut self) -> IM23_W {
        IM23_W { w: self }
    }
    #[doc = "Bit 24 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im24(&mut self) -> IM24_W {
        IM24_W { w: self }
    }
    #[doc = "Bit 25 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im25(&mut self) -> IM25_W {
        IM25_W { w: self }
    }
    #[doc = "Bit 26 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im26(&mut self) -> IM26_W {
        IM26_W { w: self }
    }
    #[doc = "Bit 27 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im27(&mut self) -> IM27_W {
        IM27_W { w: self }
    }
    #[doc = "Bit 28 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im28(&mut self) -> IM28_W {
        IM28_W { w: self }
    }
    #[doc = "Bit 29 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im29(&mut self) -> IM29_W {
        IM29_W { w: self }
    }
    #[doc = "Bit 30 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im30(&mut self) -> IM30_W {
        IM30_W { w: self }
    }
    #[doc = "Bit 31 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im31(&mut self) -> IM31_W {
        IM31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr1](index.html) module"]
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr1::R](R) reader structure"]
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr1::W](W) writer structure"]
impl crate::Writable for IMR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR1 to value 0xff9e_0000"]
impl crate::Resettable for IMR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff9e_0000
    }
}
