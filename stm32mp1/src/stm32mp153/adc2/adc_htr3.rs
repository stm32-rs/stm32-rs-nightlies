#[doc = "Register `ADC_HTR3` reader"]
pub struct R(crate::R<ADC_HTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HTR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HTR3` writer"]
pub struct W(crate::W<ADC_HTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HTR3_SPEC>;
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
impl From<crate::W<ADC_HTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTR3` reader - HTR3"]
pub struct HTR3_R(crate::FieldReader<u32, u32>);
impl HTR3_R {
    pub(crate) fn new(bits: u32) -> Self {
        HTR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTR3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTR3` writer - HTR3"]
pub struct HTR3_W<'a> {
    w: &'a mut W,
}
impl<'a> HTR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | (value as u32 & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - HTR3"]
    #[inline(always)]
    pub fn htr3(&self) -> HTR3_R {
        HTR3_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - HTR3"]
    #[inline(always)]
    pub fn htr3(&mut self) -> HTR3_W {
        HTR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC watchdog higher threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htr3](index.html) module"]
pub struct ADC_HTR3_SPEC;
impl crate::RegisterSpec for ADC_HTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_htr3::R](R) reader structure"]
impl crate::Readable for ADC_HTR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_htr3::W](W) writer structure"]
impl crate::Writable for ADC_HTR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_HTR3 to value 0x03ff_ffff"]
impl crate::Resettable for ADC_HTR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff_ffff
    }
}
