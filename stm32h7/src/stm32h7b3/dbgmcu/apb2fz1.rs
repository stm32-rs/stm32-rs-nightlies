#[doc = "Register `APB2FZ1` reader"]
pub struct R(crate::R<APB2FZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2FZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2FZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2FZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2FZ1` writer"]
pub struct W(crate::W<APB2FZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2FZ1_SPEC>;
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
impl From<crate::W<APB2FZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2FZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1` reader - TIM1 stop in debug"]
pub struct TIM1_R(crate::FieldReader<bool, bool>);
impl TIM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1` writer - TIM1 stop in debug"]
pub struct TIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_W<'a> {
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
#[doc = "Field `TIM8` reader - TIM8 stop in debug"]
pub struct TIM8_R(crate::FieldReader<bool, bool>);
impl TIM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8` writer - TIM8 stop in debug"]
pub struct TIM8_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8_W<'a> {
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
#[doc = "Field `TIM15` reader - TIM15 stop in debug"]
pub struct TIM15_R(crate::FieldReader<bool, bool>);
impl TIM15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM15` writer - TIM15 stop in debug"]
pub struct TIM15_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM15_W<'a> {
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
#[doc = "Field `TIM16` reader - TIM16 stop in debug"]
pub struct TIM16_R(crate::FieldReader<bool, bool>);
impl TIM16_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM16` writer - TIM16 stop in debug"]
pub struct TIM16_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM16_W<'a> {
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
#[doc = "Field `TIM17` reader - TIM17 stop in debug"]
pub struct TIM17_R(crate::FieldReader<bool, bool>);
impl TIM17_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM17` writer - TIM17 stop in debug"]
pub struct TIM17_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM17_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn tim1(&self) -> TIM1_R {
        TIM1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn tim8(&self) -> TIM8_R {
        TIM8_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn tim1(&mut self) -> TIM1_W {
        TIM1_W { w: self }
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn tim8(&mut self) -> TIM8_W {
        TIM8_W { w: self }
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn tim15(&mut self) -> TIM15_W {
        TIM15_W { w: self }
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn tim16(&mut self) -> TIM16_W {
        TIM16_W { w: self }
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn tim17(&mut self) -> TIM17_W {
        TIM17_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fz1](index.html) module"]
pub struct APB2FZ1_SPEC;
impl crate::RegisterSpec for APB2FZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2fz1::R](R) reader structure"]
impl crate::Readable for APB2FZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2fz1::W](W) writer structure"]
impl crate::Writable for APB2FZ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2FZ1 to value 0"]
impl crate::Resettable for APB2FZ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
