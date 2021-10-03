#[doc = "Register `FDCAN_DBTP` reader"]
pub struct R(crate::R<FDCAN_DBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_DBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_DBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_DBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DSJW` reader - Synchronization Jump Width"]
pub struct DSJW_R(crate::FieldReader<u8, u8>);
impl DSJW_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSEG2` reader - Data time segment after sample point"]
pub struct DTSEG2_R(crate::FieldReader<u8, u8>);
impl DTSEG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSEG1` reader - Data time segment after sample point"]
pub struct DTSEG1_R(crate::FieldReader<u8, u8>);
impl DTSEG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBRP` reader - Data BIt Rate Prescaler"]
pub struct DBRP_R(crate::FieldReader<u8, u8>);
impl DBRP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC` reader - Transceiver Delay Compensation"]
pub struct TDC_R(crate::FieldReader<bool, bool>);
impl TDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data BIt Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "FDCAN Data Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_dbtp](index.html) module"]
pub struct FDCAN_DBTP_SPEC;
impl crate::RegisterSpec for FDCAN_DBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_dbtp::R](R) reader structure"]
impl crate::Readable for FDCAN_DBTP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_DBTP to value 0x0a33"]
impl crate::Resettable for FDCAN_DBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a33
    }
}
