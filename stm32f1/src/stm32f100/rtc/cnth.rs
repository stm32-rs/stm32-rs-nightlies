#[doc = "Reader of register CNTH"]
pub type R = crate::R<u32, super::CNTH>;
#[doc = "Writer for register CNTH"]
pub type W = crate::W<u32, super::CNTH>;
#[doc = "Register CNTH `reset()`'s with value 0"]
impl crate::ResetValue for super::CNTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNTH`"]
pub type CNTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CNTH`"]
pub struct CNTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnth(&self) -> CNTH_R {
        CNTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnth(&mut self) -> CNTH_W {
        CNTH_W { w: self }
    }
}
