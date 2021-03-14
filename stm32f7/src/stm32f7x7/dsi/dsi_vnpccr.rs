#[doc = "Reader of register DSI_VNPCCR"]
pub type R = crate::R<u32, super::DSI_VNPCCR>;
#[doc = "Reader of field `NPSIZE`"]
pub type NPSIZE_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - Null Packet Size"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
