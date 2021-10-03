#[doc = "Register `FDCAN_TDCR` reader"]
pub struct R(crate::R<FDCAN_TDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TDCF` reader - Transmitter Delay Compensation Filter Window Length"]
pub struct TDCF_R(crate::FieldReader<u8, u8>);
impl TDCF_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCO` reader - Transmitter Delay Compensation Offset"]
pub struct TDCO_R(crate::FieldReader<u8, u8>);
impl TDCO_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length"]
    #[inline(always)]
    pub fn tdcf(&self) -> TDCF_R {
        TDCF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[doc = "FDCAN Transmitter Delay Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_tdcr](index.html) module"]
pub struct FDCAN_TDCR_SPEC;
impl crate::RegisterSpec for FDCAN_TDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_tdcr::R](R) reader structure"]
impl crate::Readable for FDCAN_TDCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TDCR to value 0"]
impl crate::Resettable for FDCAN_TDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
