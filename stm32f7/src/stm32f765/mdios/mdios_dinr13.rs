#[doc = "Reader of register MDIOS_DINR13"]
pub type R = crate::R<u32, super::MDIOS_DINR13>;
#[doc = "Reader of field `DIN13`"]
pub type DIN13_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din13(&self) -> DIN13_R {
        DIN13_R::new((self.bits & 0xffff) as u16)
    }
}
