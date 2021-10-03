#[doc = "Register `CNTCR` reader"]
pub struct R(crate::R<CNTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTCR` writer"]
pub struct W(crate::W<CNTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTCR_SPEC>;
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
impl From<crate::W<CNTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTx` reader - Timerx Counter value"]
pub struct CNTX_R(crate::FieldReader<u16, u16>);
impl CNTX_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTx` writer - Timerx Counter value"]
pub struct CNTX_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Counter value"]
    #[inline(always)]
    pub fn cntx(&mut self) -> CNTX_W {
        CNTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcr](index.html) module"]
pub struct CNTCR_SPEC;
impl crate::RegisterSpec for CNTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntcr::R](R) reader structure"]
impl crate::Readable for CNTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntcr::W](W) writer structure"]
impl crate::Writable for CNTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTCR to value 0"]
impl crate::Resettable for CNTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
