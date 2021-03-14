#[doc = "Reader of register ETH_MACHWF1R"]
pub type R = crate::R<u32, super::ETH_MACHWF1R>;
#[doc = "Reader of field `RXFIFOSIZE`"]
pub type RXFIFOSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXFIFOSIZE`"]
pub type TXFIFOSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `OSTEN`"]
pub type OSTEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTOEN`"]
pub type PTOEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADVTHWORD`"]
pub type ADVTHWORD_R = crate::R<bool, bool>;
#[doc = "Reader of field `ADDR64`"]
pub type ADDR64_R = crate::R<u8, u8>;
#[doc = "Reader of field `DCBEN`"]
pub type DCBEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPHEN`"]
pub type SPHEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSOEN`"]
pub type TSOEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `DBGMEMA`"]
pub type DBGMEMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `AVSEL`"]
pub type AVSEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `HASHTBLSZ`"]
pub type HASHTBLSZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `L3L4FNUM`"]
pub type L3L4FNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - RXFIFOSIZE"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RXFIFOSIZE_R {
        RXFIFOSIZE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - TXFIFOSIZE"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TXFIFOSIZE_R {
        TXFIFOSIZE_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - OSTEN"]
    #[inline(always)]
    pub fn osten(&self) -> OSTEN_R {
        OSTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&self) -> PTOEN_R {
        PTOEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ADVTHWORD"]
    #[inline(always)]
    pub fn advthword(&self) -> ADVTHWORD_R {
        ADVTHWORD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - ADDR64"]
    #[inline(always)]
    pub fn addr64(&self) -> ADDR64_R {
        ADDR64_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - DCBEN"]
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SPHEN"]
    #[inline(always)]
    pub fn sphen(&self) -> SPHEN_R {
        SPHEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TSOEN"]
    #[inline(always)]
    pub fn tsoen(&self) -> TSOEN_R {
        TSOEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DBGMEMA"]
    #[inline(always)]
    pub fn dbgmema(&self) -> DBGMEMA_R {
        DBGMEMA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - AVSEL"]
    #[inline(always)]
    pub fn avsel(&self) -> AVSEL_R {
        AVSEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - HASHTBLSZ"]
    #[inline(always)]
    pub fn hashtblsz(&self) -> HASHTBLSZ_R {
        HASHTBLSZ_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 27:30 - L3L4FNUM"]
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3L4FNUM_R {
        L3L4FNUM_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
