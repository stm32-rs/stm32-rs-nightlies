#[doc = "Reader of register ITLINE21"]
pub type R = crate::R<u32, super::ITLINE21>;
#[doc = "Reader of field `TIM16`"]
pub type TIM16_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM16"]
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 0x01) != 0)
    }
}
