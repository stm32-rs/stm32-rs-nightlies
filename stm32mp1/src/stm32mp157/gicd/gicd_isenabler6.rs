#[doc = "Reader of register GICD_ISENABLER6"]
pub type R = crate::R<u32, super::GICD_ISENABLER6>;
#[doc = "Writer for register GICD_ISENABLER6"]
pub type W = crate::W<u32, super::GICD_ISENABLER6>;
#[doc = "Register GICD_ISENABLER6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISENABLER6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER6`"]
pub type ISENABLER6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER6`"]
pub struct ISENABLER6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISENABLER6"]
    #[inline(always)]
    pub fn isenabler6(&self) -> ISENABLER6_R {
        ISENABLER6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER6"]
    #[inline(always)]
    pub fn isenabler6(&mut self) -> ISENABLER6_W {
        ISENABLER6_W { w: self }
    }
}
