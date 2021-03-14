#[doc = "Reader of register DINR9"]
pub type R = crate::R<u32, super::DINR9>;
#[doc = "Reader of field `DIN9`"]
pub type DIN9_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din9(&self) -> DIN9_R {
        DIN9_R::new((self.bits & 0xffff) as u16)
    }
}
