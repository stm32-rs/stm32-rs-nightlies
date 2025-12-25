///Register `SEMA_IRQ_ENABLE` reader
pub type R = crate::R<SEMA_IRQ_ENABLErs>;
///Register `SEMA_IRQ_ENABLE` writer
pub type W = crate::W<SEMA_IRQ_ENABLErs>;
///Field `LOCK` reader - semaphore locked (= one port granted) interrupt enable
pub type LOCK_R = crate::BitReader;
///Field `LOCK` writer - semaphore locked (= one port granted) interrupt enable
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UNLOCK` reader - semaphore unlocked (=no port selected) interrupt enable
pub type UNLOCK_R = crate::BitReader;
///Field `UNLOCK` writer - semaphore unlocked (=no port selected) interrupt enable
pub type UNLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - semaphore locked (= one port granted) interrupt enable
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - semaphore unlocked (=no port selected) interrupt enable
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEMA_IRQ_ENABLE")
            .field("lock", &self.lock())
            .field("unlock", &self.unlock())
            .finish()
    }
}
impl W {
    ///Bit 0 - semaphore locked (= one port granted) interrupt enable
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<'_, SEMA_IRQ_ENABLErs> {
        LOCK_W::new(self, 0)
    }
    ///Bit 1 - semaphore unlocked (=no port selected) interrupt enable
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<'_, SEMA_IRQ_ENABLErs> {
        UNLOCK_W::new(self, 1)
    }
}
/**SEMA_IRQ_ENABLE register

You can [`read`](crate::Reg::read) this register and get [`sema_irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sema_irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#RRM:SEMA_IRQ_ENABLE)*/
pub struct SEMA_IRQ_ENABLErs;
impl crate::RegisterSpec for SEMA_IRQ_ENABLErs {
    type Ux = u32;
}
///`read()` method returns [`sema_irq_enable::R`](R) reader structure
impl crate::Readable for SEMA_IRQ_ENABLErs {}
///`write(|w| ..)` method takes [`sema_irq_enable::W`](W) writer structure
impl crate::Writable for SEMA_IRQ_ENABLErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SEMA_IRQ_ENABLE to value 0
impl crate::Resettable for SEMA_IRQ_ENABLErs {}
