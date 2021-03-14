#[doc = "Reader of register DDRCTRL_DRAMTMG2"]
pub type R = crate::R<u32, super::DDRCTRL_DRAMTMG2>;
#[doc = "Writer for register DDRCTRL_DRAMTMG2"]
pub type W = crate::W<u32, super::DDRCTRL_DRAMTMG2>;
#[doc = "Register DDRCTRL_DRAMTMG2 `reset()`'s with value 0x0305_060d"]
impl crate::ResetValue for super::DDRCTRL_DRAMTMG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0305_060d
    }
}
#[doc = "Reader of field `WR2RD`"]
pub type WR2RD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WR2RD`"]
pub struct WR2RD_W<'a> {
    w: &'a mut W,
}
impl<'a> WR2RD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `RD2WR`"]
pub type RD2WR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD2WR`"]
pub struct RD2WR_W<'a> {
    w: &'a mut W,
}
impl<'a> RD2WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `READ_LATENCY`"]
pub type READ_LATENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `READ_LATENCY`"]
pub struct READ_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WRITE_LATENCY`"]
pub type WRITE_LATENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRITE_LATENCY`"]
pub struct WRITE_LATENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_LATENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - WR2RD"]
    #[inline(always)]
    pub fn wr2rd(&self) -> WR2RD_R {
        WR2RD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - RD2WR"]
    #[inline(always)]
    pub fn rd2wr(&self) -> RD2WR_R {
        RD2WR_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - READ_LATENCY"]
    #[inline(always)]
    pub fn read_latency(&self) -> READ_LATENCY_R {
        READ_LATENCY_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - WRITE_LATENCY"]
    #[inline(always)]
    pub fn write_latency(&self) -> WRITE_LATENCY_R {
        WRITE_LATENCY_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WR2RD"]
    #[inline(always)]
    pub fn wr2rd(&mut self) -> WR2RD_W {
        WR2RD_W { w: self }
    }
    #[doc = "Bits 8:13 - RD2WR"]
    #[inline(always)]
    pub fn rd2wr(&mut self) -> RD2WR_W {
        RD2WR_W { w: self }
    }
    #[doc = "Bits 16:21 - READ_LATENCY"]
    #[inline(always)]
    pub fn read_latency(&mut self) -> READ_LATENCY_W {
        READ_LATENCY_W { w: self }
    }
    #[doc = "Bits 24:29 - WRITE_LATENCY"]
    #[inline(always)]
    pub fn write_latency(&mut self) -> WRITE_LATENCY_W {
        WRITE_LATENCY_W { w: self }
    }
}
