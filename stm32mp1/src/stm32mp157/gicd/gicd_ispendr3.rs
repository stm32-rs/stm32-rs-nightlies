#[doc = "Reader of register GICD_ISPENDR3"]
pub type R = crate::R<u32, super::GICD_ISPENDR3>;
#[doc = "Writer for register GICD_ISPENDR3"]
pub type W = crate::W<u32, super::GICD_ISPENDR3>;
#[doc = "Register GICD_ISPENDR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::GICD_ISPENDR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ISPENDR3`"]
pub type ISPENDR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ISPENDR3`"]
pub struct ISPENDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPENDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISPENDR3"]
    #[inline(always)]
    pub fn ispendr3(&self) -> ISPENDR3_R {
        ISPENDR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISPENDR3"]
    #[inline(always)]
    pub fn ispendr3(&mut self) -> ISPENDR3_W {
        ISPENDR3_W { w: self }
    }
}
