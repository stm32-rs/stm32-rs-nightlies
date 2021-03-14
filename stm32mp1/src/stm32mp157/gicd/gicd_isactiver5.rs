#[doc = "Reader of register GICD_ISACTIVER5"]
pub type R = crate::R<u32, super::GICD_ISACTIVER5>;
#[doc = "Writer for register GICD_ISACTIVER5"]
pub type W = crate::W<u32, super::GICD_ISACTIVER5>;
#[doc = "Register GICD_ISACTIVER5 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISACTIVER5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISACTIVER5`"]
pub type ISACTIVER5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISACTIVER5`"]
pub struct ISACTIVER5_W<'a> {
    w: &'a mut W,
}
impl<'a> ISACTIVER5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISACTIVER5"]
    #[inline(always)]
    pub fn isactiver5(&self) -> ISACTIVER5_R {
        ISACTIVER5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER5"]
    #[inline(always)]
    pub fn isactiver5(&mut self) -> ISACTIVER5_W {
        ISACTIVER5_W { w: self }
    }
}
