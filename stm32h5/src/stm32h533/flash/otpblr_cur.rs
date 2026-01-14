///Register `OTPBLR_CUR` reader
pub type R = crate::R<OTPBLR_CURrs>;
///Field `LOCKBL` reader - OTP block lock
pub type LOCKBL_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - OTP block lock
    #[inline(always)]
    pub fn lockbl(&self) -> LOCKBL_R {
        LOCKBL_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPBLR_CUR")
            .field("lockbl", &self.lockbl())
            .finish()
    }
}
/**FLASH non-secure OTP block lock

You can [`read`](crate::Reg::read) this register and get [`otpblr_cur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H533.html#FLASH:OTPBLR_CUR)*/
pub struct OTPBLR_CURrs;
impl crate::RegisterSpec for OTPBLR_CURrs {
    type Ux = u32;
}
///`read()` method returns [`otpblr_cur::R`](R) reader structure
impl crate::Readable for OTPBLR_CURrs {}
///`reset()` method sets OTPBLR_CUR to value 0
impl crate::Resettable for OTPBLR_CURrs {}
