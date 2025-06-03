///Register `OTPSR` reader
pub type R = crate::R<OTPSRrs>;
///Field `BUSY` reader - Busy flag
pub type BUSY_R = crate::BitReader;
///Field `INIT_DONE` reader - Initialization done
pub type INIT_DONE_R = crate::BitReader;
///Field `HIDEUP` reader - Hide upper fuse words
pub type HIDEUP_R = crate::BitReader;
///Field `OTPNVIR` reader - OTP not virgin
pub type OTPNVIR_R = crate::BitReader;
///Field `OTPERR` reader - OTP with error
pub type OTPERR_R = crate::BitReader;
///Field `OTPSEC` reader - OTP with single error correction
pub type OTPSEC_R = crate::BitReader;
///Field `PROGFAIL` reader - Programming failed
pub type PROGFAIL_R = crate::BitReader;
///Field `DISTURBF` reader - Disturb flag
pub type DISTURBF_R = crate::BitReader;
///Field `DEDF` reader - Double error detection flag
pub type DEDF_R = crate::BitReader;
///Field `SECF` reader - Single error correction flag
pub type SECF_R = crate::BitReader;
///Field `PPLF` reader - Permanent programming lock flag
pub type PPLF_R = crate::BitReader;
///Field `PPLMF` reader - Permanent programming lock mismatch flag
pub type PPLMF_R = crate::BitReader;
///Field `AMEF` reader - Addresses mismatch error flag
pub type AMEF_R = crate::BitReader;
impl R {
    ///Bit 0 - Busy flag
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Initialization done
    #[inline(always)]
    pub fn init_done(&self) -> INIT_DONE_R {
        INIT_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Hide upper fuse words
    #[inline(always)]
    pub fn hideup(&self) -> HIDEUP_R {
        HIDEUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - OTP not virgin
    #[inline(always)]
    pub fn otpnvir(&self) -> OTPNVIR_R {
        OTPNVIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - OTP with error
    #[inline(always)]
    pub fn otperr(&self) -> OTPERR_R {
        OTPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - OTP with single error correction
    #[inline(always)]
    pub fn otpsec(&self) -> OTPSEC_R {
        OTPSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - Programming failed
    #[inline(always)]
    pub fn progfail(&self) -> PROGFAIL_R {
        PROGFAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Disturb flag
    #[inline(always)]
    pub fn disturbf(&self) -> DISTURBF_R {
        DISTURBF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Double error detection flag
    #[inline(always)]
    pub fn dedf(&self) -> DEDF_R {
        DEDF_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Single error correction flag
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Permanent programming lock flag
    #[inline(always)]
    pub fn pplf(&self) -> PPLF_R {
        PPLF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Permanent programming lock mismatch flag
    #[inline(always)]
    pub fn pplmf(&self) -> PPLMF_R {
        PPLMF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Addresses mismatch error flag
    #[inline(always)]
    pub fn amef(&self) -> AMEF_R {
        AMEF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTPSR")
            .field("busy", &self.busy())
            .field("init_done", &self.init_done())
            .field("hideup", &self.hideup())
            .field("otpnvir", &self.otpnvir())
            .field("otperr", &self.otperr())
            .field("otpsec", &self.otpsec())
            .field("progfail", &self.progfail())
            .field("disturbf", &self.disturbf())
            .field("dedf", &self.dedf())
            .field("secf", &self.secf())
            .field("pplf", &self.pplf())
            .field("pplmf", &self.pplmf())
            .field("amef", &self.amef())
            .finish()
    }
}
/**BSEC OTP status register

You can [`read`](crate::Reg::read) this register and get [`otpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#BSEC:OTPSR)*/
pub struct OTPSRrs;
impl crate::RegisterSpec for OTPSRrs {
    type Ux = u32;
}
///`read()` method returns [`otpsr::R`](R) reader structure
impl crate::Readable for OTPSRrs {}
///`reset()` method sets OTPSR to value 0
impl crate::Resettable for OTPSRrs {}
