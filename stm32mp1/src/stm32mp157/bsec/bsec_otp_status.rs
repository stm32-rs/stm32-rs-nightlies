#[doc = "Register `BSEC_OTP_STATUS` reader"]
pub type R = crate::R<BSEC_OTP_STATUSrs>;
#[doc = "Field `SECURE` reader - SECURE"]
pub type SECURE_R = crate::BitReader;
#[doc = "Field `FULLDBG` reader - FULLDBG"]
pub type FULLDBG_R = crate::BitReader;
#[doc = "Field `INVALID` reader - INVALID"]
pub type INVALID_R = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `PROGFAIL` reader - PROGFAIL"]
pub type PROGFAIL_R = crate::BitReader;
#[doc = "Field `PWRON` reader - PWRON"]
pub type PWRON_R = crate::BitReader;
#[doc = "Field `BIST1LOCK` reader - BIST1LOCK"]
pub type BIST1LOCK_R = crate::BitReader;
#[doc = "Field `BIST2LOCK` reader - BIST2LOCK"]
pub type BIST2LOCK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SECURE"]
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FULLDBG"]
    #[inline(always)]
    pub fn fulldbg(&self) -> FULLDBG_R {
        FULLDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INVALID"]
    #[inline(always)]
    pub fn invalid(&self) -> INVALID_R {
        INVALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PROGFAIL"]
    #[inline(always)]
    pub fn progfail(&self) -> PROGFAIL_R {
        PROGFAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWRON"]
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BIST1LOCK"]
    #[inline(always)]
    pub fn bist1lock(&self) -> BIST1LOCK_R {
        BIST1LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BIST2LOCK"]
    #[inline(always)]
    pub fn bist2lock(&self) -> BIST2LOCK_R {
        BIST2LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "BSEC OTP status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_STATUSrs;
impl crate::RegisterSpec for BSEC_OTP_STATUSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_status::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_STATUSrs {}
#[doc = "`reset()` method sets BSEC_OTP_STATUS to value 0"]
impl crate::Resettable for BSEC_OTP_STATUSrs {
    const RESET_VALUE: u32 = 0;
}
