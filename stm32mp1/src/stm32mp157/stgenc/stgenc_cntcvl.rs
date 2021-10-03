#[doc = "Register `STGENC_CNTCVL` reader"]
pub struct R(crate::R<STGENC_CNTCVL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTCVL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTCVL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTCVL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STGENC_CNTCVL` writer"]
pub struct W(crate::W<STGENC_CNTCVL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STGENC_CNTCVL_SPEC>;
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
impl From<crate::W<STGENC_CNTCVL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STGENC_CNTCVL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTCVL_L_32` reader - CNTCVL_L_32"]
pub struct CNTCVL_L_32_R(crate::FieldReader<u32, u32>);
impl CNTCVL_L_32_R {
    pub(crate) fn new(bits: u32) -> Self {
        CNTCVL_L_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTCVL_L_32_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCVL_L_32` writer - CNTCVL_L_32"]
pub struct CNTCVL_L_32_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCVL_L_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    pub fn cntcvl_l_32(&self) -> CNTCVL_L_32_R {
        CNTCVL_L_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CNTCVL_L_32"]
    #[inline(always)]
    pub fn cntcvl_l_32(&mut self) -> CNTCVL_L_32_W {
        CNTCVL_L_32_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cntcvl](index.html) module"]
pub struct STGENC_CNTCVL_SPEC;
impl crate::RegisterSpec for STGENC_CNTCVL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cntcvl::R](R) reader structure"]
impl crate::Readable for STGENC_CNTCVL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stgenc_cntcvl::W](W) writer structure"]
impl crate::Writable for STGENC_CNTCVL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STGENC_CNTCVL to value 0"]
impl crate::Resettable for STGENC_CNTCVL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
