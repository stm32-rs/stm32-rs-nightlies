///Register `OTP_SRLOCK1` reader
pub type R = crate::R<OTP_SRLOCK1rs>;
///Register `OTP_SRLOCK1` writer
pub type W = crate::W<OTP_SRLOCK1rs>;
///Field `SRLOCK` reader - SRLOCK
pub type SRLOCK_R = crate::FieldReader<u32>;
///Field `SRLOCK` writer - SRLOCK
pub type SRLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SRLOCK
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_SRLOCK1")
            .field("srlock", &self.srlock())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SRLOCK
    #[inline(always)]
    pub fn srlock(&mut self) -> SRLOCK_W<OTP_SRLOCK1rs> {
        SRLOCK_W::new(self, 0)
    }
}
/**BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.

You can [`read`](crate::Reg::read) this register and get [`otp_srlock1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_srlock1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:OTP_SRLOCK1)*/
pub struct OTP_SRLOCK1rs;
impl crate::RegisterSpec for OTP_SRLOCK1rs {
    type Ux = u32;
}
///`read()` method returns [`otp_srlock1::R`](R) reader structure
impl crate::Readable for OTP_SRLOCK1rs {}
///`write(|w| ..)` method takes [`otp_srlock1::W`](W) writer structure
impl crate::Writable for OTP_SRLOCK1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTP_SRLOCK1 to value 0
impl crate::Resettable for OTP_SRLOCK1rs {
    const RESET_VALUE: u32 = 0;
}