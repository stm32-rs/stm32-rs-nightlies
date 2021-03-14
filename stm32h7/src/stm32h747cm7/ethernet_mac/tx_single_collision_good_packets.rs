#[doc = "Reader of register TX_SINGLE_COLLISION_GOOD_PACKETS"]
pub type R = crate::R<u32, super::TX_SINGLE_COLLISION_GOOD_PACKETS>;
#[doc = "Reader of field `TXSNGLCOLG`"]
pub type TXSNGLCOLG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tx Single Collision Good Packets"]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TXSNGLCOLG_R {
        TXSNGLCOLG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
