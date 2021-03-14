#[doc = "Reader of register DINR27"]
pub type R = crate::R<u32, super::DINR27>;
#[doc = "Reader of field `DIN27`"]
pub type DIN27_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din27(&self) -> DIN27_R {
        DIN27_R::new((self.bits & 0xffff) as u16)
    }
}
