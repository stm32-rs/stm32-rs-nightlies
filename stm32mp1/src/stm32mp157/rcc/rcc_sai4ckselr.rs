#[doc = "Reader of register RCC_SAI4CKSELR"]
pub type R = crate::R<u32, super::RCC_SAI4CKSELR>;
#[doc = "Writer for register RCC_SAI4CKSELR"]
pub type W = crate::W<u32, super::RCC_SAI4CKSELR>;
#[doc = "Register RCC_SAI4CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SAI4CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SAI4SRC`"]
pub type SAI4SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SAI4SRC`"]
pub struct SAI4SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI4SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI4SRC"]
    #[inline(always)]
    pub fn sai4src(&self) -> SAI4SRC_R {
        SAI4SRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI4SRC"]
    #[inline(always)]
    pub fn sai4src(&mut self) -> SAI4SRC_W {
        SAI4SRC_W { w: self }
    }
}
