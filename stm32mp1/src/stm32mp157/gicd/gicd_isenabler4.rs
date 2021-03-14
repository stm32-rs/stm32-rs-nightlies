#[doc = "Reader of register GICD_ISENABLER4"]
pub type R = crate::R<u32, super::GICD_ISENABLER4>;
#[doc = "Writer for register GICD_ISENABLER4"]
pub type W = crate::W<u32, super::GICD_ISENABLER4>;
#[doc = "Register GICD_ISENABLER4 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISENABLER4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER4`"]
pub type ISENABLER4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER4`"]
pub struct ISENABLER4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISENABLER4"]
    #[inline(always)]
    pub fn isenabler4(&self) -> ISENABLER4_R {
        ISENABLER4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER4"]
    #[inline(always)]
    pub fn isenabler4(&mut self) -> ISENABLER4_W {
        ISENABLER4_W { w: self }
    }
}
