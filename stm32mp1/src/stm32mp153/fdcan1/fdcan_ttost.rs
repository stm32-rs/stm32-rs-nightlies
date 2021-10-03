#[doc = "Register `FDCAN_TTOST` reader"]
pub struct R(crate::R<FDCAN_TTOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EL` reader - EL"]
pub struct EL_R(crate::FieldReader<u8, u8>);
impl EL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MS` reader - MS"]
pub struct MS_R(crate::FieldReader<u8, u8>);
impl MS_R {
    pub(crate) fn new(bits: u8) -> Self {
        MS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS` reader - SYS"]
pub struct SYS_R(crate::FieldReader<u8, u8>);
impl SYS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QGTP` reader - QGTP"]
pub struct QGTP_R(crate::FieldReader<bool, bool>);
impl QGTP_R {
    pub(crate) fn new(bits: bool) -> Self {
        QGTP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QGTP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QCS` reader - QCS"]
pub struct QCS_R(crate::FieldReader<bool, bool>);
impl QCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        QCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTO` reader - RTO"]
pub struct RTO_R(crate::FieldReader<u8, u8>);
impl RTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WGTD` reader - WGTD"]
pub struct WGTD_R(crate::FieldReader<bool, bool>);
impl WGTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WGTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WGTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFI` reader - GFI"]
pub struct GFI_R(crate::FieldReader<bool, bool>);
impl GFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMP` reader - TMP"]
pub struct TMP_R(crate::FieldReader<u8, u8>);
impl TMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GSI` reader - GSI"]
pub struct GSI_R(crate::FieldReader<bool, bool>);
impl GSI_R {
    pub(crate) fn new(bits: bool) -> Self {
        GSI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GSI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFE` reader - WFE"]
pub struct WFE_R(crate::FieldReader<bool, bool>);
impl WFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWE` reader - AWE"]
pub struct AWE_R(crate::FieldReader<bool, bool>);
impl AWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WECS` reader - WECS"]
pub struct WECS_R(crate::FieldReader<bool, bool>);
impl WECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPL` reader - SPL"]
pub struct SPL_R(crate::FieldReader<bool, bool>);
impl SPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - EL"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - MS"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - SYS"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - QGTP"]
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - QCS"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - RTO"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - WGTD"]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - GFI"]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - TMP"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - GSI"]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WFE"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - AWE"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WECS"]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SPL"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "FDCAN TT operation status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttost](index.html) module"]
pub struct FDCAN_TTOST_SPEC;
impl crate::RegisterSpec for FDCAN_TTOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttost::R](R) reader structure"]
impl crate::Readable for FDCAN_TTOST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_TTOST to value 0x80"]
impl crate::Resettable for FDCAN_TTOST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
