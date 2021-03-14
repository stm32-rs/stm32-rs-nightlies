#[doc = "Reader of register ITLINE27"]
pub type R = crate::R<u32, super::ITLINE27>;
#[doc = "Reader of field `USART1`"]
pub type USART1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - USART1"]
    #[inline(always)]
    pub fn usart1(&self) -> USART1_R {
        USART1_R::new((self.bits & 0x01) != 0)
    }
}
