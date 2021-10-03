#[doc = "Register `ADCPS1` reader"]
pub struct R(crate::R<ADCPS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCPS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCPS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCPS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCPS1` writer"]
pub struct W(crate::W<ADCPS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCPS1_SPEC>;
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
impl From<crate::W<ADCPS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCPS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC5PSC` reader - ADC5PSC"]
pub struct ADC5PSC_R(crate::FieldReader<u8, u8>);
impl ADC5PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC5PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC5PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC5PSC` writer - ADC5PSC"]
pub struct ADC5PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC5PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `ADC4PSC` reader - ADC4PSC"]
pub struct ADC4PSC_R(crate::FieldReader<u8, u8>);
impl ADC4PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC4PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC4PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC4PSC` writer - ADC4PSC"]
pub struct ADC4PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC4PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `ADC3PSC` reader - ADC3PSC"]
pub struct ADC3PSC_R(crate::FieldReader<u8, u8>);
impl ADC3PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC3PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC3PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC3PSC` writer - ADC3PSC"]
pub struct ADC3PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC3PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
#[doc = "Field `ADC2PSC` reader - ADC2PSC"]
pub struct ADC2PSC_R(crate::FieldReader<u8, u8>);
impl ADC2PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC2PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2PSC` writer - ADC2PSC"]
pub struct ADC2PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `ADC1PSC` reader - ADC1PSC"]
pub struct ADC1PSC_R(crate::FieldReader<u8, u8>);
impl ADC1PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC1PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1PSC` writer - ADC1PSC"]
pub struct ADC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - ADC5PSC"]
    #[inline(always)]
    pub fn adc5psc(&self) -> ADC5PSC_R {
        ADC5PSC_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - ADC4PSC"]
    #[inline(always)]
    pub fn adc4psc(&self) -> ADC4PSC_R {
        ADC4PSC_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - ADC3PSC"]
    #[inline(always)]
    pub fn adc3psc(&self) -> ADC3PSC_R {
        ADC3PSC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - ADC2PSC"]
    #[inline(always)]
    pub fn adc2psc(&self) -> ADC2PSC_R {
        ADC2PSC_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - ADC1PSC"]
    #[inline(always)]
    pub fn adc1psc(&self) -> ADC1PSC_R {
        ADC1PSC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - ADC5PSC"]
    #[inline(always)]
    pub fn adc5psc(&mut self) -> ADC5PSC_W {
        ADC5PSC_W { w: self }
    }
    #[doc = "Bits 18:22 - ADC4PSC"]
    #[inline(always)]
    pub fn adc4psc(&mut self) -> ADC4PSC_W {
        ADC4PSC_W { w: self }
    }
    #[doc = "Bits 12:16 - ADC3PSC"]
    #[inline(always)]
    pub fn adc3psc(&mut self) -> ADC3PSC_W {
        ADC3PSC_W { w: self }
    }
    #[doc = "Bits 6:10 - ADC2PSC"]
    #[inline(always)]
    pub fn adc2psc(&mut self) -> ADC2PSC_W {
        ADC2PSC_W { w: self }
    }
    #[doc = "Bits 0:4 - ADC1PSC"]
    #[inline(always)]
    pub fn adc1psc(&mut self) -> ADC1PSC_W {
        ADC1PSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM ADC Post Scaler Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcps1](index.html) module"]
pub struct ADCPS1_SPEC;
impl crate::RegisterSpec for ADCPS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcps1::R](R) reader structure"]
impl crate::Readable for ADCPS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcps1::W](W) writer structure"]
impl crate::Writable for ADCPS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCPS1 to value 0"]
impl crate::Resettable for ADCPS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
