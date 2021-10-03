#[doc = "Register `CSGCMCCM3R` reader"]
pub struct R(crate::R<CSGCMCCM3R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMCCM3R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMCCM3R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMCCM3R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCMCCM3R` writer"]
pub struct W(crate::W<CSGCMCCM3R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMCCM3R_SPEC>;
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
impl From<crate::W<CSGCMCCM3R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMCCM3R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM3R` reader - CSGCMCCM3R"]
pub struct CSGCMCCM3R_R(crate::FieldReader<u32, u32>);
impl CSGCMCCM3R_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSGCMCCM3R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSGCMCCM3R_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSGCMCCM3R` writer - CSGCMCCM3R"]
pub struct CSGCMCCM3R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM3R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM3R"]
    #[inline(always)]
    pub fn csgcmccm3r(&self) -> CSGCMCCM3R_R {
        CSGCMCCM3R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM3R"]
    #[inline(always)]
    pub fn csgcmccm3r(&mut self) -> CSGCMCCM3R_W {
        CSGCMCCM3R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm3r](index.html) module"]
pub struct CSGCMCCM3R_SPEC;
impl crate::RegisterSpec for CSGCMCCM3R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcmccm3r::R](R) reader structure"]
impl crate::Readable for CSGCMCCM3R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcmccm3r::W](W) writer structure"]
impl crate::Writable for CSGCMCCM3R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCMCCM3R to value 0"]
impl crate::Resettable for CSGCMCCM3R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
