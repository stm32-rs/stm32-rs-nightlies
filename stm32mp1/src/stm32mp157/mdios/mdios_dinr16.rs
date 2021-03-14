#[doc = "Reader of register MDIOS_DINR16"]
pub type R = crate::R<u32, super::MDIOS_DINR16>;
#[doc = "Reader of field `DIN`"]
pub type DIN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - DIN"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
