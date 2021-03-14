#[doc = "Reader of register CCSIDR"]
pub type R = crate::R<u32, super::CCSIDR>;
#[doc = "Reader of field `LineSize`"]
pub type LINESIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `Associativity`"]
pub type ASSOCIATIVITY_R = crate::R<u16, u16>;
#[doc = "Reader of field `NumSets`"]
pub type NUMSETS_R = crate::R<u16, u16>;
#[doc = "Reader of field `WA`"]
pub type WA_R = crate::R<bool, bool>;
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<bool, bool>;
#[doc = "Reader of field `WB`"]
pub type WB_R = crate::R<bool, bool>;
#[doc = "Reader of field `WT`"]
pub type WT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - LineSize"]
    #[inline(always)]
    pub fn line_size(&self) -> LINESIZE_R {
        LINESIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:12 - Associativity"]
    #[inline(always)]
    pub fn associativity(&self) -> ASSOCIATIVITY_R {
        ASSOCIATIVITY_R::new(((self.bits >> 3) & 0x03ff) as u16)
    }
    #[doc = "Bits 13:27 - NumSets"]
    #[inline(always)]
    pub fn num_sets(&self) -> NUMSETS_R {
        NUMSETS_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 28 - WA"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WB"]
    #[inline(always)]
    pub fn wb(&self) -> WB_R {
        WB_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
