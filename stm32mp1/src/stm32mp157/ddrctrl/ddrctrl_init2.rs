#[doc = "Reader of register DDRCTRL_INIT2"]
pub type R = crate::R<u32, super::DDRCTRL_INIT2>;
#[doc = "Writer for register DDRCTRL_INIT2"]
pub type W = crate::W<u32, super::DDRCTRL_INIT2>;
#[doc = "Register DDRCTRL_INIT2 `reset()`'s with value 0x0d05"]
impl crate::ResetValue for super::DDRCTRL_INIT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0d05
    }
}
#[doc = "Reader of field `MIN_STABLE_CLOCK_X1`"]
pub type MIN_STABLE_CLOCK_X1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN_STABLE_CLOCK_X1`"]
pub struct MIN_STABLE_CLOCK_X1_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_STABLE_CLOCK_X1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `IDLE_AFTER_RESET_X32`"]
pub type IDLE_AFTER_RESET_X32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDLE_AFTER_RESET_X32`"]
pub struct IDLE_AFTER_RESET_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_AFTER_RESET_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - MIN_STABLE_CLOCK_X1"]
    #[inline(always)]
    pub fn min_stable_clock_x1(&self) -> MIN_STABLE_CLOCK_X1_R {
        MIN_STABLE_CLOCK_X1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - IDLE_AFTER_RESET_X32"]
    #[inline(always)]
    pub fn idle_after_reset_x32(&self) -> IDLE_AFTER_RESET_X32_R {
        IDLE_AFTER_RESET_X32_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MIN_STABLE_CLOCK_X1"]
    #[inline(always)]
    pub fn min_stable_clock_x1(&mut self) -> MIN_STABLE_CLOCK_X1_W {
        MIN_STABLE_CLOCK_X1_W { w: self }
    }
    #[doc = "Bits 8:15 - IDLE_AFTER_RESET_X32"]
    #[inline(always)]
    pub fn idle_after_reset_x32(&mut self) -> IDLE_AFTER_RESET_X32_W {
        IDLE_AFTER_RESET_X32_W { w: self }
    }
}
