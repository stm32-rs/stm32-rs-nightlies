#[doc = "Register `EXTI_EXTICR4` reader"]
pub struct R(crate::R<EXTI_EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_EXTICR4` writer"]
pub struct W(crate::W<EXTI_EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_EXTICR4_SPEC>;
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
impl From<crate::W<EXTI_EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12` reader - EXTI12"]
pub struct EXTI12_R(crate::FieldReader<u8, u8>);
impl EXTI12_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI12` writer - EXTI12"]
pub struct EXTI12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `EXTI13` reader - EXTI13"]
pub struct EXTI13_R(crate::FieldReader<u8, u8>);
impl EXTI13_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI13` writer - EXTI13"]
pub struct EXTI13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EXTI14` reader - EXTI14"]
pub struct EXTI14_R(crate::FieldReader<u8, u8>);
impl EXTI14_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI14` writer - EXTI14"]
pub struct EXTI14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EXTI15` reader - EXTI15"]
pub struct EXTI15_R(crate::FieldReader<u8, u8>);
impl EXTI15_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI15` writer - EXTI15"]
pub struct EXTI15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI12"]
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W {
        EXTI12_W { w: self }
    }
    #[doc = "Bits 8:15 - EXTI13"]
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W {
        EXTI13_W { w: self }
    }
    #[doc = "Bits 16:23 - EXTI14"]
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W {
        EXTI14_W { w: self }
    }
    #[doc = "Bits 24:31 - EXTI15"]
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W {
        EXTI15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr4](index.html) module"]
pub struct EXTI_EXTICR4_SPEC;
impl crate::RegisterSpec for EXTI_EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_exticr4::R](R) reader structure"]
impl crate::Readable for EXTI_EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_exticr4::W](W) writer structure"]
impl crate::Writable for EXTI_EXTICR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_EXTICR4 to value 0"]
impl crate::Resettable for EXTI_EXTICR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
