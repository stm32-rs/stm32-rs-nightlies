#[doc = "Register `APB1LFZ1` reader"]
pub struct R(crate::R<APB1LFZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LFZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LFZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LFZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LFZ1` writer"]
pub struct W(crate::W<APB1LFZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LFZ1_SPEC>;
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
impl From<crate::W<APB1LFZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LFZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2` reader - TIM2 stop in debug"]
pub struct TIM2_R(crate::FieldReader<bool, bool>);
impl TIM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2` writer - TIM2 stop in debug"]
pub struct TIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2_W<'a> {
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
#[doc = "Field `TIM3` reader - TIM3 stop in debug"]
pub struct TIM3_R(crate::FieldReader<bool, bool>);
impl TIM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3` writer - TIM3 stop in debug"]
pub struct TIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3_W<'a> {
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
#[doc = "Field `TIM4` reader - TIM4 stop in debug"]
pub struct TIM4_R(crate::FieldReader<bool, bool>);
impl TIM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM4` writer - TIM4 stop in debug"]
pub struct TIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM4_W<'a> {
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
#[doc = "Field `TIM5` reader - TIM5 stop in debug"]
pub struct TIM5_R(crate::FieldReader<bool, bool>);
impl TIM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM5` writer - TIM5 stop in debug"]
pub struct TIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM5_W<'a> {
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
#[doc = "Field `TIM6` reader - TIM6 stop in debug"]
pub struct TIM6_R(crate::FieldReader<bool, bool>);
impl TIM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM6` writer - TIM6 stop in debug"]
pub struct TIM6_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM6_W<'a> {
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
#[doc = "Field `TIM7` reader - TIM7 stop in debug"]
pub struct TIM7_R(crate::FieldReader<bool, bool>);
impl TIM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM7` writer - TIM7 stop in debug"]
pub struct TIM7_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM7_W<'a> {
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
#[doc = "Field `TIM12` reader - TIM12 stop in debug"]
pub struct TIM12_R(crate::FieldReader<bool, bool>);
impl TIM12_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM12` writer - TIM12 stop in debug"]
pub struct TIM12_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM12_W<'a> {
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
#[doc = "Field `TIM13` reader - TIM13 stop in debug"]
pub struct TIM13_R(crate::FieldReader<bool, bool>);
impl TIM13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM13` writer - TIM13 stop in debug"]
pub struct TIM13_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM13_W<'a> {
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
#[doc = "Field `TIM14` reader - TIM14 stop in debug"]
pub struct TIM14_R(crate::FieldReader<bool, bool>);
impl TIM14_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM14` writer - TIM14 stop in debug"]
pub struct TIM14_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM14_W<'a> {
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
#[doc = "Field `LPTIM1` reader - LPTIM1 stop in debug"]
pub struct LPTIM1_R(crate::FieldReader<bool, bool>);
impl LPTIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPTIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1` writer - LPTIM1 stop in debug"]
pub struct LPTIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1_W<'a> {
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
#[doc = "Field `I2C1` reader - I2C1 SMBUS timeout stop in debug"]
pub struct I2C1_R(crate::FieldReader<bool, bool>);
impl I2C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1` writer - I2C1 SMBUS timeout stop in debug"]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
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
#[doc = "Field `I2C2` reader - I2C2 SMBUS timeout stop in debug"]
pub struct I2C2_R(crate::FieldReader<bool, bool>);
impl I2C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2` writer - I2C2 SMBUS timeout stop in debug"]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
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
#[doc = "Field `I2C3` reader - I2C3 SMBUS timeout stop in debug"]
pub struct I2C3_R(crate::FieldReader<bool, bool>);
impl I2C3_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3` writer - I2C3 SMBUS timeout stop in debug"]
pub struct I2C3_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_W<'a> {
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
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn tim4(&self) -> TIM4_R {
        TIM4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn tim5(&self) -> TIM5_R {
        TIM5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn tim12(&self) -> TIM12_R {
        TIM12_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn tim13(&self) -> TIM13_R {
        TIM13_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn tim2(&mut self) -> TIM2_W {
        TIM2_W { w: self }
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn tim3(&mut self) -> TIM3_W {
        TIM3_W { w: self }
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn tim4(&mut self) -> TIM4_W {
        TIM4_W { w: self }
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn tim5(&mut self) -> TIM5_W {
        TIM5_W { w: self }
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn tim6(&mut self) -> TIM6_W {
        TIM6_W { w: self }
    }
    #[doc = "Bit 5 - TIM7 stop in debug"]
    #[inline(always)]
    pub fn tim7(&mut self) -> TIM7_W {
        TIM7_W { w: self }
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn tim12(&mut self) -> TIM12_W {
        TIM12_W { w: self }
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn tim13(&mut self) -> TIM13_W {
        TIM13_W { w: self }
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn tim14(&mut self) -> TIM14_W {
        TIM14_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn lptim1(&mut self) -> LPTIM1_W {
        LPTIM1_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2C3_W {
        I2C3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB1L peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lfz1](index.html) module"]
pub struct APB1LFZ1_SPEC;
impl crate::RegisterSpec for APB1LFZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lfz1::R](R) reader structure"]
impl crate::Readable for APB1LFZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lfz1::W](W) writer structure"]
impl crate::Writable for APB1LFZ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1LFZ1 to value 0"]
impl crate::Resettable for APB1LFZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
