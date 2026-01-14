///Register `RX_LPI_TRAN_CNTR` reader
pub type R = crate::R<RX_LPI_TRAN_CNTRrs>;
///Field `RXLPITRC` reader - Rx LPI Transition counter
pub type RXLPITRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx LPI Transition counter
    #[inline(always)]
    pub fn rxlpitrc(&self) -> RXLPITRC_R {
        RXLPITRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_LPI_TRAN_CNTR")
            .field("rxlpitrc", &self.rxlpitrc())
            .finish()
    }
}
/**Rx LPI transition counter register

You can [`read`](crate::Reg::read) this register and get [`rx_lpi_tran_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#Ethernet_MAC:RX_LPI_TRAN_CNTR)*/
pub struct RX_LPI_TRAN_CNTRrs;
impl crate::RegisterSpec for RX_LPI_TRAN_CNTRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_lpi_tran_cntr::R`](R) reader structure
impl crate::Readable for RX_LPI_TRAN_CNTRrs {}
///`reset()` method sets RX_LPI_TRAN_CNTR to value 0
impl crate::Resettable for RX_LPI_TRAN_CNTRrs {}
