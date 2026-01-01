///Register `COUNT1_TX` reader
pub type R = crate::R<COUNT1_TXrs>;
///Register `COUNT1_TX` writer
pub type W = crate::W<COUNT1_TXrs>;
///Field `COUNT1_TX` reader - Transmission byte count
pub type COUNT1_TX_R = crate::FieldReader<u16>;
///Field `COUNT1_TX` writer - Transmission byte count
pub type COUNT1_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count1_tx(&self) -> COUNT1_TX_R {
        COUNT1_TX_R::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT1_TX")
            .field("count1_tx", &self.count1_tx())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count1_tx(&mut self) -> COUNT1_TX_W<'_, COUNT1_TXrs> {
        COUNT1_TX_W::new(self, 0)
    }
}
/**Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count1_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count1_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:COUNT1_TX)*/
pub struct COUNT1_TXrs;
impl crate::RegisterSpec for COUNT1_TXrs {
    type Ux = u16;
}
///`read()` method returns [`count1_tx::R`](R) reader structure
impl crate::Readable for COUNT1_TXrs {}
///`write(|w| ..)` method takes [`count1_tx::W`](W) writer structure
impl crate::Writable for COUNT1_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COUNT1_TX to value 0
impl crate::Resettable for COUNT1_TXrs {}
