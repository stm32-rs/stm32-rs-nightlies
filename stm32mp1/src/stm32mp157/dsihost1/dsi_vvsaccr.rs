#[doc = "Reader of register DSI_VVSACCR"]
pub type R = crate::R<u32, super::DSI_VVSACCR>;
#[doc = "Reader of field `VSA`"]
pub type VSA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
