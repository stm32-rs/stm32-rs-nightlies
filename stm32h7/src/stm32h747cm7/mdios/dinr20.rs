#[doc = "Reader of register DINR20"]
pub type R = crate::R<u32, super::DINR20>;
#[doc = "Reader of field `DIN20`"]
pub type DIN20_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din20(&self) -> DIN20_R {
        DIN20_R::new((self.bits & 0xffff) as u16)
    }
}
