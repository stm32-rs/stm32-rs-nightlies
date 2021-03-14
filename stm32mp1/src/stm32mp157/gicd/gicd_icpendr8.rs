#[doc = "Reader of register GICD_ICPENDR8"]
pub type R = crate::R<u32, super::GICD_ICPENDR8>;
#[doc = "Writer for register GICD_ICPENDR8"]
pub type W = crate::W<u32, super::GICD_ICPENDR8>;
#[doc = "Register GICD_ICPENDR8 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ICPENDR8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ICPENDR8`"]
pub type ICPENDR8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ICPENDR8`"]
pub struct ICPENDR8_W<'a> {
    w: &'a mut W,
}
impl<'a> ICPENDR8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ICPENDR8"]
    #[inline(always)]
    pub fn icpendr8(&self) -> ICPENDR8_R {
        ICPENDR8_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ICPENDR8"]
    #[inline(always)]
    pub fn icpendr8(&mut self) -> ICPENDR8_W {
        ICPENDR8_W { w: self }
    }
}
