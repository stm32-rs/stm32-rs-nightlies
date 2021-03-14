#[doc = "Reader of register CHPCR"]
pub type R = crate::R<u32, super::CHPCR>;
#[doc = "Writer for register CHPCR"]
pub type W = crate::W<u32, super::CHPCR>;
#[doc = "Register CHPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STRTPW`"]
pub type STRTPW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STRTPW`"]
pub struct STRTPW_W<'a> {
    w: &'a mut W,
}
impl<'a> STRTPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `CARDTY`"]
pub type CARDTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARDTY`"]
pub struct CARDTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CARDTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CARFRQ`"]
pub type CARFRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CARFRQ`"]
pub struct CARFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CARFRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&self) -> STRTPW_R {
        STRTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn cardty(&self) -> CARDTY_R {
        CARDTY_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn carfrq(&self) -> CARFRQ_R {
        CARFRQ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&mut self) -> STRTPW_W {
        STRTPW_W { w: self }
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn cardty(&mut self) -> CARDTY_W {
        CARDTY_W { w: self }
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn carfrq(&mut self) -> CARFRQ_W {
        CARFRQ_W { w: self }
    }
}
