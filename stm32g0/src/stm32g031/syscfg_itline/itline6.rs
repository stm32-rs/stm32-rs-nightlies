#[doc = "Reader of register ITLINE6"]
pub type R = crate::R<u32, super::ITLINE6>;
#[doc = "Reader of field `EXTI2`"]
pub type EXTI2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTI3`"]
pub type EXTI3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - EXTI2"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EXTI3"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
