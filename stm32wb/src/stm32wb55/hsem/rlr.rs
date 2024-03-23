#[doc = "Register `RLR%s` reader"]
pub type R = crate::R<RLRrs>;
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type PROCID_R = crate::FieldReader;
#[doc = "Field `COREID` reader - Semaphore CoreID"]
pub type COREID_R = crate::FieldReader;
#[doc = "Field `LOCK` reader - lock indication"]
pub type LOCK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Semaphore CoreID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - lock indication"]
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
