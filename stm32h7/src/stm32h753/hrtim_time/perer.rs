#[doc = "Register `PERER` reader"]
pub struct R(crate::R<PERER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERER` writer"]
pub struct W(crate::W<PERER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERER_SPEC>;
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
impl From<crate::W<PERER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERx` reader - Timerx Period value"]
pub struct PERX_R(crate::FieldReader<u16, u16>);
impl PERX_R {
    pub(crate) fn new(bits: u16) -> Self {
        PERX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERx` writer - Timerx Period value"]
pub struct PERX_W<'a> {
    w: &'a mut W,
}
impl<'a> PERX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&mut self) -> PERX_W {
        PERX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perer](index.html) module"]
pub struct PERER_SPEC;
impl crate::RegisterSpec for PERER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perer::R](R) reader structure"]
impl crate::Readable for PERER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perer::W](W) writer structure"]
impl crate::Writable for PERER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERER to value 0xffff"]
impl crate::Resettable for PERER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
