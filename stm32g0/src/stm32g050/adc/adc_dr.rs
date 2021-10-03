#[doc = "Register `ADC_DR` reader"]
pub struct R(crate::R<ADC_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on pageÂ 389. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
pub struct DATA_R(crate::FieldReader<u16, u16>);
impl DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Converted data These bits are read-only. They contain the conversion result from the last converted channel. The data are left- or right-aligned as shown in OVSE = 0) on pageÂ 389. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr](index.html) module"]
pub struct ADC_DR_SPEC;
impl crate::RegisterSpec for ADC_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dr::R](R) reader structure"]
impl crate::Readable for ADC_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_DR to value 0"]
impl crate::Resettable for ADC_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
