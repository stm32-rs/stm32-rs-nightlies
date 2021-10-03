#[doc = "Register `DTS_RAMPVALR` reader"]
pub struct R(crate::R<DTS_RAMPVALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_RAMPVALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_RAMPVALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_RAMPVALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TS1_RAMP_COEFF` reader - TS1_RAMP_COEFF"]
pub struct TS1_RAMP_COEFF_R(crate::FieldReader<u16, u16>);
impl TS1_RAMP_COEFF_R {
    pub(crate) fn new(bits: u16) -> Self {
        TS1_RAMP_COEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS1_RAMP_COEFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - TS1_RAMP_COEFF"]
    #[inline(always)]
    pub fn ts1_ramp_coeff(&self) -> TS1_RAMP_COEFF_R {
        TS1_RAMP_COEFF_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_rampvalr](index.html) module"]
pub struct DTS_RAMPVALR_SPEC;
impl crate::RegisterSpec for DTS_RAMPVALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dts_rampvalr::R](R) reader structure"]
impl crate::Readable for DTS_RAMPVALR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DTS_RAMPVALR to value 0"]
impl crate::Resettable for DTS_RAMPVALR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
