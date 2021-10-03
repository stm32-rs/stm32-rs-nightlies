#[doc = "Register `CSGCM6R` reader"]
pub struct R(crate::R<CSGCM6R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM6R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM6R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM6R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCM6R` writer"]
pub struct W(crate::W<CSGCM6R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM6R_SPEC>;
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
impl From<crate::W<CSGCM6R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM6R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCM6` reader - CSGCM6"]
pub struct CSGCM6_R(crate::FieldReader<u32, u32>);
impl CSGCM6_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSGCM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSGCM6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSGCM6` writer - CSGCM6"]
pub struct CSGCM6_W<'a> {
    w: &'a mut W,
}
impl<'a> CSGCM6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSGCM6"]
    #[inline(always)]
    pub fn csgcm6(&self) -> CSGCM6_R {
        CSGCM6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM6"]
    #[inline(always)]
    pub fn csgcm6(&mut self) -> CSGCM6_W {
        CSGCM6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm6r](index.html) module"]
pub struct CSGCM6R_SPEC;
impl crate::RegisterSpec for CSGCM6R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcm6r::R](R) reader structure"]
impl crate::Readable for CSGCM6R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcm6r::W](W) writer structure"]
impl crate::Writable for CSGCM6R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCM6R to value 0"]
impl crate::Resettable for CSGCM6R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}