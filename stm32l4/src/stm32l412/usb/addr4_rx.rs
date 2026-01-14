///Register `ADDR4_RX` reader
pub type R = crate::R<ADDR4_RXrs>;
///Register `ADDR4_RX` writer
pub type W = crate::W<ADDR4_RXrs>;
///Field `ADDR4_RX` reader - Reception buffer address
pub type ADDR4_RX_R = crate::FieldReader<u16>;
///Field `ADDR4_RX` writer - Reception buffer address
pub type ADDR4_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr4_rx(&self) -> ADDR4_RX_R {
        ADDR4_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR4_RX")
            .field("addr4_rx", &self.addr4_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr4_rx(&mut self) -> ADDR4_RX_W<'_, ADDR4_RXrs> {
        ADDR4_RX_W::new(self, 1)
    }
}
/**Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr4_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr4_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR4_RX)*/
pub struct ADDR4_RXrs;
impl crate::RegisterSpec for ADDR4_RXrs {
    type Ux = u16;
}
///`read()` method returns [`addr4_rx::R`](R) reader structure
impl crate::Readable for ADDR4_RXrs {}
///`write(|w| ..)` method takes [`addr4_rx::W`](W) writer structure
impl crate::Writable for ADDR4_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR4_RX to value 0
impl crate::Resettable for ADDR4_RXrs {}
