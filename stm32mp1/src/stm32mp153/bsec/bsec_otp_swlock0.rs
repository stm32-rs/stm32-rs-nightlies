#[doc = "Register `BSEC_OTP_SWLOCK0` reader"]
pub type R = crate::R<BSEC_OTP_SWLOCK0rs>;
#[doc = "Register `BSEC_OTP_SWLOCK0` writer"]
pub type W = crate::W<BSEC_OTP_SWLOCK0rs>;
#[doc = "Field `SWLOCK` reader - SWLOCK"]
pub type SWLOCK_R = crate::FieldReader<u32>;
#[doc = "Field `SWLOCK` writer - SWLOCK"]
pub type SWLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SWLOCK"]
    #[inline(always)]
    pub fn swlock(&self) -> SWLOCK_R {
        SWLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SWLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn swlock(&mut self) -> SWLOCK_W<BSEC_OTP_SWLOCK0rs> {
        SWLOCK_W::new(self, 0)
    }
}
#[doc = "BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_swlock0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_otp_swlock0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_SWLOCK0rs;
impl crate::RegisterSpec for BSEC_OTP_SWLOCK0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_swlock0::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_SWLOCK0rs {}
#[doc = "`write(|w| ..)` method takes [`bsec_otp_swlock0::W`](W) writer structure"]
impl crate::Writable for BSEC_OTP_SWLOCK0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_SWLOCK0 to value 0x01"]
impl crate::Resettable for BSEC_OTP_SWLOCK0rs {
    const RESET_VALUE: u32 = 0x01;
}
