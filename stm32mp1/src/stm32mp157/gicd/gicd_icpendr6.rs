#[doc = "Reader of register GICD_ICPENDR6"]
pub type R = crate::R<u32, super::GICD_ICPENDR6>;
#[doc = "Writer for register GICD_ICPENDR6"]
pub type W = crate::W<u32, super::GICD_ICPENDR6>;
#[doc = "Register GICD_ICPENDR6 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICPENDR6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICPENDR6`"]
pub type ICPENDR6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICPENDR6`"]
pub struct ICPENDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ICPENDR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICPENDR6"]
    #[inline(always)]
    pub fn icpendr6(&self) -> ICPENDR6_R {
        ICPENDR6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR6"]
    #[inline(always)]
    pub fn icpendr6(&mut self) -> ICPENDR6_W {
        ICPENDR6_W { w: self }
    }
}
