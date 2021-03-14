#[doc = "Reader of register GICD_ICACTIVER3"]
pub type R = crate::R<u32, super::GICD_ICACTIVER3>;
#[doc = "Writer for register GICD_ICACTIVER3"]
pub type W = crate::W<u32, super::GICD_ICACTIVER3>;
#[doc = "Register GICD_ICACTIVER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICACTIVER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICACTIVER3`"]
pub type ICACTIVER3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICACTIVER3`"]
pub struct ICACTIVER3_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACTIVER3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICACTIVER3"]
    #[inline(always)]
    pub fn icactiver3(&self) -> ICACTIVER3_R {
        ICACTIVER3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICACTIVER3"]
    #[inline(always)]
    pub fn icactiver3(&mut self) -> ICACTIVER3_W {
        ICACTIVER3_W { w: self }
    }
}
