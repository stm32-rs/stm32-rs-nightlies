///Register `OTPBL_CUR` reader
pub type R = crate::R<OTPBL_CURrs>;
///Field `LOCKBL` reader - OTP Block Lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified.
pub type LOCKBL_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - OTP Block Lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\[n\] = 1 indicates that all OTP 16-bit words in OTP Block n are locked and can no longer be programmed. LOCKBL\[n\] = 0 indicates that all OTP 16-bit words in OTP Block n are not locked and can still be modified.
    #[inline(always)]
    pub fn lockbl(&self) -> LOCKBL_R {
        LOCKBL_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPBL_CUR")
            .field("lockbl", &self.lockbl())
            .finish()
    }
}
/**FLASH OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpbl_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B3.html#FLASH:OTPBL_CUR)*/
pub struct OTPBL_CURrs;
impl crate::RegisterSpec for OTPBL_CURrs {
    type Ux = u32;
}
///`read()` method returns [`otpbl_cur::R`](R) reader structure
impl crate::Readable for OTPBL_CURrs {}
///`reset()` method sets OTPBL_CUR to value 0
impl crate::Resettable for OTPBL_CURrs {}
