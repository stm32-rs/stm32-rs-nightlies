#[doc = "Reader of register ETH_MACTSSR"]
pub type R = crate::R<u32, super::ETH_MACTSSR>;
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
    #[doc = "Bit 0 - TSSOVF"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TSTARGT0"]
    #[inline(always)]
    pub fn tstargt0(&self) -> TSTARGT0_R {
        TSTARGT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AUXTSTRIG"]
    #[inline(always)]
    pub fn auxtstrig(&self) -> AUXTSTRIG_R {
        AUXTSTRIG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TSTRGTERR0"]
    #[inline(always)]
    pub fn tstrgterr0(&self) -> TSTRGTERR0_R {
        TSTRGTERR0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TXTSSIS"]
    #[inline(always)]
    pub fn txtssis(&self) -> TXTSSIS_R {
        TXTSSIS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - ATSSTN"]
    #[inline(always)]
    pub fn atsstn(&self) -> ATSSTN_R {
        ATSSTN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - ATSSTM"]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - ATSNS"]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
