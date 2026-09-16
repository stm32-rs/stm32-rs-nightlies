///Register `ADDR1_RX` reader
pub type R = crate::R<ADDR1_RXrs>;
///Register `ADDR1_RX` writer
pub type W = crate::W<ADDR1_RXrs>;
///Field `ADDR1_RX` reader - Reception buffer address
pub type ADDR1_RX_R = crate::FieldReader<u16>;
///Field `ADDR1_RX` writer - Reception buffer address
pub type ADDR1_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr1_rx(&self) -> ADDR1_RX_R {
        ADDR1_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR1_RX")
            .field("addr1_rx", &self.addr1_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Reception buffer address
    #[inline(always)]
    pub fn addr1_rx(&mut self) -> ADDR1_RX_W<'_, ADDR1_RXrs> {
        ADDR1_RX_W::new(self, 1)
    }
}
/**Reception buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr1_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr1_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR1_RX)*/
pub struct ADDR1_RXrs;
impl crate::RegisterSpec for ADDR1_RXrs {
    type Ux = u16;
}
///`read()` method returns [`addr1_rx::R`](R) reader structure
impl crate::Readable for ADDR1_RXrs {}
///`write(|w| ..)` method takes [`addr1_rx::W`](W) writer structure
impl crate::Writable for ADDR1_RXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR1_RX to value 0
impl crate::Resettable for ADDR1_RXrs {}
