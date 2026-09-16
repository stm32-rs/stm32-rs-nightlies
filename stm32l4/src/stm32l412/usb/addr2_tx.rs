///Register `ADDR2_TX` reader
pub type R = crate::R<ADDR2_TXrs>;
///Register `ADDR2_TX` writer
pub type W = crate::W<ADDR2_TXrs>;
///Field `ADDR2_TX` reader - Transmission buffer address
pub type ADDR2_TX_R = crate::FieldReader<u16>;
///Field `ADDR2_TX` writer - Transmission buffer address
pub type ADDR2_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr2_tx(&self) -> ADDR2_TX_R {
        ADDR2_TX_R::new((self.bits >> 1) & 0x7fff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR2_TX")
            .field("addr2_tx", &self.addr2_tx())
            .finish()
    }
}
impl W {
    ///Bits 1:15 - Transmission buffer address
    #[inline(always)]
    pub fn addr2_tx(&mut self) -> ADDR2_TX_W<'_, ADDR2_TXrs> {
        ADDR2_TX_W::new(self, 1)
    }
}
/**Transmission buffer address 2

You can [`read`](crate::Reg::read) this register and get [`addr2_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:ADDR2_TX)*/
pub struct ADDR2_TXrs;
impl crate::RegisterSpec for ADDR2_TXrs {
    type Ux = u16;
}
///`read()` method returns [`addr2_tx::R`](R) reader structure
impl crate::Readable for ADDR2_TXrs {}
///`write(|w| ..)` method takes [`addr2_tx::W`](W) writer structure
impl crate::Writable for ADDR2_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ADDR2_TX to value 0
impl crate::Resettable for ADDR2_TXrs {}
