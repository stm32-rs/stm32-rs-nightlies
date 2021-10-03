#[doc = "Register `CSGCMCCM5R` reader"]
pub struct R(crate::R<CSGCMCCM5R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMCCM5R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMCCM5R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMCCM5R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCMCCM5R` writer"]
pub struct W(crate::W<CSGCMCCM5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMCCM5R_SPEC>;
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
impl From<crate::W<CSGCMCCM5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMCCM5R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM5R` reader - CSGCMCCM5R"]
pub struct CSGCMCCM5R_R(crate::FieldReader<u32, u32>);
impl CSGCMCCM5R_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSGCMCCM5R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSGCMCCM5R_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSGCMCCM5R` writer - CSGCMCCM5R"]
pub struct CSGCMCCM5R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCMCCM5R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM5R"]
    #[inline(always)]
    pub fn csgcmccm5r(&self) -> CSGCMCCM5R_R {
        CSGCMCCM5R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM5R"]
    #[inline(always)]
    pub fn csgcmccm5r(&mut self) -> CSGCMCCM5R_W {
        CSGCMCCM5R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm5r](index.html) module"]
pub struct CSGCMCCM5R_SPEC;
impl crate::RegisterSpec for CSGCMCCM5R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcmccm5r::R](R) reader structure"]
impl crate::Readable for CSGCMCCM5R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcmccm5r::W](W) writer structure"]
impl crate::Writable for CSGCMCCM5R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCMCCM5R to value 0"]
impl crate::Resettable for CSGCMCCM5R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
