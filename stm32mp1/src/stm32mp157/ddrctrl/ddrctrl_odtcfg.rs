#[doc = "Reader of register DDRCTRL_ODTCFG"]
pub type R = crate::R<u32, super::DDRCTRL_ODTCFG>;
#[doc = "Writer for register DDRCTRL_ODTCFG"]
pub type W = crate::W<u32, super::DDRCTRL_ODTCFG>;
#[doc = "Register DDRCTRL_ODTCFG `reset()`'s with value 0x0400_0400"]
impl crate::ResetValue for super::DDRCTRL_ODTCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0400
    }
}
#[doc = "Reader of field `RD_ODT_DELAY`"]
pub type RD_ODT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_ODT_DELAY`"]
pub struct RD_ODT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_ODT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `RD_ODT_HOLD`"]
pub type RD_ODT_HOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_ODT_HOLD`"]
pub struct RD_ODT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_ODT_HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WR_ODT_DELAY`"]
pub type WR_ODT_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR_ODT_DELAY`"]
pub struct WR_ODT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_ODT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WR_ODT_HOLD`"]
pub type WR_ODT_HOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR_ODT_HOLD`"]
pub struct WR_ODT_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_ODT_HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:6 - RD_ODT_DELAY"]
    #[inline(always)]
    pub fn rd_odt_delay(&self) -> RD_ODT_DELAY_R {
        RD_ODT_DELAY_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - RD_ODT_HOLD"]
    #[inline(always)]
    pub fn rd_odt_hold(&self) -> RD_ODT_HOLD_R {
        RD_ODT_HOLD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - WR_ODT_DELAY"]
    #[inline(always)]
    pub fn wr_odt_delay(&self) -> WR_ODT_DELAY_R {
        WR_ODT_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - WR_ODT_HOLD"]
    #[inline(always)]
    pub fn wr_odt_hold(&self) -> WR_ODT_HOLD_R {
        WR_ODT_HOLD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6 - RD_ODT_DELAY"]
    #[inline(always)]
    pub fn rd_odt_delay(&mut self) -> RD_ODT_DELAY_W {
        RD_ODT_DELAY_W { w: self }
    }
    #[doc = "Bits 8:11 - RD_ODT_HOLD"]
    #[inline(always)]
    pub fn rd_odt_hold(&mut self) -> RD_ODT_HOLD_W {
        RD_ODT_HOLD_W { w: self }
    }
    #[doc = "Bits 16:20 - WR_ODT_DELAY"]
    #[inline(always)]
    pub fn wr_odt_delay(&mut self) -> WR_ODT_DELAY_W {
        WR_ODT_DELAY_W { w: self }
    }
    #[doc = "Bits 24:27 - WR_ODT_HOLD"]
    #[inline(always)]
    pub fn wr_odt_hold(&mut self) -> WR_ODT_HOLD_W {
        WR_ODT_HOLD_W { w: self }
    }
}
