#[doc = "Reader of register ITLINE20"]
pub type R = crate::R<u32, super::ITLINE20>;
#[doc = "Reader of field `TIM15`"]
pub type TIM15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM15"]
    #[inline(always)]
    pub fn tim15(&self) -> TIM15_R {
        TIM15_R::new((self.bits & 0x01) != 0)
    }
}
