#[doc = "Reader of register DSI_VHBPCCR"]
pub type R = crate::R<u32, super::DSI_VHBPCCR>;
#[doc = "Reader of field `HBP`"]
pub type HBP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Back-Porch duration"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
