///Register `DMAC0RxIWTR` reader
pub type R = crate::R<DMAC0RX_IWTRrs>;
///Register `DMAC0RxIWTR` writer
pub type W = crate::W<DMAC0RX_IWTRrs>;
///Field `RWT` reader - Receive Interrupt Watchdog Timer Count
pub type RWT_R = crate::FieldReader;
///Field `RWT` writer - Receive Interrupt Watchdog Timer Count
pub type RWT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0RxIWTR")
            .field("rwt", &self.rwt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<'_, DMAC0RX_IWTRrs> {
        RWT_W::new(self, 0)
    }
}
/**Channel Rx interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_iwtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_iwtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#ETH_DMA:DMAC0RxIWTR)*/
pub struct DMAC0RX_IWTRrs;
impl crate::RegisterSpec for DMAC0RX_IWTRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0rx_iwtr::R`](R) reader structure
impl crate::Readable for DMAC0RX_IWTRrs {}
///`write(|w| ..)` method takes [`dmac0rx_iwtr::W`](W) writer structure
impl crate::Writable for DMAC0RX_IWTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0RxIWTR to value 0
impl crate::Resettable for DMAC0RX_IWTRrs {}
