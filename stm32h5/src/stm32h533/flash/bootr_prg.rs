///Register `BOOTR_PRG` reader
pub type R = crate::R<BOOTR_PRGrs>;
///Register `BOOTR_PRG` writer
pub type W = crate::W<BOOTR_PRGrs>;
///Field `SECBOOT_LOCK` reader - Field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting.
pub type SECBOOT_LOCK_R = crate::FieldReader;
///Field `SECBOOT_LOCK` writer - Field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting.
pub type SECBOOT_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SECBOOTADD` reader - Secure unique boot entry address.
pub type SECBOOTADD_R = crate::FieldReader<u32>;
///Field `SECBOOTADD` writer - Secure unique boot entry address.
pub type SECBOOTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:7 - Field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting.
    #[inline(always)]
    pub fn secboot_lock(&self) -> SECBOOT_LOCK_R {
        SECBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31 - Secure unique boot entry address.
    #[inline(always)]
    pub fn secbootadd(&self) -> SECBOOTADD_R {
        SECBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOTR_PRG")
            .field("secboot_lock", &self.secboot_lock())
            .field("secbootadd", &self.secbootadd())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Field locking the values of UBE, SWAP_ BANK, and SECBOOTADD setting.
    #[inline(always)]
    pub fn secboot_lock(&mut self) -> SECBOOT_LOCK_W<'_, BOOTR_PRGrs> {
        SECBOOT_LOCK_W::new(self, 0)
    }
    ///Bits 8:31 - Secure unique boot entry address.
    #[inline(always)]
    pub fn secbootadd(&mut self) -> SECBOOTADD_W<'_, BOOTR_PRGrs> {
        SECBOOTADD_W::new(self, 8)
    }
}
/**FLASH secure boot register

You can [`read`](crate::Reg::read) this register and get [`bootr_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootr_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:BOOTR_PRG)*/
pub struct BOOTR_PRGrs;
impl crate::RegisterSpec for BOOTR_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`bootr_prg::R`](R) reader structure
impl crate::Readable for BOOTR_PRGrs {}
///`write(|w| ..)` method takes [`bootr_prg::W`](W) writer structure
impl crate::Writable for BOOTR_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BOOTR_PRG to value 0
impl crate::Resettable for BOOTR_PRGrs {}
