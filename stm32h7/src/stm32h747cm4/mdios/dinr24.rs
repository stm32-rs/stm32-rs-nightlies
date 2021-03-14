#[doc = "Reader of register DINR24"]
pub type R = crate::R<u32, super::DINR24>;
#[doc = "Reader of field `DIN24`"]
pub type DIN24_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din24(&self) -> DIN24_R {
        DIN24_R::new((self.bits & 0xffff) as u16)
    }
}
