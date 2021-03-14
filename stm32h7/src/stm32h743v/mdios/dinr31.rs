#[doc = "Reader of register DINR31"]
pub type R = crate::R<u32, super::DINR31>;
#[doc = "Reader of field `DIN31`"]
pub type DIN31_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din31(&self) -> DIN31_R {
        DIN31_R::new((self.bits & 0xffff) as u16)
    }
}
