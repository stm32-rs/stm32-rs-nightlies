///Register `DMACRxIWTR` reader
pub type R = crate::R<DMACRX_IWTRrs>;
///Register `DMACRxIWTR` writer
pub type W = crate::W<DMACRX_IWTRrs>;
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
        f.debug_struct("DMACRxIWTR")
            .field("rwt", &self.rwt())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Receive Interrupt Watchdog Timer Count
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W<'_, DMACRX_IWTRrs> {
        RWT_W::new(self, 0)
    }
}
/**Channel Rx interrupt watchdog timer register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_iwtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#Ethernet_DMA:DMACRxIWTR)*/
pub struct DMACRX_IWTRrs;
impl crate::RegisterSpec for DMACRX_IWTRrs {
    type Ux = u32;
}
///`read()` method returns [`dmacrx_iwtr::R`](R) reader structure
impl crate::Readable for DMACRX_IWTRrs {}
///`write(|w| ..)` method takes [`dmacrx_iwtr::W`](W) writer structure
impl crate::Writable for DMACRX_IWTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACRxIWTR to value 0
impl crate::Resettable for DMACRX_IWTRrs {}
