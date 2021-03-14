#[doc = "Reader of register TX_LPI_TRAN_CNTR"]
pub type R = crate::R<u32, super::TX_LPI_TRAN_CNTR>;
#[doc = "Reader of field `TXLPITRC`"]
pub type TXLPITRC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TXLPITRC"]
    #[inline(always)]
    pub fn txlpitrc(&self) -> TXLPITRC_R {
        TXLPITRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
