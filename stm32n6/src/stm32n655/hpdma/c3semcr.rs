///Register `C3SEMCR` reader
pub type R = crate::R<C3SEMCRrs>;
///Register `C3SEMCR` writer
pub type W = crate::W<C3SEMCRrs>;
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
        f.debug_struct("C3SEMCR")
            .field("sem_mutex", &self.sem_mutex())
            .field("sem_ccid", &self.sem_ccid())
            .finish()
    }
}
impl W {
    ///Bit 0 - mutual exclusion semaphore for the CID allocation of the channel x (in semaphore mode)
    #[inline(always)]
    pub fn sem_mutex(&mut self) -> SEM_MUTEX_W<'_, C3SEMCRrs> {
        SEM_MUTEX_W::new(self, 0)
    }
}
/**HPDMA channel 3 semaphore control register

You can [`read`](crate::Reg::read) this register and get [`c3semcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3semcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#HPDMA:C3SEMCR)*/
pub struct C3SEMCRrs;
impl crate::RegisterSpec for C3SEMCRrs {
    type Ux = u32;
}
///`read()` method returns [`c3semcr::R`](R) reader structure
impl crate::Readable for C3SEMCRrs {}
///`write(|w| ..)` method takes [`c3semcr::W`](W) writer structure
impl crate::Writable for C3SEMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C3SEMCR to value 0
impl crate::Resettable for C3SEMCRrs {}
