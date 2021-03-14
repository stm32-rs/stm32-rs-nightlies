#[doc = "Reader of register ITLINE5"]
pub type R = crate::R<u32, super::ITLINE5>;
#[doc = "Reader of field `EXTI0`"]
pub type EXTI0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI1`"]
pub type EXTI1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EXTI0"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI1"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
