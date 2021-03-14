#[doc = "Reader of register RX_CRC_ERROR_PACKETS"]
pub type R = crate::R<u32, super::RX_CRC_ERROR_PACKETS>;
#[doc = "Reader of field `RXCRCERR`"]
pub type RXCRCERR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Rx CRC Error Packets"]
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RXCRCERR_R {
        RXCRCERR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
