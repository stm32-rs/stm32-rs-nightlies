#[doc = "Reader of register MDIOS_DINR30"]
pub type R = crate::R<u32, super::MDIOS_DINR30>;
#[doc = "Reader of field `DIN30`"]
pub type DIN30_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din30(&self) -> DIN30_R {
        DIN30_R::new((self.bits & 0xffff) as u16)
    }
}
