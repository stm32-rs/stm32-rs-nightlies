///Register `SECBOOTR_CUR` reader
pub type R = crate::R<SECBOOTR_CURrs>;
///Field `SECBOOT_LOCK` reader - A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings.
pub type SECBOOT_LOCK_R = crate::FieldReader;
///Field `SECBOOTADD` reader - Unique boot entry secure address These bits reflect the Secure UBE address
pub type SECBOOTADD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:7 - A field locking the values of UBE, SWAP_BANK, and SECBOOTADD settings.
    #[inline(always)]
    pub fn secboot_lock(&self) -> SECBOOT_LOCK_R {
        SECBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31 - Unique boot entry secure address These bits reflect the Secure UBE address
    #[inline(always)]
    pub fn secbootadd(&self) -> SECBOOTADD_R {
        SECBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECBOOTR_CUR")
            .field("secboot_lock", &self.secboot_lock())
            .field("secbootadd", &self.secbootadd())
            .finish()
    }
}
/**FLASH secure boot register

You can [`read`](crate::Reg::read) this register and get [`secbootr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#FLASH:SECBOOTR_CUR)*/
pub struct SECBOOTR_CURrs;
impl crate::RegisterSpec for SECBOOTR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`secbootr_cur::R`](R) reader structure
impl crate::Readable for SECBOOTR_CURrs {}
///`reset()` method sets SECBOOTR_CUR to value 0
impl crate::Resettable for SECBOOTR_CURrs {}
