#[doc = "Reader of register MDIOS_DINR6"]
pub type R = crate::R<u32, super::MDIOS_DINR6>;
#[doc = "Reader of field `DIN6`"]
pub type DIN6_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din6(&self) -> DIN6_R {
        DIN6_R::new((self.bits & 0xffff) as u16)
    }
}
