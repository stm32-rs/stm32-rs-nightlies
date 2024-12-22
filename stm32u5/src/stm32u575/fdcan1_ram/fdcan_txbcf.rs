///Register `FDCAN_TXBCF` reader
pub type R = crate::R<FDCAN_TXBCFrs>;
///Field `CF` reader - Cancellation Finished
pub type CF_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Cancellation Finished
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCF")
            .field("cf", &self.cf())
            .finish()
    }
}
/**FDCAN Tx Buffer Cancellation Finished Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FDCAN1_RAM:FDCAN_TXBCF)*/
pub struct FDCAN_TXBCFrs;
impl crate::RegisterSpec for FDCAN_TXBCFrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbcf::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCFrs {}
///`reset()` method sets FDCAN_TXBCF to value 0
impl crate::Resettable for FDCAN_TXBCFrs {
    const RESET_VALUE: u32 = 0;
}
