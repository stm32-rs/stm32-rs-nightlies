#[doc = "Register `TIM4_CCR6` reader"]
pub struct R(crate::R<TIM4_CCR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_CCR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_CCR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_CCR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM4_CCR6` writer"]
pub struct W(crate::W<TIM4_CCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_CCR6_SPEC>;
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
impl From<crate::W<TIM4_CCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_CCR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR6` reader - CCR6"]
pub struct CCR6_R(crate::FieldReader<u16, u16>);
impl CCR6_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR6` writer - CCR6"]
pub struct CCR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CCR6"]
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR6"]
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W {
        CCR6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM4 capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr6](index.html) module"]
pub struct TIM4_CCR6_SPEC;
impl crate::RegisterSpec for TIM4_CCR6_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim4_ccr6::R](R) reader structure"]
impl crate::Readable for TIM4_CCR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim4_ccr6::W](W) writer structure"]
impl crate::Writable for TIM4_CCR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM4_CCR6 to value 0"]
impl crate::Resettable for TIM4_CCR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
