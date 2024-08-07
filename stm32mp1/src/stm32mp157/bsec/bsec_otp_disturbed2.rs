///Register `BSEC_OTP_DISTURBED2` reader
pub type R = crate::R<BSEC_OTP_DISTURBED2rs>;
///Field `DIS` reader - DIS
pub type DIS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - DIS
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSEC_OTP_DISTURBED2")
            .field("dis", &self.dis())
            .finish()
    }
}
/**BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_disturbed2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:BSEC_OTP_DISTURBED2)*/
pub struct BSEC_OTP_DISTURBED2rs;
impl crate::RegisterSpec for BSEC_OTP_DISTURBED2rs {
    type Ux = u32;
}
///`read()` method returns [`bsec_otp_disturbed2::R`](R) reader structure
impl crate::Readable for BSEC_OTP_DISTURBED2rs {}
///`reset()` method sets BSEC_OTP_DISTURBED2 to value 0
impl crate::Resettable for BSEC_OTP_DISTURBED2rs {
    const RESET_VALUE: u32 = 0;
}
