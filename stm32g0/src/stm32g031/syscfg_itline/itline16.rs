#[doc = "Reader of register ITLINE16"]
pub type R = crate::R<u32, super::ITLINE16>;
#[doc = "Reader of field `TIM3`"]
pub type TIM3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM3"]
    #[inline(always)]
    pub fn tim3(&self) -> TIM3_R {
        TIM3_R::new((self.bits & 0x01) != 0)
    }
}
