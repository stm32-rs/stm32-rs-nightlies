#[doc = "Reader of register MDIOS_DINR23"]
pub type R = crate::R<u32, super::MDIOS_DINR23>;
#[doc = "Reader of field `DIN23`"]
pub type DIN23_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din23(&self) -> DIN23_R {
        DIN23_R::new((self.bits & 0xffff) as u16)
    }
}
