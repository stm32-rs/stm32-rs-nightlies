///Register `ADDR5_TX` reader
pub type R = crate::R<ADDR5_TXrs>;
///Register `ADDR5_TX` writer
pub type W = crate::W<ADDR5_TXrs>;
///Field `ADDR5_TX` reader - Transmission buffer address
pub type ADDR5_TX_R = crate::FieldReader<u16>;
///Field `ADDR5_TX` writer - Transmission buffer address
pub type ADDR5_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr5_tx(&self) -> ADDR5_TX_R {
        ADDR5_TX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR5_TX")
            .field("addr5_tx", &self.addr5_tx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr5_tx(&mut self) -> ADDR5_TX_W<'_, ADDR5_TXrs> {
        ADDR5_TX_W::new(self, 1)
    }
}
/**Transmission buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr5_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr5_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR5_TX)*/
pub struct ADDR5_TXrs;
impl crate::RegisterSpec for ADDR5_TXrs {
    type Ux = u16;
}
///`read()` method returns [`addr5_tx::R`](R) reader structure
impl crate::Readable for ADDR5_TXrs {}
///`write(|w| ..)` method takes [`addr5_tx::W`](W) writer structure
impl crate::Writable for ADDR5_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR5_TX to value 0
impl crate::Resettable for ADDR5_TXrs {}
