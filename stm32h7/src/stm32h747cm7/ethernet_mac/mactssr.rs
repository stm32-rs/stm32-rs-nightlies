#[doc = "Reader of register MACTSSR"]
pub type R = crate::R<u32, super::MACTSSR>;
#[doc = "Reader of field `TSSOVF`"]
pub type TSSOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTARGT0`"]
pub type TSTARGT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXTSTRIG`"]
pub type AUXTSTRIG_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTRGTERR0`"]
pub type TSTRGTERR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXTSSIS`"]
pub type TXTSSIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ATSSTN`"]
pub type ATSSTN_R = crate::R<u8, u8>;
#[doc = "Reader of field `ATSSTM`"]
pub type ATSSTM_R = crate::R<bool, bool>;
#[doc = "Reader of field `ATSNS`"]
pub type ATSNS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Timestamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Timestamp Target Time Reached"]
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auxiliary Timestamp Trigger Snapshot"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Timestamp Status Interrupt Status"]
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Auxiliary Timestamp Snapshot Trigger Identifier"]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Auxiliary Timestamp Snapshot Trigger Missed"]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Number of Auxiliary Timestamp Snapshots"]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
