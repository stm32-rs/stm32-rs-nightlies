#[doc = "Reader of register TX_LPI_USEC_CNTR"]
pub type R = crate::R<u32, super::TX_LPI_USEC_CNTR>;
#[doc = "Reader of field `TXLPIUSC`"]
pub type TXLPIUSC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx LPI Microseconds Counter"]
    #[inline(always)]
    pub fn txlpiusc(&self) -> TXLPIUSC_R {
        TXLPIUSC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
