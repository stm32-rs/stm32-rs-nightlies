#[doc = "Reader of register DINR0"]
pub type R = crate::R<u32, super::DINR0>;
#[doc = "Reader of field `DIN0`"]
pub type DIN0_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din0(&self) -> DIN0_R {
        DIN0_R::new((self.bits & 0xffff) as u16)
    }
}
