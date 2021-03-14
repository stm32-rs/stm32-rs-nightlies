#[doc = "Reader of register GICD_ICPENDR7"]
pub type R = crate::R<u32, super::GICD_ICPENDR7>;
#[doc = "Writer for register GICD_ICPENDR7"]
pub type W = crate::W<u32, super::GICD_ICPENDR7>;
#[doc = "Register GICD_ICPENDR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICPENDR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICPENDR7`"]
pub type ICPENDR7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICPENDR7`"]
pub struct ICPENDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ICPENDR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICPENDR7"]
    #[inline(always)]
    pub fn icpendr7(&self) -> ICPENDR7_R {
        ICPENDR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR7"]
    #[inline(always)]
    pub fn icpendr7(&mut self) -> ICPENDR7_W {
        ICPENDR7_W { w: self }
    }
}
