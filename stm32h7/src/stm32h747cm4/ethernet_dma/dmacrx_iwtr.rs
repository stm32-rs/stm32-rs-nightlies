#[doc = "Reader of register DMACRxIWTR"]
pub type R = crate::R<u32, super::DMACRXIWTR>;
#[doc = "Writer for register DMACRxIWTR"]
pub type W = crate::W<u32, super::DMACRXIWTR>;
#[doc = "Register DMACRxIWTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACRXIWTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RWT`"]
pub type RWT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWT`"]
pub struct RWT_W<'a> {
    w: &'a mut W,
}
impl<'a> RWT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RWT_W {
        RWT_W { w: self }
    }
}
