#[doc = "Register `UR16` reader"]
pub type R = crate::R<UR16rs>;
#[doc = "Field `FZIWDGSTP` reader - Freeze independent watchdog in Stop mode"]
pub type FZIWDGSTP_R = crate::BitReader;
#[doc = "Field `PKP` reader - Private key programmed"]
pub type PKP_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Freeze independent watchdog in Stop mode"]
    #[inline(always)]
    pub fn fziwdgstp(&self) -> FZIWDGSTP_R {
        FZIWDGSTP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Private key programmed"]
    #[inline(always)]
    pub fn pkp(&self) -> PKP_R {
        PKP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ur16::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UR16rs;
impl crate::RegisterSpec for UR16rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur16::R`](R) reader structure"]
impl crate::Readable for UR16rs {}
#[doc = "`reset()` method sets UR16 to value 0"]
impl crate::Resettable for UR16rs {
    const RESET_VALUE: u32 = 0;
}
