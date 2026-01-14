///Register `RX_FPE_FRAG_CR` reader
pub type R = crate::R<RX_FPE_FRAG_CRrs>;
///Field `FFC` reader - Rx FPE Fragment Counter
pub type FFC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Rx FPE Fragment Counter
    #[inline(always)]
    pub fn ffc(&self) -> FFC_R {
        FFC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_FPE_FRAG_CR")
            .field("ffc", &self.ffc())
            .finish()
    }
}
/**MMC Rx FPE fragments counter register

You can [`read`](crate::Reg::read) this register and get [`rx_fpe_frag_cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:RX_FPE_FRAG_CR)*/
pub struct RX_FPE_FRAG_CRrs;
impl crate::RegisterSpec for RX_FPE_FRAG_CRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_fpe_frag_cr::R`](R) reader structure
impl crate::Readable for RX_FPE_FRAG_CRrs {}
///`reset()` method sets RX_FPE_FRAG_CR to value 0
impl crate::Resettable for RX_FPE_FRAG_CRrs {}
