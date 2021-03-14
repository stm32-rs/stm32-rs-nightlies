#[doc = "Reader of register DFSDM_FLT1ISR"]
pub type R = crate::R<u32, super::DFSDM_FLT1ISR>;
#[doc = "Reader of field `JEOCF`"]
pub type JEOCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `REOCF`"]
pub type REOCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `JOVRF`"]
pub type JOVRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROVRF`"]
pub type ROVRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWDF`"]
pub type AWDF_R = crate::R<bool, bool>;
#[doc = "Reader of field `JCIP`"]
pub type JCIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCIP`"]
pub type RCIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CKABF`"]
pub type CKABF_R = crate::R<u8, u8>;
#[doc = "Reader of field `SCDF`"]
pub type SCDF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - JEOCF"]
    #[inline(always)]
    pub fn jeocf(&self) -> JEOCF_R {
        JEOCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - REOCF"]
    #[inline(always)]
    pub fn reocf(&self) -> REOCF_R {
        REOCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - JOVRF"]
    #[inline(always)]
    pub fn jovrf(&self) -> JOVRF_R {
        JOVRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ROVRF"]
    #[inline(always)]
    pub fn rovrf(&self) -> ROVRF_R {
        ROVRF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AWDF"]
    #[inline(always)]
    pub fn awdf(&self) -> AWDF_R {
        AWDF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 13 - JCIP"]
    #[inline(always)]
    pub fn jcip(&self) -> JCIP_R {
        JCIP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RCIP"]
    #[inline(always)]
    pub fn rcip(&self) -> RCIP_R {
        RCIP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - CKABF"]
    #[inline(always)]
    pub fn ckabf(&self) -> CKABF_R {
        CKABF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCDF"]
    #[inline(always)]
    pub fn scdf(&self) -> SCDF_R {
        SCDF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
