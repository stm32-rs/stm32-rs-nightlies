///Register `SEC_SYNC` reader
pub type R = crate::R<SEC_SYNCrs>;
///Register `SEC_SYNC` writer
pub type W = crate::W<SEC_SYNCrs>;
///Field `SEC_SYNC` reader - Secondary Synchro word.
pub type SEC_SYNC_R = crate::FieldReader<u32>;
///Field `SEC_SYNC` writer - Secondary Synchro word.
pub type SEC_SYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Secondary Synchro word.
    #[inline(always)]
    pub fn sec_sync(&self) -> SEC_SYNC_R {
        SEC_SYNC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_SYNC")
            .field("sec_sync", &self.sec_sync())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Secondary Synchro word.
    #[inline(always)]
    pub fn sec_sync(&mut self) -> SEC_SYNC_W<'_, SEC_SYNCrs> {
        SEC_SYNC_W::new(self, 0)
    }
}
/**SEC_SYNC register

You can [`read`](crate::Reg::read) this register and get [`sec_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:SEC_SYNC)*/
pub struct SEC_SYNCrs;
impl crate::RegisterSpec for SEC_SYNCrs {
    type Ux = u32;
}
///`read()` method returns [`sec_sync::R`](R) reader structure
impl crate::Readable for SEC_SYNCrs {}
///`write(|w| ..)` method takes [`sec_sync::W`](W) writer structure
impl crate::Writable for SEC_SYNCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEC_SYNC to value 0
impl crate::Resettable for SEC_SYNCrs {}
