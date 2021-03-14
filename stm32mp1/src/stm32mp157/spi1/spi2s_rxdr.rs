#[doc = "Reader of register SPI2S_RXDR"]
pub type R = crate::R<u32, super::SPI2S_RXDR>;
#[doc = "Reader of field `RXDR`"]
pub type RXDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RXDR"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
