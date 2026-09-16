///Register `OTP_DISTURBED1` reader
pub type R = crate::R<OTP_DISTURBED1rs>;
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
        f.debug_struct("OTP_DISTURBED1")
            .field("dis", &self.dis())
            .finish()
    }
}
/**BSEC_OTP_DISTURBED0 is used to report disturbed state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP). BSEC_OTP_DISTURBED1 is used to report disturbed state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63. BSEC_OTP_DISTURBED2 is used to report disturbed state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95.

You can [`read`](crate::Reg::read) this register and get [`otp_disturbed1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_DISTURBED1)*/
pub struct OTP_DISTURBED1rs;
impl crate::RegisterSpec for OTP_DISTURBED1rs {
    type Ux = u32;
}
///`read()` method returns [`otp_disturbed1::R`](R) reader structure
impl crate::Readable for OTP_DISTURBED1rs {}
///`reset()` method sets OTP_DISTURBED1 to value 0
impl crate::Resettable for OTP_DISTURBED1rs {}
