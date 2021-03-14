#[doc = "Reader of register TXBTO"]
pub type R = crate::R<u32, super::TXBTO>;
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - TO"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0x07) as u8)
    }
}
