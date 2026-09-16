///Register `SECR` reader
pub type R = crate::R<SECRrs>;
///Register `SECR` writer
pub type W = crate::W<SECRrs>;
///Field `SEC_SIZE` reader - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type SEC_SIZE_R = crate::FieldReader;
///Field `SEC_SIZE` writer - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
pub type SEC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOT_LOCK {
    ///0: Boot based on the pad/option bit configuration
    B0x0 = 0,
    ///1: Boot forced from Main flash memory
    B0x1 = 1,
}
impl From<BOOT_LOCK> for bool {
    #[inline(always)]
    fn from(variant: BOOT_LOCK) -> Self {
        variant as u8 != 0
    }
}
///Field `BOOT_LOCK` reader - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
pub type BOOT_LOCK_R = crate::BitReader<BOOT_LOCK>;
impl BOOT_LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOOT_LOCK {
        match self.bits {
            false => BOOT_LOCK::B0x0,
            true => BOOT_LOCK::B0x1,
        }
    }
    ///Boot based on the pad/option bit configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BOOT_LOCK::B0x0
    }
    ///Boot forced from Main flash memory
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BOOT_LOCK::B0x1
    }
}
///Field `BOOT_LOCK` writer - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG, BOOT_LOCK>;
impl<'a, REG> BOOT_LOCK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Boot based on the pad/option bit configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_LOCK::B0x0)
    }
    ///Boot forced from Main flash memory
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BOOT_LOCK::B0x1)
    }
}
impl R {
    ///Bits 0:6 - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0x7f) as u8)
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
    ///Bits 0:6 - Securable memory area size Contains the number of securable flash memory pages. Note: The number of effective bits depends on the size of the flash memory in the device.
    #[inline(always)]
    pub fn sec_size(&mut self) -> SEC_SIZE_W<'_, SECRrs> {
        SEC_SIZE_W::new(self, 0)
    }
    ///Bit 16 - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<'_, SECRrs> {
        BOOT_LOCK_W::new(self, 16)
    }
}
/**FLASH security register

You can [`read`](crate::Reg::read) this register and get [`secr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FLASH:SECR)*/
pub struct SECRrs;
impl crate::RegisterSpec for SECRrs {
    type Ux = u32;
}
///`read()` method returns [`secr::R`](R) reader structure
impl crate::Readable for SECRrs {}
///`write(|w| ..)` method takes [`secr::W`](W) writer structure
impl crate::Writable for SECRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECR to value 0
impl crate::Resettable for SECRrs {}
