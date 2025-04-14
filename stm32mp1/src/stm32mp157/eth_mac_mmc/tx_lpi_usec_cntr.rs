///Register `TX_LPI_USEC_CNTR` reader
pub type R = crate::R<TX_LPI_USEC_CNTRrs>;
///Field `TXLPIUSC` reader - TXLPIUSC
pub type TXLPIUSC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - TXLPIUSC
    #[inline(always)]
    pub fn txlpiusc(&self) -> TXLPIUSC_R {
        TXLPIUSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_LPI_USEC_CNTR")
            .field("txlpiusc", &self.txlpiusc())
            .finish()
    }
}
/**This register provides the number of microseconds Tx LPI is asserted by Ethernet peripheral.

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_usec_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_MAC_MMC:TX_LPI_USEC_CNTR)*/
pub struct TX_LPI_USEC_CNTRrs;
impl crate::RegisterSpec for TX_LPI_USEC_CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`tx_lpi_usec_cntr::R`](R) reader structure
impl crate::Readable for TX_LPI_USEC_CNTRrs {}
///`reset()` method sets TX_LPI_USEC_CNTR to value 0
impl crate::Resettable for TX_LPI_USEC_CNTRrs {}
