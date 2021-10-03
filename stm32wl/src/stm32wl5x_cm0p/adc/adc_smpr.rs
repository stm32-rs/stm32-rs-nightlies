#[doc = "Register `ADC_SMPR` reader"]
pub struct R(crate::R<ADC_SMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_SMPR` writer"]
pub struct W(crate::W<ADC_SMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SMPR_SPEC>;
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
impl From<crate::W<ADC_SMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP1` reader - SMP1"]
pub struct SMP1_R(crate::FieldReader<u8, u8>);
impl SMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP1` writer - SMP1"]
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SMP2` reader - SMP2"]
pub struct SMP2_R(crate::FieldReader<u8, u8>);
impl SMP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP2` writer - SMP2"]
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SMPSEL` reader - SMPSEL"]
pub struct SMPSEL_R(crate::FieldReader<u32, u32>);
impl SMPSEL_R {
    pub(crate) fn new(bits: u32) -> Self {
        SMPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPSEL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPSEL` writer - SMPSEL"]
pub struct SMPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 8)) | ((value as u32 & 0x0003_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SMP1"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - SMP2"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:25 - SMPSEL"]
    #[inline(always)]
    pub fn smpsel(&self) -> SMPSEL_R {
        SMPSEL_R::new(((self.bits >> 8) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP1"]
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W {
        SMP1_W { w: self }
    }
    #[doc = "Bits 4:6 - SMP2"]
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W {
        SMP2_W { w: self }
    }
    #[doc = "Bits 8:25 - SMPSEL"]
    #[inline(always)]
    pub fn smpsel(&mut self) -> SMPSEL_W {
        SMPSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sampling time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_smpr](index.html) module"]
pub struct ADC_SMPR_SPEC;
impl crate::RegisterSpec for ADC_SMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_smpr::R](R) reader structure"]
impl crate::Readable for ADC_SMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_smpr::W](W) writer structure"]
impl crate::Writable for ADC_SMPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_SMPR to value 0"]
impl crate::Resettable for ADC_SMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
