///Register `SECR` reader
pub type R = crate::R<SECRrs>;
///Field `SEC_SIZE` reader - Securable memory area size
pub type SEC_SIZE_R = crate::FieldReader;
///Field `BOOT_LOCK` reader - used to force boot from user area
pub type BOOT_LOCK_R = crate::BitReader;
impl R {
    ///Bits 0:6 - Securable memory area size
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 16 - used to force boot from user area
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECR")
            .field("sec_size", &self.sec_size())
            .field("boot_lock", &self.boot_lock())
            .finish()
    }
}
/**Flash Security register

You can [`read`](crate::Reg::read) this register and get [`secr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#FLASH:SECR)*/
pub struct SECRrs;
impl crate::RegisterSpec for SECRrs {
    type Ux = u32;
}
///`read()` method returns [`secr::R`](R) reader structure
impl crate::Readable for SECRrs {}
///`reset()` method sets SECR to value 0xf000_0000
impl crate::Resettable for SECRrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
