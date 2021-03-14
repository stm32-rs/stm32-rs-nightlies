#[doc = "Reader of register GICD_ICENABLER7"]
pub type R = crate::R<u32, super::GICD_ICENABLER7>;
#[doc = "Writer for register GICD_ICENABLER7"]
pub type W = crate::W<u32, super::GICD_ICENABLER7>;
#[doc = "Register GICD_ICENABLER7 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICENABLER7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICENABLER7`"]
pub type ICENABLER7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICENABLER7`"]
pub struct ICENABLER7_W<'a> {
    w: &'a mut W,
}
impl<'a> ICENABLER7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICENABLER7"]
    #[inline(always)]
    pub fn icenabler7(&self) -> ICENABLER7_R {
        ICENABLER7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICENABLER7"]
    #[inline(always)]
    pub fn icenabler7(&mut self) -> ICENABLER7_W {
        ICENABLER7_W { w: self }
    }
}
