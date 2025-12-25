///Register `SEMA_IRQ_STATUS` reader
pub type R = crate::R<SEMA_IRQ_STATUSrs>;
///Register `SEMA_IRQ_STATUS` writer
pub type W = crate::W<SEMA_IRQ_STATUSrs>;
///Field `LOCK` reader - On read, returns the semaphore locked interrupt status.
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - On read, returns the semaphore locked interrupt status.
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNLOCK` reader - On read, returns the semaphore unlocked interrupt status.
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - On read, returns the semaphore unlocked interrupt status.
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - On read, returns the semaphore locked interrupt status.
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - On read, returns the semaphore unlocked interrupt status.
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEMA_IRQ_STATUS")
            .field("lock", &self.lock())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bit 0 - On read, returns the semaphore locked interrupt status.
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, SEMA_IRQ_STATUSrs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - On read, returns the semaphore unlocked interrupt status.
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<'_, SEMA_IRQ_STATUSrs> {
        UNLOCK_W::new(self, 1)
    }
}
/**SEMA_IRQ_STATUS register

You can [`read`](crate::Reg::read) this register and get [`sema_irq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sema_irq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RRM:SEMA_IRQ_STATUS)*/
pub struct SEMA_IRQ_STATUSrs;
impl crate::RegisterSpec for SEMA_IRQ_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`sema_irq_status::R`](R) reader structure
impl crate::Readable for SEMA_IRQ_STATUSrs {}
///`write(|w| ..)` method takes [`sema_irq_status::W`](W) writer structure
impl crate::Writable for SEMA_IRQ_STATUSrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEMA_IRQ_STATUS to value 0
impl crate::Resettable for SEMA_IRQ_STATUSrs {}
