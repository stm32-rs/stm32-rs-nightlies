///Register `RWD` reader
pub type R = crate::R<RWDrs>;
///Register `RWD` writer
pub type W = crate::W<RWDrs>;
///Field `WDC` reader - WDC
pub type WDC_R = crate::FieldReader;
///Field `WDC` writer - WDC
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WDV` reader - WDV
pub type WDV_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - WDC
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - WDV
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWD")
            .field("wdc", &self.wdc())
            .field("wdv", &self.wdv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - WDC
    #[inline(always)]
    pub fn wdc(&mut self) -> WDC_W<'_, RWDrs> {
        WDC_W::new(self, 0)
    }
}
/**The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access starts the Message RAM Watchdog Counter with the value configured by the RWD\[WDC\] bits. The counter is reloaded with RWD\[WDC\] bits when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to 0, the counter stops and interrupt flag IR\[WDI\] bit is set. The RAM Watchdog Counter is clocked by the fdcan_pclk clock.

You can [`read`](crate::Reg::read) this register and get [`rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FDCAN1:RWD)*/
pub struct RWDrs;
impl crate::RegisterSpec for RWDrs {
    type Ux = u32;
}
///`read()` method returns [`rwd::R`](R) reader structure
impl crate::Readable for RWDrs {}
///`write(|w| ..)` method takes [`rwd::W`](W) writer structure
impl crate::Writable for RWDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RWD to value 0
impl crate::Resettable for RWDrs {}
