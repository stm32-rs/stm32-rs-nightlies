#[doc = "Reader of register DINR5"]
pub type R = crate::R<u32, super::DINR5>;
#[doc = "Reader of field `DIN5`"]
pub type DIN5_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din5(&self) -> DIN5_R {
        DIN5_R::new((self.bits & 0xffff) as u16)
    }
}
