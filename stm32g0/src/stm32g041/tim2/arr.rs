#[doc = "Register `ARR` reader"]
pub struct R(crate::R<ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARR` writer"]
pub struct W(crate::W<ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARR_SPEC>;
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
impl From<crate::W<ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR_H` reader - High Auto-reload value (TIM2 only)"]
pub struct ARR_H_R(crate::FieldReader<u16, u16>);
impl ARR_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        ARR_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARR_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARR_H` writer - High Auto-reload value (TIM2 only)"]
pub struct ARR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `ARR_L` reader - Low Auto-reload value"]
pub struct ARR_L_R(crate::FieldReader<u16, u16>);
impl ARR_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        ARR_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARR_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARR_L` writer - Low Auto-reload value"]
pub struct ARR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - High Auto-reload value (TIM2 only)"]
    #[inline(always)]
    pub fn arr_h(&self) -> ARR_H_R {
        ARR_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arr_l(&self) -> ARR_L_R {
        ARR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - High Auto-reload value (TIM2 only)"]
    #[inline(always)]
    pub fn arr_h(&mut self) -> ARR_H_W {
        ARR_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arr_l(&mut self) -> ARR_L_W {
        ARR_L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arr](index.html) module"]
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arr::R](R) reader structure"]
impl crate::Readable for ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arr::W](W) writer structure"]
impl crate::Writable for ARR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::Resettable for ARR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
