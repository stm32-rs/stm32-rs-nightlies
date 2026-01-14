///Register `CMDREG` writer
pub type W = crate::W<CMDREGrs>;
///Field `TXRXSKIP` writer - Transmission/Reception skip command.
pub type TXRXSKIP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEARSEMAREQ` writer - Semaphore Clear command.
pub type CLEARSEMAREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDREGrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Transmission/Reception skip command.
    #[inline(always)]
    pub fn txrxskip(&mut self) -> TXRXSKIP_W<'_, CMDREGrs> {
        TXRXSKIP_W::new(self, 0)
    }
    ///Bit 3 - Semaphore Clear command.
    #[inline(always)]
    pub fn clearsemareq(&mut self) -> CLEARSEMAREQ_W<'_, CMDREGrs> {
        CLEARSEMAREQ_W::new(self, 3)
    }
}
/**CMDREG register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdreg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#BLUE:CMDREG)*/
pub struct CMDREGrs;
impl crate::RegisterSpec for CMDREGrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmdreg::W`](W) writer structure
impl crate::Writable for CMDREGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CMDREG to value 0
impl crate::Resettable for CMDREGrs {}
