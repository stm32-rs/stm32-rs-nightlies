#[doc = "Register `ADC_CALFACT` reader"]
pub struct R(crate::R<ADC_CALFACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CALFACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CALFACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CALFACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CALFACT` writer"]
pub struct W(crate::W<ADC_CALFACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CALFACT_SPEC>;
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
impl From<crate::W<ADC_CALFACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CALFACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALFACT_S` reader - CALFACT_S"]
pub struct CALFACT_S_R(crate::FieldReader<u16, u16>);
impl CALFACT_S_R {
    pub(crate) fn new(bits: u16) -> Self {
        CALFACT_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALFACT_S_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALFACT_S` writer - CALFACT_S"]
pub struct CALFACT_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `CALFACT_D` reader - CALFACT_D"]
pub struct CALFACT_D_R(crate::FieldReader<u16, u16>);
impl CALFACT_D_R {
    pub(crate) fn new(bits: u16) -> Self {
        CALFACT_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALFACT_D_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALFACT_D` writer - CALFACT_D"]
pub struct CALFACT_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CALFACT_S_W {
        CALFACT_S_W { w: self }
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CALFACT_D_W {
        CALFACT_D_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factors register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_calfact](index.html) module"]
pub struct ADC_CALFACT_SPEC;
impl crate::RegisterSpec for ADC_CALFACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_calfact::R](R) reader structure"]
impl crate::Readable for ADC_CALFACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_calfact::W](W) writer structure"]
impl crate::Writable for ADC_CALFACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CALFACT to value 0"]
impl crate::Resettable for ADC_CALFACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
