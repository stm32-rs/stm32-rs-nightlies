///Register `FDCAN_TXBRP` reader
pub type R = crate::R<FDCAN_TXBRPrs>;
///Field `TRP` reader - Transmission Request Pending
pub type TRP_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Transmission Request Pending
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBRP")
            .field("trp", &self.trp())
            .finish()
    }
}
/**FDCAN Tx Buffer Request Pending Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#CAN_CCU:FDCAN_TXBRP)*/
pub struct FDCAN_TXBRPrs;
impl crate::RegisterSpec for FDCAN_TXBRPrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbrp::R`](R) reader structure
impl crate::Readable for FDCAN_TXBRPrs {}
///`reset()` method sets FDCAN_TXBRP to value 0
impl crate::Resettable for FDCAN_TXBRPrs {
    const RESET_VALUE: u32 = 0;
}
