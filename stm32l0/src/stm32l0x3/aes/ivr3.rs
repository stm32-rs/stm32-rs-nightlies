#[doc = "Reader of register IVR3"]
pub type R = crate::R<u32, super::IVR3>;
#[doc = "Writer for register IVR3"]
pub type W = crate::W<u32, super::IVR3>;
#[doc = "Register IVR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::IVR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV3`"]
pub type IV3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IV3`"]
pub struct IV3_W<'a> {
    w: &'a mut W,
}
impl<'a> IV3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn iv3(&self) -> IV3_R {
        IV3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization Vector Register (MSB IVR \\[127:96\\])"]
    #[inline(always)]
    pub fn iv3(&mut self) -> IV3_W {
        IV3_W { w: self }
    }
}
