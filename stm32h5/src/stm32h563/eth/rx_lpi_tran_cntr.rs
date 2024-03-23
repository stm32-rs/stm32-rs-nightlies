#[doc = "Register `RX_LPI_TRAN_CNTR` reader"]
pub type R = crate::R<RX_LPI_TRAN_CNTRrs>;
#[doc = "Field `RXLPITRC` reader - Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred."]
pub type RXLPITRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Rx LPI Transition counter This field indicates the number of times Rx LPI Entry has occurred."]
    #[inline(always)]
    pub fn rxlpitrc(&self) -> RXLPITRC_R {
        RXLPITRC_R::new(self.bits)
    }
}
#[doc = "Rx LPI transition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_lpi_tran_cntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_LPI_TRAN_CNTRrs;
impl crate::RegisterSpec for RX_LPI_TRAN_CNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_lpi_tran_cntr::R`](R) reader structure"]
impl crate::Readable for RX_LPI_TRAN_CNTRrs {}
#[doc = "`reset()` method sets RX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for RX_LPI_TRAN_CNTRrs {
    const RESET_VALUE: u32 = 0;
}
