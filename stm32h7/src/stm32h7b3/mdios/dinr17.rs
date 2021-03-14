#[doc = "Reader of register DINR17"]
pub type R = crate::R<u32, super::DINR17>;
#[doc = "Reader of field `DIN17`"]
pub type DIN17_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din17(&self) -> DIN17_R {
        DIN17_R::new((self.bits & 0xffff) as u16)
    }
}
