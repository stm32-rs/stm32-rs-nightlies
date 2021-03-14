#[doc = "Reader of register DINR14"]
pub type R = crate::R<u32, super::DINR14>;
#[doc = "Reader of field `DIN14`"]
pub type DIN14_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din14(&self) -> DIN14_R {
        DIN14_R::new((self.bits & 0xffff) as u16)
    }
}
