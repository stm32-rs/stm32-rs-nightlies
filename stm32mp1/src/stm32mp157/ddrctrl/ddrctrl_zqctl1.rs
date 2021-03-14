#[doc = "Reader of register DDRCTRL_ZQCTL1"]
pub type R = crate::R<u32, super::DDRCTRL_ZQCTL1>;
#[doc = "Writer for register DDRCTRL_ZQCTL1"]
pub type W = crate::W<u32, super::DDRCTRL_ZQCTL1>;
#[doc = "Register DDRCTRL_ZQCTL1 `reset()`'s with value 0x0200_0100"]
impl crate::ResetValue for super::DDRCTRL_ZQCTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0100
    }
}
#[doc = "Reader of field `T_ZQ_SHORT_INTERVAL_X1024`"]
pub type T_ZQ_SHORT_INTERVAL_X1024_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `T_ZQ_SHORT_INTERVAL_X1024`"]
pub struct T_ZQ_SHORT_INTERVAL_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_SHORT_INTERVAL_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
#[doc = "Reader of field `T_ZQ_RESET_NOP`"]
pub type T_ZQ_RESET_NOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `T_ZQ_RESET_NOP`"]
pub struct T_ZQ_RESET_NOP_W<'a> {
    w: &'a mut W,
}
impl<'a> T_ZQ_RESET_NOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024"]
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&self) -> T_ZQ_SHORT_INTERVAL_X1024_R {
        T_ZQ_SHORT_INTERVAL_X1024_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 20:29 - T_ZQ_RESET_NOP"]
    #[inline(always)]
    pub fn t_zq_reset_nop(&self) -> T_ZQ_RESET_NOP_R {
        T_ZQ_RESET_NOP_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - T_ZQ_SHORT_INTERVAL_X1024"]
    #[inline(always)]
    pub fn t_zq_short_interval_x1024(&mut self) -> T_ZQ_SHORT_INTERVAL_X1024_W {
        T_ZQ_SHORT_INTERVAL_X1024_W { w: self }
    }
    #[doc = "Bits 20:29 - T_ZQ_RESET_NOP"]
    #[inline(always)]
    pub fn t_zq_reset_nop(&mut self) -> T_ZQ_RESET_NOP_W {
        T_ZQ_RESET_NOP_W { w: self }
    }
}
