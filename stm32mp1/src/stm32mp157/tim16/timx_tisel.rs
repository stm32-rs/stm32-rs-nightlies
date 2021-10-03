#[doc = "Register `TIMx_TISEL` reader"]
pub struct R(crate::R<TIMX_TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_TISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_TISEL` writer"]
pub struct W(crate::W<TIMX_TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_TISEL_SPEC>;
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
impl From<crate::W<TIMX_TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_TISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub struct TI1SEL_R(crate::FieldReader<u8, u8>);
impl TI1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TI1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub struct TI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM17 input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_tisel](index.html) module"]
pub struct TIMX_TISEL_SPEC;
impl crate::RegisterSpec for TIMX_TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_tisel::R](R) reader structure"]
impl crate::Readable for TIMX_TISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_tisel::W](W) writer structure"]
impl crate::Writable for TIMX_TISEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_TISEL to value 0"]
impl crate::Resettable for TIMX_TISEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}