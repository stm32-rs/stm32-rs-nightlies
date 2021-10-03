#[doc = "Register `DFSDM_FLT5AWCFR` reader"]
pub struct R(crate::R<DFSDM_FLT5AWCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT5AWCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT5AWCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT5AWCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_FLT5AWCFR` writer"]
pub struct W(crate::W<DFSDM_FLT5AWCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT5AWCFR_SPEC>;
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
impl From<crate::W<DFSDM_FLT5AWCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT5AWCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRAWLTF` reader - CLRAWLTF"]
pub struct CLRAWLTF_R(crate::FieldReader<u8, u8>);
impl CLRAWLTF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLRAWLTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRAWLTF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRAWLTF` writer - CLRAWLTF"]
pub struct CLRAWLTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRAWLTF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CLRAWHTF` reader - CLRAWHTF"]
pub struct CLRAWHTF_R(crate::FieldReader<u8, u8>);
impl CLRAWHTF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLRAWHTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLRAWHTF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLRAWHTF` writer - CLRAWHTF"]
pub struct CLRAWHTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRAWHTF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CLRAWLTF"]
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CLRAWHTF"]
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLRAWLTF"]
    #[inline(always)]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W {
        CLRAWLTF_W { w: self }
    }
    #[doc = "Bits 8:15 - CLRAWHTF"]
    #[inline(always)]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W {
        CLRAWHTF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM filter 5 analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt5awcfr](index.html) module"]
pub struct DFSDM_FLT5AWCFR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT5AWCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt5awcfr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT5AWCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt5awcfr::W](W) writer structure"]
impl crate::Writable for DFSDM_FLT5AWCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM_FLT5AWCFR to value 0"]
impl crate::Resettable for DFSDM_FLT5AWCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
