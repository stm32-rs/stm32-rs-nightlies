#[doc = "Reader of register GICD_ICENABLER6"]
pub type R = crate::R<u32, super::GICD_ICENABLER6>;
#[doc = "Writer for register GICD_ICENABLER6"]
pub type W = crate::W<u32, super::GICD_ICENABLER6>;
#[doc = "Register GICD_ICENABLER6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICENABLER6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICENABLER6`"]
pub type ICENABLER6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICENABLER6`"]
pub struct ICENABLER6_W<'a> {
    w: &'a mut W,
}
impl<'a> ICENABLER6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICENABLER6"]
    #[inline(always)]
    pub fn icenabler6(&self) -> ICENABLER6_R {
        ICENABLER6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICENABLER6"]
    #[inline(always)]
    pub fn icenabler6(&mut self) -> ICENABLER6_W {
        ICENABLER6_W { w: self }
    }
}
