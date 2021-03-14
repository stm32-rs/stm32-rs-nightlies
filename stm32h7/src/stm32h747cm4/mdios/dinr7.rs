#[doc = "Reader of register DINR7"]
pub type R = crate::R<u32, super::DINR7>;
#[doc = "Reader of field `DIN7`"]
pub type DIN7_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din7(&self) -> DIN7_R {
        DIN7_R::new((self.bits & 0xffff) as u16)
    }
}
