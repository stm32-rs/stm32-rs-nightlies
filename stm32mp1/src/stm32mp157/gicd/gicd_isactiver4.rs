#[doc = "Reader of register GICD_ISACTIVER4"]
pub type R = crate::R<u32, super::GICD_ISACTIVER4>;
#[doc = "Writer for register GICD_ISACTIVER4"]
pub type W = crate::W<u32, super::GICD_ISACTIVER4>;
#[doc = "Register GICD_ISACTIVER4 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISACTIVER4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISACTIVER4`"]
pub type ISACTIVER4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISACTIVER4`"]
pub struct ISACTIVER4_W<'a> {
    w: &'a mut W,
}
impl<'a> ISACTIVER4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISACTIVER4"]
    #[inline(always)]
    pub fn isactiver4(&self) -> ISACTIVER4_R {
        ISACTIVER4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER4"]
    #[inline(always)]
    pub fn isactiver4(&mut self) -> ISACTIVER4_W {
        ISACTIVER4_W { w: self }
    }
}
