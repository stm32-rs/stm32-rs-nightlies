#[doc = "Reader of register DINR4"]
pub type R = crate::R<u32, super::DINR4>;
#[doc = "Reader of field `DIN4`"]
pub type DIN4_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din4(&self) -> DIN4_R {
        DIN4_R::new((self.bits & 0xffff) as u16)
    }
}
