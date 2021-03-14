#[doc = "Reader of register HYSCR4"]
pub type R = crate::R<u32, super::HYSCR4>;
#[doc = "Writer for register HYSCR4"]
pub type W = crate::W<u32, super::HYSCR4>;
#[doc = "Register HYSCR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::HYSCR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PG`"]
pub type PG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PG`"]
pub struct PG_W<'a> {
    w: &'a mut W,
}
impl<'a> PG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port G hysteresis control on/off"]
    #[inline(always)]
    pub fn pg(&self) -> PG_R {
        PG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port G hysteresis control on/off"]
    #[inline(always)]
    pub fn pg(&mut self) -> PG_W {
        PG_W { w: self }
    }
}
