#[doc = "Reader of register PSMAR"]
pub type R = crate::R<u32, super::PSMAR>;
#[doc = "Writer for register PSMAR"]
pub type W = crate::W<u32, super::PSMAR>;
#[doc = "Register PSMAR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INTERVAL`"]
pub type INTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INTERVAL`"]
pub struct INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Polling interval"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Polling interval"]
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W {
        INTERVAL_W { w: self }
    }
}
