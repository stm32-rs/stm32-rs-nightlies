#[doc = "Reader of register GICD_ISENABLER8"]
pub type R = crate::R<u32, super::GICD_ISENABLER8>;
#[doc = "Writer for register GICD_ISENABLER8"]
pub type W = crate::W<u32, super::GICD_ISENABLER8>;
#[doc = "Register GICD_ISENABLER8 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISENABLER8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER8`"]
pub type ISENABLER8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER8`"]
pub struct ISENABLER8_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISENABLER8"]
    #[inline(always)]
    pub fn isenabler8(&self) -> ISENABLER8_R {
        ISENABLER8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER8"]
    #[inline(always)]
    pub fn isenabler8(&mut self) -> ISENABLER8_W {
        ISENABLER8_W { w: self }
    }
}
