#[doc = "Reader of register ITLINE18"]
pub type R = crate::R<u32, super::ITLINE18>;
#[doc = "Reader of field `TIM7`"]
pub type TIM7_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPTIM2`"]
pub type LPTIM2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM7"]
    #[inline(always)]
    pub fn tim7(&self) -> TIM7_R {
        TIM7_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPTIM2"]
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
