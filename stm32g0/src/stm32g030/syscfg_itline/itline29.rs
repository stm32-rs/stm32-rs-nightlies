#[doc = "Reader of register ITLINE29"]
pub type R = crate::R<u32, super::ITLINE29>;
#[doc = "Reader of field `USART5`"]
pub type USART5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - USART5"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
