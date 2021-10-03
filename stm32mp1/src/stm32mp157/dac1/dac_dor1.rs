#[doc = "Register `DAC_DOR1` reader"]
pub struct R(crate::R<DAC_DOR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DOR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DOR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DOR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DACC1DOR` reader - DACC1DOR"]
pub struct DACC1DOR_R(crate::FieldReader<u16, u16>);
impl DACC1DOR_R {
    pub(crate) fn new(bits: u16) -> Self {
        DACC1DOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACC1DOR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - DACC1DOR"]
    #[inline(always)]
    pub fn dacc1dor(&self) -> DACC1DOR_R {
        DACC1DOR_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DAC channel1 data output register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_dor1](index.html) module"]
pub struct DAC_DOR1_SPEC;
impl crate::RegisterSpec for DAC_DOR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_dor1::R](R) reader structure"]
impl crate::Readable for DAC_DOR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAC_DOR1 to value 0"]
impl crate::Resettable for DAC_DOR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
