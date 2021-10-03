#[doc = "Register `TIM2_OR` reader"]
pub struct R(crate::R<TIM2_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM2_OR` writer"]
pub struct W(crate::W<TIM2_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_OR_SPEC>;
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
impl From<crate::W<TIM2_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IT4_RMP` reader - Timer Input 4 remap"]
pub struct IT4_RMP_R(crate::FieldReader<u8, u8>);
impl IT4_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IT4_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IT4_RMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IT4_RMP` writer - Timer Input 4 remap"]
pub struct IT4_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> IT4_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn it4_rmp(&self) -> IT4_RMP_R {
        IT4_RMP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn it4_rmp(&mut self) -> IT4_RMP_W {
        IT4_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 option register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_or](index.html) module"]
pub struct TIM2_OR_SPEC;
impl crate::RegisterSpec for TIM2_OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim2_or::R](R) reader structure"]
impl crate::Readable for TIM2_OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim2_or::W](W) writer structure"]
impl crate::Writable for TIM2_OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM2_OR to value 0"]
impl crate::Resettable for TIM2_OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
