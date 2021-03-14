#[doc = "Reader of register ITLINE29"]
pub type R = crate::R<u32, super::ITLINE29>;
#[doc = "Reader of field `USART3`"]
pub type USART3_R = crate::R<bool, bool>;
#[doc = "Reader of field `USART4`"]
pub type USART4_R = crate::R<bool, bool>;
#[doc = "Reader of field `USART5`"]
pub type USART5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - USART3"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USART4"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USART5"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
