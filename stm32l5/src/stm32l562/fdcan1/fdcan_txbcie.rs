#[doc = "Reader of register FDCAN_TXBCIE"]
pub type R = crate::R<u32, super::FDCAN_TXBCIE>;
#[doc = "Writer for register FDCAN_TXBCIE"]
pub type W = crate::W<u32, super::FDCAN_TXBCIE>;
#[doc = "Register FDCAN_TXBCIE `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TXBCIE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CF`"]
pub struct CF_W<'a> {
    w: &'a mut W,
}
impl<'a> CF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Cancellation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
}
