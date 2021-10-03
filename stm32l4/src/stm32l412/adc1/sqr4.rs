#[doc = "Register `SQR4` reader"]
pub struct R(crate::R<SQR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR4` writer"]
pub struct W(crate::W<SQR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR4_SPEC>;
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
impl From<crate::W<SQR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ16` reader - SQ16"]
pub struct SQ16_R(crate::FieldReader<u8, u8>);
impl SQ16_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ16` writer - SQ16"]
pub struct SQ16_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `SQ15` reader - SQ15"]
pub struct SQ15_R(crate::FieldReader<u8, u8>);
impl SQ15_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ15` writer - SQ15"]
pub struct SQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:10 - SQ16"]
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ16_W {
        SQ16_W { w: self }
    }
    #[doc = "Bits 0:4 - SQ15"]
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ15_W {
        SQ15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr4](index.html) module"]
pub struct SQR4_SPEC;
impl crate::RegisterSpec for SQR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr4::R](R) reader structure"]
impl crate::Readable for SQR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr4::W](W) writer structure"]
impl crate::Writable for SQR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR4 to value 0"]
impl crate::Resettable for SQR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
