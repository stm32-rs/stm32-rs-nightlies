#[doc = "Reader of register DDRCTRL_DRAMTMG0"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG0>;
#[doc = "Writer for register DDRCTRL_DRAMTMG0"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG0>;
#[doc = "Register DDRCTRL_DRAMTMG0 `reset()`'s with value 0x0f10_1b0f"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f10_1b0f
    }
}
#[doc = "Reader of field `T_RAS_MIN`"]
pub type T_RAS_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_RAS_MIN`"]
pub struct T_RAS_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RAS_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `T_RAS_MAX`"]
pub type T_RAS_MAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_RAS_MAX`"]
pub struct T_RAS_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RAS_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `T_FAW`"]
pub type T_FAW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_FAW`"]
pub struct T_FAW_W<'a> {
    w: &'a mut W,
}
impl<'a> T_FAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WR2PRE`"]
pub type WR2PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR2PRE`"]
pub struct WR2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> WR2PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - T_RAS_MIN"]
    #[inline(always)]
    pub fn t_ras_min(&self) -> T_RAS_MIN_R {
        T_RAS_MIN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:14 - T_RAS_MAX"]
    #[inline(always)]
    pub fn t_ras_max(&self) -> T_RAS_MAX_R {
        T_RAS_MAX_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - T_FAW"]
    #[inline(always)]
    pub fn t_faw(&self) -> T_FAW_R {
        T_FAW_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:30 - WR2PRE"]
    #[inline(always)]
    pub fn wr2pre(&self) -> WR2PRE_R {
        WR2PRE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - T_RAS_MIN"]
    #[inline(always)]
    pub fn t_ras_min(&mut self) -> T_RAS_MIN_W {
        T_RAS_MIN_W { w: self }
    }
    #[doc = "Bits 8:14 - T_RAS_MAX"]
    #[inline(always)]
    pub fn t_ras_max(&mut self) -> T_RAS_MAX_W {
        T_RAS_MAX_W { w: self }
    }
    #[doc = "Bits 16:21 - T_FAW"]
    #[inline(always)]
    pub fn t_faw(&mut self) -> T_FAW_W {
        T_FAW_W { w: self }
    }
    #[doc = "Bits 24:30 - WR2PRE"]
    #[inline(always)]
    pub fn wr2pre(&mut self) -> WR2PRE_W {
        WR2PRE_W { w: self }
    }
}
