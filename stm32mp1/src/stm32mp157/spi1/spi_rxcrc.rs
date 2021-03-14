#[doc = "Reader of register SPI_RXCRC"]
pub type R = crate::R<u32, super::SPI_RXCRC>;
#[doc = "Reader of field `RXCRC`"]
pub type RXCRC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RXCRC"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
