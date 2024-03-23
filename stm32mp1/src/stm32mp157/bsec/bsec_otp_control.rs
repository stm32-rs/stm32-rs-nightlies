#[doc = "Register `BSEC_OTP_CONTROL` reader"]
pub type R = crate::R<BSEC_OTP_CONTROLrs>;
#[doc = "Register `BSEC_OTP_CONTROL` writer"]
pub type W = crate::W<BSEC_OTP_CONTROLrs>;
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PROG` reader - PROG"]
pub type PROG_R = crate::BitReader;
#[doc = "Field `PROG` writer - PROG"]
pub type PROG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - LOCK"]
pub type LOCK_R = crate::BitReader;
#[doc = "Field `LOCK` writer - LOCK"]
pub type LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - PROG"]
    #[inline(always)]
    pub fn prog(&self) -> PROG_R {
        PROG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LOCK"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<BSEC_OTP_CONTROLrs> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 8 - PROG"]
    #[inline(always)]
    #[must_use]
    pub fn prog(&mut self) -> PROG_W<BSEC_OTP_CONTROLrs> {
        PROG_W::new(self, 8)
    }
    #[doc = "Bit 9 - LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<BSEC_OTP_CONTROLrs> {
        LOCK_W::new(self, 9)
    }
}
#[doc = "BSEC OTP control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsec_otp_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsec_otp_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSEC_OTP_CONTROLrs;
impl crate::RegisterSpec for BSEC_OTP_CONTROLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsec_otp_control::R`](R) reader structure"]
impl crate::Readable for BSEC_OTP_CONTROLrs {}
#[doc = "`write(|w| ..)` method takes [`bsec_otp_control::W`](W) writer structure"]
impl crate::Writable for BSEC_OTP_CONTROLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BSEC_OTP_CONTROL to value 0"]
impl crate::Resettable for BSEC_OTP_CONTROLrs {
    const RESET_VALUE: u32 = 0;
}
