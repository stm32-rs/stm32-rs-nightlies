#[doc = "Reader of register GICD_IGROUPR2"]
pub type R = crate::R<u32, super::GICD_IGROUPR2>;
#[doc = "Writer for register GICD_IGROUPR2"]
pub type W = crate::W<u32, super::GICD_IGROUPR2>;
#[doc = "Register GICD_IGROUPR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IGROUPR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGROUPR2`"]
pub type IGROUPR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IGROUPR2`"]
pub struct IGROUPR2_W<'a> {
    w: &'a mut W,
}
impl<'a> IGROUPR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IGROUPR2"]
    #[inline(always)]
    pub fn igroupr2(&self) -> IGROUPR2_R {
        IGROUPR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR2"]
    #[inline(always)]
    pub fn igroupr2(&mut self) -> IGROUPR2_W {
        IGROUPR2_W { w: self }
    }
}
