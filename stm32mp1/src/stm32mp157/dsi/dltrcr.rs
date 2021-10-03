#[doc = "Register `DLTRCR` reader"]
pub struct R(crate::R<DLTRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLTRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLTRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLTRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLTRCR` writer"]
pub struct W(crate::W<DLTRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLTRCR_SPEC>;
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
impl From<crate::W<DLTRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLTRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRD_TIME` reader - MRD_TIME"]
pub struct MRD_TIME_R(crate::FieldReader<u16, u16>);
impl MRD_TIME_R {
    pub(crate) fn new(bits: u16) -> Self {
        MRD_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRD_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRD_TIME` writer - MRD_TIME"]
pub struct MRD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> MRD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - MRD_TIME"]
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - MRD_TIME"]
    #[inline(always)]
    pub fn mrd_time(&mut self) -> MRD_TIME_W {
        MRD_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host data lane timer read configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dltrcr](index.html) module"]
pub struct DLTRCR_SPEC;
impl crate::RegisterSpec for DLTRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dltrcr::R](R) reader structure"]
impl crate::Readable for DLTRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dltrcr::W](W) writer structure"]
impl crate::Writable for DLTRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLTRCR to value 0"]
impl crate::Resettable for DLTRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
