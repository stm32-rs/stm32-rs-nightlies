#[doc = "Reader of register RCC_RNG2CKSELR"]
pub type R = crate::R<u32, super::RCC_RNG2CKSELR>;
#[doc = "Writer for register RCC_RNG2CKSELR"]
pub type W = crate::W<u32, super::RCC_RNG2CKSELR>;
#[doc = "Register RCC_RNG2CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_RNG2CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RNG2SRC`"]
pub type RNG2SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RNG2SRC`"]
pub struct RNG2SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG2SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RNG2SRC"]
    #[inline(always)]
    pub fn rng2src(&self) -> RNG2SRC_R {
        RNG2SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RNG2SRC"]
    #[inline(always)]
    pub fn rng2src(&mut self) -> RNG2SRC_W {
        RNG2SRC_W { w: self }
    }
}
