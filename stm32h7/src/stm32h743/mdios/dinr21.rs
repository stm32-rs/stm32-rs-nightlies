#[doc = "Reader of register DINR21"]
pub type R = crate::R<u32, super::DINR21>;
#[doc = "Reader of field `DIN21`"]
pub type DIN21_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din21(&self) -> DIN21_R {
        DIN21_R::new((self.bits & 0xffff) as u16)
    }
}
