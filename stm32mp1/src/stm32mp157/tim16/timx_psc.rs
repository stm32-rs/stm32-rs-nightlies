#[doc = "Register `TIMx_PSC` reader"]
pub struct R(crate::R<TIMX_PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_PSC` writer"]
pub struct W(crate::W<TIMX_PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_PSC_SPEC>;
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
impl From<crate::W<TIMX_PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - PSC"]
pub struct PSC_R(crate::FieldReader<u16, u16>);
impl PSC_R {
    pub(crate) fn new(bits: u16) -> Self {
        PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSC` writer - PSC"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_psc](index.html) module"]
pub struct TIMX_PSC_SPEC;
impl crate::RegisterSpec for TIMX_PSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timx_psc::R](R) reader structure"]
impl crate::Readable for TIMX_PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_psc::W](W) writer structure"]
impl crate::Writable for TIMX_PSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_PSC to value 0"]
impl crate::Resettable for TIMX_PSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}