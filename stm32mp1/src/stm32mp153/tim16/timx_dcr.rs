#[doc = "Register `TIMx_DCR` reader"]
pub struct R(crate::R<TIMX_DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_DCR` writer"]
pub struct W(crate::W<TIMX_DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_DCR_SPEC>;
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
impl From<crate::W<TIMX_DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBA` reader - DBA"]
pub struct DBA_R(crate::FieldReader<u8, u8>);
impl DBA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBA` writer - DBA"]
pub struct DBA_W<'a> {
    w: &'a mut W,
}
impl<'a> DBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
#[doc = "Field `DBL` reader - DBL"]
pub struct DBL_R(crate::FieldReader<u8, u8>);
impl DBL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBL` writer - DBL"]
pub struct DBL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u16 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DBA"]
    #[inline(always)]
    pub fn dba(&self) -> DBA_R {
        DBA_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DBL"]
    #[inline(always)]
    pub fn dbl(&self) -> DBL_R {
        DBL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DBA"]
    #[inline(always)]
    pub fn dba(&mut self) -> DBA_W {
        DBA_W { w: self }
    }
    #[doc = "Bits 8:12 - DBL"]
    #[inline(always)]
    pub fn dbl(&mut self) -> DBL_W {
        DBL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dcr](index.html) module"]
pub struct TIMX_DCR_SPEC;
impl crate::RegisterSpec for TIMX_DCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timx_dcr::R](R) reader structure"]
impl crate::Readable for TIMX_DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_dcr::W](W) writer structure"]
impl crate::Writable for TIMX_DCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_DCR to value 0"]
impl crate::Resettable for TIMX_DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
