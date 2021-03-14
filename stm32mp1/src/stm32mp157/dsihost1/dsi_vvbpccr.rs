#[doc = "Reader of register DSI_VVBPCCR"]
pub type R = crate::R<u32, super::DSI_VVBPCCR>;
#[doc = "Reader of field `VBP`"]
pub type VBP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
