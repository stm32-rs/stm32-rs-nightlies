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
#[doc = "Field `JQOVF_MST` reader - Injected Context Queue Overflow flag of the master ADC"]
pub struct JQOVF_MST_R(crate::FieldReader<bool, bool>);
impl JQOVF_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        JQOVF_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JQOVF_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD3_MST` reader - Analog watchdog 3 flag of the master ADC"]
pub struct AWD3_MST_R(crate::FieldReader<bool, bool>);
impl AWD3_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD3_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD3_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD2_MST` reader - Analog watchdog 2 flag of the master ADC"]
pub struct AWD2_MST_R(crate::FieldReader<bool, bool>);
impl AWD2_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD2_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD2_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWD1_MST` reader - Analog watchdog 1 flag of the master ADC"]
pub struct AWD1_MST_R(crate::FieldReader<bool, bool>);
impl AWD1_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWD1_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWD1_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOS_MST` reader - End of injected sequence flag of the master ADC"]
pub struct JEOS_MST_R(crate::FieldReader<bool, bool>);
impl JEOS_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOS_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEOS_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JEOC_MST` reader - End of injected conversion flag of the master ADC"]
pub struct JEOC_MST_R(crate::FieldReader<bool, bool>);
impl JEOC_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        JEOC_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JEOC_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_MST` reader - Overrun flag of the master ADC"]
pub struct OVR_MST_R(crate::FieldReader<bool, bool>);
impl OVR_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOS_MST` reader - End of regular sequence flag of the master ADC"]
pub struct EOS_MST_R(crate::FieldReader<bool, bool>);
impl EOS_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOS_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOS_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOC_MST` reader - End of regular conversion flag of the master ADC"]
pub struct EOC_MST_R(crate::FieldReader<bool, bool>);
impl EOC_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOC_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOC_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EOSMP_MST` reader - End of Sampling phase flag of the master ADC"]
pub struct EOSMP_MST_R(crate::FieldReader<bool, bool>);
impl EOSMP_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EOSMP_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EOSMP_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADRDY_MST` reader - master ADC ready"]
pub struct ADRDY_MST_R(crate::FieldReader<bool, bool>);
impl ADRDY_MST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADRDY_MST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADRDY_MST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 10 - Injected Context Queue Overflow flag of the master ADC"]
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Analog watchdog 3 flag of the master ADC"]
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog 2 flag of the master ADC"]
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog watchdog 1 flag of the master ADC"]
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End of injected sequence flag of the master ADC"]
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of injected conversion flag of the master ADC"]
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Overrun flag of the master ADC"]
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of regular sequence flag of the master ADC"]
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of regular conversion flag of the master ADC"]
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Sampling phase flag of the master ADC"]
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - master ADC ready"]
    #[inline(always)]
    pub fn adrdy_mst(&self) -> ADRDY_MST_R {
        ADRDY_MST_R::new((self.bits & 0x01) != 0)
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
