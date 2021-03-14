#[doc = "Reader of register DINR28"]
pub type R = crate::R<u32, super::DINR28>;
#[doc = "Reader of field `DIN28`"]
pub type DIN28_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din28(&self) -> DIN28_R {
        DIN28_R::new((self.bits & 0xffff) as u16)
    }
}
