#[doc = "Reader of register DINR10"]
pub type R = crate::R<u32, super::DINR10>;
#[doc = "Reader of field `DIN10`"]
pub type DIN10_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din10(&self) -> DIN10_R {
        DIN10_R::new((self.bits & 0xffff) as u16)
    }
}
