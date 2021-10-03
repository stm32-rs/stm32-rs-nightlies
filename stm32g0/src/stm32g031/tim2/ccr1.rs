#[doc = "Register `CCR1` reader"]
pub struct R(crate::R<CCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR1` writer"]
pub struct W(crate::W<CCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR1_SPEC>;
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
impl From<crate::W<CCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR1_H` reader - High Capture/Compare 1 value (TIM2 only)"]
pub struct CCR1_H_R(crate::FieldReader<u16, u16>);
impl CCR1_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR1_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR1_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR1_H` writer - High Capture/Compare 1 value (TIM2 only)"]
pub struct CCR1_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `CCR1_L` reader - Low Capture/Compare 1 value"]
pub struct CCR1_L_R(crate::FieldReader<u16, u16>);
impl CCR1_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR1_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR1_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR1_L` writer - Low Capture/Compare 1 value"]
pub struct CCR1_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr1_h(&self) -> CCR1_H_R {
        CCR1_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1_l(&self) -> CCR1_L_R {
        CCR1_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - High Capture/Compare 1 value (TIM2 only)"]
    #[inline(always)]
    pub fn ccr1_h(&mut self) -> CCR1_H_W {
        CCR1_H_W { w: self }
    }
    #[doc = "Bits 0:15 - Low Capture/Compare 1 value"]
    #[inline(always)]
    pub fn ccr1_l(&mut self) -> CCR1_L_W {
        CCR1_L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr1](index.html) module"]
pub struct CCR1_SPEC;
impl crate::RegisterSpec for CCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr1::R](R) reader structure"]
impl crate::Readable for CCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr1::W](W) writer structure"]
impl crate::Writable for CCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for CCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
