///Register `SYNC` reader
pub type R = crate::R<SYNCrs>;
///Register `SYNC` writer
pub type W = crate::W<SYNCrs>;
///Field `SYNC` reader - Synchro word.
pub type SYNC_R = crate::FieldReader<u32>;
///Field `SYNC` writer - Synchro word.
pub type SYNC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Synchro word.
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC").field("sync", &self.sync()).finish()
    }
}
impl W {
    ///Bits 0:31 - Synchro word.
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<'_, SYNCrs> {
        SYNC_W::new(self, 0)
    }
}
/**SYNC register

You can [`read`](crate::Reg::read) this register and get [`sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#STATIC:SYNC)*/
pub struct SYNCrs;
impl crate::RegisterSpec for SYNCrs {
    type Ux = u32;
}
///`read()` method returns [`sync::R`](R) reader structure
impl crate::Readable for SYNCrs {}
///`write(|w| ..)` method takes [`sync::W`](W) writer structure
impl crate::Writable for SYNCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SYNC to value 0x2323_2323
impl crate::Resettable for SYNCrs {
    const RESET_VALUE: u32 = 0x2323_2323;
}
