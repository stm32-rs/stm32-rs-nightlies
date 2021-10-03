#[doc = "Register `PKA_SR` reader"]
pub struct R(crate::R<PKA_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKA_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKA_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKA_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - PKA operation in progress"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROCENDF` reader - PKA end of operation flag"]
pub struct PROCENDF_R(crate::FieldReader<bool, bool>);
impl PROCENDF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROCENDF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROCENDF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMERRF` reader - PKA ram error flag"]
pub struct RAMERRF_R(crate::FieldReader<bool, bool>);
impl RAMERRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMERRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMERRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRERRF` reader - address er flag"]
pub struct ADDRERRF_R(crate::FieldReader<bool, bool>);
impl ADDRERRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRERRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRERRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 16 - PKA operation in progress"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PKA end of operation flag"]
    #[inline(always)]
    pub fn procendf(&self) -> PROCENDF_R {
        PROCENDF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PKA ram error flag"]
    #[inline(always)]
    pub fn ramerrf(&self) -> RAMERRF_R {
        RAMERRF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - address er flag"]
    #[inline(always)]
    pub fn addrerrf(&self) -> ADDRERRF_R {
        ADDRERRF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
#[doc = "PKA status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pka_sr](index.html) module"]
pub struct PKA_SR_SPEC;
impl crate::RegisterSpec for PKA_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pka_sr::R](R) reader structure"]
impl crate::Readable for PKA_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKA_SR to value 0"]
impl crate::Resettable for PKA_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
