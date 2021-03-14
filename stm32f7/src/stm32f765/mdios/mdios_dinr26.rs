#[doc = "Reader of register MDIOS_DINR26"]
pub type R = crate::R<u32, super::MDIOS_DINR26>;
#[doc = "Reader of field `DIN26`"]
pub type DIN26_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din26(&self) -> DIN26_R {
        DIN26_R::new((self.bits & 0xffff) as u16)
    }
}
