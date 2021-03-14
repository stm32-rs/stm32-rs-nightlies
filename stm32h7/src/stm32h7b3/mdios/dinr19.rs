#[doc = "Reader of register DINR19"]
pub type R = crate::R<u32, super::DINR19>;
#[doc = "Reader of field `DIN19`"]
pub type DIN19_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din19(&self) -> DIN19_R {
        DIN19_R::new((self.bits & 0xffff) as u16)
    }
}
