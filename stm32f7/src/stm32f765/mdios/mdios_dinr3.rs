#[doc = "Reader of register MDIOS_DINR3"]
pub type R = crate::R<u32, super::MDIOS_DINR3>;
#[doc = "Reader of field `DIN3`"]
pub type DIN3_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din3(&self) -> DIN3_R {
        DIN3_R::new((self.bits & 0xffff) as u16)
    }
}
