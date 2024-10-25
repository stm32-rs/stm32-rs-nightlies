///Register `FDCAN_TXBCIE` reader
pub type R = crate::R<FDCAN_TXBCIErs>;
///Register `FDCAN_TXBCIE` writer
pub type W = crate::W<FDCAN_TXBCIErs>;
///Field `CF` reader - Cancellation Finished Interrupt Enable
pub type CF_R = crate::FieldReader<u32>;
///Field `CF` writer - Cancellation Finished Interrupt Enable
pub type CF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Cancellation Finished Interrupt Enable
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXBCIE")
            .field("cf", &self.cf())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Cancellation Finished Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CF_W<FDCAN_TXBCIErs> {
        CF_W::new(self, 0)
    }
}
/**FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3x.html#CAN_CCU:FDCAN_TXBCIE)*/
pub struct FDCAN_TXBCIErs;
impl crate::RegisterSpec for FDCAN_TXBCIErs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txbcie::R`](R) reader structure
impl crate::Readable for FDCAN_TXBCIErs {}
///`write(|w| ..)` method takes [`fdcan_txbcie::W`](W) writer structure
impl crate::Writable for FDCAN_TXBCIErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TXBCIE to value 0
impl crate::Resettable for FDCAN_TXBCIErs {
    const RESET_VALUE: u32 = 0;
}
