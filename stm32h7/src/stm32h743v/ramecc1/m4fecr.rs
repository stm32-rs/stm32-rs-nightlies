#[doc = "Register `M4FECR` reader"]
pub struct R(crate::R<M4FECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4FECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4FECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4FECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4FECR` writer"]
pub struct W(crate::W<M4FECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4FECR_SPEC>;
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
impl From<crate::W<M4FECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4FECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEC` reader - ECC failing code"]
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
impl R {
    #[doc = "Bits 0:31 - ECC failing code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor 4 failing error code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4fecr](index.html) module"]
pub struct M4FECR_SPEC;
impl crate::RegisterSpec for M4FECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4fecr::R](R) reader structure"]
impl crate::Readable for M4FECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4fecr::W](W) writer structure"]
impl crate::Writable for M4FECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4FECR to value 0"]
impl crate::Resettable for M4FECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
