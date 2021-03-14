#[doc = "Reader of register GICD_ISENABLER2"]
pub type R = crate::R<u32, super::GICD_ISENABLER2>;
#[doc = "Writer for register GICD_ISENABLER2"]
pub type W = crate::W<u32, super::GICD_ISENABLER2>;
#[doc = "Register GICD_ISENABLER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISENABLER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER2`"]
pub type ISENABLER2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER2`"]
pub struct ISENABLER2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISENABLER2"]
    #[inline(always)]
    pub fn isenabler2(&self) -> ISENABLER2_R {
        ISENABLER2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER2"]
    #[inline(always)]
    pub fn isenabler2(&mut self) -> ISENABLER2_W {
        ISENABLER2_W { w: self }
    }
}
