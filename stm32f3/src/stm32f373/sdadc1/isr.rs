#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `INITRDY`"]
pub type INITRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `STABIP`"]
pub type STABIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCIP`"]
pub type RCIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `JCIP`"]
pub type JCIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CALIBIP`"]
pub type CALIBIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROVRF`"]
pub type ROVRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REOCF`"]
pub type REOCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `JOVRF`"]
pub type JOVRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `JEOCF`"]
pub type JEOCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOCALF`"]
pub type EOCALF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 31 - Initialization mode is ready"]
    #[inline(always)]
    pub fn initrdy(&self) -> INITRDY_R {
        INITRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Stabilization in progress status"]
    #[inline(always)]
    pub fn stabip(&self) -> STABIP_R {
        STABIP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Calibration in progress status"]
    #[inline(always)]
    pub fn calibip(&self) -> CALIBIP_R {
        CALIBIP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Regular conversion overrun flag"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of regular conversion flag"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of injected conversion flag"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - End of calibration flag"]
    #[inline(always)]
    pub fn eocalf(&self) -> EOCALF_R {
        EOCALF_R::new((self.bits & 0x01) != 0)
    }
}
