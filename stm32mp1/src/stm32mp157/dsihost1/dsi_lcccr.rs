#[doc = "Reader of register DSI_LCCCR"]
pub type R = crate::R<u32, super::DSI_LCCCR>;
#[doc = "Reader of field `COLC`"]
pub type COLC_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPE`"]
pub type LPE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
