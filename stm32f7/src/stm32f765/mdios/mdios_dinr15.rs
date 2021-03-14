#[doc = "Reader of register MDIOS_DINR15"]
pub type R = crate::R<u32, super::MDIOS_DINR15>;
#[doc = "Reader of field `DIN15`"]
pub type DIN15_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din15(&self) -> DIN15_R {
        DIN15_R::new((self.bits & 0xffff) as u16)
    }
}
