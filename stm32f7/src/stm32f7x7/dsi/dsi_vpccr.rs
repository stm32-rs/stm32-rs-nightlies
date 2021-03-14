#[doc = "Reader of register DSI_VPCCR"]
pub type R = crate::R<u32, super::DSI_VPCCR>;
#[doc = "Reader of field `VPSIZE`"]
pub type VPSIZE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
