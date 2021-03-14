#[doc = "Reader of register FDCAN_TXESC"]
pub type R = crate::R<u32, super::FDCAN_TXESC>;
#[doc = "Reader of field `TBDS`"]
pub type TBDS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - TBDS"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 0x07) as u8)
    }
}
