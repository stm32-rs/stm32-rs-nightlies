#[doc = "Reader of register DMACMFCR"]
pub type R = crate::R<u32, super::DMACMFCR>;
#[doc = "Reader of field `MFC`"]
pub type MFC_R = crate::R<u16, u16>;
#[doc = "Reader of field `MFCO`"]
pub type MFCO_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
