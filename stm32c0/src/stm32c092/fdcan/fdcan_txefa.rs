///Register `FDCAN_TXEFA` reader
pub type R = crate::R<FDCAN_TXEFArs>;
///Register `FDCAN_TXEFA` writer
pub type W = crate::W<FDCAN_TXEFArs>;
///Field `EFAI` reader - Event FIFO acknowledge index
pub type EFAI_R = crate::FieldReader;
///Field `EFAI` writer - Event FIFO acknowledge index
pub type EFAI_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:1 - Event FIFO acknowledge index
    #[inline(always)]
    pub fn efai(&self) -> EFAI_R {
        EFAI_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TXEFA")
            .field("efai", &self.efai())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Event FIFO acknowledge index
    #[inline(always)]
    pub fn efai(&mut self) -> EFAI_W<'_, FDCAN_TXEFArs> {
        EFAI_W::new(self, 0)
    }
}
/**FDCAN Tx event FIFO acknowledge register

You can [`read`](crate::Reg::read) this register and get [`fdcan_txefa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_TXEFA)*/
pub struct FDCAN_TXEFArs;
impl crate::RegisterSpec for FDCAN_TXEFArs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_txefa::R`](R) reader structure
impl crate::Readable for FDCAN_TXEFArs {}
///`write(|w| ..)` method takes [`fdcan_txefa::W`](W) writer structure
impl crate::Writable for FDCAN_TXEFArs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_TXEFA to value 0
impl crate::Resettable for FDCAN_TXEFArs {}
