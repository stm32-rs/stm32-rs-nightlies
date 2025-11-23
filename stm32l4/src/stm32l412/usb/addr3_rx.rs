///Register `ADDR3_RX` reader
pub type R = crate::R<ADDR3_RXrs>;
///Register `ADDR3_RX` writer
pub type W = crate::W<ADDR3_RXrs>;
///Field `ADDR3_RX` reader - Reception buffer address
pub type ADDR3_RX_R = crate::FieldReader<u16>;
///Field `ADDR3_RX` writer - Reception buffer address
pub type ADDR3_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr3_rx(&self) -> ADDR3_RX_R {
        ADDR3_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR3_RX")
            .field("addr3_rx", &self.addr3_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr3_rx(&mut self) -> ADDR3_RX_W<'_, ADDR3_RXrs> {
        ADDR3_RX_W::new(self, 1)
    }
}
/**Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr3_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr3_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR3_RX)*/
pub struct ADDR3_RXrs;
impl crate::RegisterSpec for ADDR3_RXrs {
    type Ux = u16;
}
///`read()` method returns [`addr3_rx::R`](R) reader structure
impl crate::Readable for ADDR3_RXrs {}
///`write(|w| ..)` method takes [`addr3_rx::W`](W) writer structure
impl crate::Writable for ADDR3_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR3_RX to value 0
impl crate::Resettable for ADDR3_RXrs {}
