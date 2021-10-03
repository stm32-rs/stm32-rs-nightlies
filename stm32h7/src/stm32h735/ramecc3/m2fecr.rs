#[doc = "Register `M2FECR` reader"]
pub struct R(crate::R<M2FECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M2FECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M2FECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M2FECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M2FECR` writer"]
pub struct W(crate::W<M2FECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M2FECR_SPEC>;
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
impl From<crate::W<M2FECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M2FECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEC` reader - Failing error code"]
pub struct FEC_R(crate::FieldReader<u32, u32>);
impl FEC_R {
    pub(crate) fn new(bits: u32) -> Self {
        FEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEC` writer - Failing error code"]
pub struct FEC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W {
        FEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m2fecr](index.html) module"]
pub struct M2FECR_SPEC;
impl crate::RegisterSpec for M2FECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m2fecr::R](R) reader structure"]
impl crate::Readable for M2FECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m2fecr::W](W) writer structure"]
impl crate::Writable for M2FECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M2FECR to value 0"]
impl crate::Resettable for M2FECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
