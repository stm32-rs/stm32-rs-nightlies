///Register `ADDR2_RX` reader
pub type R = crate::R<ADDR2_RXrs>;
///Register `ADDR2_RX` writer
pub type W = crate::W<ADDR2_RXrs>;
///Field `ADDR2_RX` reader - Reception buffer address
pub type ADDR2_RX_R = crate::FieldReader<u16>;
///Field `ADDR2_RX` writer - Reception buffer address
pub type ADDR2_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr2_rx(&self) -> ADDR2_RX_R {
        ADDR2_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR2_RX")
            .field("addr2_rx", &self.addr2_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr2_rx(&mut self) -> ADDR2_RX_W<'_, ADDR2_RXrs> {
        ADDR2_RX_W::new(self, 1)
    }
}
/**Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr2_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:ADDR2_RX)*/
pub struct ADDR2_RXrs;
impl crate::RegisterSpec for ADDR2_RXrs {
    type Ux = u16;
}
///`read()` method returns [`addr2_rx::R`](R) reader structure
impl crate::Readable for ADDR2_RXrs {}
///`write(|w| ..)` method takes [`addr2_rx::W`](W) writer structure
impl crate::Writable for ADDR2_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR2_RX to value 0
impl crate::Resettable for ADDR2_RXrs {}
