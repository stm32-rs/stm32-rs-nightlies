#[doc = "Reader of register CHPDR"]
pub type R = crate::R<u32, super::CHPDR>;
#[doc = "Writer for register CHPDR"]
pub type W = crate::W<u32, super::CHPDR>;
#[doc = "Register CHPDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CHPDR {
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | (((value as u32) & 0x0f) << 7);
        self.w
    }
}
#[doc = "Reader of field `CHPDTY`"]
pub type CHPDTY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHPDTY`"]
pub struct CHPDTY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPDTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CHPFRQ`"]
pub type CHPFRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHPFRQ`"]
pub struct CHPFRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPFRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    pub fn chpdty(&self) -> CHPDTY_R {
        CHPDTY_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&self) -> CHPFRQ_R {
        CHPFRQ_R::new((self.bits & 0x0f) as u8)
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
    pub fn chpdty(&mut self) -> CHPDTY_W {
        CHPDTY_W { w: self }
    }
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&mut self) -> CHPFRQ_W {
        CHPFRQ_W { w: self }
    }
}
