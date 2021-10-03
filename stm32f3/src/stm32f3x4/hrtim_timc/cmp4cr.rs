#[doc = "Register `CMP4CR` reader"]
pub struct R(crate::R<CMP4CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP4CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP4CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP4CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP4CR` writer"]
pub struct W(crate::W<CMP4CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP4CR_SPEC>;
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
impl From<crate::W<CMP4CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP4CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP4x` reader - Timerx Compare 4 value"]
pub struct CMP4X_R(crate::FieldReader<u16, u16>);
impl CMP4X_R {
    pub(crate) fn new(bits: u16) -> Self {
        CMP4X_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP4X_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP4x` writer - Timerx Compare 4 value"]
pub struct CMP4X_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP4X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&self) -> CMP4X_R {
        CMP4X_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Compare 4 value"]
    #[inline(always)]
    pub fn cmp4x(&mut self) -> CMP4X_W {
        CMP4X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Compare 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp4cr](index.html) module"]
pub struct CMP4CR_SPEC;
impl crate::RegisterSpec for CMP4CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp4cr::R](R) reader structure"]
impl crate::Readable for CMP4CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp4cr::W](W) writer structure"]
impl crate::Writable for CMP4CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP4CR to value 0"]
impl crate::Resettable for CMP4CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
