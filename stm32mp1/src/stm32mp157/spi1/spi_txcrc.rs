#[doc = "Reader of register SPI_TXCRC"]
pub type R = crate::R<u32, super::SPI_TXCRC>;
#[doc = "Reader of field `TXCRC`"]
pub type TXCRC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TXCRC"]
    #[inline(always)]
    pub fn txcrc(&self) -> TXCRC_R {
        TXCRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
