#[doc = "Reader of register DDRCTRL_DRAMTMG1"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG1>;
#[doc = "Writer for register DDRCTRL_DRAMTMG1"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG1>;
#[doc = "Register DDRCTRL_DRAMTMG1 `reset()`'s with value 0x0008_0414"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0414
    }
}
#[doc = "Reader of field `T_RC`"]
pub type T_RC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_RC`"]
pub struct T_RC_W<'a> {
    w: &'a mut W,
}
impl<'a> T_RC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `RD2PRE`"]
pub type RD2PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD2PRE`"]
pub struct RD2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RD2PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `T_XP`"]
pub type T_XP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T_XP`"]
pub struct T_XP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_XP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - T_RC"]
    #[inline(always)]
    pub fn t_rc(&self) -> T_RC_R {
        T_RC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - RD2PRE"]
    #[inline(always)]
    pub fn rd2pre(&self) -> RD2PRE_R {
        RD2PRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - T_XP"]
    #[inline(always)]
    pub fn t_xp(&self) -> T_XP_R {
        T_XP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - T_RC"]
    #[inline(always)]
    pub fn t_rc(&mut self) -> T_RC_W {
        T_RC_W { w: self }
    }
    #[doc = "Bits 8:13 - RD2PRE"]
    #[inline(always)]
    pub fn rd2pre(&mut self) -> RD2PRE_W {
        RD2PRE_W { w: self }
    }
    #[doc = "Bits 16:20 - T_XP"]
    #[inline(always)]
    pub fn t_xp(&mut self) -> T_XP_W {
        T_XP_W { w: self }
    }
}
