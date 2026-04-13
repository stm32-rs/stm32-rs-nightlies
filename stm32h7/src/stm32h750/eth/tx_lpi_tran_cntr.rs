///Register `TX_LPI_TRAN_CNTR` reader
pub type R = crate::R<TX_LPI_TRAN_CNTRrs>;
///Field `TXLPITRC` reader - Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred. Even if Tx LPI Entry occurs in Automate mode (because of LPITXA bit set in the (ETH_MACLCSR)), the counter will increment.
pub type TXLPITRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred. Even if Tx LPI Entry occurs in Automate mode (because of LPITXA bit set in the (ETH_MACLCSR)), the counter will increment.
    #[inline(always)]
    pub fn txlpitrc(&self) -> TXLPITRC_R {
        TXLPITRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_LPI_TRAN_CNTR")
            .field("txlpitrc", &self.txlpitrc())
            .finish()
    }
}
/**Tx LPI transition counter register

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_tran_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:TX_LPI_TRAN_CNTR)*/
pub struct TX_LPI_TRAN_CNTRrs;
impl crate::RegisterSpec for TX_LPI_TRAN_CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`tx_lpi_tran_cntr::R`](R) reader structure
impl crate::Readable for TX_LPI_TRAN_CNTRrs {}
///`reset()` method sets TX_LPI_TRAN_CNTR to value 0
impl crate::Resettable for TX_LPI_TRAN_CNTRrs {}
