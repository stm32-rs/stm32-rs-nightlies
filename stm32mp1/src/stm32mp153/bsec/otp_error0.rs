///Register `OTP_ERROR0` reader
pub type R = crate::R<OTP_ERROR0rs>;
///Field `ERR` reader - ERR
pub type ERR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ERR
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_ERROR0")
            .field("err", &self.err())
            .finish()
    }
}
/**BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.

You can [`read`](crate::Reg::read) this register and get [`otp_error0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:OTP_ERROR0)*/
pub struct OTP_ERROR0rs;
impl crate::RegisterSpec for OTP_ERROR0rs {
    type Ux = u32;
}
///`read()` method returns [`otp_error0::R`](R) reader structure
impl crate::Readable for OTP_ERROR0rs {}
///`reset()` method sets OTP_ERROR0 to value 0
impl crate::Resettable for OTP_ERROR0rs {}
