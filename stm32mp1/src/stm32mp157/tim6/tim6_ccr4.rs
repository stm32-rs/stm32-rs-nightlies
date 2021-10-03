#[doc = "Register `TIM6_CCR4` reader"]
pub struct R(crate::R<TIM6_CCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_CCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_CCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_CCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM6_CCR4` writer"]
pub struct W(crate::W<TIM6_CCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_CCR4_SPEC>;
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
impl From<crate::W<TIM6_CCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_CCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR4` reader - CCR4"]
pub struct CCR4_R(crate::FieldReader<u16, u16>);
impl CCR4_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR4` writer - CCR4"]
pub struct CCR4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CCR4"]
    #[inline(always)]
    pub fn ccr4(&self) -> CCR4_R {
        CCR4_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR4"]
    #[inline(always)]
    pub fn ccr4(&mut self) -> CCR4_W {
        CCR4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM6 capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr4](index.html) module"]
pub struct TIM6_CCR4_SPEC;
impl crate::RegisterSpec for TIM6_CCR4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim6_ccr4::R](R) reader structure"]
impl crate::Readable for TIM6_CCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim6_ccr4::W](W) writer structure"]
impl crate::Writable for TIM6_CCR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM6_CCR4 to value 0"]
impl crate::Resettable for TIM6_CCR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
