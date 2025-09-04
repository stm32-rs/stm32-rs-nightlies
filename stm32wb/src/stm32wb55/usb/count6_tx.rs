///Register `COUNT6_TX` reader
pub type R = crate::R<COUNT6_TXrs>;
///Register `COUNT6_TX` writer
pub type W = crate::W<COUNT6_TXrs>;
///Field `COUNT6_TX` reader - Transmission byte count
pub type COUNT6_TX_R = crate::FieldReader<u16>;
///Field `COUNT6_TX` writer - Transmission byte count
pub type COUNT6_TX_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count6_tx(&self) -> COUNT6_TX_R {
        COUNT6_TX_R::new(self.bits & 0x03ff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COUNT6_TX")
            .field("count6_tx", &self.count6_tx())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count6_tx(&mut self) -> COUNT6_TX_W<COUNT6_TXrs> {
        COUNT6_TX_W::new(self, 0)
    }
}
/**Transmission byte count 0

You can [`read`](crate::Reg::read) this register and get [`count6_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count6_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#USB:COUNT6_TX)*/
pub struct COUNT6_TXrs;
impl crate::RegisterSpec for COUNT6_TXrs {
    type Ux = u16;
}
///`read()` method returns [`count6_tx::R`](R) reader structure
impl crate::Readable for COUNT6_TXrs {}
///`write(|w| ..)` method takes [`count6_tx::W`](W) writer structure
impl crate::Writable for COUNT6_TXrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets COUNT6_TX to value 0
impl crate::Resettable for COUNT6_TXrs {}
