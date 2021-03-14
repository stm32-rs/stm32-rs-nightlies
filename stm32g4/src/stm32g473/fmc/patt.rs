#[doc = "Reader of register PATT"]
pub type R = crate::R<u32, super::PATT>;
#[doc = "Writer for register PATT"]
pub type W = crate::W<u32, super::PATT>;
#[doc = "Register PATT `reset()`'s with value 0xfcfc_fcfc"]
impl crate::ResetValue for super::PATT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfcfc_fcfc
    }
}
#[doc = "Reader of field `ATTHIZx`"]
pub type ATTHIZX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTHIZx`"]
pub struct ATTHIZX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHIZX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `ATTHOLDx`"]
pub type ATTHOLDX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTHOLDx`"]
pub struct ATTHOLDX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTHOLDX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `ATTWAITx`"]
pub type ATTWAITX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTWAITx`"]
pub struct ATTWAITX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTWAITX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `ATTSETx`"]
pub type ATTSETX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATTSETx`"]
pub struct ATTSETX_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTSETX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - ATTHIZx"]
    #[inline(always)]
    pub fn atthizx(&mut self) -> ATTHIZX_W {
        ATTHIZX_W { w: self }
    }
    #[doc = "Bits 16:23 - ATTHOLDx"]
    #[inline(always)]
    pub fn attholdx(&mut self) -> ATTHOLDX_W {
        ATTHOLDX_W { w: self }
    }
    #[doc = "Bits 8:15 - ATTWAITx"]
    #[inline(always)]
    pub fn attwaitx(&mut self) -> ATTWAITX_W {
        ATTWAITX_W { w: self }
    }
    #[doc = "Bits 0:7 - ATTSETx"]
    #[inline(always)]
    pub fn attsetx(&mut self) -> ATTSETX_W {
        ATTSETX_W { w: self }
    }
}
