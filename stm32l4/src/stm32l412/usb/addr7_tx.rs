///Register `ADDR7_TX` reader
pub type R = crate::R<ADDR7_TXrs>;
///Register `ADDR7_TX` writer
pub type W = crate::W<ADDR7_TXrs>;
///Field `ADDR7_TX` reader - Transmission buffer address
pub type ADDR7_TX_R = crate::FieldReader<u16>;
///Field `ADDR7_TX` writer - Transmission buffer address
pub type ADDR7_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr7_tx(&self) -> ADDR7_TX_R {
        ADDR7_TX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR7_TX")
            .field("addr7_tx", &self.addr7_tx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr7_tx(&mut self) -> ADDR7_TX_W<'_, ADDR7_TXrs> {
        ADDR7_TX_W::new(self, 1)
    }
}
/**Transmission buffer address 0

You can [`read`](crate::Reg::read) this register and get [`addr7_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr7_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR7_TX)*/
pub struct ADDR7_TXrs;
impl crate::RegisterSpec for ADDR7_TXrs {
    type Ux = u16;
}
///`read()` method returns [`addr7_tx::R`](R) reader structure
impl crate::Readable for ADDR7_TXrs {}
///`write(|w| ..)` method takes [`addr7_tx::W`](W) writer structure
impl crate::Writable for ADDR7_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR7_TX to value 0
impl crate::Resettable for ADDR7_TXrs {}
