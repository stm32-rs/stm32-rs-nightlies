#[doc = "Reader of register GICD_ICACTIVER2"]
pub type R = crate::R<u32, super::GICD_ICACTIVER2>;
#[doc = "Writer for register GICD_ICACTIVER2"]
pub type W = crate::W<u32, super::GICD_ICACTIVER2>;
#[doc = "Register GICD_ICACTIVER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICACTIVER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICACTIVER2`"]
pub type ICACTIVER2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICACTIVER2`"]
pub struct ICACTIVER2_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER2"]
    #[inline(always)]
    pub fn icactiver2(&self) -> ICACTIVER2_R {
        ICACTIVER2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER2"]
    #[inline(always)]
    pub fn icactiver2(&mut self) -> ICACTIVER2_W {
        ICACTIVER2_W { w: self }
    }
}
