///Register `FLASH_SECR` reader
pub type R = crate::R<FLASH_SECRrs>;
///Register `FLASH_SECR` writer
pub type W = crate::W<FLASH_SECRrs>;
///Field `HDP1_PEND` reader - Last page of the first hide protection area
pub type HDP1_PEND_R = crate::FieldReader;
///Field `HDP1_PEND` writer - Last page of the first hide protection area
pub type HDP1_PEND_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `BOOT_LOCK` reader - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
pub type BOOT_LOCK_R = crate::BitReader;
///Field `BOOT_LOCK` writer - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDP1EN` reader - Hide protection area enable
pub type HDP1EN_R = crate::FieldReader;
///Field `HDP1EN` writer - Hide protection area enable
pub type HDP1EN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:6 - Last page of the first hide protection area
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 16 - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 24:31 - Hide protection area enable
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SECR")
            .field("hdp1_pend", &self.hdp1_pend())
            .field("boot_lock", &self.boot_lock())
            .field("hdp1en", &self.hdp1en())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Last page of the first hide protection area
    #[inline(always)]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W<FLASH_SECRrs> {
        HDP1_PEND_W::new(self, 0)
    }
    ///Bit 16 - used to force boot from user area If the bit is set in association with RDP level 1, the debug capabilities are disabled, except in the case of a bad OBL (mismatch).
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<FLASH_SECRrs> {
        BOOT_LOCK_W::new(self, 16)
    }
    ///Bits 24:31 - Hide protection area enable
    #[inline(always)]
    pub fn hdp1en(&mut self) -> HDP1EN_W<FLASH_SECRrs> {
        HDP1EN_W::new(self, 24)
    }
}
/**FLASH security register

You can [`read`](crate::Reg::read) this register and get [`flash_secr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_secr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#FLASH:FLASH_SECR)*/
pub struct FLASH_SECRrs;
impl crate::RegisterSpec for FLASH_SECRrs {
    type Ux = u32;
}
///`read()` method returns [`flash_secr::R`](R) reader structure
impl crate::Readable for FLASH_SECRrs {}
///`write(|w| ..)` method takes [`flash_secr::W`](W) writer structure
impl crate::Writable for FLASH_SECRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_SECR to value 0
impl crate::Resettable for FLASH_SECRrs {
    const RESET_VALUE: u32 = 0;
}
