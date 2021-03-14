#[doc = "Reader of register STGENC_CNTSR"]
pub type R = crate::R<u32, super::STGENC_CNTSR>;
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `HLTDBG`"]
pub type HLTDBG_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HLTDBG"]
    #[inline(always)]
    pub fn hltdbg(&self) -> HLTDBG_R {
        HLTDBG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
