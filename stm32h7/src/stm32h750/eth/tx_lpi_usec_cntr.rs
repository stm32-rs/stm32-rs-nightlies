///Register `TX_LPI_USEC_CNTR` reader
pub type R = crate::R<TX_LPI_USEC_CNTRrs>;
///Field `TXLPIUSC` reader - Tx LPI Microseconds Counter This field indicates the number of microseconds Tx LPI is asserted. For every Tx LPI Entry and Exit, the Timer value can have an error of +/- 1 microsecond.
pub type TXLPIUSC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Tx LPI Microseconds Counter This field indicates the number of microseconds Tx LPI is asserted. For every Tx LPI Entry and Exit, the Timer value can have an error of +/- 1 microsecond.
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
/**Tx LPI microsecond timer register

You can [`read`](crate::Reg::read) this register and get [`tx_lpi_usec_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H750.html#ETH:TX_LPI_USEC_CNTR)*/
pub struct TX_LPI_USEC_CNTRrs;
impl crate::RegisterSpec for TX_LPI_USEC_CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`tx_lpi_usec_cntr::R`](R) reader structure
impl crate::Readable for TX_LPI_USEC_CNTRrs {}
///`reset()` method sets TX_LPI_USEC_CNTR to value 0
impl crate::Resettable for TX_LPI_USEC_CNTRrs {}
