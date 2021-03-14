#[doc = "Reader of register GICD_ICACTIVER7"]
pub type R = crate::R<u32, super::GICD_ICACTIVER7>;
#[doc = "Writer for register GICD_ICACTIVER7"]
pub type W = crate::W<u32, super::GICD_ICACTIVER7>;
#[doc = "Register GICD_ICACTIVER7 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICACTIVER7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICACTIVER7`"]
pub type ICACTIVER7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICACTIVER7`"]
pub struct ICACTIVER7_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER7"]
    #[inline(always)]
    pub fn icactiver7(&self) -> ICACTIVER7_R {
        ICACTIVER7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER7"]
    #[inline(always)]
    pub fn icactiver7(&mut self) -> ICACTIVER7_W {
        ICACTIVER7_W { w: self }
    }
}
