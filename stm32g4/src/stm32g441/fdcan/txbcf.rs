#[doc = "Reader of register TXBCF"]
pub type R = crate::R<u32, super::TXBCF>;
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - CF"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0x07) as u8)
    }
}
