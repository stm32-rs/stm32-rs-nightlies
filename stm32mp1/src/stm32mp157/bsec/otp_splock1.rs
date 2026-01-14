///Register `OTP_SPLOCK1` reader
pub type R = crate::R<OTP_SPLOCK1rs>;
///Register `OTP_SPLOCK1` writer
pub type W = crate::W<OTP_SPLOCK1rs>;
///Field `SPLOCK` reader - SPLOCK
pub type SPLOCK_R = crate::FieldReader<u32>;
///Field `SPLOCK` writer - SPLOCK
pub type SPLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SPLOCK
    #[inline(always)]
    pub fn splock(&self) -> SPLOCK_R {
        SPLOCK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_SPLOCK1")
            .field("splock", &self.splock())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SPLOCK
    #[inline(always)]
    pub fn splock(&mut self) -> SPLOCK_W<'_, OTP_SPLOCK1rs> {
        SPLOCK_W::new(self, 0)
    }
}
/**BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.

You can [`read`](crate::Reg::read) this register and get [`otp_splock1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_splock1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:OTP_SPLOCK1)*/
pub struct OTP_SPLOCK1rs;
impl crate::RegisterSpec for OTP_SPLOCK1rs {
    type Ux = u32;
}
///`read()` method returns [`otp_splock1::R`](R) reader structure
impl crate::Readable for OTP_SPLOCK1rs {}
///`write(|w| ..)` method takes [`otp_splock1::W`](W) writer structure
impl crate::Writable for OTP_SPLOCK1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTP_SPLOCK1 to value 0
impl crate::Resettable for OTP_SPLOCK1rs {}
