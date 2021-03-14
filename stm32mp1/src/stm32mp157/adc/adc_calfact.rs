#[doc = "Reader of register ADC_CALFACT"]
pub type R = crate::R<u32, super::ADC_CALFACT>;
#[doc = "Writer for register ADC_CALFACT"]
pub type W = crate::W<u32, super::ADC_CALFACT>;
#[doc = "Register ADC_CALFACT `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CALFACT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALFACT_S`"]
pub type CALFACT_S_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CALFACT_S`"]
pub struct CALFACT_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `CALFACT_D`"]
pub type CALFACT_D_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CALFACT_D`"]
pub struct CALFACT_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CALFACT_S_R {
        CALFACT_S_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CALFACT_D_R {
        CALFACT_D_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CALFACT_S"]
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CALFACT_S_W {
        CALFACT_S_W { w: self }
    }
    #[doc = "Bits 16:26 - CALFACT_D"]
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CALFACT_D_W {
        CALFACT_D_W { w: self }
    }
}
