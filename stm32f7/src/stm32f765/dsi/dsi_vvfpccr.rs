#[doc = "Reader of register DSI_VVFPCCR"]
pub type R = crate::R<u32, super::DSI_VVFPCCR>;
#[doc = "Reader of field `VFP`"]
pub type VFP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical Front-Porch duration"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
