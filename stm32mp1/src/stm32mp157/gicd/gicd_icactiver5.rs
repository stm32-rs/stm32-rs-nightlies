#[doc = "Reader of register GICD_ICACTIVER5"]
pub type R = crate::R<u32, super::GICD_ICACTIVER5>;
#[doc = "Writer for register GICD_ICACTIVER5"]
pub type W = crate::W<u32, super::GICD_ICACTIVER5>;
#[doc = "Register GICD_ICACTIVER5 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICACTIVER5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICACTIVER5`"]
pub type ICACTIVER5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICACTIVER5`"]
pub struct ICACTIVER5_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER5"]
    #[inline(always)]
    pub fn icactiver5(&self) -> ICACTIVER5_R {
        ICACTIVER5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER5"]
    #[inline(always)]
    pub fn icactiver5(&mut self) -> ICACTIVER5_W {
        ICACTIVER5_W { w: self }
    }
}
