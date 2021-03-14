#[doc = "Reader of register MDIOS_DINR25"]
pub type R = crate::R<u32, super::MDIOS_DINR25>;
#[doc = "Reader of field `DIN25`"]
pub type DIN25_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din25(&self) -> DIN25_R {
        DIN25_R::new((self.bits & 0xffff) as u16)
    }
}
