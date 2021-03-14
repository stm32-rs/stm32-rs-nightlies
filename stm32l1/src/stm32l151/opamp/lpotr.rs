#[doc = "Reader of register LPOTR"]
pub type R = crate::R<u32, super::LPOTR>;
#[doc = "Writer for register LPOTR"]
pub type W = crate::W<u32, super::LPOTR>;
#[doc = "Register LPOTR `reset()`'s with value 0"]
impl crate::ResetValue for super::LPOTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AO3_OPT_OFFSET_TRIM_LP`"]
pub type AO3_OPT_OFFSET_TRIM_LP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AO3_OPT_OFFSET_TRIM_LP`"]
pub struct AO3_OPT_OFFSET_TRIM_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> AO3_OPT_OFFSET_TRIM_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Reader of field `AO2_OPT_OFFSET_TRIM_LP`"]
pub type AO2_OPT_OFFSET_TRIM_LP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AO2_OPT_OFFSET_TRIM_LP`"]
pub struct AO2_OPT_OFFSET_TRIM_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> AO2_OPT_OFFSET_TRIM_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `AO1_OPT_OFFSET_TRIM_LP`"]
pub type AO1_OPT_OFFSET_TRIM_LP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `AO1_OPT_OFFSET_TRIM_LP`"]
pub struct AO1_OPT_OFFSET_TRIM_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> AO1_OPT_OFFSET_TRIM_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim_lp(&self) -> AO3_OPT_OFFSET_TRIM_LP_R {
        AO3_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim_lp(&self) -> AO2_OPT_OFFSET_TRIM_LP_R {
        AO2_OPT_OFFSET_TRIM_LP_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim_lp(&self) -> AO1_OPT_OFFSET_TRIM_LP_R {
        AO1_OPT_OFFSET_TRIM_LP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim_lp(&mut self) -> AO3_OPT_OFFSET_TRIM_LP_W {
        AO3_OPT_OFFSET_TRIM_LP_W { w: self }
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim_lp(&mut self) -> AO2_OPT_OFFSET_TRIM_LP_W {
        AO2_OPT_OFFSET_TRIM_LP_W { w: self }
    }
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for low power mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim_lp(&mut self) -> AO1_OPT_OFFSET_TRIM_LP_W {
        AO1_OPT_OFFSET_TRIM_LP_W { w: self }
    }
}
