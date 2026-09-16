///Register `RX_LPI_USEC_CNTR` reader
pub type R = crate::R<RX_LPI_USEC_CNTRrs>;
///Field `RXLPIUSC` reader - Rx LPI Microseconds Counter
pub type RXLPIUSC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx LPI Microseconds Counter
    #[inline(always)]
    pub fn rxlpiusc(&self) -> RXLPIUSC_R {
        RXLPIUSC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_LPI_USEC_CNTR")
            .field("rxlpiusc", &self.rxlpiusc())
            .finish()
    }
}
/**Rx LPI microsecond counter register

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_usec_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:RX_LPI_USEC_CNTR)*/
pub struct RX_LPI_USEC_CNTRrs;
impl crate::RegisterSpec for RX_LPI_USEC_CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_lpi_usec_cntr::R`](R) reader structure
impl crate::Readable for RX_LPI_USEC_CNTRrs {}
///`reset()` method sets RX_LPI_USEC_CNTR to value 0
impl crate::Resettable for RX_LPI_USEC_CNTRrs {}
