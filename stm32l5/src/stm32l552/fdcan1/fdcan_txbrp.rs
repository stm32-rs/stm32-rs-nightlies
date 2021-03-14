#[doc = "Reader of register FDCAN_TXBRP"]
pub type R = crate::R<u32, super::FDCAN_TXBRP>;
#[doc = "Reader of field `TRP`"]
pub type TRP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Transmission Request Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0x07) as u8)
    }
}
