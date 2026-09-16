///Register `COUNT0_TX` reader
pub type R = crate::R<COUNT0_TXrs>;
///Register `COUNT0_TX` writer
pub type W = crate::W<COUNT0_TXrs>;
///Field `COUNT0_TX` reader - Transmission byte count
pub type COUNT0_TX_R = crate::FieldReader<u16>;
///Field `COUNT0_TX` writer - Transmission byte count
pub type COUNT0_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count0_tx(&self) -> COUNT0_TX_R {
        COUNT0_TX_R::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT0_TX")
            .field("count0_tx", &self.count0_tx())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count0_tx(&mut self) -> COUNT0_TX_W<'_, COUNT0_TXrs> {
        COUNT0_TX_W::new(self, 0)
    }
}
/**Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count0_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count0_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT0_TX)*/
pub struct COUNT0_TXrs;
impl crate::RegisterSpec for COUNT0_TXrs {
    type Ux = u16;
}
///`read()` method returns [`count0_tx::R`](R) reader structure
impl crate::Readable for COUNT0_TXrs {}
///`write(|w| ..)` method takes [`count0_tx::W`](W) writer structure
impl crate::Writable for COUNT0_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COUNT0_TX to value 0
impl crate::Resettable for COUNT0_TXrs {}
