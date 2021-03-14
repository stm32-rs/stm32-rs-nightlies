#[doc = "Reader of register ITLINE18"]
pub type R = crate::R<u32, super::ITLINE18>;
#[doc = "Reader of field `LPTIM2`"]
pub type LPTIM2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - LPTIM2"]
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
