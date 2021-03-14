#[doc = "Reader of register ITLINE15"]
pub type R = crate::R<u32, super::ITLINE15>;
#[doc = "Reader of field `TIM2`"]
pub type TIM2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM2"]
    #[inline(always)]
    pub fn tim2(&self) -> TIM2_R {
        TIM2_R::new((self.bits & 0x01) != 0)
    }
}
