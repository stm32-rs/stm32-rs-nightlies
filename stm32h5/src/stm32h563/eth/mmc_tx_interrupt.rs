///Register `MMC_TX_INTERRUPT` reader
pub type R = crate::R<MMC_TX_INTERRUPTrs>;
///Register `MMC_TX_INTERRUPT` writer
pub type W = crate::W<MMC_TX_INTERRUPTrs>;
/**Field `TXSCOLGPIS` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXSCOLGPIS_R = crate::BitReader;
///Field `TXSCOLGPIS` writer - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.
pub type TXSCOLGPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TXMCOLGPIS` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXMCOLGPIS_R = crate::BitReader;
///Field `TXMCOLGPIS` writer - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.
pub type TXMCOLGPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TXGPKTIS` reader - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXGPKTIS_R = crate::BitReader;
///Field `TXGPKTIS` writer - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value.
pub type TXGPKTIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TXLPIUSCIS` reader - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXLPIUSCIS_R = crate::BitReader;
///Field `TXLPIUSCIS` writer - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.
pub type TXLPIUSCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `TXLPITRCIS` reader - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.

<div class="warning">The field is <b>cleared</b> (set to zero) following a read operation.</div>*/
pub type TXLPITRCIS_R = crate::BitReader;
///Field `TXLPITRCIS` writer - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.
pub type TXLPITRCIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txscolgpis(&self) -> TXSCOLGPIS_R {
        TXSCOLGPIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TXMCOLGPIS_R {
        TXMCOLGPIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txgpktis(&self) -> TXGPKTIS_R {
        TXGPKTIS_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 26 - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TXLPIUSCIS_R {
        TXLPIUSCIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TXLPITRCIS_R {
        TXLPITRCIS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_TX_INTERRUPT").finish()
    }
}
impl W {
    ///Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Status This bit is set when the Tx single collision good packets register (ETH_TX_SINGLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txscolgpis(&mut self) -> TXSCOLGPIS_W<'_, MMC_TX_INTERRUPTrs> {
        TXSCOLGPIS_W::new(self, 14)
    }
    ///Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Status This bit is set when the Tx multiple collision good packets register (ETH_TX_MULTIPLE_COLLISION_GOOD_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txmcolgpis(&mut self) -> TXMCOLGPIS_W<'_, MMC_TX_INTERRUPTrs> {
        TXMCOLGPIS_W::new(self, 15)
    }
    ///Bit 21 - MMC Transmit Good Packet Counter Interrupt Status This bit is set when the Tx packet count good register (ETH_TX_PACKET_COUNT_GOOD) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txgpktis(&mut self) -> TXGPKTIS_W<'_, MMC_TX_INTERRUPTrs> {
        TXGPKTIS_W::new(self, 21)
    }
    ///Bit 26 - MMC Transmit LPI microsecond counter interrupt status This bit is set when the Tx LPI microsecond timer register (ETH_TX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txlpiuscis(&mut self) -> TXLPIUSCIS_W<'_, MMC_TX_INTERRUPTrs> {
        TXLPIUSCIS_W::new(self, 26)
    }
    ///Bit 27 - MMC Transmit LPI transition counter interrupt status This bit is set when the Tx LPI transition counter register (ETH_TX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn txlpitrcis(&mut self) -> TXLPITRCIS_W<'_, MMC_TX_INTERRUPTrs> {
        TXLPITRCIS_W::new(self, 27)
    }
}
/**MMC Tx interrupt register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#ETH:MMC_TX_INTERRUPT)*/
pub struct MMC_TX_INTERRUPTrs;
impl crate::RegisterSpec for MMC_TX_INTERRUPTrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_tx_interrupt::R`](R) reader structure
impl crate::Readable for MMC_TX_INTERRUPTrs {}
///`write(|w| ..)` method takes [`mmc_tx_interrupt::W`](W) writer structure
impl crate::Writable for MMC_TX_INTERRUPTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MMC_TX_INTERRUPT to value 0
impl crate::Resettable for MMC_TX_INTERRUPTrs {}
