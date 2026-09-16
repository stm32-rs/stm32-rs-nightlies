///Register `COUNT4_TX` reader
pub type R = crate::R<COUNT4_TXrs>;
///Register `COUNT4_TX` writer
pub type W = crate::W<COUNT4_TXrs>;
///Field `COUNT4_TX` reader - Transmission byte count
pub type COUNT4_TX_R = crate::FieldReader<u16>;
///Field `COUNT4_TX` writer - Transmission byte count
pub type COUNT4_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count4_tx(&self) -> COUNT4_TX_R {
        COUNT4_TX_R::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT4_TX")
            .field("count4_tx", &self.count4_tx())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count4_tx(&mut self) -> COUNT4_TX_W<'_, COUNT4_TXrs> {
        COUNT4_TX_W::new(self, 0)
    }
}
/**Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count4_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count4_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#USB:COUNT4_TX)*/
pub struct COUNT4_TXrs;
impl crate::RegisterSpec for COUNT4_TXrs {
    type Ux = u16;
}
///`read()` method returns [`count4_tx::R`](R) reader structure
impl crate::Readable for COUNT4_TXrs {}
///`write(|w| ..)` method takes [`count4_tx::W`](W) writer structure
impl crate::Writable for COUNT4_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COUNT4_TX to value 0
impl crate::Resettable for COUNT4_TXrs {}
