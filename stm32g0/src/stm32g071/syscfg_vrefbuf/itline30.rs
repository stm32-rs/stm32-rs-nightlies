#[doc = "Reader of register ITLINE30"]
pub type R = crate::R<u32, super::ITLINE30>;
#[doc = "Reader of field `USART2`"]
pub type USART2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CEC"]
    #[inline(always)]
    pub fn usart2(&self) -> USART2_R {
        USART2_R::new((self.bits & 0x01) != 0)
    }
}
