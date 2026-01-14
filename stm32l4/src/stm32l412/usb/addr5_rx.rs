///Register `ADDR5_RX` reader
pub type R = crate::R<ADDR5_RXrs>;
///Register `ADDR5_RX` writer
pub type W = crate::W<ADDR5_RXrs>;
///Field `ADDR5_RX` reader - Reception buffer address
pub type ADDR5_RX_R = crate::FieldReader<u16>;
///Field `ADDR5_RX` writer - Reception buffer address
pub type ADDR5_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr5_rx(&self) -> ADDR5_RX_R {
        ADDR5_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR5_RX")
            .field("addr5_rx", &self.addr5_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr5_rx(&mut self) -> ADDR5_RX_W<'_, ADDR5_RXrs> {
        ADDR5_RX_W::new(self, 1)
    }
}
/**Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr5_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr5_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR5_RX)*/
pub struct ADDR5_RXrs;
impl crate::RegisterSpec for ADDR5_RXrs {
    type Ux = u16;
}
///`read()` method returns [`addr5_rx::R`](R) reader structure
impl crate::Readable for ADDR5_RXrs {}
///`write(|w| ..)` method takes [`addr5_rx::W`](W) writer structure
impl crate::Writable for ADDR5_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR5_RX to value 0
impl crate::Resettable for ADDR5_RXrs {}
