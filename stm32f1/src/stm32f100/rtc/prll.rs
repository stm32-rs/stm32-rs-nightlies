#[doc = "Writer for register PRLL"]
pub type W = crate::W<u32, super::PRLL>;
#[doc = "Register PRLL `reset()`'s with value 0x8000"]
impl crate::ResetValue for super::PRLL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000
    }
}
#[doc = "Write proxy for field `PRLL`"]
pub struct PRLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    pub fn prll(&mut self) -> PRLL_W {
        PRLL_W { w: self }
    }
}
