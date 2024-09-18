///Register `BSEC_OTP_LOCK` reader
pub type R = crate::R<BSEC_OTP_LOCKrs>;
///Register `BSEC_OTP_LOCK` writer
pub type W = crate::W<BSEC_OTP_LOCKrs>;
///Field `OTP` reader - OTP
pub type OTP_R = crate::BitReader;
///Field `OTP` writer - OTP
pub type OTP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROMLOCK` reader - ROMLOCK
pub type ROMLOCK_R = crate::BitReader;
///Field `ROMLOCK` writer - ROMLOCK
pub type ROMLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DENREG` reader - DENREG
pub type DENREG_R = crate::BitReader;
///Field `DENREG` writer - DENREG
pub type DENREG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPLOCK` reader - GPLOCK
pub type GPLOCK_R = crate::BitReader;
///Field `GPLOCK` writer - GPLOCK
pub type GPLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - OTP
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ROMLOCK
    #[inline(always)]
    pub fn romlock(&self) -> ROMLOCK_R {
        ROMLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DENREG
    #[inline(always)]
    pub fn denreg(&self) -> DENREG_R {
        DENREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - GPLOCK
    #[inline(always)]
    pub fn gplock(&self) -> GPLOCK_R {
        GPLOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BSEC_OTP_LOCK")
            .field("otp", &self.otp())
            .field("romlock", &self.romlock())
            .field("denreg", &self.denreg())
            .field("gplock", &self.gplock())
            .finish()
    }
}
impl W {
    ///Bit 0 - OTP
    #[inline(always)]
    #[must_use]
    pub fn otp(&mut self) -> OTP_W<BSEC_OTP_LOCKrs> {
        OTP_W::new(self, 0)
    }
    ///Bit 1 - ROMLOCK
    #[inline(always)]
    #[must_use]
    pub fn romlock(&mut self) -> ROMLOCK_W<BSEC_OTP_LOCKrs> {
        ROMLOCK_W::new(self, 1)
    }
    ///Bit 2 - DENREG
    #[inline(always)]
    #[must_use]
    pub fn denreg(&mut self) -> DENREG_W<BSEC_OTP_LOCKrs> {
        DENREG_W::new(self, 2)
    }
    ///Bit 4 - GPLOCK
    #[inline(always)]
    #[must_use]
    pub fn gplock(&mut self) -> GPLOCK_W<BSEC_OTP_LOCKrs> {
        GPLOCK_W::new(self, 4)
    }
}
/**BSEC OTP lock configuration register

You can [`read`](crate::Reg::read) this register and get [`bsec_otp_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsec_otp_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#BSEC:BSEC_OTP_LOCK)*/
pub struct BSEC_OTP_LOCKrs;
impl crate::RegisterSpec for BSEC_OTP_LOCKrs {
    type Ux = u32;
}
///`read()` method returns [`bsec_otp_lock::R`](R) reader structure
impl crate::Readable for BSEC_OTP_LOCKrs {}
///`write(|w| ..)` method takes [`bsec_otp_lock::W`](W) writer structure
impl crate::Writable for BSEC_OTP_LOCKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BSEC_OTP_LOCK to value 0
impl crate::Resettable for BSEC_OTP_LOCKrs {
    const RESET_VALUE: u32 = 0;
}
