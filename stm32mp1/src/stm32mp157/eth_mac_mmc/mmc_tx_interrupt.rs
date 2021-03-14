#[doc = "Reader of register MMC_TX_INTERRUPT"]
pub type R = crate::R<u32, super::MMC_TX_INTERRUPT>;
#[doc = "Reader of field `TXSCOLGPIS`"]
pub type TXSCOLGPIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMCOLGPIS`"]
pub type TXMCOLGPIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXGPKTIS`"]
pub type TXGPKTIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXLPIUSCIS`"]
pub type TXLPIUSCIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXLPITRCIS`"]
pub type TXLPITRCIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 14 - TXSCOLGPIS"]
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TXMCOLGPIS"]
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TXGPKTIS"]
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - TXLPIUSCIS"]
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - TXLPITRCIS"]
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
