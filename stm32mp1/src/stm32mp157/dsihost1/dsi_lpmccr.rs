#[doc = "Reader of register DSI_LPMCCR"]
pub type R = crate::R<u32, super::DSI_LPMCCR>;
#[doc = "Reader of field `VLPSIZE`"]
pub type VLPSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPSIZE`"]
pub type LPSIZE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VLPSIZE_R {
        VLPSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&self) -> LPSIZE_R {
        LPSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
