#[doc = "Reader of register MDIOS_DINR16"]
pub type R = crate::R<u32, super::MDIOS_DINR16>;
#[doc = "Reader of field `DIN16`"]
pub type DIN16_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din16(&self) -> DIN16_R {
        DIN16_R::new((self.bits & 0xffff) as u16)
    }
}
