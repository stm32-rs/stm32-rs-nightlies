///Register `TSCSDIFHALTR` writer
pub type W = crate::W<TSCSDIFHALTRrs>;
///Field `SDIF_STOP` writer - Serial data interface (SDIF) stop
pub type SDIF_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TSCSDIFHALTRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Serial data interface (SDIF) stop
    #[inline(always)]
    pub fn sdif_stop(&mut self) -> SDIF_STOP_W<'_, TSCSDIFHALTRrs> {
        SDIF_STOP_W::new(self, 0)
    }
}
/**DTS TSC SDIF halt register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscsdifhaltr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#DTS:TSCSDIFHALTR)*/
pub struct TSCSDIFHALTRrs;
impl crate::RegisterSpec for TSCSDIFHALTRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`tscsdifhaltr::W`](W) writer structure
impl crate::Writable for TSCSDIFHALTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TSCSDIFHALTR to value 0
impl crate::Resettable for TSCSDIFHALTRrs {}
