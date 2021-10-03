#[doc = "Register `ADC_CHSELR0` reader"]
pub struct R(crate::R<ADC_CHSELR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CHSELR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CHSELR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CHSELR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CHSELR0` writer"]
pub struct W(crate::W<ADC_CHSELR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CHSELR0_SPEC>;
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
impl From<crate::W<ADC_CHSELR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CHSELR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL` reader - CHSEL"]
pub struct CHSEL_R(crate::FieldReader<u32, u32>);
impl CHSEL_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - CHSEL"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_chselr0](index.html) module"]
pub struct ADC_CHSELR0_SPEC;
impl crate::RegisterSpec for ADC_CHSELR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_chselr0::R](R) reader structure"]
impl crate::Readable for ADC_CHSELR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_chselr0::W](W) writer structure"]
impl crate::Writable for ADC_CHSELR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CHSELR0 to value 0"]
impl crate::Resettable for ADC_CHSELR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
