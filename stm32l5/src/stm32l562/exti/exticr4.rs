#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0_7` reader - EXTIm GPIO port selection"]
pub struct EXTI0_7_R(crate::FieldReader<u8, u8>);
impl EXTI0_7_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI0_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI0_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI0_7` writer - EXTIm GPIO port selection"]
pub struct EXTI0_7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI0_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `EXTI8_15` reader - EXTIm+1 GPIO port selection"]
pub struct EXTI8_15_R(crate::FieldReader<u8, u8>);
impl EXTI8_15_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI8_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI8_15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI8_15` writer - EXTIm+1 GPIO port selection"]
pub struct EXTI8_15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EXTI16_23` reader - EXTIm+2 GPIO port selection"]
pub struct EXTI16_23_R(crate::FieldReader<u8, u8>);
impl EXTI16_23_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI16_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI16_23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI16_23` writer - EXTIm+2 GPIO port selection"]
pub struct EXTI16_23_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI16_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EXTI24_31` reader - EXTIm+3 GPIO port selection"]
pub struct EXTI24_31_R(crate::FieldReader<u8, u8>);
impl EXTI24_31_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI24_31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI24_31_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI24_31` writer - EXTIm+3 GPIO port selection"]
pub struct EXTI24_31_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI24_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTIm GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&mut self) -> EXTI0_7_W {
        EXTI0_7_W { w: self }
    }
    #[doc = "Bits 8:15 - EXTIm+1 GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&mut self) -> EXTI8_15_W {
        EXTI8_15_W { w: self }
    }
    #[doc = "Bits 16:23 - EXTIm+2 GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&mut self) -> EXTI16_23_W {
        EXTI16_23_W { w: self }
    }
    #[doc = "Bits 24:31 - EXTIm+3 GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&mut self) -> EXTI24_31_W {
        EXTI24_31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI external interrupt selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
