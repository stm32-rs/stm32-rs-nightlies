#[doc = "Reader of register GICD_IGROUPR3"]
pub type R = crate::R<u32, super::GICD_IGROUPR3>;
#[doc = "Writer for register GICD_IGROUPR3"]
pub type W = crate::W<u32, super::GICD_IGROUPR3>;
#[doc = "Register GICD_IGROUPR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IGROUPR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGROUPR3`"]
pub type IGROUPR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IGROUPR3`"]
pub struct IGROUPR3_W<'a> {
    w: &'a mut W,
}
impl<'a> IGROUPR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IGROUPR3"]
    #[inline(always)]
    pub fn igroupr3(&self) -> IGROUPR3_R {
        IGROUPR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR3"]
    #[inline(always)]
    pub fn igroupr3(&mut self) -> IGROUPR3_W {
        IGROUPR3_W { w: self }
    }
}
