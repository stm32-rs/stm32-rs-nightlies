#[doc = "Register `R%s` reader"]
pub type R = crate::R<Rrs>;
#[doc = "Register `R%s` writer"]
pub type W = crate::W<Rrs>;
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type PROCID_R = crate::FieldReader;
#[doc = "Field `PROCID` writer - Semaphore ProcessID"]
pub type PROCID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
#[doc = "Field `COREID` reader - Semaphore MASTERID"]
pub type COREID_R = crate::FieldReader;
#[doc = "Field `COREID` writer - Semaphore MASTERID"]
pub type COREID_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Lock indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR {
    #[doc = "0: Semaphore is free"]
    Free = 0,
    #[doc = "1: Semaphore is locked"]
    Locked = 1,
}
impl From<LOCKR> for bool {
    #[inline(always)]
    fn from(variant: LOCKR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock indication"]
pub type LOCK_R = crate::BitReader<LOCKR>;
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LOCKR {
        match self.bits {
            false => LOCKR::Free,
            true => LOCKR::Locked,
        }
    }
    #[doc = "Semaphore is free"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == LOCKR::Free
    }
    #[doc = "Semaphore is locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR::Locked
    }
}
#[doc = "Lock indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW {
    #[doc = "0: Free semaphore"]
    Free = 0,
    #[doc = "1: Try to lock semaphore"]
    TryLock = 1,
}
impl From<LOCKW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` writer - Lock indication"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG, LOCKW>;
impl<'a, REG> LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Free semaphore"]
    #[inline(always)]
    pub fn free(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::Free)
    }
    #[doc = "Try to lock semaphore"]
    #[inline(always)]
    pub fn try_lock(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKW::TryLock)
    }
}
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Semaphore MASTERID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    #[must_use]
    pub fn procid(&mut self) -> PROCID_W<Rrs> {
        PROCID_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Semaphore MASTERID"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<Rrs> {
        COREID_W::new(self, 8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<Rrs> {
        LOCK_W::new(self, 31)
    }
}
#[doc = "HSEM register HSEM_R%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rrs;
impl crate::RegisterSpec for Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r::R`](R) reader structure"]
impl crate::Readable for Rrs {}
#[doc = "`write(|w| ..)` method takes [`r::W`](W) writer structure"]
impl crate::Writable for Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R%s to value 0"]
impl crate::Resettable for Rrs {
    const RESET_VALUE: u32 = 0;
}
