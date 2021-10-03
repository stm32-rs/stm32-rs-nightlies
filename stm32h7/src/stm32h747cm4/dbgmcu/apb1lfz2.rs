#[doc = "Register `APB1LFZ2` reader"]
pub struct R(crate::R<APB1LFZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LFZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LFZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LFZ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LFZ2` writer"]
pub struct W(crate::W<APB1LFZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LFZ2_SPEC>;
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
impl From<crate::W<APB1LFZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LFZ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_TIM2` reader - TIM2 stop in debug"]
pub struct DBG_TIM2_R(crate::FieldReader<bool, bool>);
impl DBG_TIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM2` writer - TIM2 stop in debug"]
pub struct DBG_TIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM2_W<'a> {
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
#[doc = "Field `DBG_TIM3` reader - TIM3 stop in debug"]
pub struct DBG_TIM3_R(crate::FieldReader<bool, bool>);
impl DBG_TIM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM3` writer - TIM3 stop in debug"]
pub struct DBG_TIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM3_W<'a> {
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
#[doc = "Field `DBG_TIM4` reader - TIM4 stop in debug"]
pub struct DBG_TIM4_R(crate::FieldReader<bool, bool>);
impl DBG_TIM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM4` writer - TIM4 stop in debug"]
pub struct DBG_TIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM4_W<'a> {
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
#[doc = "Field `DBG_TIM5` reader - TIM5 stop in debug"]
pub struct DBG_TIM5_R(crate::FieldReader<bool, bool>);
impl DBG_TIM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM5` writer - TIM5 stop in debug"]
pub struct DBG_TIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM5_W<'a> {
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
#[doc = "Field `DBG_TIM6` reader - TIM6 stop in debug"]
pub struct DBG_TIM6_R(crate::FieldReader<bool, bool>);
impl DBG_TIM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM6` writer - TIM6 stop in debug"]
pub struct DBG_TIM6_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM6_W<'a> {
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
#[doc = "Field `DBG_TIM7` reader - TIM4 stop in debug"]
pub struct DBG_TIM7_R(crate::FieldReader<bool, bool>);
impl DBG_TIM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM7` writer - TIM4 stop in debug"]
pub struct DBG_TIM7_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM7_W<'a> {
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
#[doc = "Field `DBG_TIM12` reader - TIM12 stop in debug"]
pub struct DBG_TIM12_R(crate::FieldReader<bool, bool>);
impl DBG_TIM12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM12` writer - TIM12 stop in debug"]
pub struct DBG_TIM12_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM12_W<'a> {
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
#[doc = "Field `DBG_TIM13` reader - TIM13 stop in debug"]
pub struct DBG_TIM13_R(crate::FieldReader<bool, bool>);
impl DBG_TIM13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM13` writer - TIM13 stop in debug"]
pub struct DBG_TIM13_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM13_W<'a> {
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
#[doc = "Field `DBG_TIM14` reader - TIM14 stop in debug"]
pub struct DBG_TIM14_R(crate::FieldReader<bool, bool>);
impl DBG_TIM14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_TIM14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_TIM14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_TIM14` writer - TIM14 stop in debug"]
pub struct DBG_TIM14_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_TIM14_W<'a> {
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
#[doc = "Field `DBG_LPTIM1` reader - LPTIM1 stop in debug"]
pub struct DBG_LPTIM1_R(crate::FieldReader<bool, bool>);
impl DBG_LPTIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_LPTIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_LPTIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_LPTIM1` writer - LPTIM1 stop in debug"]
pub struct DBG_LPTIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_LPTIM1_W<'a> {
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
#[doc = "Field `DBG_WWDG2` reader - WWDG2 stop in debug"]
pub struct DBG_WWDG2_R(crate::FieldReader<bool, bool>);
impl DBG_WWDG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_WWDG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_WWDG2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_WWDG2` writer - WWDG2 stop in debug"]
pub struct DBG_WWDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_WWDG2_W<'a> {
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
#[doc = "Field `DBG_I2C1` reader - I2C1 SMBUS timeout stop in debug"]
pub struct DBG_I2C1_R(crate::FieldReader<bool, bool>);
impl DBG_I2C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C1` writer - I2C1 SMBUS timeout stop in debug"]
pub struct DBG_I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C1_W<'a> {
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
#[doc = "Field `DBG_I2C2` reader - I2C2 SMBUS timeout stop in debug"]
pub struct DBG_I2C2_R(crate::FieldReader<bool, bool>);
impl DBG_I2C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C2` writer - I2C2 SMBUS timeout stop in debug"]
pub struct DBG_I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C2_W<'a> {
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
#[doc = "Field `DBG_I2C3` reader - I2C3 SMBUS timeout stop in debug"]
pub struct DBG_I2C3_R(crate::FieldReader<bool, bool>);
impl DBG_I2C3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_I2C3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_I2C3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBG_I2C3` writer - I2C3 SMBUS timeout stop in debug"]
pub struct DBG_I2C3_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_I2C3_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim2(&self) -> DBG_TIM2_R {
        DBG_TIM2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3(&self) -> DBG_TIM3_R {
        DBG_TIM3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4(&self) -> DBG_TIM4_R {
        DBG_TIM4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim5(&self) -> DBG_TIM5_R {
        DBG_TIM5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim6(&self) -> DBG_TIM6_R {
        DBG_TIM6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim7(&self) -> DBG_TIM7_R {
        DBG_TIM7_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim12(&self) -> DBG_TIM12_R {
        DBG_TIM12_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim13(&self) -> DBG_TIM13_R {
        DBG_TIM13_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim14(&self) -> DBG_TIM14_R {
        DBG_TIM14_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1(&self) -> DBG_LPTIM1_R {
        DBG_LPTIM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - WWDG2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wwdg2(&self) -> DBG_WWDG2_R {
        DBG_WWDG2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c1(&self) -> DBG_I2C1_R {
        DBG_I2C1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c2(&self) -> DBG_I2C2_R {
        DBG_I2C2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3(&self) -> DBG_I2C3_R {
        DBG_I2C3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim2(&mut self) -> DBG_TIM2_W {
        DBG_TIM2_W { w: self }
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3(&mut self) -> DBG_TIM3_W {
        DBG_TIM3_W { w: self }
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4(&mut self) -> DBG_TIM4_W {
        DBG_TIM4_W { w: self }
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim5(&mut self) -> DBG_TIM5_W {
        DBG_TIM5_W { w: self }
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim6(&mut self) -> DBG_TIM6_W {
        DBG_TIM6_W { w: self }
    }
    #[doc = "Bit 5 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim7(&mut self) -> DBG_TIM7_W {
        DBG_TIM7_W { w: self }
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim12(&mut self) -> DBG_TIM12_W {
        DBG_TIM12_W { w: self }
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim13(&mut self) -> DBG_TIM13_W {
        DBG_TIM13_W { w: self }
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim14(&mut self) -> DBG_TIM14_W {
        DBG_TIM14_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1(&mut self) -> DBG_LPTIM1_W {
        DBG_LPTIM1_W { w: self }
    }
    #[doc = "Bit 11 - WWDG2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wwdg2(&mut self) -> DBG_WWDG2_W {
        DBG_WWDG2_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c1(&mut self) -> DBG_I2C1_W {
        DBG_I2C1_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c2(&mut self) -> DBG_I2C2_W {
        DBG_I2C2_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3(&mut self) -> DBG_I2C3_W {
        DBG_I2C3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB1L peripheral freeze register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lfz2](index.html) module"]
pub struct APB1LFZ2_SPEC;
impl crate::RegisterSpec for APB1LFZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lfz2::R](R) reader structure"]
impl crate::Readable for APB1LFZ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lfz2::W](W) writer structure"]
impl crate::Writable for APB1LFZ2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1LFZ2 to value 0"]
impl crate::Resettable for APB1LFZ2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
