#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR` writer"]
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM8_ETR_ADC2_RMP` reader - TIM8_ETR_ADC2 remapping capability"]
pub struct TIM8_ETR_ADC2_RMP_R(crate::FieldReader<u8, u8>);
impl TIM8_ETR_ADC2_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM8_ETR_ADC2_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8_ETR_ADC2_RMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8_ETR_ADC2_RMP` writer - TIM8_ETR_ADC2 remapping capability"]
pub struct TIM8_ETR_ADC2_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8_ETR_ADC2_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TIM8_ETR_ADC3_RMP` reader - TIM8_ETR_ADC3 remapping capability"]
pub struct TIM8_ETR_ADC3_RMP_R(crate::FieldReader<u8, u8>);
impl TIM8_ETR_ADC3_RMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM8_ETR_ADC3_RMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM8_ETR_ADC3_RMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM8_ETR_ADC3_RMP` writer - TIM8_ETR_ADC3 remapping capability"]
pub struct TIM8_ETR_ADC3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM8_ETR_ADC3_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TIM8_ETR_ADC2 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&self) -> TIM8_ETR_ADC2_RMP_R {
        TIM8_ETR_ADC2_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - TIM8_ETR_ADC3 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&self) -> TIM8_ETR_ADC3_RMP_R {
        TIM8_ETR_ADC3_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIM8_ETR_ADC2 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc2_rmp(&mut self) -> TIM8_ETR_ADC2_RMP_W {
        TIM8_ETR_ADC2_RMP_W { w: self }
    }
    #[doc = "Bits 2:3 - TIM8_ETR_ADC3 remapping capability"]
    #[inline(always)]
    pub fn tim8_etr_adc3_rmp(&mut self) -> TIM8_ETR_ADC3_RMP_W {
        TIM8_ETR_ADC3_RMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "option registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or::W](W) writer structure"]
impl crate::Writable for OR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
