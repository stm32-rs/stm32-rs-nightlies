#[doc = "Reader of register RCC_SPDIFCKSELR"]
pub type R = crate::R<u32, super::RCC_SPDIFCKSELR>;
#[doc = "Writer for register RCC_SPDIFCKSELR"]
pub type W = crate::W<u32, super::RCC_SPDIFCKSELR>;
#[doc = "Register RCC_SPDIFCKSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_SPDIFCKSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPDIFSRC`"]
pub type SPDIFSRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPDIFSRC`"]
pub struct SPDIFSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPDIFSRC"]
    #[inline(always)]
    pub fn spdifsrc(&self) -> SPDIFSRC_R {
        SPDIFSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPDIFSRC"]
    #[inline(always)]
    pub fn spdifsrc(&mut self) -> SPDIFSRC_W {
        SPDIFSRC_W { w: self }
    }
}
