///Register `ADDR4_TX` reader
pub type R = crate::R<ADDR4_TXrs>;
///Register `ADDR4_TX` writer
pub type W = crate::W<ADDR4_TXrs>;
///Field `ADDR4_RX` reader - Transmission buffer address
pub type ADDR4_RX_R = crate::FieldReader<u16>;
///Field `ADDR4_RX` writer - Transmission buffer address
pub type ADDR4_RX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr4_rx(&self) -> ADDR4_RX_R {
        ADDR4_RX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR4_TX")
            .field("addr4_rx", &self.addr4_rx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr4_rx(&mut self) -> ADDR4_RX_W<'_, ADDR4_TXrs> {
        ADDR4_RX_W::new(self, 1)
    }
}
/**Transmission buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr4_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr4_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR4_TX)*/
pub struct ADDR4_TXrs;
impl crate::RegisterSpec for ADDR4_TXrs {
    type Ux = u16;
}
///`read()` method returns [`addr4_tx::R`](R) reader structure
impl crate::Readable for ADDR4_TXrs {}
///`write(|w| ..)` method takes [`addr4_tx::W`](W) writer structure
impl crate::Writable for ADDR4_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR4_TX to value 0
impl crate::Resettable for ADDR4_TXrs {}
