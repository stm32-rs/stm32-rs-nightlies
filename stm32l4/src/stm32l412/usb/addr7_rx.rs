///Register `ADDR7_RX` reader
pub type R = crate::R<ADDR7_RXrs>;
///Register `ADDR7_RX` writer
pub type W = crate::W<ADDR7_RXrs>;
///Field `ADDR7_RX` reader - Reception buffer address
pub type ADDR7_RX_R = crate::FieldReader<u16>;
///Field `ADDR7_RX` writer - Reception buffer address
pub type ADDR7_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr7_rx(&self) -> ADDR7_RX_R {
        ADDR7_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR7_RX")
            .field("addr7_rx", &self.addr7_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr7_rx(&mut self) -> ADDR7_RX_W<ADDR7_RXrs> {
        ADDR7_RX_W::new(self, 1)
    }
}
/**Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr7_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr7_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR7_RX)*/
pub struct ADDR7_RXrs;
impl crate::RegisterSpec for ADDR7_RXrs {
    type Ux = u16;
}
///`read()` method returns [`addr7_rx::R`](R) reader structure
impl crate::Readable for ADDR7_RXrs {}
///`write(|w| ..)` method takes [`addr7_rx::W`](W) writer structure
impl crate::Writable for ADDR7_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR7_RX to value 0
impl crate::Resettable for ADDR7_RXrs {}
