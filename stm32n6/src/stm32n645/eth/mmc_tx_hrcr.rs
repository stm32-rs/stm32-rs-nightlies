///Register `MMC_TX_HRCR` reader
pub type R = crate::R<MMC_TX_HRCRrs>;
///Field `TXHRC` reader - Tx Hold Request Counter
pub type TXHRC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Tx Hold Request Counter
    #[inline(always)]
    pub fn txhrc(&self) -> TXHRC_R {
        TXHRC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_TX_HRCR")
            .field("txhrc", &self.txhrc())
            .finish()
    }
}
/**MMC Tx hold request counter register

You can [`read`](crate::Reg::read) this register and get [`mmc_tx_hrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MMC_TX_HRCR)*/
pub struct MMC_TX_HRCRrs;
impl crate::RegisterSpec for MMC_TX_HRCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_tx_hrcr::R`](R) reader structure
impl crate::Readable for MMC_TX_HRCRrs {}
///`reset()` method sets MMC_TX_HRCR to value 0
impl crate::Resettable for MMC_TX_HRCRrs {}
