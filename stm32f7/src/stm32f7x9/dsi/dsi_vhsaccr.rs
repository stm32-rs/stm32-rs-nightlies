#[doc = "Reader of register DSI_VHSACCR"]
pub type R = crate::R<u32, super::DSI_VHSACCR>;
#[doc = "Reader of field `HSA`"]
pub type HSA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
