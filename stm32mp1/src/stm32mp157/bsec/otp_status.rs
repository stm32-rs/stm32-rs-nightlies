///Register `OTP_STATUS` reader
pub type R = crate::R<OTP_STATUSrs>;
///Field `SECURE` reader - SECURE
pub type SECURE_R = crate::BitReader;
///Field `FULLDBG` reader - FULLDBG
pub type FULLDBG_R = crate::BitReader;
///Field `INVALID` reader - INVALID
pub type INVALID_R = crate::BitReader;
///Field `BUSY` reader - BUSY
pub type BUSY_R = crate::BitReader;
///Field `PROGFAIL` reader - PROGFAIL
pub type PROGFAIL_R = crate::BitReader;
///Field `PWRON` reader - PWRON
pub type PWRON_R = crate::BitReader;
///Field `BIST1LOCK` reader - BIST1LOCK
pub type BIST1LOCK_R = crate::BitReader;
///Field `BIST2LOCK` reader - BIST2LOCK
pub type BIST2LOCK_R = crate::BitReader;
impl R {
    ///Bit 0 - SECURE
    #[inline(always)]
    pub fn secure(&self) -> SECURE_R {
        SECURE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FULLDBG
    #[inline(always)]
    pub fn fulldbg(&self) -> FULLDBG_R {
        FULLDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - INVALID
    #[inline(always)]
    pub fn invalid(&self) -> INVALID_R {
        INVALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BUSY
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PROGFAIL
    #[inline(always)]
    pub fn progfail(&self) -> PROGFAIL_R {
        PROGFAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PWRON
    #[inline(always)]
    pub fn pwron(&self) -> PWRON_R {
        PWRON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BIST1LOCK
    #[inline(always)]
    pub fn bist1lock(&self) -> BIST1LOCK_R {
        BIST1LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BIST2LOCK
    #[inline(always)]
    pub fn bist2lock(&self) -> BIST2LOCK_R {
        BIST2LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_STATUS")
            .field("secure", &self.secure())
            .field("fulldbg", &self.fulldbg())
            .field("invalid", &self.invalid())
            .field("busy", &self.busy())
            .field("progfail", &self.progfail())
            .field("pwron", &self.pwron())
            .field("bist1lock", &self.bist1lock())
            .field("bist2lock", &self.bist2lock())
            .finish()
    }
}
/**BSEC OTP status register

You can [`read`](crate::Reg::read) this register and get [`otp_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#BSEC:OTP_STATUS)*/
pub struct OTP_STATUSrs;
impl crate::RegisterSpec for OTP_STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`otp_status::R`](R) reader structure
impl crate::Readable for OTP_STATUSrs {}
///`reset()` method sets OTP_STATUS to value 0
impl crate::Resettable for OTP_STATUSrs {}
