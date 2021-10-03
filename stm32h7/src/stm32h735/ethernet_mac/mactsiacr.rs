#[doc = "Register `MACTSIACR` reader"]
pub struct R(crate::R<MACTSIACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSIACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSIACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSIACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACTSIACR` writer"]
pub struct W(crate::W<MACTSIACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTSIACR_SPEC>;
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
impl From<crate::W<MACTSIACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTSIACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSTIAC` reader - One-Step Timestamp Ingress Asymmetry Correction"]
pub struct OSTIAC_R(crate::FieldReader<u32, u32>);
impl OSTIAC_R {
    pub(crate) fn new(bits: u32) -> Self {
        OSTIAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSTIAC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSTIAC` writer - One-Step Timestamp Ingress Asymmetry Correction"]
pub struct OSTIAC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - One-Step Timestamp Ingress Asymmetry Correction"]
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - One-Step Timestamp Ingress Asymmetry Correction"]
    #[inline(always)]
    pub fn ostiac(&mut self) -> OSTIAC_W {
        OSTIAC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Ingress asymmetric correction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mactsiacr](index.html) module"]
pub struct MACTSIACR_SPEC;
impl crate::RegisterSpec for MACTSIACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mactsiacr::R](R) reader structure"]
impl crate::Readable for MACTSIACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mactsiacr::W](W) writer structure"]
impl crate::Writable for MACTSIACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACTSIACR to value 0"]
impl crate::Resettable for MACTSIACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
