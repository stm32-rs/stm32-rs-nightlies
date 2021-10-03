#[doc = "Register `MDIOS_CWRFR` reader"]
pub struct R(crate::R<MDIOS_CWRFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_CWRFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_CWRFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_CWRFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIOS_CWRFR` writer"]
pub struct W(crate::W<MDIOS_CWRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOS_CWRFR_SPEC>;
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
impl From<crate::W<MDIOS_CWRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOS_CWRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CWRF` reader - Clear the write flag"]
pub struct CWRF_R(crate::FieldReader<u32, u32>);
impl CWRF_R {
    pub(crate) fn new(bits: u32) -> Self {
        CWRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CWRF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CWRF` writer - Clear the write flag"]
pub struct CWRF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    pub fn cwrf(&self) -> CWRF_R {
        CWRF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    pub fn cwrf(&mut self) -> CWRF_W {
        CWRF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS clear write flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_cwrfr](index.html) module"]
pub struct MDIOS_CWRFR_SPEC;
impl crate::RegisterSpec for MDIOS_CWRFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_cwrfr::R](R) reader structure"]
impl crate::Readable for MDIOS_CWRFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdios_cwrfr::W](W) writer structure"]
impl crate::Writable for MDIOS_CWRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIOS_CWRFR to value 0"]
impl crate::Resettable for MDIOS_CWRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
