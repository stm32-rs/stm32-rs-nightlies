#[doc = "Reader of register DDRPHYC_DTPR0"]
pub type R = crate::R<u32, super::DDRPHYC_DTPR0>;
#[doc = "Writer for register DDRPHYC_DTPR0"]
pub type W = crate::W<u32, super::DDRPHYC_DTPR0>;
#[doc = "Register DDRPHYC_DTPR0 `reset()`'s with value 0x3012_666e"]
impl crate::ResetValue for super::DDRPHYC_DTPR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3012_666e
    }
}
#[doc = "Reader of field `TMRD`"]
pub type TMRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRD`"]
pub struct TMRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TRTP`"]
pub type TRTP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRTP`"]
pub struct TRTP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRTP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `TWTR`"]
pub type TWTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TWTR`"]
pub struct TWTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `TRP`"]
pub type TRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRP`"]
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRCD`"]
pub type TRCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRCD`"]
pub struct TRCD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `TRAS`"]
pub type TRAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRAS`"]
pub struct TRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRRD`"]
pub type TRRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRRD`"]
pub struct TRRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | (((value as u32) & 0x0f) << 21);
        self.w
    }
}
#[doc = "Reader of field `TRC`"]
pub type TRC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRC`"]
pub struct TRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 25)) | (((value as u32) & 0x3f) << 25);
        self.w
    }
}
#[doc = "Reader of field `TCCD`"]
pub type TCCD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCCD`"]
pub struct TCCD_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - TMRD"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - TRTP"]
    #[inline(always)]
    pub fn trtp(&self) -> TRTP_R {
        TRTP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 5:7 - TWTR"]
    #[inline(always)]
    pub fn twtr(&self) -> TWTR_R {
        TWTR_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - TRP"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TRCD"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - TRAS"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:24 - TRRD"]
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:30 - TRC"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - TCCD"]
    #[inline(always)]
    pub fn tccd(&self) -> TCCD_R {
        TCCD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TMRD"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W {
        TMRD_W { w: self }
    }
    #[doc = "Bits 2:4 - TRTP"]
    #[inline(always)]
    pub fn trtp(&mut self) -> TRTP_W {
        TRTP_W { w: self }
    }
    #[doc = "Bits 5:7 - TWTR"]
    #[inline(always)]
    pub fn twtr(&mut self) -> TWTR_W {
        TWTR_W { w: self }
    }
    #[doc = "Bits 8:11 - TRP"]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
    #[doc = "Bits 12:15 - TRCD"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W {
        TRCD_W { w: self }
    }
    #[doc = "Bits 16:20 - TRAS"]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W {
        TRAS_W { w: self }
    }
    #[doc = "Bits 21:24 - TRRD"]
    #[inline(always)]
    pub fn trrd(&mut self) -> TRRD_W {
        TRRD_W { w: self }
    }
    #[doc = "Bits 25:30 - TRC"]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W {
        TRC_W { w: self }
    }
    #[doc = "Bit 31 - TCCD"]
    #[inline(always)]
    pub fn tccd(&mut self) -> TCCD_W {
        TCCD_W { w: self }
    }
}
