#[doc = "Reader of register GICD_ISENABLER1"]
pub type R = crate::R<u32, super::GICD_ISENABLER1>;
#[doc = "Writer for register GICD_ISENABLER1"]
pub type W = crate::W<u32, super::GICD_ISENABLER1>;
#[doc = "Register GICD_ISENABLER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISENABLER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISENABLER1`"]
pub type ISENABLER1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISENABLER1`"]
pub struct ISENABLER1_W<'a> {
    w: &'a mut W,
}
impl<'a> ISENABLER1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISENABLER1"]
    #[inline(always)]
    pub fn isenabler1(&self) -> ISENABLER1_R {
        ISENABLER1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISENABLER1"]
    #[inline(always)]
    pub fn isenabler1(&mut self) -> ISENABLER1_W {
        ISENABLER1_W { w: self }
    }
}
