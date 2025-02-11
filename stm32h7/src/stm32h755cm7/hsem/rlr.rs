///Register `RLR%s` reader
pub type R = crate::R<RLRrs>;
///Field `PROCID` reader - Semaphore ProcessID
pub type PROCID_R = crate::FieldReader;
///Field `COREID` reader - Semaphore COREID
pub type COREID_R = crate::FieldReader;
///Field `LOCK` reader - Lock indication
pub type LOCK_R = crate::BitReader;
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
        f.debug_struct("RLR")
            .field("procid", &self.procid())
            .field("coreid", &self.coreid())
            .field("lock", &self.lock())
            .finish()
    }
}
/**Semaphore %s read lock register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#HSEM:RLR[0])*/
pub struct RLRrs;
impl crate::RegisterSpec for RLRrs {
    type Ux = u32;
}
///`read()` method returns [`rlr::R`](R) reader structure
impl crate::Readable for RLRrs {}
///`reset()` method sets RLR%s to value 0
impl crate::Resettable for RLRrs {
    const RESET_VALUE: u32 = 0;
}
