#[doc = "Reader of register DSI_VLCCR"]
pub type R = crate::R<u32, super::DSI_VLCCR>;
#[doc = "Reader of field `HLINE`"]
pub type HLINE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Horizontal Line duration"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
