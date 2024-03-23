#[doc = "Register `BSEC_OTP_SPLOCK2` reader"]
pub type R = crate::R<BSEC_OTP_SPLOCK2rs>;
#[doc = "Register `BSEC_OTP_SPLOCK2` writer"]
pub type W = crate::W<BSEC_OTP_SPLOCK2rs>;
#[doc = "Field `SPLOCK` reader - SPLOCK"]
pub type SPLOCK_R = crate::FieldReader<u32>;
#[doc = "Field `SPLOCK` writer - SPLOCK"]
pub type SPLOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    pub fn splock(&self) -> SPLOCK_R {
        SPLOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn splock(&mut self) -> SPLOCK_W<BSEC_OTP_SPLOCK2rs> {
        SPLOCK_W::new(self, 0)
    }
}
#[doc = "BSEC_OTP_SPLOCK0 is used to lock the programming of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset BSEC_OTP_SPLOCK1 is used to lock the programming of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset BSEC_OTP_SPLOCK2 is used to lock the programming of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset Attempt to sticky program locked OTP word are silently ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_splock2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_otp_splock2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_SPLOCK2rs;
impl crate::RegisterSpec for BSEC_OTP_SPLOCK2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_splock2::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_SPLOCK2rs {}
#[doc = "`write(|w| ..)` method takes [`bsec_otp_splock2::W`](W) writer structure"]
impl crate::Writable for BSEC_OTP_SPLOCK2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_SPLOCK2 to value 0"]
impl crate::Resettable for BSEC_OTP_SPLOCK2rs {
    const RESET_VALUE: u32 = 0;
}
