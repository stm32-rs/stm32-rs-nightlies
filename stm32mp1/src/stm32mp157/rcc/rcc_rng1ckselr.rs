#[doc = "Reader of register RCC_RNG1CKSELR"]
pub type R = crate::R<u32, super::RCC_RNG1CKSELR>;
#[doc = "Writer for register RCC_RNG1CKSELR"]
pub type W = crate::W<u32, super::RCC_RNG1CKSELR>;
#[doc = "Register RCC_RNG1CKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_RNG1CKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RNG1SRC`"]
pub type RNG1SRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RNG1SRC`"]
pub struct RNG1SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RNG1SRC"]
    #[inline(always)]
    pub fn rng1src(&self) -> RNG1SRC_R {
        RNG1SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RNG1SRC"]
    #[inline(always)]
    pub fn rng1src(&mut self) -> RNG1SRC_W {
        RNG1SRC_W { w: self }
    }
}
