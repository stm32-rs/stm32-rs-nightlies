///Register `ADDR3_TX` reader
pub type R = crate::R<ADDR3_TXrs>;
///Register `ADDR3_TX` writer
pub type W = crate::W<ADDR3_TXrs>;
///Field `ADDR3_TX` reader - Transmission buffer address
pub type ADDR3_TX_R = crate::FieldReader<u16>;
///Field `ADDR3_TX` writer - Transmission buffer address
pub type ADDR3_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr3_tx(&self) -> ADDR3_TX_R {
        ADDR3_TX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR3_TX")
            .field("addr3_tx", &self.addr3_tx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr3_tx(&mut self) -> ADDR3_TX_W<'_, ADDR3_TXrs> {
        ADDR3_TX_W::new(self, 1)
    }
}
/**Transmission buffer address 3

You can [`read`](crate::Reg::read) this register and get [`addr3_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr3_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR3_TX)*/
pub struct ADDR3_TXrs;
impl crate::RegisterSpec for ADDR3_TXrs {
    type Ux = u16;
}
///`read()` method returns [`addr3_tx::R`](R) reader structure
impl crate::Readable for ADDR3_TXrs {}
///`write(|w| ..)` method takes [`addr3_tx::W`](W) writer structure
impl crate::Writable for ADDR3_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR3_TX to value 0
impl crate::Resettable for ADDR3_TXrs {}
