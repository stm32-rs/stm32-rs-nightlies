///Register `NSBOOTR_PRG` reader
pub type R = crate::R<NSBOOTR_PRGrs>;
///Register `NSBOOTR_PRG` writer
pub type W = crate::W<NSBOOTR_PRGrs>;
///Field `NSBOOT_LOCK` reader - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
pub type NSBOOT_LOCK_R = crate::FieldReader;
///Field `NSBOOT_LOCK` writer - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
pub type NSBOOT_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NSBOOTADD` reader - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
pub type NSBOOTADD_R = crate::FieldReader<u32>;
///Field `NSBOOTADD` writer - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
pub type NSBOOTADD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:7 - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
    #[inline(always)]
    pub fn nsboot_lock(&self) -> NSBOOT_LOCK_R {
        NSBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:31 - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
    #[inline(always)]
    pub fn nsbootadd(&self) -> NSBOOTADD_R {
        NSBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NSBOOTR_PRG")
            .field("nsboot_lock", &self.nsboot_lock())
            .field("nsbootadd", &self.nsbootadd())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - A field locking the values of SWAP_ BANK, and NSBOOTADD settings.
    #[inline(always)]
    pub fn nsboot_lock(&mut self) -> NSBOOT_LOCK_W<'_, NSBOOTR_PRGrs> {
        NSBOOT_LOCK_W::new(self, 0)
    }
    ///Bits 8:31 - Non secure unique boot entry address These bits allow configuring the Non secure BOOT address
    #[inline(always)]
    pub fn nsbootadd(&mut self) -> NSBOOTADD_W<'_, NSBOOTR_PRGrs> {
        NSBOOTADD_W::new(self, 8)
    }
}
/**FLASH non-secure boot register

You can [`read`](crate::Reg::read) this register and get [`nsbootr_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nsbootr_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FLASH:NSBOOTR_PRG)*/
pub struct NSBOOTR_PRGrs;
impl crate::RegisterSpec for NSBOOTR_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`nsbootr_prg::R`](R) reader structure
impl crate::Readable for NSBOOTR_PRGrs {}
///`write(|w| ..)` method takes [`nsbootr_prg::W`](W) writer structure
impl crate::Writable for NSBOOTR_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NSBOOTR_PRG to value 0
impl crate::Resettable for NSBOOTR_PRGrs {}
