///Register `OTP_WRLOCK1` reader
pub type R = crate::R<OTP_WRLOCK1rs>;
///Field `WRLOCK` reader - WRLOCK
pub type WRLOCK_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - WRLOCK
    #[inline(always)]
    pub fn wrlock(&self) -> WRLOCK_R {
        WRLOCK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_WRLOCK1")
            .field("wrlock", &self.wrlock())
            .finish()
    }
}
/**BSEC_OTP_WLOCK0 is used to report permanent write lock of BSEC_OTP_DATA0 to BSEC_OTP_DATA31. BSEC_OTP_WLOCK1 is used to report permanent write lock of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_WLOCK2 is used to report permanent write lock of BSEC_OTP_DATA64 to BSEC_OTP_DATA95. Permanent write lock requires a programming sequence to lock a word (see section:Section3.3.6: OTP operations on page178).

You can [`read`](crate::Reg::read) this register and get [`otp_wrlock1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_WRLOCK1)*/
pub struct OTP_WRLOCK1rs;
impl crate::RegisterSpec for OTP_WRLOCK1rs {
    type Ux = u32;
}
///`read()` method returns [`otp_wrlock1::R`](R) reader structure
impl crate::Readable for OTP_WRLOCK1rs {}
///`reset()` method sets OTP_WRLOCK1 to value 0
impl crate::Resettable for OTP_WRLOCK1rs {
    const RESET_VALUE: u32 = 0;
}