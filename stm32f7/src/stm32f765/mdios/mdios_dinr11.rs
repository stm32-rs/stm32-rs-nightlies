#[doc = "Reader of register MDIOS_DINR11"]
pub type R = crate::R<u32, super::MDIOS_DINR11>;
#[doc = "Reader of field `DIN11`"]
pub type DIN11_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din11(&self) -> DIN11_R {
        DIN11_R::new((self.bits & 0xffff) as u16)
    }
}
