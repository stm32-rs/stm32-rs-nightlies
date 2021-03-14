#[doc = "Reader of register GICD_ISPENDR7"]
pub type R = crate::R<u32, super::GICD_ISPENDR7>;
#[doc = "Writer for register GICD_ISPENDR7"]
pub type W = crate::W<u32, super::GICD_ISPENDR7>;
#[doc = "Register GICD_ISPENDR7 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISPENDR7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPENDR7`"]
pub type ISPENDR7_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPENDR7`"]
pub struct ISPENDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPENDR7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISPENDR7"]
    #[inline(always)]
    pub fn ispendr7(&self) -> ISPENDR7_R {
        ISPENDR7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR7"]
    #[inline(always)]
    pub fn ispendr7(&mut self) -> ISPENDR7_W {
        ISPENDR7_W { w: self }
    }
}
