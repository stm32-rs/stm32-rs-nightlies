#[doc = "Reader of register DDRCTRL_PERFLPR1"]
pub type R = crate::R<u32, super::DDRCTRL_PERFLPR1>;
#[doc = "Writer for register DDRCTRL_PERFLPR1"]
pub type W = crate::W<u32, super::DDRCTRL_PERFLPR1>;
#[doc = "Register DDRCTRL_PERFLPR1 `reset()`'s with value 0x0f00_007f"]
impl crate::ResetValue for super::DDRCTRL_PERFLPR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f00_007f
    }
}
#[doc = "Reader of field `LPR_MAX_STARVE`"]
pub type LPR_MAX_STARVE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LPR_MAX_STARVE`"]
pub struct LPR_MAX_STARVE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPR_MAX_STARVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `LPR_XACT_RUN_LENGTH`"]
pub type LPR_XACT_RUN_LENGTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPR_XACT_RUN_LENGTH`"]
pub struct LPR_XACT_RUN_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LPR_XACT_RUN_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - LPR_MAX_STARVE"]
    #[inline(always)]
    pub fn lpr_max_starve(&self) -> LPR_MAX_STARVE_R {
        LPR_MAX_STARVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - LPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn lpr_xact_run_length(&self) -> LPR_XACT_RUN_LENGTH_R {
        LPR_XACT_RUN_LENGTH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPR_MAX_STARVE"]
    #[inline(always)]
    pub fn lpr_max_starve(&mut self) -> LPR_MAX_STARVE_W {
        LPR_MAX_STARVE_W { w: self }
    }
    #[doc = "Bits 24:31 - LPR_XACT_RUN_LENGTH"]
    #[inline(always)]
    pub fn lpr_xact_run_length(&mut self) -> LPR_XACT_RUN_LENGTH_W {
        LPR_XACT_RUN_LENGTH_W { w: self }
    }
}
