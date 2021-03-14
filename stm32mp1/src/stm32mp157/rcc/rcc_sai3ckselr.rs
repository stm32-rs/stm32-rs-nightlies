#[doc = "Reader of register RCC_SAI3CKSELR"]
pub type R = crate::R<u32, super::RCC_SAI3CKSELR>;
#[doc = "Writer for register RCC_SAI3CKSELR"]
pub type W = crate::W<u32, super::RCC_SAI3CKSELR>;
#[doc = "Register RCC_SAI3CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SAI3CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAI3SRC`"]
pub type SAI3SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAI3SRC`"]
pub struct SAI3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI3SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI3SRC"]
    #[inline(always)]
    pub fn sai3src(&self) -> SAI3SRC_R {
        SAI3SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI3SRC"]
    #[inline(always)]
    pub fn sai3src(&mut self) -> SAI3SRC_W {
        SAI3SRC_W { w: self }
    }
}
