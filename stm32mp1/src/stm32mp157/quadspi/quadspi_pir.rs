#[doc = "Reader of register QUADSPI_PIR"]
pub type R = crate::R<u32, super::QUADSPI_PIR>;
#[doc = "Writer for register QUADSPI_PIR"]
pub type W = crate::W<u32, super::QUADSPI_PIR>;
#[doc = "Register QUADSPI_PIR `reset()`'s with value 0"]
impl crate::ResetValue for super::QUADSPI_PIR {
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
    #[doc = "Bits 0:15 - INTERVAL"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - INTERVAL"]
    #[inline(always)]
    pub fn interval(&mut self) -> INTERVAL_W {
        INTERVAL_W { w: self }
    }
}
