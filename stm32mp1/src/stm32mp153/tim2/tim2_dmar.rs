#[doc = "Register `TIM2_DMAR` reader"]
pub struct R(crate::R<TIM2_DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM2_DMAR` writer"]
pub struct W(crate::W<TIM2_DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_DMAR_SPEC>;
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
impl From<crate::W<TIM2_DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAB` reader - DMAB"]
pub struct DMAB_R(crate::FieldReader<u32, u32>);
impl DMAB_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAB` writer - DMAB"]
pub struct DMAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMAB"]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMAB"]
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W {
        DMAB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_dmar](index.html) module"]
pub struct TIM2_DMAR_SPEC;
impl crate::RegisterSpec for TIM2_DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim2_dmar::R](R) reader structure"]
impl crate::Readable for TIM2_DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim2_dmar::W](W) writer structure"]
impl crate::Writable for TIM2_DMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM2_DMAR to value 0"]
impl crate::Resettable for TIM2_DMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}