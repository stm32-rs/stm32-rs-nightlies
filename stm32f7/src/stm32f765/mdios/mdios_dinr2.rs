#[doc = "Reader of register MDIOS_DINR2"]
pub type R = crate::R<u32, super::MDIOS_DINR2>;
#[doc = "Reader of field `DIN2`"]
pub type DIN2_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din2(&self) -> DIN2_R {
        DIN2_R::new((self.bits & 0xffff) as u16)
    }
}
