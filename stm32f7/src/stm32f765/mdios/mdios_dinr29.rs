#[doc = "Reader of register MDIOS_DINR29"]
pub type R = crate::R<u32, super::MDIOS_DINR29>;
#[doc = "Reader of field `DIN29`"]
pub type DIN29_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din29(&self) -> DIN29_R {
        DIN29_R::new((self.bits & 0xffff) as u16)
    }
}
