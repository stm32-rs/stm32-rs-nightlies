#[doc = "Register `TIM5_ARR` reader"]
pub struct R(crate::R<TIM5_ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_ARR` writer"]
pub struct W(crate::W<TIM5_ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_ARR_SPEC>;
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
impl From<crate::W<TIM5_ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR` reader - ARR"]
pub struct ARR_R(crate::FieldReader<u16, u16>);
impl ARR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ARR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARR` writer - ARR"]
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_arr](index.html) module"]
pub struct TIM5_ARR_SPEC;
impl crate::RegisterSpec for TIM5_ARR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim5_arr::R](R) reader structure"]
impl crate::Readable for TIM5_ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_arr::W](W) writer structure"]
impl crate::Writable for TIM5_ARR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM5_ARR to value 0xffff"]
impl crate::Resettable for TIM5_ARR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
