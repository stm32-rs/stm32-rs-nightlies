#[doc = "Register `EXTI_EXTICR2` reader"]
pub struct R(crate::R<EXTI_EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_EXTICR2` writer"]
pub struct W(crate::W<EXTI_EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_EXTICR2_SPEC>;
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
impl From<crate::W<EXTI_EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI4` reader - EXTI4"]
pub struct EXTI4_R(crate::FieldReader<u8, u8>);
impl EXTI4_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI4` writer - EXTI4"]
pub struct EXTI4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `EXTI5` reader - EXTI5"]
pub struct EXTI5_R(crate::FieldReader<u8, u8>);
impl EXTI5_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI5` writer - EXTI5"]
pub struct EXTI5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EXTI6` reader - EXTI6"]
pub struct EXTI6_R(crate::FieldReader<u8, u8>);
impl EXTI6_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI6` writer - EXTI6"]
pub struct EXTI6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EXTI7` reader - EXTI7"]
pub struct EXTI7_R(crate::FieldReader<u8, u8>);
impl EXTI7_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI7` writer - EXTI7"]
pub struct EXTI7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI4"]
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W {
        EXTI4_W { w: self }
    }
    #[doc = "Bits 8:15 - EXTI5"]
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W {
        EXTI5_W { w: self }
    }
    #[doc = "Bits 16:23 - EXTI6"]
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W {
        EXTI6_W { w: self }
    }
    #[doc = "Bits 24:31 - EXTI7"]
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W {
        EXTI7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr2](index.html) module"]
pub struct EXTI_EXTICR2_SPEC;
impl crate::RegisterSpec for EXTI_EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_exticr2::R](R) reader structure"]
impl crate::Readable for EXTI_EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_exticr2::W](W) writer structure"]
impl crate::Writable for EXTI_EXTICR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_EXTICR2 to value 0"]
impl crate::Resettable for EXTI_EXTICR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
