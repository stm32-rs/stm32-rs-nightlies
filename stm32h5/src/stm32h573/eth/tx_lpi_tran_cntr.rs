#[doc = "Register `TX_LPI_TRAN_CNTR` reader"]
pub type R = crate::R<TX_LPI_TRAN_CNTRrs>;
#[doc = "Field `TXLPITRC` reader - Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred. Even if Tx LPI Entry occurs in Automate mode (because of LPITXA bit set in the LPI control and status register (ETH_MACLCSR)), the counter is incremented."]
pub type TXLPITRC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Tx LPI Transition counter This field indicates the number of times Tx LPI Entry has occurred. Even if Tx LPI Entry occurs in Automate mode (because of LPITXA bit set in the LPI control and status register (ETH_MACLCSR)), the counter is incremented."]
    #[inline(always)]
    pub fn txlpitrc(&self) -> TXLPITRC_R {
        TXLPITRC_R::new(self.bits)
    }
}
#[doc = "Tx LPI transition counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_lpi_tran_cntr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_LPI_TRAN_CNTRrs;
impl crate::RegisterSpec for TX_LPI_TRAN_CNTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_lpi_tran_cntr::R`](R) reader structure"]
impl crate::Readable for TX_LPI_TRAN_CNTRrs {}
#[doc = "`reset()` method sets TX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for TX_LPI_TRAN_CNTRrs {
    const RESET_VALUE: u32 = 0;
}
