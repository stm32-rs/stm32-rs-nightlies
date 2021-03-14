#[doc = "Reader of register FDCAN_TXBTO"]
pub type R = crate::R<u32, super::FDCAN_TXBTO>;
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - TO"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
