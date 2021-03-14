#[doc = "Reader of register GICD_ISPENDR2"]
pub type R = crate::R<u32, super::GICD_ISPENDR2>;
#[doc = "Writer for register GICD_ISPENDR2"]
pub type W = crate::W<u32, super::GICD_ISPENDR2>;
#[doc = "Register GICD_ISPENDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISPENDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPENDR2`"]
pub type ISPENDR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPENDR2`"]
pub struct ISPENDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPENDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISPENDR2"]
    #[inline(always)]
    pub fn ispendr2(&self) -> ISPENDR2_R {
        ISPENDR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR2"]
    #[inline(always)]
    pub fn ispendr2(&mut self) -> ISPENDR2_W {
        ISPENDR2_W { w: self }
    }
}
