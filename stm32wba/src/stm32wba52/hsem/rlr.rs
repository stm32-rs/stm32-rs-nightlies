///Register `RLR%s` reader
pub type R = crate::R<RLRrs>;
///Field `PROCID` reader - Semaphore processor ID This field is read only by software at this address. - On a read when the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and hardware sets the PROCID to 0. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns the PROCID of the AHB bus master that has locked the semaphore.
pub type PROCID_R = crate::FieldReader;
///Field `LOCKID` reader - Semaphore LOCKID This field is read only by software at this address. On a read, when the semaphore is free, the hardware sets the LOCKID to the AHB bus master ID reading the semaphore. The LOCKID of the AHB bus master locking the semaphore is read. On a read when the semaphore is locked, this field returns the LOCKID of the AHB bus master that has locked the semaphore.
pub type LOCKID_R = crate::FieldReader;
///Field `SEC` reader - Semaphore secure. This field is read only by software at this address. - When the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and hardware sets the SEC to the valid AHB bus master security definition. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns the SEC of the AHB bus master that has locked the semaphore.
pub type SEC_R = crate::BitReader;
///Field `PRIV` reader - Semaphore privilege This field is read only by software at this address. - When the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and hardware sets the PRIV to the valid AHB bus master privileged definition. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns the PRIV of the AHB bus master that has locked the semaphore.
pub type PRIV_R = crate::BitReader;
///Field `LOCK` reader - Lock indication This bit is read only by software at this address. - When the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and returns 1. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns 1 (the LOCKID and SEC and PRIV and PROCID reflect the already locked semaphore information).
pub type LOCK_R = crate::BitReader;
impl R {
    ///Bits 0:7 - Semaphore processor ID This field is read only by software at this address. - On a read when the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and hardware sets the PROCID to 0. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns the PROCID of the AHB bus master that has locked the semaphore.
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - Semaphore LOCKID This field is read only by software at this address. On a read, when the semaphore is free, the hardware sets the LOCKID to the AHB bus master ID reading the semaphore. The LOCKID of the AHB bus master locking the semaphore is read. On a read when the semaphore is locked, this field returns the LOCKID of the AHB bus master that has locked the semaphore.
    #[inline(always)]
    pub fn lockid(&self) -> LOCKID_R {
        LOCKID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Semaphore secure. This field is read only by software at this address. - When the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and hardware sets the SEC to the valid AHB bus master security definition. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns the SEC of the AHB bus master that has locked the semaphore.
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Semaphore privilege This field is read only by software at this address. - When the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and hardware sets the PRIV to the valid AHB bus master privileged definition. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns the PRIV of the AHB bus master that has locked the semaphore.
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 31 - Lock indication This bit is read only by software at this address. - When the semaphore is free: A read with a valid AHB bus master ID and SEC and PRIV locks the semaphore and returns 1. - When the semaphore is locked: A read with a valid AHB bus master ID and SEC and PRIV returns 1 (the LOCKID and SEC and PRIV and PROCID reflect the already locked semaphore information).
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RLR")
            .field("procid", &self.procid())
            .field("lockid", &self.lockid())
            .field("sec", &self.sec())
            .field("priv_", &self.priv_())
            .field("lock", &self.lock())
            .finish()
    }
}
/**Semaphore %s read lock register

You can [`read`](crate::Reg::read) this register and get [`rlr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#HSEM:RLR[0])*/
pub struct RLRrs;
impl crate::RegisterSpec for RLRrs {
    type Ux = u32;
}
///`read()` method returns [`rlr::R`](R) reader structure
impl crate::Readable for RLRrs {}
///`reset()` method sets RLR%s to value 0
impl crate::Resettable for RLRrs {}
