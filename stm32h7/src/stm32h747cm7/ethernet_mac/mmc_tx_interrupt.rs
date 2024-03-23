#[doc = "Register `MMC_TX_INTERRUPT` reader"]
pub type R = crate::R<MMC_TX_INTERRUPTrs>;
#[doc = "Field `TXSCOLGPIS` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Status"]
pub type TXSCOLGPIS_R = crate::BitReader;
#[doc = "Field `TXMCOLGPIS` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status"]
pub type TXMCOLGPIS_R = crate::BitReader;
#[doc = "Field `TXGPKTIS` reader - MMC Transmit Good Packet Counter Interrupt Status"]
pub type TXGPKTIS_R = crate::BitReader;
#[doc = "Field `TXLPIUSCIS` reader - MMC Transmit LPI microsecond counter interrupt status"]
pub type TXLPIUSCIS_R = crate::BitReader;
#[doc = "Field `TXLPITRCIS` reader - MMC Transmit LPI transition counter interrupt status"]
pub type TXLPITRCIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Status"]
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt status"]
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt status"]
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "MMC Tx interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_tx_interrupt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_TX_INTERRUPTrs;
impl crate::RegisterSpec for MMC_TX_INTERRUPTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_interrupt::R`](R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPTrs {}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT to value 0"]
impl crate::Resettable for MMC_TX_INTERRUPTrs {
    const RESET_VALUE: u32 = 0;
}
