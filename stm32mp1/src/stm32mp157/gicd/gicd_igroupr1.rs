#[doc = "Reader of register GICD_IGROUPR1"]
pub type R = crate::R<u32, super::GICD_IGROUPR1>;
#[doc = "Writer for register GICD_IGROUPR1"]
pub type W = crate::W<u32, super::GICD_IGROUPR1>;
#[doc = "Register GICD_IGROUPR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_IGROUPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IGROUPR1`"]
pub type IGROUPR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IGROUPR1`"]
pub struct IGROUPR1_W<'a> {
    w: &'a mut W,
}
impl<'a> IGROUPR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IGROUPR1"]
    #[inline(always)]
    pub fn igroupr1(&self) -> IGROUPR1_R {
        IGROUPR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IGROUPR1"]
    #[inline(always)]
    pub fn igroupr1(&mut self) -> IGROUPR1_W {
        IGROUPR1_W { w: self }
    }
}
