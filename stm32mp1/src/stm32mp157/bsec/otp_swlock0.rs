///Register `OTP_SWLOCK0` reader
pub type R = crate::R<OTP_SWLOCK0rs>;
///Register `OTP_SWLOCK0` writer
pub type W = crate::W<OTP_SWLOCK0rs>;
///Field `SWLOCK` reader - SWLOCK
pub type SWLOCK_R = crate::FieldReader<u32>;
///Field `SWLOCK` writer - SWLOCK
pub type SWLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - SWLOCK
    #[inline(always)]
    pub fn swlock(&self) -> SWLOCK_R {
        SWLOCK_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_SWLOCK0")
            .field("swlock", &self.swlock())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - SWLOCK
    #[inline(always)]
    pub fn swlock(&mut self) -> SWLOCK_W<'_, OTP_SWLOCK0rs> {
        SWLOCK_W::new(self, 0)
    }
}
/**BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.

You can [`read`](crate::Reg::read) this register and get [`otp_swlock0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_swlock0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:OTP_SWLOCK0)*/
pub struct OTP_SWLOCK0rs;
impl crate::RegisterSpec for OTP_SWLOCK0rs {
    type Ux = u32;
}
///`read()` method returns [`otp_swlock0::R`](R) reader structure
impl crate::Readable for OTP_SWLOCK0rs {}
///`write(|w| ..)` method takes [`otp_swlock0::W`](W) writer structure
impl crate::Writable for OTP_SWLOCK0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTP_SWLOCK0 to value 0x01
impl crate::Resettable for OTP_SWLOCK0rs {
    const RESET_VALUE: u32 = 0x01;
}
