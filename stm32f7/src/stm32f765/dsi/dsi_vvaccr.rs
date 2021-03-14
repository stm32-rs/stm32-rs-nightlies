#[doc = "Reader of register DSI_VVACCR"]
pub type R = crate::R<u32, super::DSI_VVACCR>;
#[doc = "Reader of field `VA`"]
pub type VA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Vertical Active duration"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
