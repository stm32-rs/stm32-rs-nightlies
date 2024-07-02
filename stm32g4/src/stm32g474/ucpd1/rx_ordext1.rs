///Register `RX_ORDEXT1` reader
pub type R = crate::R<RX_ORDEXT1rs>;
///Register `RX_ORDEXT1` writer
pub type W = crate::W<RX_ORDEXT1rs>;
///Field `RXSOPX1` reader - RXSOPX1
pub type RXSOPX1_R = crate::FieldReader<u32>;
///Field `RXSOPX1` writer - RXSOPX1
pub type RXSOPX1_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - RXSOPX1
    #[inline(always)]
    pub fn rxsopx1(&self) -> RXSOPX1_R {
        RXSOPX1_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_ORDEXT1")
            .field("rxsopx1", &self.rxsopx1())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - RXSOPX1
    #[inline(always)]
    #[must_use]
    pub fn rxsopx1(&mut self) -> RXSOPX1_W<RX_ORDEXT1rs> {
        RXSOPX1_W::new(self, 0)
    }
}
/**UCPD Rx Ordered Set Extension Register 1

You can [`read`](crate::Reg::read) this register and get [`rx_ordext1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ordext1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#UCPD1:RX_ORDEXT1)*/
pub struct RX_ORDEXT1rs;
impl crate::RegisterSpec for RX_ORDEXT1rs {
    type Ux = u32;
}
///`read()` method returns [`rx_ordext1::R`](R) reader structure
impl crate::Readable for RX_ORDEXT1rs {}
///`write(|w| ..)` method takes [`rx_ordext1::W`](W) writer structure
impl crate::Writable for RX_ORDEXT1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_ORDEXT1 to value 0
impl crate::Resettable for RX_ORDEXT1rs {
    const RESET_VALUE: u32 = 0;
}
