///Register `C7SEMCR` reader
pub type R = crate::R<C7SEMCRrs>;
///Register `C7SEMCR` writer
pub type W = crate::W<C7SEMCRrs>;
///Field `SEM_MUTEX` reader - mutual exclusion semaphore for the CID allocation of the channel x (in semaphore mode)
pub type SEM_MUTEX_R = crate::BitReader;
///Field `SEM_MUTEX` writer - mutual exclusion semaphore for the CID allocation of the channel x (in semaphore mode)
pub type SEM_MUTEX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEM_CCID` reader - current CID allocated to the channel x (in semaphore mode)
pub type SEM_CCID_R = crate::FieldReader;
impl R {
    ///Bit 0 - mutual exclusion semaphore for the CID allocation of the channel x (in semaphore mode)
    #[inline(always)]
    pub fn sem_mutex(&self) -> SEM_MUTEX_R {
        SEM_MUTEX_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:6 - current CID allocated to the channel x (in semaphore mode)
    #[inline(always)]
    pub fn sem_ccid(&self) -> SEM_CCID_R {
        SEM_CCID_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C7SEMCR")
            .field("sem_mutex", &self.sem_mutex())
            .field("sem_ccid", &self.sem_ccid())
            .finish()
    }
}
impl W {
    ///Bit 0 - mutual exclusion semaphore for the CID allocation of the channel x (in semaphore mode)
    #[inline(always)]
    pub fn sem_mutex(&mut self) -> SEM_MUTEX_W<'_, C7SEMCRrs> {
        SEM_MUTEX_W::new(self, 0)
    }
}
/**HPDMA channel 7 semaphore control register

You can [`read`](crate::Reg::read) this register and get [`c7semcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7semcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HPDMA:C7SEMCR)*/
pub struct C7SEMCRrs;
impl crate::RegisterSpec for C7SEMCRrs {
    type Ux = u32;
}
///`read()` method returns [`c7semcr::R`](R) reader structure
impl crate::Readable for C7SEMCRrs {}
///`write(|w| ..)` method takes [`c7semcr::W`](W) writer structure
impl crate::Writable for C7SEMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C7SEMCR to value 0
impl crate::Resettable for C7SEMCRrs {}
