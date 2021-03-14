#[doc = "Reader of register GICD_ICACTIVER0"]
pub type R = crate::R<u32, super::GICD_ICACTIVER0>;
#[doc = "Writer for register GICD_ICACTIVER0"]
pub type W = crate::W<u32, super::GICD_ICACTIVER0>;
#[doc = "Register GICD_ICACTIVER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICACTIVER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICACTIVER0`"]
pub type ICACTIVER0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICACTIVER0`"]
pub struct ICACTIVER0_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER0"]
    #[inline(always)]
    pub fn icactiver0(&self) -> ICACTIVER0_R {
        ICACTIVER0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER0"]
    #[inline(always)]
    pub fn icactiver0(&mut self) -> ICACTIVER0_W {
        ICACTIVER0_W { w: self }
    }
}
