#[doc = "Reader of register TXBRP"]
pub type R = crate::R<u32, super::TXBRP>;
#[doc = "Reader of field `TRP`"]
pub type TRP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - TRP"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0x07) as u8)
    }
}
