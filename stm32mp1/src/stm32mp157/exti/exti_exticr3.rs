#[doc = "Register `EXTI_EXTICR3` reader"]
pub struct R(crate::R<EXTI_EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_EXTICR3` writer"]
pub struct W(crate::W<EXTI_EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_EXTICR3_SPEC>;
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
impl From<crate::W<EXTI_EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8` reader - EXTI8"]
pub struct EXTI8_R(crate::FieldReader<u8, u8>);
impl EXTI8_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI8` writer - EXTI8"]
pub struct EXTI8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `EXTI9` reader - EXTI9"]
pub struct EXTI9_R(crate::FieldReader<u8, u8>);
impl EXTI9_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI9` writer - EXTI9"]
pub struct EXTI9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `EXTI10` reader - EXTI10"]
pub struct EXTI10_R(crate::FieldReader<u8, u8>);
impl EXTI10_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI10` writer - EXTI10"]
pub struct EXTI10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `EXTI11` reader - EXTI11"]
pub struct EXTI11_R(crate::FieldReader<u8, u8>);
impl EXTI11_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTI11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTI11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTI11` writer - EXTI11"]
pub struct EXTI11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTI11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - EXTI8"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - EXTI9"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - EXTI11"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXTI8"]
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W {
        EXTI8_W { w: self }
    }
    #[doc = "Bits 8:15 - EXTI9"]
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W {
        EXTI9_W { w: self }
    }
    #[doc = "Bits 16:23 - EXTI10"]
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W {
        EXTI10_W { w: self }
    }
    #[doc = "Bits 24:31 - EXTI11"]
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W {
        EXTI11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTIm fields contain only the number of bits in line with the nb_ioport configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_exticr3](index.html) module"]
pub struct EXTI_EXTICR3_SPEC;
impl crate::RegisterSpec for EXTI_EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_exticr3::R](R) reader structure"]
impl crate::Readable for EXTI_EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_exticr3::W](W) writer structure"]
impl crate::Writable for EXTI_EXTICR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_EXTICR3 to value 0"]
impl crate::Resettable for EXTI_EXTICR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
