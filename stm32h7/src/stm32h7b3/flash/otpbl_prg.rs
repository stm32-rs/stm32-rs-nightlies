///Register `OTPBL_PRG` reader
pub type R = crate::R<OTPBL_PRGrs>;
///Register `OTPBL_PRG` writer
pub type W = crate::W<OTPBL_PRGrs>;
///Field `LOCKBL` reader - OTP Block Lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBL_CUR is cleared.
pub type LOCKBL_R = crate::FieldReader<u16>;
///Field `LOCKBL` writer - OTP Block Lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBL_CUR is cleared.
pub type LOCKBL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - OTP Block Lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBL_CUR is cleared.
    #[inline(always)]
    pub fn lockbl(&self) -> LOCKBL_R {
        LOCKBL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPBL_PRG")
            .field("lockbl", &self.lockbl())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - OTP Block Lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBL_CUR is cleared.
    #[inline(always)]
    pub fn lockbl(&mut self) -> LOCKBL_W<'_, OTPBL_PRGrs> {
        LOCKBL_W::new(self, 0)
    }
}
/**FLASH OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpbl_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpbl_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#FLASH:OTPBL_PRG)*/
pub struct OTPBL_PRGrs;
impl crate::RegisterSpec for OTPBL_PRGrs {
    type Ux = u32;
}
///`read()` method returns [`otpbl_prg::R`](R) reader structure
impl crate::Readable for OTPBL_PRGrs {}
///`write(|w| ..)` method takes [`otpbl_prg::W`](W) writer structure
impl crate::Writable for OTPBL_PRGrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTPBL_PRG to value 0
impl crate::Resettable for OTPBL_PRGrs {}
