#[doc = "Reader of register GICD_IGROUPR6"]
pub type R = crate::R<u32, super::GICD_IGROUPR6>;
#[doc = "Writer for register GICD_IGROUPR6"]
pub type W = crate::W<u32, super::GICD_IGROUPR6>;
#[doc = "Register GICD_IGROUPR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IGROUPR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGROUPR6`"]
pub type IGROUPR6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IGROUPR6`"]
pub struct IGROUPR6_W<'a> {
    w: &'a mut W,
}
impl<'a> IGROUPR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IGROUPR6"]
    #[inline(always)]
    pub fn igroupr6(&self) -> IGROUPR6_R {
        IGROUPR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR6"]
    #[inline(always)]
    pub fn igroupr6(&mut self) -> IGROUPR6_W {
        IGROUPR6_W { w: self }
    }
}
