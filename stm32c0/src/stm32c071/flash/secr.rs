///Register `SECR` reader
pub type R = crate::R<SECRrs>;
///Register `SECR` writer
pub type W = crate::W<SECRrs>;
///Field `SEC_SIZE` reader - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type SEC_SIZE_R = crate::FieldReader;
///Field `SEC_SIZE` writer - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type SEC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `BOOT_LOCK` reader - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
pub type BOOT_LOCK_R = crate::BitReader;
///Field `BOOT_LOCK` writer - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0x3f) as u8)
    }
    ///Bit 16 - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
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
impl W {
    ///Bits 0:5 - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn sec_size(&mut self) -> SEC_SIZE_W<SECRrs> {
        SEC_SIZE_W::new(self, 0)
    }
    ///Bit 16 - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<SECRrs> {
        BOOT_LOCK_W::new(self, 16)
    }
}
/**FLASH security register

You can [`read`](crate::Reg::read) this register and get [`secr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#FLASH:SECR)*/
pub struct SECRrs;
impl crate::RegisterSpec for SECRrs {
    type Ux = u32;
}
///`read()` method returns [`secr::R`](R) reader structure
impl crate::Readable for SECRrs {}
///`write(|w| ..)` method takes [`secr::W`](W) writer structure
impl crate::Writable for SECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SECR to value 0
impl crate::Resettable for SECRrs {
    const RESET_VALUE: u32 = 0;
}
