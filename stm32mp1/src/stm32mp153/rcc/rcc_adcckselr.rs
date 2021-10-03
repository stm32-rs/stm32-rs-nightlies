#[doc = "Register `RCC_ADCCKSELR` reader"]
pub struct R(crate::R<RCC_ADCCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ADCCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ADCCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ADCCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_ADCCKSELR` writer"]
pub struct W(crate::W<RCC_ADCCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ADCCKSELR_SPEC>;
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
impl From<crate::W<RCC_ADCCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ADCCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSRC` reader - ADCSRC"]
pub struct ADCSRC_R(crate::FieldReader<u8, u8>);
impl ADCSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSRC` writer - ADCSRC"]
pub struct ADCSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&self) -> ADCSRC_R {
        ADCSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADCSRC"]
    #[inline(always)]
    pub fn adcsrc(&mut self) -> ADCSRC_W {
        ADCSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the ADC block.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_adcckselr](index.html) module"]
pub struct RCC_ADCCKSELR_SPEC;
impl crate::RegisterSpec for RCC_ADCCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_adcckselr::R](R) reader structure"]
impl crate::Readable for RCC_ADCCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_adcckselr::W](W) writer structure"]
impl crate::Writable for RCC_ADCCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_ADCCKSELR to value 0"]
impl crate::Resettable for RCC_ADCCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
