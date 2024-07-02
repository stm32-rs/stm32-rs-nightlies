///Register `RX_PAYSZ` reader
pub type R = crate::R<RX_PAYSZrs>;
///Register `RX_PAYSZ` writer
pub type W = crate::W<RX_PAYSZrs>;
///Field `RXPAYSZ` reader - RXPAYSZ
pub type RXPAYSZ_R = crate::FieldReader<u16>;
///Field `RXPAYSZ` writer - RXPAYSZ
pub type RXPAYSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    pub fn rxpaysz(&self) -> RXPAYSZ_R {
        RXPAYSZ_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_PAYSZ")
            .field("rxpaysz", &self.rxpaysz())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - RXPAYSZ
    #[inline(always)]
    #[must_use]
    pub fn rxpaysz(&mut self) -> RXPAYSZ_W<RX_PAYSZrs> {
        RXPAYSZ_W::new(self, 0)
    }
}
/**UCPD Rx Paysize Register

You can [`read`](crate::Reg::read) this register and get [`rx_paysz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_paysz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G081.html#UCPD1:RX_PAYSZ)*/
pub struct RX_PAYSZrs;
impl crate::RegisterSpec for RX_PAYSZrs {
    type Ux = u32;
}
///`read()` method returns [`rx_paysz::R`](R) reader structure
impl crate::Readable for RX_PAYSZrs {}
///`write(|w| ..)` method takes [`rx_paysz::W`](W) writer structure
impl crate::Writable for RX_PAYSZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_PAYSZ to value 0
impl crate::Resettable for RX_PAYSZrs {
    const RESET_VALUE: u32 = 0;
}
