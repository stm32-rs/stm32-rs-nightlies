#[doc = "Reader of register ITLINE22"]
pub type R = crate::R<u32, super::ITLINE22>;
#[doc = "Reader of field `TIM17`"]
pub type TIM17_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TIM17"]
    #[inline(always)]
    pub fn tim17(&self) -> TIM17_R {
        TIM17_R::new((self.bits & 0x01) != 0)
    }
}
