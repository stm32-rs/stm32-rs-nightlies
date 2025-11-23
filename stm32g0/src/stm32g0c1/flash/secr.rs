///Register `SECR` reader
pub type R = crate::R<SECRrs>;
///Register `SECR` writer
pub type W = crate::W<SECRrs>;
///Field `SEC_SIZE` reader - Securable memory area size
pub type SEC_SIZE_R = crate::FieldReader;
///Field `SEC_SIZE` writer - Securable memory area size
pub type SEC_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BOOT_LOCK` reader - used to force boot from user area
pub type BOOT_LOCK_R = crate::BitReader;
///Field `BOOT_LOCK` writer - used to force boot from user area
pub type BOOT_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEC_SIZE2` reader - Securable memory area size
pub type SEC_SIZE2_R = crate::FieldReader;
///Field `SEC_SIZE2` writer - Securable memory area size
pub type SEC_SIZE2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Securable memory area size
    #[inline(always)]
    pub fn sec_size(&self) -> SEC_SIZE_R {
        SEC_SIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - used to force boot from user area
    #[inline(always)]
    pub fn boot_lock(&self) -> BOOT_LOCK_R {
        BOOT_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:27 - Securable memory area size
    #[inline(always)]
    pub fn sec_size2(&self) -> SEC_SIZE2_R {
        SEC_SIZE2_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECR")
            .field("sec_size", &self.sec_size())
            .field("boot_lock", &self.boot_lock())
            .field("sec_size2", &self.sec_size2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Securable memory area size
    #[inline(always)]
    pub fn sec_size(&mut self) -> SEC_SIZE_W<'_, SECRrs> {
        SEC_SIZE_W::new(self, 0)
    }
    ///Bit 16 - used to force boot from user area
    #[inline(always)]
    pub fn boot_lock(&mut self) -> BOOT_LOCK_W<'_, SECRrs> {
        BOOT_LOCK_W::new(self, 16)
    }
    ///Bits 20:27 - Securable memory area size
    #[inline(always)]
    pub fn sec_size2(&mut self) -> SEC_SIZE2_W<'_, SECRrs> {
        SEC_SIZE2_W::new(self, 20)
    }
}
/**Flash Security register

You can [`read`](crate::Reg::read) this register and get [`secr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0C1.html#FLASH:SECR)*/
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
///`reset()` method sets SECR to value 0xf000_0000
impl crate::Resettable for SECRrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
