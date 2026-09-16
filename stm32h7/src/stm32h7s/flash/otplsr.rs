///Register `OTPLSR` reader
pub type R = crate::R<OTPLSRrs>;
///Field `OTPL` reader - OTP lock n Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. OTPL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. OTPL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified.
pub type OTPL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - OTP lock n Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. OTPL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. OTPL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified.
    #[inline(always)]
    pub fn otpl(&self) -> OTPL_R {
        OTPL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPLSR")
            .field("otpl", &self.otpl())
            .finish()
    }
}
/**FLASH OTP lock status register

You can [`read`](crate::Reg::read) this register and get [`otplsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#FLASH:OTPLSR)*/
pub struct OTPLSRrs;
impl crate::RegisterSpec for OTPLSRrs {
    type Ux = u32;
}
///`read()` method returns [`otplsr::R`](R) reader structure
impl crate::Readable for OTPLSRrs {}
///`reset()` method sets OTPLSR to value 0
impl crate::Resettable for OTPLSRrs {}
