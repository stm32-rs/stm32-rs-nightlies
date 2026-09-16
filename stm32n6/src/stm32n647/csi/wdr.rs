///Register `WDR` reader
pub type R = crate::R<WDRrs>;
///Register `WDR` writer
pub type W = crate::W<WDRrs>;
///Field `CNT` reader - Watchdog counter
pub type CNT_R = crate::FieldReader<u32>;
///Field `CNT` writer - Watchdog counter
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Watchdog counter
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDR").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    ///Bits 0:31 - Watchdog counter
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, WDRrs> {
        CNT_W::new(self, 0)
    }
}
/**CSI-2 Host watchdog register

You can [`read`](crate::Reg::read) this register and get [`wdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CSI:WDR)*/
pub struct WDRrs;
impl crate::RegisterSpec for WDRrs {
    type Ux = u32;
}
///`read()` method returns [`wdr::R`](R) reader structure
impl crate::Readable for WDRrs {}
///`write(|w| ..)` method takes [`wdr::W`](W) writer structure
impl crate::Writable for WDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WDR to value 0
impl crate::Resettable for WDRrs {}
