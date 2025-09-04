///Register `R%s` reader
pub type R = crate::R<Rrs>;
///Register `R%s` writer
pub type W = crate::W<Rrs>;
///Field `PROCID` reader - Semaphore ProcessID
pub type PROCID_R = crate::FieldReader;
///Field `PROCID` writer - Semaphore ProcessID
pub type PROCID_W<'a, REG> = crate::FieldWriter<'a, REG, 8, u8, crate::Safe>;
///Field `COREID` reader - Semaphore COREID
pub type COREID_R = crate::FieldReader;
///Field `COREID` writer - Semaphore COREID
pub type COREID_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Lock indication

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR {
    ///0: Semaphore is free
    Free = 0,
    ///1: Semaphore is locked
    Locked = 1,
}
impl From<LOCKR> for bool {
    #[inline(always)]
    fn from(variant: LOCKR) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` reader - Lock indication
pub type LOCK_R = crate::BitReader<LOCKR>;
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LOCKR {
        match self.bits {
            false => LOCKR::Free,
            true => LOCKR::Locked,
        }
    }
    ///Semaphore is free
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == LOCKR::Free
    }
    ///Semaphore is locked
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::Locked
    }
}
/**Lock indication

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW {
    ///0: Free semaphore
    Free = 0,
    ///1: Try to lock semaphore
    TryLock = 1,
}
impl From<LOCKW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - Lock indication
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKW>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Free semaphore
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::Free)
    }
    ///Try to lock semaphore
    #[inline(always)]
    pub fn try_lock(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::TryLock)
    }
}
impl R {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Semaphore COREID
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R")
            .field("procid", &self.procid())
            .field("coreid", &self.coreid())
            .field("lock", &self.lock())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&mut self) -> PROCID_W<Rrs> {
        PROCID_W::new(self, 0)
    }
    ///Bits 8:11 - Semaphore COREID
    #[inline(always)]
    pub fn coreid(&mut self) -> COREID_W<Rrs> {
        COREID_W::new(self, 8)
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<Rrs> {
        LOCK_W::new(self, 31)
    }
}
/**HSEM register HSEM_R%s

You can [`read`](crate::Reg::read) this register and get [`r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#HSEM:R[0])*/
pub struct Rrs;
impl crate::RegisterSpec for Rrs {
    type Ux = u32;
}
///`read()` method returns [`r::R`](R) reader structure
impl crate::Readable for Rrs {}
///`write(|w| ..)` method takes [`r::W`](W) writer structure
impl crate::Writable for Rrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets R%s to value 0
impl crate::Resettable for Rrs {}
