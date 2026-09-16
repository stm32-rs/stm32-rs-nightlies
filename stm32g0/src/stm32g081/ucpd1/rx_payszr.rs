///Register `RX_PAYSZR` reader
pub type R = crate::R<RX_PAYSZRrs>;
///Register `RX_PAYSZR` writer
pub type W = crate::W<RX_PAYSZRrs>;
///Field `RXPAYSZ` reader - RXPAYSZ
pub type RXPAYSZ_R = crate::FieldReader<u16>;
///Field `RXPAYSZ` writer - RXPAYSZ
pub type RXPAYSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
impl R {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PAYSZR")
            .field("rxpaysz", &self.rxpaysz())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&mut self) -> RXPAYSZ_W<'_, RX_PAYSZRrs> {
        RXPAYSZ_W::new(self, 0)
    }
}
/**UCPD Rx Paysize Register

You can [`read`](crate::Reg::read) this register and get [`rx_payszr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_payszr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#UCPD1:RX_PAYSZR)*/
pub struct RX_PAYSZRrs;
impl crate::RegisterSpec for RX_PAYSZRrs {
    type Ux = u32;
}
///`read()` method returns [`rx_payszr::R`](R) reader structure
impl crate::Readable for RX_PAYSZRrs {}
///`write(|w| ..)` method takes [`rx_payszr::W`](W) writer structure
impl crate::Writable for RX_PAYSZRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RX_PAYSZR to value 0
impl crate::Resettable for RX_PAYSZRrs {}
