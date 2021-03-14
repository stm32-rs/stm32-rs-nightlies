#[doc = "Reader of register ITLINE14"]
pub type R = crate::R<u32, super::ITLINE14>;
#[doc = "Reader of field `TIM1_CC`"]
pub type TIM1_CC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM1_CC"]
    #[inline(always)]
    pub fn tim1_cc(&self) -> TIM1_CC_R {
        TIM1_CC_R::new((self.bits & 0x01) != 0)
    }
}
