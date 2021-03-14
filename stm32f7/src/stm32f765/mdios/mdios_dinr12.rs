#[doc = "Reader of register MDIOS_DINR12"]
pub type R = crate::R<u32, super::MDIOS_DINR12>;
#[doc = "Reader of field `DIN12`"]
pub type DIN12_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din12(&self) -> DIN12_R {
        DIN12_R::new((self.bits & 0xffff) as u16)
    }
}
