#[doc = "Register `BSEC_OTP_LOCK` reader"]
pub type R = crate::R<BSEC_OTP_LOCKrs>;
#[doc = "Register `BSEC_OTP_LOCK` writer"]
pub type W = crate::W<BSEC_OTP_LOCKrs>;
#[doc = "Field `OTP` reader - OTP"]
pub type OTP_R = crate::BitReader;
#[doc = "Field `OTP` writer - OTP"]
pub type OTP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROMLOCK` reader - ROMLOCK"]
pub type ROMLOCK_R = crate::BitReader;
#[doc = "Field `ROMLOCK` writer - ROMLOCK"]
pub type ROMLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DENREG` reader - DENREG"]
pub type DENREG_R = crate::BitReader;
#[doc = "Field `DENREG` writer - DENREG"]
pub type DENREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPLOCK` reader - GPLOCK"]
pub type GPLOCK_R = crate::BitReader;
#[doc = "Field `GPLOCK` writer - GPLOCK"]
pub type GPLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    pub fn romlock(&self) -> ROMLOCK_R {
        ROMLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    pub fn denreg(&self) -> DENREG_R {
        DENREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    pub fn gplock(&self) -> GPLOCK_R {
        GPLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OTP"]
    #[inline(always)]
    #[must_use]
    pub fn otp(&mut self) -> OTP_W<BSEC_OTP_LOCKrs> {
        OTP_W::new(self, 0)
    }
    #[doc = "Bit 1 - ROMLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn romlock(&mut self) -> ROMLOCK_W<BSEC_OTP_LOCKrs> {
        ROMLOCK_W::new(self, 1)
    }
    #[doc = "Bit 2 - DENREG"]
    #[inline(always)]
    #[must_use]
    pub fn denreg(&mut self) -> DENREG_W<BSEC_OTP_LOCKrs> {
        DENREG_W::new(self, 2)
    }
    #[doc = "Bit 4 - GPLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn gplock(&mut self) -> GPLOCK_W<BSEC_OTP_LOCKrs> {
        GPLOCK_W::new(self, 4)
    }
}
#[doc = "BSEC OTP lock configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_otp_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_LOCKrs;
impl crate::RegisterSpec for BSEC_OTP_LOCKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_lock::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_LOCKrs {}
#[doc = "`write(|w| ..)` method takes [`bsec_otp_lock::W`](W) writer structure"]
impl crate::Writable for BSEC_OTP_LOCKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_LOCK to value 0"]
impl crate::Resettable for BSEC_OTP_LOCKrs {
    const RESET_VALUE: u32 = 0;
}
