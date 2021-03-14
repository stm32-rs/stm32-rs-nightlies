#[doc = "Reader of register IVR1"]
pub type R = crate::R<u32, super::IVR1>;
#[doc = "Writer for register IVR1"]
pub type W = crate::W<u32, super::IVR1>;
#[doc = "Register IVR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::IVR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV1`"]
pub type IV1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IV1`"]
pub struct IV1_W<'a> {
    w: &'a mut W,
}
impl<'a> IV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn iv1(&self) -> IV1_R {
        IV1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (IVR \\[63:32\\])"]
    #[inline(always)]
    pub fn iv1(&mut self) -> IV1_W {
        IV1_W { w: self }
    }
}
