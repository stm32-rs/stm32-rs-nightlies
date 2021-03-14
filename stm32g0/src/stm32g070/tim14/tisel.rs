#[doc = "Reader of register TISEL"]
pub type R = crate::R<u32, super::TISEL>;
#[doc = "Writer for register TISEL"]
pub type W = crate::W<u32, super::TISEL>;
#[doc = "Register TISEL `reset()`'s with value 0"]
impl crate::ResetValue for super::TISEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TISEL`"]
pub type TISEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TISEL`"]
pub struct TISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    pub fn tisel(&self) -> TISEL_R {
        TISEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection"]
    #[inline(always)]
    pub fn tisel(&mut self) -> TISEL_W {
        TISEL_W { w: self }
    }
}
