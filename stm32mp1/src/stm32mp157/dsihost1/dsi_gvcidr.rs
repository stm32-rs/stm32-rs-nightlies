#[doc = "Reader of register DSI_GVCIDR"]
pub type R = crate::R<u32, super::DSI_GVCIDR>;
#[doc = "Reader of field `VCID`"]
pub type VCID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 0x03) as u8)
    }
}
