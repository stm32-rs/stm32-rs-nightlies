#[doc = "Register `MACISR` reader"]
pub struct R(crate::R<MACISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PHYIS` reader - PHY Interrupt"]
pub struct PHYIS_R(crate::FieldReader<bool, bool>);
impl PHYIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PHYIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMTIS` reader - PMT Interrupt Status"]
pub struct PMTIS_R(crate::FieldReader<bool, bool>);
impl PMTIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMTIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIIS` reader - LPI Interrupt Status"]
pub struct LPIIS_R(crate::FieldReader<bool, bool>);
impl LPIIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPIIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCIS` reader - MMC Interrupt Status"]
pub struct MMCIS_R(crate::FieldReader<bool, bool>);
impl MMCIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCRXIS` reader - MMC Receive Interrupt Status"]
pub struct MMCRXIS_R(crate::FieldReader<bool, bool>);
impl MMCRXIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCRXIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCRXIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMCTXIS` reader - MMC Transmit Interrupt Status"]
pub struct MMCTXIS_R(crate::FieldReader<bool, bool>);
impl MMCTXIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMCTXIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MMCTXIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIS` reader - Timestamp Interrupt Status"]
pub struct TSIS_R(crate::FieldReader<bool, bool>);
impl TSIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTSIS` reader - Transmit Status Interrupt"]
pub struct TXSTSIS_R(crate::FieldReader<bool, bool>);
impl TXSTSIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTSIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTSIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTSIS` reader - Receive Status Interrupt"]
pub struct RXSTSIS_R(crate::FieldReader<bool, bool>);
impl RXSTSIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTSIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTSIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - PHY Interrupt"]
    #[inline(always)]
    pub fn phyis(&self) -> PHYIS_R {
        PHYIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn pmtis(&self) -> PMTIS_R {
        PMTIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Status"]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn mmcis(&self) -> MMCIS_R {
        MMCIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MMCRXIS_R {
        MMCRXIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MMCTXIS_R {
        MMCTXIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Status"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt"]
    #[inline(always)]
    pub fn txstsis(&self) -> TXSTSIS_R {
        TXSTSIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt"]
    #[inline(always)]
    pub fn rxstsis(&self) -> RXSTSIS_R {
        RXSTSIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macisr](index.html) module"]
pub struct MACISR_SPEC;
impl crate::RegisterSpec for MACISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macisr::R](R) reader structure"]
impl crate::Readable for MACISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MACISR to value 0"]
impl crate::Resettable for MACISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
