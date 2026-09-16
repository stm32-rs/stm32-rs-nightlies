///Register `MMC_FPE_TX_FCR` reader
pub type R = crate::R<MMC_FPE_TX_FCRrs>;
///Field `TXFFC` reader - Tx FPE Fragment counter
pub type TXFFC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Tx FPE Fragment counter
    #[inline(always)]
    pub fn txffc(&self) -> TXFFC_R {
        TXFFC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MMC_FPE_TX_FCR")
            .field("txffc", &self.txffc())
            .finish()
    }
}
/**MMC FPE Tx fragment counter register

You can [`read`](crate::Reg::read) this register and get [`mmc_fpe_tx_fcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MMC_FPE_TX_FCR)*/
pub struct MMC_FPE_TX_FCRrs;
impl crate::RegisterSpec for MMC_FPE_TX_FCRrs {
    type Ux = u32;
}
///`read()` method returns [`mmc_fpe_tx_fcr::R`](R) reader structure
impl crate::Readable for MMC_FPE_TX_FCRrs {}
///`reset()` method sets MMC_FPE_TX_FCR to value 0
impl crate::Resettable for MMC_FPE_TX_FCRrs {}
