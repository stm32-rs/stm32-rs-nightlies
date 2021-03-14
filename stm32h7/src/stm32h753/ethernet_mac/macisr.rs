#[doc = "Reader of register MACISR"]
pub type R = crate::R<u32, super::MACISR>;
#[doc = "Reader of field `PHYIS`"]
pub type PHYIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `PMTIS`"]
pub type PMTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPIIS`"]
pub type LPIIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCIS`"]
pub type MMCIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCRXIS`"]
pub type MMCRXIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMCTXIS`"]
pub type MMCTXIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSIS`"]
pub type TSIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTSIS`"]
pub type TXSTSIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSTSIS`"]
pub type RXSTSIS_R = crate::R<bool, bool>;
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
