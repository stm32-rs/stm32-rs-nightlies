#[doc = "Register `ITLINE12` reader"]
pub struct R(crate::R<ITLINE12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADC` reader - ADC"]
pub struct ADC_R(crate::FieldReader<bool, bool>);
impl ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP1` reader - COMP1"]
pub struct COMP1_R(crate::FieldReader<bool, bool>);
impl COMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP2` reader - COMP2"]
pub struct COMP2_R(crate::FieldReader<bool, bool>);
impl COMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COMP1"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - COMP2"]
    #[inline(always)]
    pub fn comp2(&self) -> COMP2_R {
        COMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "interrupt line 12 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline12](index.html) module"]
pub struct ITLINE12_SPEC;
impl crate::RegisterSpec for ITLINE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline12::R](R) reader structure"]
impl crate::Readable for ITLINE12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE12 to value 0"]
impl crate::Resettable for ITLINE12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
