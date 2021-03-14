#[doc = "Reader of register VVBPCR"]
pub type R = crate::R<u32, super::VVBPCR>;
#[doc = "Writer for register VVBPCR"]
pub type W = crate::W<u32, super::VVBPCR>;
#[doc = "Register VVBPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::VVBPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VBP`"]
pub type VBP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VBP`"]
pub struct VBP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical back-porch duration"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical back-porch duration"]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W {
        VBP_W { w: self }
    }
}
