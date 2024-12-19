///Register `FDCAN_TXBTO` reader
pub type R = crate::R<FDCAN_TXBTOrs>;
///Field `TO` reader - Transmission Occurred.
pub type TO_R = crate::FieldReader;
impl R {
    ///Bits 0:2 - Transmission Occurred.
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBTO")
            .field("to", &self.to())
            .finish()
    }
}
/**FDCAN Tx Buffer Transmission Occurred Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbto::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#FDCAN1:FDCAN_TXBTO)*/
pub struct FDCAN_TXBTOrs;
impl crate::RegisterSpec for FDCAN_TXBTOrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbto::R`](R) reader structure
impl crate::Readable for FDCAN_TXBTOrs {}
///`reset()` method sets FDCAN_TXBTO to value 0
impl crate::Resettable for FDCAN_TXBTOrs {
    const RESET_VALUE: u32 = 0;
}
