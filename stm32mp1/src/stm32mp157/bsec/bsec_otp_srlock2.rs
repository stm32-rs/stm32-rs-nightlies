#[doc = "Register `BSEC_OTP_SRLOCK2` reader"]
pub type R = crate::R<BSEC_OTP_SRLOCK2rs>;
#[doc = "Register `BSEC_OTP_SRLOCK2` writer"]
pub type W = crate::W<BSEC_OTP_SRLOCK2rs>;
#[doc = "Field `SRLOCK` reader - SRLOCK"]
pub type SRLOCK_R = crate::FieldReader<u32>;
#[doc = "Field `SRLOCK` writer - SRLOCK"]
pub type SRLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SRLOCK"]
    #[inline(always)]
    pub fn srlock(&self) -> SRLOCK_R {
        SRLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SRLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn srlock(&mut self) -> SRLOCK_W<BSEC_OTP_SRLOCK2rs> {
        SRLOCK_W::new(self, 0)
    }
}
#[doc = "BSEC_OTP_SRLOCK0 is used to prevent reloading of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SRLOCK1 is used to prevent reloading of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SRLOCK2 is used to prevent reloading of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Setting SRLOCK bits or attempt to reload a locked OTP do not clear the corresponding BSEC_OTP_DATAx shadow register. BSEC_OTP_SRLOCK0 bit 0 is controlled by hardware according to fuse_ok, writing to this bit has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_srlock2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_otp_srlock2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_SRLOCK2rs;
impl crate::RegisterSpec for BSEC_OTP_SRLOCK2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_srlock2::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_SRLOCK2rs {}
#[doc = "`write(|w| ..)` method takes [`bsec_otp_srlock2::W`](W) writer structure"]
impl crate::Writable for BSEC_OTP_SRLOCK2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_SRLOCK2 to value 0"]
impl crate::Resettable for BSEC_OTP_SRLOCK2rs {
    const RESET_VALUE: u32 = 0;
}
