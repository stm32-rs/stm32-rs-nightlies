#[doc = "Register `ADC_LTR1` reader"]
pub struct R(crate::R<ADC_LTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_LTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_LTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_LTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_LTR1` writer"]
pub struct W(crate::W<ADC_LTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_LTR1_SPEC>;
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
impl From<crate::W<ADC_LTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_LTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTR1` reader - LTR1"]
pub struct LTR1_R(crate::FieldReader<u32, u32>);
impl LTR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        LTR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LTR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LTR1` writer - LTR1"]
pub struct LTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> LTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - LTR1"]
    #[inline(always)]
    pub fn ltr1(&self) -> LTR1_R {
        LTR1_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - LTR1"]
    #[inline(always)]
    pub fn ltr1(&mut self) -> LTR1_W {
        LTR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC watchdog threshold register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ltr1](index.html) module"]
pub struct ADC_LTR1_SPEC;
impl crate::RegisterSpec for ADC_LTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ltr1::R](R) reader structure"]
impl crate::Readable for ADC_LTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ltr1::W](W) writer structure"]
impl crate::Writable for ADC_LTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_LTR1 to value 0"]
impl crate::Resettable for ADC_LTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
