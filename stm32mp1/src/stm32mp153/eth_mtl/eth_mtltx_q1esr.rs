///Register `ETH_MTLTxQ1ESR` reader
pub type R = crate::R<ETH_MTLTX_Q1ESRrs>;
///Field `ABS` reader - ABS
pub type ABS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - ABS
    #[inline(always)]
    pub fn abs(&self) -> ABS_R {
        ABS_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETH_MTLTxQ1ESR")
            .field("abs", &self.abs())
            .finish()
    }
}
/**Tx queue x ETS status Register

You can [`read`](crate::Reg::read) this register and get [`eth_mtltx_q1esr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_MTL:ETH_MTLTxQ1ESR)*/
pub struct ETH_MTLTX_Q1ESRrs;
impl crate::RegisterSpec for ETH_MTLTX_Q1ESRrs {
    type Ux = u32;
}
///`read()` method returns [`eth_mtltx_q1esr::R`](R) reader structure
impl crate::Readable for ETH_MTLTX_Q1ESRrs {}
///`reset()` method sets ETH_MTLTxQ1ESR to value 0
impl crate::Resettable for ETH_MTLTX_Q1ESRrs {
    const RESET_VALUE: u32 = 0;
}
