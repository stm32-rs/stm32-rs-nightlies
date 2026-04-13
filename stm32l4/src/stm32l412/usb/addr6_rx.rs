///Register `ADDR6_RX` reader
pub type R = crate::R<ADDR6_RXrs>;
///Register `ADDR6_RX` writer
pub type W = crate::W<ADDR6_RXrs>;
///Field `ADDR6_RX` reader - Reception buffer address
pub type ADDR6_RX_R = crate::FieldReader<u16>;
///Field `ADDR6_RX` writer - Reception buffer address
pub type ADDR6_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr6_rx(&self) -> ADDR6_RX_R {
        ADDR6_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR6_RX")
            .field("addr6_rx", &self.addr6_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr6_rx(&mut self) -> ADDR6_RX_W<'_, ADDR6_RXrs> {
        ADDR6_RX_W::new(self, 1)
    }
}
/**Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr6_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr6_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR6_RX)*/
pub struct ADDR6_RXrs;
impl crate::RegisterSpec for ADDR6_RXrs {
    type Ux = u16;
}
///`read()` method returns [`addr6_rx::R`](R) reader structure
impl crate::Readable for ADDR6_RXrs {}
///`write(|w| ..)` method takes [`addr6_rx::W`](W) writer structure
impl crate::Writable for ADDR6_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR6_RX to value 0
impl crate::Resettable for ADDR6_RXrs {}
