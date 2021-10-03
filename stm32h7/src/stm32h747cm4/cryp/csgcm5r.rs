#[doc = "Register `CSGCM5R` reader"]
pub struct R(crate::R<CSGCM5R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM5R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM5R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM5R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCM5R` writer"]
pub struct W(crate::W<CSGCM5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM5R_SPEC>;
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
impl From<crate::W<CSGCM5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM5R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCM5R` reader - CSGCM5R"]
pub struct CSGCM5R_R(crate::FieldReader<u32, u32>);
impl CSGCM5R_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSGCM5R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSGCM5R_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSGCM5R` writer - CSGCM5R"]
pub struct CSGCM5R_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM5R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM5R"]
    #[inline(always)]
    pub fn csgcm5r(&self) -> CSGCM5R_R {
        CSGCM5R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM5R"]
    #[inline(always)]
    pub fn csgcm5r(&mut self) -> CSGCM5R_W {
        CSGCM5R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm5r](index.html) module"]
pub struct CSGCM5R_SPEC;
impl crate::RegisterSpec for CSGCM5R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcm5r::R](R) reader structure"]
impl crate::Readable for CSGCM5R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcm5r::W](W) writer structure"]
impl crate::Writable for CSGCM5R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCM5R to value 0"]
impl crate::Resettable for CSGCM5R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
