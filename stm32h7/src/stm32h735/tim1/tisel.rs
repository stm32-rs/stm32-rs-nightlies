#[doc = "Register `TISEL` reader"]
pub struct R(crate::R<TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TISEL` writer"]
pub struct W(crate::W<TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TISEL_SPEC>;
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
impl From<crate::W<TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1SEL` reader - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
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
#[doc = "Field `TI1SEL` writer - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
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
#[doc = "Field `TI2SEL` reader - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
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
#[doc = "Field `TI2SEL` writer - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
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
#[doc = "Field `TI3SEL` reader - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
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
#[doc = "Field `TI3SEL` writer - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
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
#[doc = "Field `TI4SEL` reader - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
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
#[doc = "Field `TI4SEL` writer - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
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
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W {
        TI2SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
    #[inline(always)]
    pub fn ti3sel(&mut self) -> TI3SEL_W {
        TI3SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
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
#[doc = "TIM1 timer input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tisel](index.html) module"]
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tisel::R](R) reader structure"]
impl crate::Readable for TISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tisel::W](W) writer structure"]
impl crate::Writable for TISEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
