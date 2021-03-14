#[doc = "Reader of register TX_PACKET_COUNT_GOOD"]
pub type R = crate::R<u32, super::TX_PACKET_COUNT_GOOD>;
#[doc = "Reader of field `TXPKTG`"]
pub type TXPKTG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TXPKTG"]
    #[inline(always)]
    pub fn txpktg(&self) -> TXPKTG_R {
        TXPKTG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
