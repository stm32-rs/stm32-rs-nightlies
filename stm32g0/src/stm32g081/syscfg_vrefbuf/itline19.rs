#[doc = "Reader of register ITLINE19"]
pub type R = crate::R<u32, super::ITLINE19>;
#[doc = "Reader of field `TIM14`"]
pub type TIM14_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM14"]
    #[inline(always)]
    pub fn tim14(&self) -> TIM14_R {
        TIM14_R::new((self.bits & 0x01) != 0)
    }
}
