#[doc = "Reader of register POL"]
pub type R = crate::R<u32, super::POL>;
#[doc = "Writer for register POL"]
pub type W = crate::W<u32, super::POL>;
#[doc = "Register POL `reset()`'s with value 0x04c1_1db7"]
impl crate::ResetValue for super::POL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04c1_1db7
    }
}
#[doc = "Reader of field `Polynomialcoefficients`"]
pub type POLYNOMIALCOEFFICIENTS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `Polynomialcoefficients`"]
pub struct POLYNOMIALCOEFFICIENTS_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYNOMIALCOEFFICIENTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn polynomialcoefficients(&self) -> POLYNOMIALCOEFFICIENTS_R {
        POLYNOMIALCOEFFICIENTS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Programmable polynomial"]
    #[inline(always)]
    pub fn polynomialcoefficients(&mut self) -> POLYNOMIALCOEFFICIENTS_W {
        POLYNOMIALCOEFFICIENTS_W { w: self }
    }
}
