#[doc = "Reader of register CALFACT"]
pub type R = crate::R<u32, super::CALFACT>;
#[doc = "Writer for register CALFACT"]
pub type W = crate::W<u32, super::CALFACT>;
#[doc = "Register CALFACT `reset()`'s with value 0"]
impl crate::ResetValue for super::CALFACT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALFACT_D`"]
pub type CALFACT_D_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALFACT_D`"]
pub struct CALFACT_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CALFACT_S`"]
pub type CALFACT_S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALFACT_S`"]
pub struct CALFACT_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22 - Calibration Factors in differential mode"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6 - Calibration Factors In single-ended mode"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:22 - Calibration Factors in differential mode"]
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CALFACT_D_W {
        CALFACT_D_W { w: self }
    }
    #[doc = "Bits 0:6 - Calibration Factors In single-ended mode"]
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CALFACT_S_W {
        CALFACT_S_W { w: self }
    }
}
