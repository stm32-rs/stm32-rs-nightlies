///Register `MMC_TX_INTERRUPT` reader
pub type R = crate::R<MMC_TX_INTERRUPTrs>;
///Field `TXSCOLGPIS` reader - TXSCOLGPIS
pub type TXSCOLGPIS_R = crate::BitReader;
///Field `TXMCOLGPIS` reader - TXMCOLGPIS
pub type TXMCOLGPIS_R = crate::BitReader;
///Field `TXGPKTIS` reader - TXGPKTIS
pub type TXGPKTIS_R = crate::BitReader;
///Field `TXLPIUSCIS` reader - TXLPIUSCIS
pub type TXLPIUSCIS_R = crate::BitReader;
///Field `TXLPITRCIS` reader - TXLPITRCIS
pub type TXLPITRCIS_R = crate::BitReader;
impl R {
    ///Bit 14 - TXSCOLGPIS
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TXMCOLGPIS
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - TXGPKTIS
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 26 - TXLPIUSCIS
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXLPITRCIS
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_TX_INTERRUPT")
            .field("txscolgpis", &self.txscolgpis())
            .field("txmcolgpis", &self.txmcolgpis())
            .field("txgpktis", &self.txgpktis())
            .field("txlpiuscis", &self.txlpiuscis())
            .field("txlpitrcis", &self.txlpitrcis())
            .finish()
    }
}
/**This register maintains the interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt register maintains the interrupts generated when transmit statistic counters reach half their maximum values (0x8000_0000 for 32 bit counter and 0x8000 for 16 bit counter), and when they cross their maximum values (0xFFFF_FFFF for 32-bit counter and 0xFFFF for 16-bit counter). When Counter Stop Rollover is set, the interrupts are set but the counter remains at all-ones. The MMC Transmit Interrupt register is a 32 bit register. An interrupt bit is cleared when the respective MMC counter that caused the interrupt is read. The least significant byte lane (Bits\[7:0\]) of the respective counter must be read to clear the interrupt bit.

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:MMC_TX_INTERRUPT)*/
pub struct MMC_TX_INTERRUPTrs;
impl crate::RegisterSpec for MMC_TX_INTERRUPTrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_tx_interrupt::R`](R) reader structure
impl crate::Readable for MMC_TX_INTERRUPTrs {}
///`reset()` method sets MMC_TX_INTERRUPT to value 0
impl crate::Resettable for MMC_TX_INTERRUPTrs {}
