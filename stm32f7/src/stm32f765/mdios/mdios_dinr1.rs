#[doc = "Reader of register MDIOS_DINR1"]
pub type R = crate::R<u32, super::MDIOS_DINR1>;
#[doc = "Reader of field `DIN1`"]
pub type DIN1_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new((self.bits & 0xffff) as u16)
    }
}
