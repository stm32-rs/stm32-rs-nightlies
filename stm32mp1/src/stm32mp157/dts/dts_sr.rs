#[doc = "Reader of register DTS_SR"]
pub type R = crate::R<u32, super::DTS_SR>;
#[doc = "Reader of field `TS1_ITEF`"]
pub type TS1_ITEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TS1_ITLF`"]
pub type TS1_ITLF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TS1_ITHF`"]
pub type TS1_ITHF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TS1_AITEF`"]
pub type TS1_AITEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TS1_AITLF`"]
pub type TS1_AITLF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TS1_AITHF`"]
pub type TS1_AITHF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TS1_RDY`"]
pub type TS1_RDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TS1_ITEF"]
    #[inline(always)]
    pub fn ts1_itef(&self) -> TS1_ITEF_R {
        TS1_ITEF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TS1_ITLF"]
    #[inline(always)]
    pub fn ts1_itlf(&self) -> TS1_ITLF_R {
        TS1_ITLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TS1_ITHF"]
    #[inline(always)]
    pub fn ts1_ithf(&self) -> TS1_ITHF_R {
        TS1_ITHF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_AITEF"]
    #[inline(always)]
    pub fn ts1_aitef(&self) -> TS1_AITEF_R {
        TS1_AITEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TS1_AITLF"]
    #[inline(always)]
    pub fn ts1_aitlf(&self) -> TS1_AITLF_R {
        TS1_AITLF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TS1_AITHF"]
    #[inline(always)]
    pub fn ts1_aithf(&self) -> TS1_AITHF_R {
        TS1_AITHF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TS1_RDY"]
    #[inline(always)]
    pub fn ts1_rdy(&self) -> TS1_RDY_R {
        TS1_RDY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
