#[doc = "Reader of register GICC_RPR"]
pub type R = crate::R<u32, super::GICC_RPR>;
#[doc = "Reader of field `PRIORITY`"]
pub type PRIORITY_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
