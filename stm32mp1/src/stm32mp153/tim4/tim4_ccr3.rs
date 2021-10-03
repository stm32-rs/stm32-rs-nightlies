#[doc = "Register `TIM4_CCR3` reader"]
pub struct R(crate::R<TIM4_CCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM4_CCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM4_CCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM4_CCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM4_CCR3` writer"]
pub struct W(crate::W<TIM4_CCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM4_CCR3_SPEC>;
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
impl From<crate::W<TIM4_CCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM4_CCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR3` reader - CCR3"]
pub struct CCR3_R(crate::FieldReader<u16, u16>);
impl CCR3_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR3_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR3` writer - CCR3"]
pub struct CCR3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    pub fn ccr3(&self) -> CCR3_R {
        CCR3_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR3"]
    #[inline(always)]
    pub fn ccr3(&mut self) -> CCR3_W {
        CCR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM4 capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim4_ccr3](index.html) module"]
pub struct TIM4_CCR3_SPEC;
impl crate::RegisterSpec for TIM4_CCR3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim4_ccr3::R](R) reader structure"]
impl crate::Readable for TIM4_CCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim4_ccr3::W](W) writer structure"]
impl crate::Writable for TIM4_CCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM4_CCR3 to value 0"]
impl crate::Resettable for TIM4_CCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
