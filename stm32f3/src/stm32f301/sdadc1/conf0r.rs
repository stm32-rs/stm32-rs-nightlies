#[doc = "Reader of register CONF0R"]
pub type R = crate::R<u32, super::CONF0R>;
#[doc = "Writer for register CONF0R"]
pub type W = crate::W<u32, super::CONF0R>;
#[doc = "Register CONF0R `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COMMON0`"]
pub type COMMON0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMMON0`"]
pub struct COMMON0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Reader of field `SE0`"]
pub type SE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SE0`"]
pub struct SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `GAIN0`"]
pub type GAIN0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAIN0`"]
pub struct GAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `OFFSET0`"]
pub type OFFSET0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSET0`"]
pub struct OFFSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    pub fn common0(&self) -> COMMON0_R {
        COMMON0_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    pub fn offset0(&self) -> OFFSET0_R {
        OFFSET0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    pub fn common0(&mut self) -> COMMON0_W {
        COMMON0_W { w: self }
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    pub fn se0(&mut self) -> SE0_W {
        SE0_W { w: self }
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    pub fn gain0(&mut self) -> GAIN0_W {
        GAIN0_W { w: self }
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    pub fn offset0(&mut self) -> OFFSET0_W {
        OFFSET0_W { w: self }
    }
}
