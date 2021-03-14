#[doc = "Reader of register GICD_ICACTIVER6"]
pub type R = crate::R<u32, super::GICD_ICACTIVER6>;
#[doc = "Writer for register GICD_ICACTIVER6"]
pub type W = crate::W<u32, super::GICD_ICACTIVER6>;
#[doc = "Register GICD_ICACTIVER6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICACTIVER6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICACTIVER6`"]
pub type ICACTIVER6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICACTIVER6`"]
pub struct ICACTIVER6_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER6"]
    #[inline(always)]
    pub fn icactiver6(&self) -> ICACTIVER6_R {
        ICACTIVER6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER6"]
    #[inline(always)]
    pub fn icactiver6(&mut self) -> ICACTIVER6_W {
        ICACTIVER6_W { w: self }
    }
}
