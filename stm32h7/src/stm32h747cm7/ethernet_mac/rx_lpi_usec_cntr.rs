#[doc = "Reader of register RX_LPI_USEC_CNTR"]
pub type R = crate::R<u32, super::RX_LPI_USEC_CNTR>;
#[doc = "Reader of field `RXLPIUSC`"]
pub type RXLPIUSC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx LPI Microseconds Counter"]
    #[inline(always)]
    pub fn rxlpiusc(&self) -> RXLPIUSC_R {
        RXLPIUSC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
