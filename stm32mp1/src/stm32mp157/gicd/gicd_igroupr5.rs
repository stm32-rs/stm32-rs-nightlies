#[doc = "Reader of register GICD_IGROUPR5"]
pub type R = crate::R<u32, super::GICD_IGROUPR5>;
#[doc = "Writer for register GICD_IGROUPR5"]
pub type W = crate::W<u32, super::GICD_IGROUPR5>;
#[doc = "Register GICD_IGROUPR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IGROUPR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGROUPR5`"]
pub type IGROUPR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IGROUPR5`"]
pub struct IGROUPR5_W<'a> {
    w: &'a mut W,
}
impl<'a> IGROUPR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IGROUPR5"]
    #[inline(always)]
    pub fn igroupr5(&self) -> IGROUPR5_R {
        IGROUPR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR5"]
    #[inline(always)]
    pub fn igroupr5(&mut self) -> IGROUPR5_W {
        IGROUPR5_W { w: self }
    }
}
