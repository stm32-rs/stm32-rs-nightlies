#[doc = "Reader of register GICD_ISPENDR6"]
pub type R = crate::R<u32, super::GICD_ISPENDR6>;
#[doc = "Writer for register GICD_ISPENDR6"]
pub type W = crate::W<u32, super::GICD_ISPENDR6>;
#[doc = "Register GICD_ISPENDR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISPENDR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPENDR6`"]
pub type ISPENDR6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPENDR6`"]
pub struct ISPENDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPENDR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISPENDR6"]
    #[inline(always)]
    pub fn ispendr6(&self) -> ISPENDR6_R {
        ISPENDR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR6"]
    #[inline(always)]
    pub fn ispendr6(&mut self) -> ISPENDR6_W {
        ISPENDR6_W { w: self }
    }
}
