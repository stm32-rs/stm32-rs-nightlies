#[doc = "Register `ICACHE_SR` reader"]
pub struct R(crate::R<ICACHE_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSYF` reader - BUSYF"]
pub struct BUSYF_R(crate::FieldReader<bool, bool>);
impl BUSYF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSYENDF` reader - BSYENDF"]
pub struct BSYENDF_R(crate::FieldReader<bool, bool>);
impl BSYENDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSYENDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSYENDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRF` reader - ERRF"]
pub struct ERRF_R(crate::FieldReader<bool, bool>);
impl ERRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - BUSYF"]
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BSYENDF"]
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ERRF"]
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "ICACHE status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_sr](index.html) module"]
pub struct ICACHE_SR_SPEC;
impl crate::RegisterSpec for ICACHE_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_sr::R](R) reader structure"]
impl crate::Readable for ICACHE_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ICACHE_SR to value 0x01"]
impl crate::Resettable for ICACHE_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
