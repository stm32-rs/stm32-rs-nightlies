#[doc = "Reader of register MMC_RX_INTERRUPT"]
pub type R = crate::R<u32, super::MMC_RX_INTERRUPT>;
#[doc = "Reader of field `RXCRCERPIS`"]
pub type RXCRCERPIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXALGNERPIS`"]
pub type RXALGNERPIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUCGPIS`"]
pub type RXUCGPIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXLPIUSCIS`"]
pub type RXLPIUSCIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXLPITRCIS`"]
pub type RXLPITRCIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RXCRCERPIS_R {
        RXCRCERPIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RXALGNERPIS_R {
        RXALGNERPIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RXUCGPIS_R {
        RXUCGPIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt status"]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RXLPIUSCIS_R {
        RXLPIUSCIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Receive LPI transition counter interrupt status"]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RXLPITRCIS_R {
        RXLPITRCIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
