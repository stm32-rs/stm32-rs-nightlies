#[doc = "Reader of register DINR18"]
pub type R = crate::R<u32, super::DINR18>;
#[doc = "Reader of field `DIN18`"]
pub type DIN18_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din18(&self) -> DIN18_R {
        DIN18_R::new((self.bits & 0xffff) as u16)
    }
}
