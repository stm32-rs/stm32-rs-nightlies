#[doc = "Reader of register GICD_ICPENDR4"]
pub type R = crate::R<u32, super::GICD_ICPENDR4>;
#[doc = "Writer for register GICD_ICPENDR4"]
pub type W = crate::W<u32, super::GICD_ICPENDR4>;
#[doc = "Register GICD_ICPENDR4 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICPENDR4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICPENDR4`"]
pub type ICPENDR4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICPENDR4`"]
pub struct ICPENDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ICPENDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICPENDR4"]
    #[inline(always)]
    pub fn icpendr4(&self) -> ICPENDR4_R {
        ICPENDR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR4"]
    #[inline(always)]
    pub fn icpendr4(&mut self) -> ICPENDR4_W {
        ICPENDR4_W { w: self }
    }
}
