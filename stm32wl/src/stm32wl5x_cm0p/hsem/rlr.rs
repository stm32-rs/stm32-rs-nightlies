#[doc = "Register `RLR%s` reader"]
pub type R = crate::R<RLRrs>;
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type PROCID_R = crate::FieldReader;
#[doc = "Field `COREID` reader - COREID"]
pub type COREID_R = crate::FieldReader;
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
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - COREID"]
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
#[doc = "Semaphore %s read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RLRrs;
impl crate::RegisterSpec for RLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rlr::R`](R) reader structure"]
impl crate::Readable for RLRrs {}
#[doc = "`reset()` method sets RLR%s to value 0"]
impl crate::Resettable for RLRrs {
    const RESET_VALUE: u32 = 0;
}
