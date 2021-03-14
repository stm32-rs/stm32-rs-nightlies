#[doc = "Reader of register GICD_IGROUPR0"]
pub type R = crate::R<u32, super::GICD_IGROUPR0>;
#[doc = "Writer for register GICD_IGROUPR0"]
pub type W = crate::W<u32, super::GICD_IGROUPR0>;
#[doc = "Register GICD_IGROUPR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IGROUPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGROUPR0`"]
pub type IGROUPR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IGROUPR0`"]
pub struct IGROUPR0_W<'a> {
    w: &'a mut W,
}
impl<'a> IGROUPR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IGROUPR0"]
    #[inline(always)]
    pub fn igroupr0(&self) -> IGROUPR0_R {
        IGROUPR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR0"]
    #[inline(always)]
    pub fn igroupr0(&mut self) -> IGROUPR0_W {
        IGROUPR0_W { w: self }
    }
}
