#[doc = "Reader of register MDIOS_DINR22"]
pub type R = crate::R<u32, super::MDIOS_DINR22>;
#[doc = "Reader of field `DIN22`"]
pub type DIN22_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din22(&self) -> DIN22_R {
        DIN22_R::new((self.bits & 0xffff) as u16)
    }
}
