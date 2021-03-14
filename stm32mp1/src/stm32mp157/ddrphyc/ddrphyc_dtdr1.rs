#[doc = "Reader of register DDRPHYC_DTDR1"]
pub type R = crate::R<u32, super::DDRPHYC_DTDR1>;
#[doc = "Writer for register DDRPHYC_DTDR1"]
pub type W = crate::W<u32, super::DDRPHYC_DTDR1>;
#[doc = "Register DDRPHYC_DTDR1 `reset()`'s with value 0x7788_bb44"]
impl crate::ResetValue for super::DDRPHYC_DTDR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7788_bb44
    }
}
#[doc = "Reader of field `DTBYTE4`"]
pub type DTBYTE4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE4`"]
pub struct DTBYTE4_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DTBYTE5`"]
pub type DTBYTE5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE5`"]
pub struct DTBYTE5_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DTBYTE6`"]
pub type DTBYTE6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE6`"]
pub struct DTBYTE6_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DTBYTE7`"]
pub type DTBYTE7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTBYTE7`"]
pub struct DTBYTE7_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    pub fn dtbyte4(&self) -> DTBYTE4_R {
        DTBYTE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    pub fn dtbyte5(&self) -> DTBYTE5_R {
        DTBYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    pub fn dtbyte6(&self) -> DTBYTE6_R {
        DTBYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    pub fn dtbyte7(&self) -> DTBYTE7_R {
        DTBYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    pub fn dtbyte4(&mut self) -> DTBYTE4_W {
        DTBYTE4_W { w: self }
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    pub fn dtbyte5(&mut self) -> DTBYTE5_W {
        DTBYTE5_W { w: self }
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    pub fn dtbyte6(&mut self) -> DTBYTE6_W {
        DTBYTE6_W { w: self }
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    pub fn dtbyte7(&mut self) -> DTBYTE7_W {
        DTBYTE7_W { w: self }
    }
}
