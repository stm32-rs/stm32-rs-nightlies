#[doc = "Register `MDIOS_SR` reader"]
pub struct R(crate::R<MDIOS_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERF` reader - Preamble error flag"]
pub struct PERF_R(crate::FieldReader<bool, bool>);
impl PERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERF` reader - Start error flag"]
pub struct SERF_R(crate::FieldReader<bool, bool>);
impl SERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERF` reader - Turnaround error flag"]
pub struct TERF_R(crate::FieldReader<bool, bool>);
impl TERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Preamble error flag"]
    #[inline(always)]
    pub fn perf(&self) -> PERF_R {
        PERF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start error flag"]
    #[inline(always)]
    pub fn serf(&self) -> SERF_R {
        SERF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Turnaround error flag"]
    #[inline(always)]
    pub fn terf(&self) -> TERF_R {
        TERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "MDIOS status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_sr](index.html) module"]
pub struct MDIOS_SR_SPEC;
impl crate::RegisterSpec for MDIOS_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_sr::R](R) reader structure"]
impl crate::Readable for MDIOS_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_SR to value 0"]
impl crate::Resettable for MDIOS_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
