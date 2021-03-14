#[doc = "Reader of register GICD_ICACTIVER4"]
pub type R = crate::R<u32, super::GICD_ICACTIVER4>;
#[doc = "Writer for register GICD_ICACTIVER4"]
pub type W = crate::W<u32, super::GICD_ICACTIVER4>;
#[doc = "Register GICD_ICACTIVER4 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICACTIVER4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICACTIVER4`"]
pub type ICACTIVER4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICACTIVER4`"]
pub struct ICACTIVER4_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER4"]
    #[inline(always)]
    pub fn icactiver4(&self) -> ICACTIVER4_R {
        ICACTIVER4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER4"]
    #[inline(always)]
    pub fn icactiver4(&mut self) -> ICACTIVER4_W {
        ICACTIVER4_W { w: self }
    }
}
