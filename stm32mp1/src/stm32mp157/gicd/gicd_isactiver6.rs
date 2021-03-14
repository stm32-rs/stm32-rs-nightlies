#[doc = "Reader of register GICD_ISACTIVER6"]
pub type R = crate::R<u32, super::GICD_ISACTIVER6>;
#[doc = "Writer for register GICD_ISACTIVER6"]
pub type W = crate::W<u32, super::GICD_ISACTIVER6>;
#[doc = "Register GICD_ISACTIVER6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISACTIVER6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISACTIVER6`"]
pub type ISACTIVER6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISACTIVER6`"]
pub struct ISACTIVER6_W<'a> {
    w: &'a mut W,
}
impl<'a> ISACTIVER6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISACTIVER6"]
    #[inline(always)]
    pub fn isactiver6(&self) -> ISACTIVER6_R {
        ISACTIVER6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER6"]
    #[inline(always)]
    pub fn isactiver6(&mut self) -> ISACTIVER6_W {
        ISACTIVER6_W { w: self }
    }
}
