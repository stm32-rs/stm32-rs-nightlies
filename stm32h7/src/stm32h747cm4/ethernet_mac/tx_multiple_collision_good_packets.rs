#[doc = "Reader of register TX_MULTIPLE_COLLISION_GOOD_PACKETS"]
pub type R = crate::R<u32, super::TX_MULTIPLE_COLLISION_GOOD_PACKETS>;
#[doc = "Reader of field `TXMULTCOLG`"]
pub type TXMULTCOLG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx Multiple Collision Good Packets"]
    #[inline(always)]
    pub fn txmultcolg(&self) -> TXMULTCOLG_R {
        TXMULTCOLG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
