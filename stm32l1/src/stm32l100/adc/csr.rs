#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog flag of the ADC"]
pub struct AWD1_R(crate::FieldReader<bool, bool>);
impl AWD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC1` reader - End of conversion of the ADC"]
pub struct EOC1_R(crate::FieldReader<bool, bool>);
impl EOC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOC1` reader - Injected channel end of conversion of the ADC"]
pub struct JEOC1_R(crate::FieldReader<bool, bool>);
impl JEOC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEOC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSTRT1` reader - Injected channel Start flag of the ADC"]
pub struct JSTRT1_R(crate::FieldReader<bool, bool>);
impl JSTRT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSTRT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JSTRT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRT1` reader - Regular channel Start flag of the ADC"]
pub struct STRT1_R(crate::FieldReader<bool, bool>);
impl STRT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR1` reader - Overrun flag of the ADC"]
pub struct OVR1_R(crate::FieldReader<bool, bool>);
impl OVR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADONS1` reader - ADON Status of ADC1"]
pub struct ADONS1_R(crate::FieldReader<bool, bool>);
impl ADONS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADONS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADONS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Analog watchdog flag of the ADC"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of conversion of the ADC"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of the ADC"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of the ADC"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of the ADC"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of the ADC"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADON Status of ADC1"]
    #[inline(always)]
    pub fn adons1(&self) -> ADONS1_R {
        ADONS1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "ADC common status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
