#[doc = "Reader of register RX_LPI_TRAN_CNTR"]
pub type R = crate::R<u32, super::RX_LPI_TRAN_CNTR>;
#[doc = "Reader of field `RXLPITRC`"]
pub type RXLPITRC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RXLPITRC"]
    #[inline(always)]
    pub fn rxlpitrc(&self) -> RXLPITRC_R {
        RXLPITRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
