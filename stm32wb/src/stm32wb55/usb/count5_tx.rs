///Register `COUNT5_TX` reader
pub type R = crate::R<COUNT5_TXrs>;
///Register `COUNT5_TX` writer
pub type W = crate::W<COUNT5_TXrs>;
///Field `COUNT5_TX` reader - Transmission byte count
pub type COUNT5_TX_R = crate::FieldReader<u16>;
///Field `COUNT5_TX` writer - Transmission byte count
pub type COUNT5_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count5_tx(&self) -> COUNT5_TX_R {
        COUNT5_TX_R::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT5_TX")
            .field("count5_tx", &self.count5_tx())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count5_tx(&mut self) -> COUNT5_TX_W<'_, COUNT5_TXrs> {
        COUNT5_TX_W::new(self, 0)
    }
}
/**Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count5_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count5_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT5_TX)*/
pub struct COUNT5_TXrs;
impl crate::RegisterSpec for COUNT5_TXrs {
    type Ux = u16;
}
///`read()` method returns [`count5_tx::R`](R) reader structure
impl crate::Readable for COUNT5_TXrs {}
///`write(|w| ..)` method takes [`count5_tx::W`](W) writer structure
impl crate::Writable for COUNT5_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COUNT5_TX to value 0
impl crate::Resettable for COUNT5_TXrs {}
