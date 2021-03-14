#[doc = "Reader of register DDRCTRL_SCHED1"]
pub type R = crate::R<u32, super::DDRCTRL_SCHED1>;
#[doc = "Writer for register DDRCTRL_SCHED1"]
pub type W = crate::W<u32, super::DDRCTRL_SCHED1>;
#[doc = "Register DDRCTRL_SCHED1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_SCHED1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAGECLOSE_TIMER`"]
pub type PAGECLOSE_TIMER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAGECLOSE_TIMER`"]
pub struct PAGECLOSE_TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGECLOSE_TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PAGECLOSE_TIMER"]
    #[inline(always)]
    pub fn pageclose_timer(&self) -> PAGECLOSE_TIMER_R {
        PAGECLOSE_TIMER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAGECLOSE_TIMER"]
    #[inline(always)]
    pub fn pageclose_timer(&mut self) -> PAGECLOSE_TIMER_W {
        PAGECLOSE_TIMER_W { w: self }
    }
}
