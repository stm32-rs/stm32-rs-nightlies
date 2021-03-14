#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `TXIS`"]
pub type TXIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMSGDISC`"]
pub type TXMSGDISC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMSGSENT`"]
pub type TXMSGSENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXMSGABT`"]
pub type TXMSGABT_R = crate::R<bool, bool>;
#[doc = "Reader of field `HRSTDISC`"]
pub type HRSTDISC_R = crate::R<bool, bool>;
#[doc = "Reader of field `HRSTSENT`"]
pub type HRSTSENT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUND`"]
pub type TXUND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXNE`"]
pub type RXNE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXORDDET`"]
pub type RXORDDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXHRSTDET`"]
pub type RXHRSTDET_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOVR`"]
pub type RXOVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXMSGEND`"]
pub type RXMSGEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXERR`"]
pub type RXERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TYPECEVT1`"]
pub type TYPECEVT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TYPECEVT2`"]
pub type TYPECEVT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TYPEC_VSTATE_CC1`"]
pub type TYPEC_VSTATE_CC1_R = crate::R<u8, u8>;
#[doc = "Reader of field `TYPEC_VSTATE_CC2`"]
pub type TYPEC_VSTATE_CC2_R = crate::R<u8, u8>;
#[doc = "Reader of field `FRSEVT`"]
pub type FRSEVT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TXIS"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXMSGDISC"]
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXMSGSENT"]
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TXMSGABT"]
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - HRSTDISC"]
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HRSTSENT"]
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TXUND"]
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RXNE"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RXORDDET"]
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RXHRSTDET"]
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RXOVR"]
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RXMSGEND"]
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RXERR"]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TYPECEVT1"]
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TYPECEVT2"]
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - TYPEC_VSTATE_CC1"]
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - TYPEC_VSTATE_CC2"]
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - FRSEVT"]
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
