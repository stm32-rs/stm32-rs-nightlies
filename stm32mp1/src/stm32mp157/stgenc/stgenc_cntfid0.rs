#[doc = "Register `STGENC_CNTFID0` reader"]
pub struct R(crate::R<STGENC_CNTFID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTFID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTFID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTFID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STGENC_CNTFID0` writer"]
pub struct W(crate::W<STGENC_CNTFID0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STGENC_CNTFID0_SPEC>;
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
impl From<crate::W<STGENC_CNTFID0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STGENC_CNTFID0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQ` reader - FREQ"]
pub struct FREQ_R(crate::FieldReader<u32, u32>);
impl FREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        FREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQ` writer - FREQ"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - FREQ"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntfid0](index.html) module"]
pub struct STGENC_CNTFID0_SPEC;
impl crate::RegisterSpec for STGENC_CNTFID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cntfid0::R](R) reader structure"]
impl crate::Readable for STGENC_CNTFID0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stgenc_cntfid0::W](W) writer structure"]
impl crate::Writable for STGENC_CNTFID0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STGENC_CNTFID0 to value 0"]
impl crate::Resettable for STGENC_CNTFID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
