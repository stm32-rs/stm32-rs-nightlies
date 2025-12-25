///Register `LPTR` reader
pub type R = crate::R<LPTRrs>;
///Register `LPTR` writer
pub type W = crate::W<LPTRrs>;
///Field `TIMEOUT` reader - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0.
pub type TIMEOUT_R = crate::FieldReader<u16>;
///Field `TIMEOUT` writer - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0.
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:15 - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPTR")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timeout period After each access in memory-mapped mode, the QUADSPI prefetches the subsequent bytes and holds these bytes in the FIFO. This field indicates how many CLK cycles the QUADSPI waits after the FIFO becomes full until it raises nCS, putting the Flash memory in a lower-consumption state. This field can be written only when BUSY = 0.
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<'_, LPTRrs> {
        TIMEOUT_W::new(self, 0)
    }
}
/**QUADSPI low-power timeout register

You can [`read`](crate::Reg::read) this register and get [`lptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743V.html#QUADSPI:LPTR)*/
pub struct LPTRrs;
impl crate::RegisterSpec for LPTRrs {
    type Ux = u32;
}
///`read()` method returns [`lptr::R`](R) reader structure
impl crate::Readable for LPTRrs {}
///`write(|w| ..)` method takes [`lptr::W`](W) writer structure
impl crate::Writable for LPTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPTR to value 0
impl crate::Resettable for LPTRrs {}
