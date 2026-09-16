///Register `RLR%s` reader
pub type R = crate::R<RLRrs>;
///Field `PROCID` reader - Semaphore ProcessID
pub type PROCID_R = crate::FieldReader;
///Field `MASTERID` reader - Semaphore MasterID
pub type MASTERID_R = crate::FieldReader;
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
impl R {
    ///Bits 0:7 - Semaphore ProcessID
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Semaphore MasterID
    #[inline(always)]
    pub fn masterid(&self) -> MASTERID_R {
        MASTERID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 31 - Lock indication
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLR")
            .field("procid", &self.procid())
            .field("masterid", &self.masterid())
            .field("lock", &self.lock())
            .finish()
    }
}
/**Semaphore %s read lock register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#HSEM:RLR[0])*/
pub struct RLRrs;
impl crate::RegisterSpec for RLRrs {
    type Ux = u32;
}
///`read()` method returns [`rlr::R`](R) reader structure
impl crate::Readable for RLRrs {}
///`reset()` method sets RLR%s to value 0
impl crate::Resettable for RLRrs {}
