#[doc = "Register `TIM1_TISEL` reader"]
pub struct R(crate::R<TIM1_TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_TISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM1_TISEL` writer"]
pub struct W(crate::W<TIM1_TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_TISEL_SPEC>;
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
impl From<crate::W<TIM1_TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_TISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub struct TI1SEL_R(crate::FieldReader<u8, u8>);
impl TI1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub struct TI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TI2SEL` reader - TI2SEL"]
pub struct TI2SEL_R(crate::FieldReader<u8, u8>);
impl TI2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI2SEL` writer - TI2SEL"]
pub struct TI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TI3SEL` reader - TI3SEL"]
pub struct TI3SEL_R(crate::FieldReader<u8, u8>);
impl TI3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI3SEL` writer - TI3SEL"]
pub struct TI3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TI4SEL` reader - TI4SEL"]
pub struct TI4SEL_R(crate::FieldReader<u8, u8>);
impl TI4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI4SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI4SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI4SEL` writer - TI4SEL"]
pub struct TI4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TI3SEL"]
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - TI4SEL"]
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W {
        TI2SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - TI3SEL"]
    #[inline(always)]
    pub fn ti3sel(&mut self) -> TI3SEL_W {
        TI3SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - TI4SEL"]
    #[inline(always)]
    pub fn ti4sel(&mut self) -> TI4SEL_W {
        TI4SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM1 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim1_tisel](index.html) module"]
pub struct TIM1_TISEL_SPEC;
impl crate::RegisterSpec for TIM1_TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim1_tisel::R](R) reader structure"]
impl crate::Readable for TIM1_TISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim1_tisel::W](W) writer structure"]
impl crate::Writable for TIM1_TISEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM1_TISEL to value 0"]
impl crate::Resettable for TIM1_TISEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
