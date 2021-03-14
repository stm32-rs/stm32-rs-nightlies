#[doc = "Reader of register MTLRxQMPOCR"]
pub type R = crate::R<u32, super::MTLRXQMPOCR>;
#[doc = "Reader of field `OVFPKTCNT`"]
pub type OVFPKTCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `OVFCNTOVF`"]
pub type OVFCNTOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `MISPKTCNT`"]
pub type MISPKTCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `MISCNTOVF`"]
pub type MISCNTOVF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
