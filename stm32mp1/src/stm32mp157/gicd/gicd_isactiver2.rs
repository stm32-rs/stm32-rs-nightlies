#[doc = "Reader of register GICD_ISACTIVER2"]
pub type R = crate::R<u32, super::GICD_ISACTIVER2>;
#[doc = "Writer for register GICD_ISACTIVER2"]
pub type W = crate::W<u32, super::GICD_ISACTIVER2>;
#[doc = "Register GICD_ISACTIVER2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISACTIVER2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISACTIVER2`"]
pub type ISACTIVER2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISACTIVER2`"]
pub struct ISACTIVER2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISACTIVER2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISACTIVER2"]
    #[inline(always)]
    pub fn isactiver2(&self) -> ISACTIVER2_R {
        ISACTIVER2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISACTIVER2"]
    #[inline(always)]
    pub fn isactiver2(&mut self) -> ISACTIVER2_W {
        ISACTIVER2_W { w: self }
    }
}
