#[doc = "Reader of register CALFACT2"]
pub type R = crate::R<u32, super::CALFACT2>;
#[doc = "Writer for register CALFACT2"]
pub type W = crate::W<u32, super::CALFACT2>;
#[doc = "Register CALFACT2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CALFACT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LINCALFACT`"]
pub type LINCALFACT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LINCALFACT`"]
pub struct LINCALFACT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALFACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&self) -> LINCALFACT_R {
        LINCALFACT_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&mut self) -> LINCALFACT_W {
        LINCALFACT_W { w: self }
    }
}
