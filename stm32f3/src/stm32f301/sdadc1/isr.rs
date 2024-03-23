#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISRrs>;
#[doc = "Field `EOCALF` reader - End of calibration flag"]
pub type EOCALF_R = crate::BitReader;
#[doc = "Field `JEOCF` reader - End of injected conversion flag"]
pub type JEOCF_R = crate::BitReader;
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag"]
pub type JOVRF_R = crate::BitReader;
#[doc = "Field `REOCF` reader - End of regular conversion flag"]
pub type REOCF_R = crate::BitReader;
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag"]
pub type ROVRF_R = crate::BitReader;
#[doc = "Field `CALIBIP` reader - Calibration in progress status"]
pub type CALIBIP_R = crate::BitReader;
#[doc = "Field `JCIP` reader - Injected conversion in progress status"]
pub type JCIP_R = crate::BitReader;
#[doc = "Field `RCIP` reader - Regular conversion in progress status"]
pub type RCIP_R = crate::BitReader;
#[doc = "Field `STABIP` reader - Stabilization in progress status"]
pub type STABIP_R = crate::BitReader;
#[doc = "Field `INITRDY` reader - Initialization mode is ready"]
pub type INITRDY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of calibration flag"]
    #[inline(always)]
    pub fn eocalf(&self) -> EOCALF_R {
        EOCALF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of injected conversion flag"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of regular conversion flag"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular conversion overrun flag"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 12 - Calibration in progress status"]
    #[inline(always)]
    pub fn calibip(&self) -> CALIBIP_R {
        CALIBIP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Stabilization in progress status"]
    #[inline(always)]
    pub fn stabip(&self) -> STABIP_R {
        STABIP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Initialization mode is ready"]
    #[inline(always)]
    pub fn initrdy(&self) -> INITRDY_R {
        INITRDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISRrs {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISRrs {
    const RESET_VALUE: u32 = 0;
}
