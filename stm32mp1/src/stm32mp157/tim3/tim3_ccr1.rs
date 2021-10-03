#[doc = "Register `TIM3_CCR1` reader"]
pub struct R(crate::R<TIM3_CCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_CCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_CCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_CCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM3_CCR1` writer"]
pub struct W(crate::W<TIM3_CCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_CCR1_SPEC>;
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
impl From<crate::W<TIM3_CCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_CCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR1` reader - CCR1"]
pub struct CCR1_R(crate::FieldReader<u16, u16>);
impl CCR1_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR1` writer - CCR1"]
pub struct CCR1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CCR1"]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR1"]
    #[inline(always)]
    pub fn ccr1(&mut self) -> CCR1_W {
        CCR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM3 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_ccr1](index.html) module"]
pub struct TIM3_CCR1_SPEC;
impl crate::RegisterSpec for TIM3_CCR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim3_ccr1::R](R) reader structure"]
impl crate::Readable for TIM3_CCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim3_ccr1::W](W) writer structure"]
impl crate::Writable for TIM3_CCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM3_CCR1 to value 0"]
impl crate::Resettable for TIM3_CCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
