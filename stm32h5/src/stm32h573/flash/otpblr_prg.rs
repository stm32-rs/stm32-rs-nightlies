///Register `OTPBLR_PRG` reader
pub type R = crate::R<OTPBLR_PRGrs>;
///Register `OTPBLR_PRG` writer
pub type W = crate::W<OTPBLR_PRGrs>;
///Field `LOCKBL` reader - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared.
pub type LOCKBL_R = crate::FieldReader<u32>;
///Field `LOCKBL` writer - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared.
pub type LOCKBL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared.
    #[inline(always)]
    pub fn lockbl(&self) -> LOCKBL_R {
        LOCKBL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPBLR_PRG")
            .field("lockbl", &self.lockbl())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared.
    #[inline(always)]
    pub fn lockbl(&mut self) -> LOCKBL_W<'_, OTPBLR_PRGrs> {
        LOCKBL_W::new(self, 0)
    }
}
/**FLASH non-secure OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpblr_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpblr_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#FLASH:OTPBLR_PRG)*/
pub struct OTPBLR_PRGrs;
impl crate::RegisterSpec for OTPBLR_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`otpblr_prg::R`](R) reader structure
impl crate::Readable for OTPBLR_PRGrs {}
///`write(|w| ..)` method takes [`otpblr_prg::W`](W) writer structure
impl crate::Writable for OTPBLR_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTPBLR_PRG to value 0
impl crate::Resettable for OTPBLR_PRGrs {}
