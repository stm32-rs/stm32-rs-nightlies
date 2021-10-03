#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC4` reader - IC4"]
pub struct IC4_R(crate::FieldReader<bool, bool>);
impl IC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4` writer - IC4"]
pub struct IC4_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4_W<'a> {
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
#[doc = "Field `IC3` reader - IC3"]
pub struct IC3_R(crate::FieldReader<bool, bool>);
impl IC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3` writer - IC3"]
pub struct IC3_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3_W<'a> {
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
#[doc = "Field `IC2` reader - IC2"]
pub struct IC2_R(crate::FieldReader<bool, bool>);
impl IC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2` writer - IC2"]
pub struct IC2_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2_W<'a> {
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
#[doc = "Field `IC1` reader - IC1"]
pub struct IC1_R(crate::FieldReader<bool, bool>);
impl IC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1` writer - IC1"]
pub struct IC1_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1_W<'a> {
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
#[doc = "Field `TIM` reader - Timer select bits"]
pub struct TIM_R(crate::FieldReader<u8, u8>);
impl TIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM` writer - Timer select bits"]
pub struct TIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `IC4IOS` reader - Input capture 4 select bits"]
pub struct IC4IOS_R(crate::FieldReader<u8, u8>);
impl IC4IOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC4IOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4IOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4IOS` writer - Input capture 4 select bits"]
pub struct IC4IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `IC3IOS` reader - Input capture 3 select bits"]
pub struct IC3IOS_R(crate::FieldReader<u8, u8>);
impl IC3IOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3IOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3IOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3IOS` writer - Input capture 3 select bits"]
pub struct IC3IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `IC2IOS` reader - Input capture 2 select bits"]
pub struct IC2IOS_R(crate::FieldReader<u8, u8>);
impl IC2IOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC2IOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2IOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2IOS` writer - Input capture 2 select bits"]
pub struct IC2IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `IC1IOS` reader - Input capture 1 select bits"]
pub struct IC1IOS_R(crate::FieldReader<u8, u8>);
impl IC1IOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1IOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1IOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1IOS` writer - Input capture 1 select bits"]
pub struct IC1IOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    pub fn ic4(&self) -> IC4_R {
        IC4_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    pub fn ic3(&self) -> IC3_R {
        IC3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    pub fn ic2(&self) -> IC2_R {
        IC2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    pub fn ic1(&self) -> IC1_R {
        IC1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    pub fn ic4ios(&self) -> IC4IOS_R {
        IC4IOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    pub fn ic3ios(&self) -> IC3IOS_R {
        IC3IOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    pub fn ic2ios(&self) -> IC2IOS_R {
        IC2IOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    pub fn ic1ios(&self) -> IC1IOS_R {
        IC1IOS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    pub fn ic4(&mut self) -> IC4_W {
        IC4_W { w: self }
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    pub fn ic3(&mut self) -> IC3_W {
        IC3_W { w: self }
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    pub fn ic2(&mut self) -> IC2_W {
        IC2_W { w: self }
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    pub fn ic1(&mut self) -> IC1_W {
        IC1_W { w: self }
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    pub fn tim(&mut self) -> TIM_W {
        TIM_W { w: self }
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    pub fn ic4ios(&mut self) -> IC4IOS_W {
        IC4IOS_W { w: self }
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    pub fn ic3ios(&mut self) -> IC3IOS_W {
        IC3IOS_W { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    pub fn ic2ios(&mut self) -> IC2IOS_W {
        IC2IOS_W { w: self }
    }
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    pub fn ic1ios(&mut self) -> IC1IOS_W {
        IC1IOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RI input capture register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
