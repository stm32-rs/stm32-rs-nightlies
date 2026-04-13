///Register `ADDR6_TX` reader
pub type R = crate::R<ADDR6_TXrs>;
///Register `ADDR6_TX` writer
pub type W = crate::W<ADDR6_TXrs>;
///Field `ADDR6_TX` reader - Transmission buffer address
pub type ADDR6_TX_R = crate::FieldReader<u16>;
///Field `ADDR6_TX` writer - Transmission buffer address
pub type ADDR6_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr6_tx(&self) -> ADDR6_TX_R {
        ADDR6_TX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR6_TX")
            .field("addr6_tx", &self.addr6_tx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr6_tx(&mut self) -> ADDR6_TX_W<'_, ADDR6_TXrs> {
        ADDR6_TX_W::new(self, 1)
    }
}
/**Transmission buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr6_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr6_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR6_TX)*/
pub struct ADDR6_TXrs;
impl crate::RegisterSpec for ADDR6_TXrs {
    type Ux = u16;
}
///`read()` method returns [`addr6_tx::R`](R) reader structure
impl crate::Readable for ADDR6_TXrs {}
///`write(|w| ..)` method takes [`addr6_tx::W`](W) writer structure
impl crate::Writable for ADDR6_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR6_TX to value 0
impl crate::Resettable for ADDR6_TXrs {}
