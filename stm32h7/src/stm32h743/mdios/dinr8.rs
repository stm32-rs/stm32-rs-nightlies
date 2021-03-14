#[doc = "Reader of register DINR8"]
pub type R = crate::R<u32, super::DINR8>;
#[doc = "Reader of field `DIN8`"]
pub type DIN8_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din8(&self) -> DIN8_R {
        DIN8_R::new((self.bits & 0xffff) as u16)
    }
}
